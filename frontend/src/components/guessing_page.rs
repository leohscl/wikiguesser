use std::cmp::Ordering;

use yew::prelude::*;
use web_sys::{Event, InputEvent, HtmlInputElement};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use crate::entities::interfaces::Status;
use crate::entities::interfaces::{Article, WordResult};
use crate::service::{articles::get_one_article, words::query, future::handle_future};
use crate::utils::similar_word::same_root;
use crate::entities::interfaces::User;
use super::hidden_field::HiddenField;

//TODO(leo): mettre vert nouveaux mots -- ish
//TODO(leo): Victoire !! -- ADD link to wikipedia ?


#[derive(Debug, Clone, PartialEq, Eq)]
struct StringAndPos {
    str: String,
    pos: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum RevealStrength {
    Revealed,
    VeryClose(StringAndPos),
    Close(StringAndPos),
    Distant(StringAndPos),
    NotRevealed,
}

impl PartialOrd for StringAndPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}
impl Ord for StringAndPos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

type VString = Vec<String>;
type VIndex = Vec<RevealStrength>;

#[derive(PartialEq)]
struct HiddenText {
    is_title: bool,
    text: VString,
    revealed: VIndex,
    new_revelations: VIndex,
    fully_revealed: bool,
}

#[derive(Clone, PartialEq)]
struct Page {
    title: HiddenText,
    content: HiddenText,
    input: String,
}

impl Page {
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

fn render_string(str_to_render: &str, rgb_num: u8, true_word: &str, is_new: bool) -> Html {
    html!{
        <HiddenField 
            hidden_string={true_word.to_string()}
            close_word={str_to_render.to_string()}
            rgb_num={rgb_num}
            is_new={is_new}
        />
    }
}

impl HiddenText {
    fn render(&self) -> Html {
        self.text
            .iter()
            .zip(&self.revealed)
            .zip(&self.new_revelations)
            .map(|((text, revealed), new_reveal)| {
                match new_reveal {
                    RevealStrength::NotRevealed => {
                        match revealed {
                            RevealStrength::Revealed => html!{<span>{text}</span>},
                            RevealStrength::VeryClose(str_pos) => {
                                render_string(&str_pos.str, 230, text, false)
                            },
                            RevealStrength::Close(str_pos)=> {
                                render_string(&str_pos.str, 196, text, false)
                            },
                            RevealStrength::Distant(str_pos) => {
                                render_string(&str_pos.str, 132, text, false)
                            },
                            _ => {
                                render_string("", 0, text, false)
                            }
                        }
                    },
                    RevealStrength::Revealed => {
                        let green_style = format!("background-color: rgb(200, {}, 200);color: rgb(0, {}, 0);", 250, 50);
                        html!{<span style={green_style}> {text}</span>}
                    },
                    RevealStrength::VeryClose(str_pos)=> { render_string(&str_pos.str, 232, text, true) },
                    RevealStrength::Close(str_pos) => { render_string(&str_pos.str, 182, text, true) },
                    RevealStrength::Distant(str_pos) => { render_string(&str_pos.str, 122, text, true) },
                }
            })
            .collect::<Html>()
    }

    fn reveal_all(&mut self) {
        self.revealed = std::iter::repeat(RevealStrength::Revealed).take(self.revealed.len()).collect();
        self.new_revelations = vec![RevealStrength::NotRevealed; self.revealed.len()];
        self.fully_revealed = true;
    }

    fn reveal(&mut self, word_res: &WordResult) -> bool {
        let vec_matches: Vec<_> =
            self.text.clone()
            .into_iter()
            .map(|string_hidden| {
                let string_hidden_lowercase = string_hidden.to_lowercase();
                let word_lowercase = word_res.word.to_lowercase();
                if word_lowercase == string_hidden_lowercase {
                    RevealStrength::Revealed
                } else if same_root(&word_lowercase, &string_hidden_lowercase) {
                    RevealStrength::Revealed
                } else {
                    match word_res.close_words.iter().position(|candidate| candidate.str.to_lowercase() == string_hidden_lowercase) {
                        None => RevealStrength::NotRevealed,
                        Some(position) => {
                            let str_pos = StringAndPos{str:word_lowercase, pos: position};
                            // log::info!("position: {}", position);
                            if position < 10 {
                                RevealStrength::VeryClose(str_pos)
                            } else if position < 100 {
                                RevealStrength::Close(str_pos)
                            } else {
                                RevealStrength::Distant(str_pos)
                            } 
                        },
                    }
                }
            })
            .collect();
        // log::info!("matches: {:?}", vec_matches);
        // log::info!("old_reveal: {:?}", self.revealed);
        let vec_new_revelation: Vec<_> = vec_matches.clone().into_iter()
            .zip(self.revealed.iter())
            .map(|(reveal_new, reveal_old)| {
                if &reveal_new < reveal_old {
                    reveal_new
                } else {
                    RevealStrength::NotRevealed
                }
            })
            .collect();
        let revealed: Vec<RevealStrength> = vec_matches.into_iter()
            .zip(self.revealed.iter())
            .map(|(reveal_new, reveal_old)| {
                if &reveal_new < reveal_old {
                    reveal_new
                } else {
                    reveal_old.clone()
                }
            })
            .collect();
        self.new_revelations = vec_new_revelation;
        let all_revealed = revealed.iter().all(|rev_strength| matches!(rev_strength, RevealStrength::Revealed));
        self.revealed = revealed;
        all_revealed
    }
}

impl Clone for HiddenText {
    fn clone(&self) -> HiddenText {
        HiddenText {
            is_title: self.is_title,
            text: self.text.clone(),
            revealed: self.revealed.clone(),
            new_revelations: self.new_revelations.clone(),
            fully_revealed: self.fully_revealed,
        }
    }
}

impl ToString for HiddenText {
    fn to_string(&self) -> String {
        self.text
            .iter()
            .zip(self.revealed.clone())
            .map(|(text, revealed)| {
                match revealed {
                    RevealStrength::Revealed => text.clone(),
                    _ => std::iter::repeat("*").take(text.len()).collect()
                }
            })
        .collect::<Vec<_>>()
        .join(" ")
    }
}

enum ArticleAction {
    Render(Page),
    SetInput(String),
    Reveal(WordResult),
    RevealAll,
    Reset,
}

#[derive(PartialEq)]
struct ArticleState {
    opt_page: Option<Page>,
    victory: bool,
    num_moves: u32,
    opt_user: Option<User>,
}

impl Default for ArticleState {
    fn default() -> Self {
        Self { 
            opt_page: None,  
            num_moves: 0,
            victory: false,
            opt_user: None,
        }
    }
}

impl Reducible for ArticleState {
    type Action = ArticleAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            ArticleAction::Render(page) => {
                Self { 
                    opt_page: Some(page.clone()),
                    num_moves: 0,
                    victory: false,
                    opt_user: self.opt_user.clone(),
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
                }.into()
            },
            ArticleAction::Reveal(word_res) => {
                // TODO(leo): check word exists
                // TODO(leo): check words close
                let mut page_clone = self.opt_page.clone().expect("There should be a page now..");
                // log::info!("Reveal called !");
                let victory = page_clone.reveal(&word_res);
                page_clone.input = "".to_string();
                Self { 
                    opt_page: Some(page_clone),
                    num_moves: self.num_moves + 1,
                    victory,
                    opt_user: self.opt_user.clone(),
                }.into()
            },
            ArticleAction::RevealAll => {
                let mut page_clone = self.opt_page.clone().expect("There should be a page now..");
                // log::info!("Reveal called !");
                page_clone.reveal_all();
                Self { 
                    opt_page: Some(page_clone),
                    num_moves: self.num_moves,
                    victory: true,
                    opt_user: self.opt_user.clone(),
                }.into()
            },
            ArticleAction::Reset => {
                Self::default().into()
            }
        }
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct GuessingPageProps {
    pub opt_user: Option<User>,
    pub opt_cat: Option<String>,
    pub dummy: bool,
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
                    let future = async move { get_one_article(opt_cat).await };
                    handle_future(future, move |data: Result<Article, Status>| {
                        match data {
                            Ok(article) => {
                                let state = state.clone();
                                let page = page_from_json(article);
                                state.dispatch(ArticleAction::Render(page));
                            }
                            Err(_) => {
                                log::info!("Error loading the data !");
                            },
                        };
                    });
                } else {
                    let state = state.clone();
                    let article = Article { id: 1, wiki_id: 2, title: "thé".to_string(), content: "thé".to_string() };
                    let page = page_from_json(article);
                    state.dispatch(ArticleAction::Render(page));
                }
                || {}
            }
        },
        (),
    );
    let onclick = {
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
            state.dispatch(ArticleAction::RevealAll);
        })
    };

    let onclick_new_page = {
        let state = state.clone();
        let opt_cat = props.opt_cat.clone();
        Callback::from( move |_| {
            state.dispatch(ArticleAction::Reset);
            let opt_cat = opt_cat.clone();
            let state = state.clone();
            let future = async move { get_one_article(opt_cat).await };
            handle_future(future, move |data: Result<Article, Status>| {
                match data {
                    Ok(article) => {
                        let state = state.clone();
                        let page = page_from_json(article);
                        state.dispatch(ArticleAction::Render(page));
                    }
                    Err(_) => {
                        log::info!("Error loading the data !");
                    },
                };
            });
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
            // let new_revelations = &page.new_revelations;
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
            // log::info!("close: {}", num_close);
            html! {
            <div >
                {
                    if victory {
                        let victory_text = format!("Page trouvée en {} coups", state.num_moves); 
                        html! {<span id="victory"> {victory_text} </span>}
                    } else {
                        html!{}
                    }
                }
                <br/>
                <input type="text" value={page.input.clone()} {oninput} {onkeypress} id="input_reveal" name="input_reveal" size=10/>
                {
                    if victory {
                        html! {
                            <button onclick={onclick}>
                                { "Révéler tous les mots" }
                            </button>
                        }
                    } else {
                        html!{}
                    }
                }
                <br/>
                {
                if num_found + num_close == 0 {
                    if state.num_moves != 0 && !page.content.fully_revealed {
                        html!{<span > {red_emo.to_string()}</span>}
                    } else {
                        html!{}
                    }
                } else {
                    html! {<span > {std::iter::repeat(green_emo).take(num_found).chain(std::iter::repeat(orange_emo).take(num_close)).collect::<String>()}</span>}
                }
                }

                <br/>
                <div id="title">
                    { page.title.render() }
                </div>
                <br/>
                <br/>
                <div id="content">
                    { page.content.render() }
                </div>
                {
                    if let Some(_user) = &props.opt_user {
                        // html! {<span > {"User logged in !"}</span>}
                        html!{
                            <button onclick={onclick_like}>
                                { "Like" }
                            </button>
                        }
                    } else {
                        html!{}
                    }
                }
                    //TODO: add confirmation to button
                <button onclick={onclick_give_up}>
                    { "Give up" }
                </button>
                {
                    if victory {
                        html! {
                            <button onclick={onclick_new_page}>
                                { "Try another page !" }
                            </button>
                        }
                    } else {
                        html!{}
                    }
                }
            </div>
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

fn initialize_revealed_vector(vec_text: &VString) -> VIndex {
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
fn create_string_vector(text: String) -> VString {
    // TODO(leo): handle other separators
    let processed_text = text.replace("\n\n\n", "").to_string();
    let processed_text = processed_text.replace("()", "").to_string();
    let separators = [' ', '\'', '.', '(', ')', ',', '!', '?', ';', ':', '/', '§', '%', '*', '€', ']', '[', '-'];
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
            // log::info!("Slices processed, start:{}, end:{}", start, end);
            let chunk = &text[start..end];
            // log::info!("Processed string:{}", chunk);
            let chunk_string = chunk.to_string();
            // log::info!("Processed string:{}", chunk_string);
            chunk_string
        })
        .map(|str| str.to_string())
        .collect()
}
fn trigger_query(state: UseReducerHandle<ArticleState>) {
    if let Some(_page) = &(*state).opt_page {
        let state = state.clone();
        let word = (*state).opt_page.as_ref().expect("There should be a Page").input.clone();
        let future = async move { query(&word.to_lowercase()).await };
        handle_future(future, move |data: Result<WordResult, Status>| {
            match data {
                Ok(word_res) => {
                    let state = state.clone();
                    state.dispatch(ArticleAction::Reveal(word_res));
                }
                Err(_) => {
                    log::info!("Error loading the data !");
                },
            };
        });
    }
}
