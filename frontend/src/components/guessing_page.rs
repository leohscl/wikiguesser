use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{Event, InputEvent, HtmlInputElement};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use crate::entities::interfaces::{Status, OngoingGame, Game, GameEngine, StringAndPos};
use crate::entities::interfaces::{Article, WordResult};
use crate::service::games::{get_game, self, finish_game};
use crate::service::articles::get_engine;
use crate::service::future::handle_future;
use crate::entities::interfaces::User;
use crate::entities::hidden_text::HiddenText;
use super::app::Route;
use super::rating::Rating;
use gloo::dialogs::confirm;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RevealStrength {
    Revealed,
    VeryClose(StringAndPos),
    Close(StringAndPos),
    Distant(StringAndPos),
    NotRevealed,
}

#[derive(Clone, PartialEq)]
struct Page {
    title: HiddenText,
    content: HiddenText,
    input: String,
}

impl Page {
    fn reveal_with_engine(&mut self, word: &str, result_engine: &Vec<StringAndPos>) -> bool {
        let title_fully_revealed = self.title.reveal_with_engine(word, result_engine);
        self.content.reveal_with_engine(word, result_engine);
        title_fully_revealed
    }
    fn reveal(&mut self, word_res: &WordResult) -> bool {
        let title_fully_revealed = self.title.reveal(word_res);
        self.content.reveal(word_res);
        title_fully_revealed
    }
    fn reveal_all(&mut self) {
        self.title.reveal_all();
        self.content.reveal_all();
    }
}





enum ArticleAction {
    Render(Page, Option<OngoingGame>, Option<GameEngine>),
    SetInput(String),
    _Reveal(WordResult),
    RevealWithEngine(String),
    RevealAll,
}

#[derive(PartialEq)]
struct ArticleState {
    opt_page: Option<Page>,
    victory: bool,
    num_moves: u32,
    opt_user: Option<User>,
    opt_game: Option<OngoingGame>,
    opt_engine: Option<GameEngine>,
}

impl Default for ArticleState {
    fn default() -> Self {
        Self { 
            opt_page: None,  
            num_moves: 0,
            victory: false,
            opt_user: None,
            opt_game: None,
            opt_engine: None,
        }
    }
}

impl Reducible for ArticleState {
    type Action = ArticleAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            ArticleAction::Render(page, opt_game, opt_engine) => {
                Self { 
                    opt_page: Some(page.clone()),
                    num_moves: 0,
                    victory: false,
                    opt_user: self.opt_user.clone(),
                    opt_game,
                    opt_engine,
                }.into()
            },
            ArticleAction::SetInput(input) => {
                let mut page_clone = self.opt_page.clone().expect("There should be a page now..");
                page_clone.input = input;
                Self { 
                    opt_page: Some(page_clone),
                    num_moves: self.num_moves,
                    victory: self.victory,
                    opt_user: self.opt_user.clone(),
                    opt_game: self.opt_game.clone(),
                    opt_engine: self.opt_engine.clone(),
                }.into()
            },
            ArticleAction::RevealWithEngine(word) => {
                let mut page_clone = self.opt_page.clone().expect("There should be a page now..");
                let engine = self.opt_engine.clone().expect("There should be an engine now..");
                let empty_vec = Vec::new();
                let (result, num_moves) = if let Some(res) = engine.reveals.get(&word) {
                    (res, self.num_moves + 1)
                } else {
                    (&empty_vec, self.num_moves)
                };
                let victory = page_clone.reveal_with_engine(&word, result);
                if victory {
                    if let Some(ongoing_game) = self.opt_game.clone() {
                        let future = async move { finish_game(ongoing_game.game.id).await };
                        handle_future(future, move |data: Result<Game, Status>| {
                            match data {
                                Ok(game) => {
                                    log::info!("Game finished: {:?}", game);
                                }
                                Err(_) => {
                                    log::info!("Error loading the data !");
                                },
                            };
                        });
                    }
                }
                page_clone.input = "".to_string();
                Self { 
                    opt_page: Some(page_clone),
                    num_moves,
                    victory,
                    opt_user: self.opt_user.clone(),
                    opt_game: self.opt_game.clone(),
                    opt_engine: self.opt_engine.clone(),
                }.into()
            },
            ArticleAction::_Reveal(word_res) => {
                let mut page_clone = self.opt_page.clone().expect("There should be a page now..");
                let victory = page_clone.reveal(&word_res);
                if victory {
                    if let Some(ongoing_game) = self.opt_game.clone() {
                        let future = async move { finish_game(ongoing_game.game.id).await };
                        handle_future(future, move |data: Result<Game, Status>| {
                            match data {
                                Ok(game) => {
                                    log::info!("Game finished: {:?}", game);
                                }
                                Err(_) => {
                                    log::info!("Error loading the data !");
                                },
                            };
                        });
                    }
                }
                page_clone.input = "".to_string();
                Self { 
                    opt_page: Some(page_clone),
                    num_moves: self.num_moves + 1,
                    victory,
                    opt_user: self.opt_user.clone(),
                    opt_game: self.opt_game.clone(),
                    opt_engine: self.opt_engine.clone(),
                }.into()
            },
            ArticleAction::RevealAll => {
                let mut page_clone = self.opt_page.clone().expect("There should be a page now..");
                page_clone.reveal_all();
                if let Some(ongoing_game) = self.opt_game.clone() {
                    let future = async move { finish_game(ongoing_game.game.id).await };
                    handle_future(future, move |data: Result<Game, Status>| {
                        match data {
                            Ok(game) => {
                                log::info!("Game finished: {:?}", game);
                            }
                            Err(_) => {
                                log::info!("Error loading the data !");
                            },
                        };
                    });
                }
                Self { 
                    opt_page: Some(page_clone),
                    num_moves: self.num_moves,
                    victory: true,
                    opt_user: self.opt_user.clone(),
                    opt_game: self.opt_game.clone(),
                    opt_engine: None,
                }.into()
            },
        }
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct GuessingPageProps {
    pub opt_user: Option<User>,
    pub opt_cat: Option<String>,
    pub dummy: bool,
}

// Use macro to simplify html
macro_rules! ifcond {
    ($cond:expr, $html_vic:expr) => {
        {
            if $cond {
                $html_vic
            } else {
                html!{}
            }
        }
    };
}

#[function_component(GuessingPage)]
pub fn guessing_page(props: &GuessingPageProps) -> Html {
    let state = use_reducer(move || ArticleState::default());

    use_effect_with_deps(
        {
            let dummy = props.dummy;
            let opt_cat = props.opt_cat.clone();
            let state = state.clone();
            move |_| {
                if !dummy {
                    // let future = async move { get_one_article(opt_cat).await };
                    let state = state.clone();
                    let future = async move { get_game(opt_cat).await };
                    handle_future(future, move |data: Result<OngoingGame, Status>| {
                        match data {
                            Ok(ongoing_game) => {
                                let state_1 = state.clone();
                                let article = ongoing_game.article.clone();
                                let page = page_from_json(article);
                                let article_id = ongoing_game.article.id;
                                let future = async move { get_engine(article_id).await };
                                let state = state.clone();
                                let all_results = ongoing_game.all_results.clone();
                                handle_future(future, move |data: Result<GameEngine, Status>| {
                                    match data {
                                        Ok(game_engine) => {
                                            log::info!("Game engine loaded: {:?}", game_engine);
                                            state_1.dispatch(ArticleAction::Render(page.clone(), Some(ongoing_game.clone()), Some(game_engine)));
                                            for opt_res in all_results.clone().into_iter() {
                                                if let Some(word_res) = opt_res {
                                                    state.dispatch(ArticleAction::RevealWithEngine(word_res.word));
                                                    // state.dispatch(ArticleAction::Reveal(word_res));
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            log::info!("Error loading game engine!");
                                        },
                                    };
                                });
                            }
                            Err(_) => {
                                log::info!("Error loading the data !");
                            },
                        };
                    });
                } else {
                    let state = state.clone();
                    let article = Article { id: 1, wiki_id: 2, title: "thé".to_string(), content: "thé".to_string(), views: 0 };
                    let page = page_from_json(article);
                    state.dispatch(ArticleAction::Render(page, None, None));
                }
                || {}
            }
        },
        (),
    );
    let onclick_reveal_all = {
        let state = state.clone();
        Callback::from( move |_| {
            state.dispatch(ArticleAction::RevealAll);
        })
    };

    let onclick_like = {
        let _state = state.clone();
        Callback::from( move |_| {
            log::info!("Click from like button !");
        })
    };

    let onclick_give_up = {
        let state = state.clone();
        Callback::from( move |_| {
            if confirm("Are you sure you want to give up ?") {
                state.dispatch(ArticleAction::RevealAll);
            }
        })
    };

    let history = use_history().unwrap();
    let onclick_report_page = {
        let state = state.clone();
        let history = history.clone();
        Callback::from( move |_| {
            if let Some(ongoing_game) = &state.opt_game {
                history.push(Route::ReportForm{ article_id: ongoing_game.article.id });
            }
        })
    };

    let onclick_new_page = {
        Callback::from( move |_| {
            history.push(Route::LaunchPage);
        })
    };

    let oninput = {
        let state = state.clone();
        Callback::from( move |input_event: InputEvent| {
            let event: Event = input_event.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            let value = target.value();
            state.dispatch(ArticleAction::SetInput(value));
        })
    };
    let onkeypress = {
        let state = state.clone();
        Callback::from( move |keydown_event: KeyboardEvent| {
            let state = state.clone();
            if keydown_event.key() == "Enter" {
                trigger_query(state);
            }
        })
    };
    let green_emo = '🟩';
    let orange_emo = '🟧';
    let red_emo = '🟥';
    let victory = state.victory;
    match &(*state.clone()).opt_page {
        None => html!{<span>{"Chargement.."}</span>},
        Some(page) => {
            let views_string = if let Some(game) = &state.opt_game {
                log::info!("Views found");
                let daily_views = game.article.views / 30;
                "daily views: ".to_string() + &daily_views.to_string()
            } else {
                "".to_string()
            };
            let content_new = &page.content.new_revelations;
            let mut num_found = 0;
            let mut num_close = 0;
            for elt in content_new.iter() {
                match elt {
                    RevealStrength::Revealed => num_found+=1,
                    RevealStrength::NotRevealed => (),
                    _ => num_close+=1,
                }
            }
            html! {
                <p align="justified" class="content">
                    {
                        ifcond!(
                            victory,
                            {
                                let victory_text = format!("Page trouvée en {} coups", state.num_moves);
                                html! {<span id="victory"> {victory_text} </span>}
                            }
                        )
                    }
                    <div/>
                    <input type="text" value={page.input.clone()} {oninput} {onkeypress} id="input_reveal" name="input_reveal" size=10/>
                    {
                        ifcond!(
                            victory,
                            html! { <button onclick={onclick_reveal_all}> { "Révéler tous les mots" } </button> }
                        )
                    }
                    <div/>
                    {
                    if num_found + num_close == 0 {
                        if state.num_moves != 0 && !page.content.fully_revealed {
                            html!{<span > {red_emo.to_string()} </span>}
                        } else {
                            html!{}
                        }
                    } else {
                        html! {<span > {std::iter::repeat(green_emo).take(num_found).chain(std::iter::repeat(orange_emo).take(num_close)).collect::<String>()}</span>}
                    }
                    }

                    <div/>
                    <div id="title">
                        { page.title.render() }
                    </div>
                    <div id="content" class="content">
                        { page.content.render() }
                    </div>
                    {
                        if let Some(_user) = &props.opt_user {
                            html!{
                                <button onclick={onclick_like}>
                                    { "Like" }
                                </button>
                            }
                        } else {
                            html!{}
                        }
                    }
                    {
                        ifcond!(
                            !victory,
                            html! { <button onclick={onclick_give_up}> { "Give up" } </button> }
                        )
                    }
                    {
                        ifcond!(
                            victory,
                            html! { <button onclick={onclick_report_page}> { "Report an issue" } </button> }
                        )
                    }
                    {
                        ifcond!(
                            victory,
                            html! { <button onclick={onclick_new_page}> { "Try another page !" } </button> }
                        )
                    }
                    {
                        {
                            let html_rating = if let Some(ongoing_game) = &state.opt_game {
                                let article_id = ongoing_game.article.id;
                                html! {
                                    <Rating {article_id}/>
                                }
                            } else {
                                html!{}
                            };
                            ifcond!(victory, html_rating)
                        }
                    }
                    {
                        ifcond!(victory, html! { <b> {views_string}</b> })
                    }
                </p>
            }
        },
    }
}


fn page_from_json(article: Article) -> Page {
    let title = String::from(article.title + " ");
    let content = String::from(article.content + " ");

    let title_vec = create_string_vector(title);
    let content_vec = create_string_vector(content);
    let revealed_title = initialize_revealed_vector(&title_vec);
    let revealed_content = initialize_revealed_vector(&content_vec);
    let title_vec_len = title_vec.len();
    let content_vec_len = content_vec.len();
    let hidden_title = HiddenText {
        is_title: true,
        text: title_vec,
        revealed: revealed_title,
        new_revelations: vec![RevealStrength::NotRevealed; title_vec_len],
        fully_revealed: false,
    };
    let hidden_content = HiddenText {
        is_title: false,
        text: content_vec,
        revealed: revealed_content,
        new_revelations: vec![RevealStrength::NotRevealed; content_vec_len],
        fully_revealed: false,
    };
    Page {
        title: hidden_title,
        content: hidden_content,
        input: "".to_string(),
    }
}

fn initialize_revealed_vector(vec_text: &Vec<String>) -> Vec<RevealStrength> {
    //TODO(léo): handle all pre_revealed words
    let determinants = vec!["le", "la", "les", "un", "une", "des"];
    let pronoms = vec!["ce", "ces", "de", "du"];
    let avoir_conj = vec!["eu", "aura", "a"];
    let etre_conj = vec!["était", "sera", "est"];
    let conjonction_coord = vec!["et", "en"];
    let pre_revealed: Vec<_> = [determinants, pronoms, avoir_conj, etre_conj, conjonction_coord].concat();
    vec_text
        .iter()
        .map(|str| {
            match str.chars().count() <= 1 {
                true => RevealStrength::Revealed,
                false => {
                    if let Some(_) = pre_revealed.iter().position(|candidate| candidate.to_lowercase() == str.to_lowercase()) {
                        RevealStrength::Revealed
                    } else {
                        RevealStrength::NotRevealed
                    }
                }
            }
        })
        .collect()
}
fn create_string_vector(text: String) -> Vec<String> {
    let processed_text = text.replace("\n\n\n", "").to_string();
    let processed_text = processed_text.replace("()", "").to_string();
    let separators = [' ', '\'', '.', '(', ')', ',', '!', '?', ';', ':', '/', '§', '%', '*', '€', ']', '[', '-', '\n'];
    let separator_indexes: Vec<_> = [0].into_iter().chain(
        processed_text
        .char_indices()
        .filter_map(|(index, char)| {
            match separators.iter().find(|c| *c == &char) {
                Some(_) => {
                    Some([index, index+1])
                },
                None => None,
            }
        })
        .flatten())
        .collect();
    separator_indexes
        .windows(2)
        .map(|slice| {
            let start = *slice.get(0).expect("slice should have 2 elements");
            let end = *slice.get(1).expect("slice should have 2 elements");
            let chunk = &text[start..end];
            let chunk_string = chunk.to_string();
            chunk_string
        })
        .map(|str| str.to_string())
        .collect()
}
fn trigger_query(state: UseReducerHandle<ArticleState>) {
    if let Some(_page) = &(*state).opt_page {
        let state = state.clone();
        let word = (*state).opt_page.as_ref().expect("There should be a Page").input.clone();
        state.dispatch(ArticleAction::RevealWithEngine(word.clone()));
        // let future = async move { query(&word.to_lowercase()).await };
        let ongoing_game = state.opt_game.clone().expect("There should be a game");
        let future = async move { games::update_game(ongoing_game.game.id, &word.to_lowercase()).await };
        handle_future(future, move |data: Result<Option<WordResult>, Status>| {
            match data {
                Ok(opt_word_res) => {
                    // let state = state.clone();
                    if let Some(_word_res) = opt_word_res {
                        // state.dispatch(ArticleAction::Reveal(word_res));
                    }
                }
                Err(_) => {
                    log::info!("Error loading the data !");
                },
            };
        });
    }
}
