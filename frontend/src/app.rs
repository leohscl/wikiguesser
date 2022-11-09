use yew::prelude::*;
use web_sys::{Event, InputEvent, HtmlInputElement};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use crate::entities::interfaces::Status;
use common::models::{Article, WordResult};
use crate::service::{articles::get_one_article, words::query, future::handle_future};

//TODO(leo): handle féminin/masculin
//TODO(leo): handle pluriel
//TODO(leo): handle numbers
//TODO(leo): nombre lettres
//TODO(leo): gaps should always represent length of hidden number
//TODO(leo): mettre vert nouveaux mots
//TODO(leo): Victoire !!

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct StringAndPos {
    str: String,
    pos: usize,
}

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
enum RevealStrength {
    Revealed,
    VeryClose(StringAndPos),
    Close(StringAndPos),
    Distant(StringAndPos),
    NotRevealed,
}
type VString = Vec<String>;
type VIndex = Vec<RevealStrength>;

#[derive(PartialEq)]
struct HiddenText {
    is_title: bool,
    text: VString,
    revealed: VIndex,
    new_revelations: VIndex,
}



#[derive(Clone, PartialEq)]
struct Page {
    title: HiddenText,
    content: HiddenText,
    input: String,
    victory: bool,
    reveal_called_once: bool,
}

impl Page {
    fn reveal(&mut self, word_res: &WordResult) {
        self.title.reveal(word_res);
        self.content.reveal(word_res);
    }
}

fn render_string(str_to_render: &str, rgb_num: u8, true_word: &str, is_new: bool) -> Html {
    let red = {
        if is_new {
            255
        } else {
            rgb_num
        }
    };
    let green = rgb_num;
    let blue = {
        if is_new {
            0
        } else {
            rgb_num
        }
    };
    let style = format!("background-color: rgb(51, 51, 51);color: rgb({}, {}, {});", red, green, blue);

    let padding = {
        // Add some padding when the true word is a lot bigger
        // than the close word
        if true_word.len()+1 > str_to_render.len() {
            1 + (true_word.len() - str_to_render.len()) / 2 
        } else {
            1
        }
    };
    let string_with_padding = std::iter::repeat('\u{00a0}').take(padding)
        .chain(str_to_render.chars())
        .chain(std::iter::repeat('\u{00a0}').take(padding))
        .collect::<String>();
    html!{
        <span style={style}>
            {string_with_padding}
        </span>
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
                            _ => html!{<span class="w" >{std::iter::repeat('\u{00a0}').take(text.len()).collect::<String>()}</span>},
                        }
                    },
                    RevealStrength::Revealed => {
                        let green_style = format!("background-color: rgb(200, {}, 200);color: rgb(0, {}, 0);", 250, 100);
                        html!{<span style={green_style}> {text}</span>}
                    },
                    RevealStrength::VeryClose(str_pos)=> { render_string(&str_pos.str, 232, text, true) },
                    RevealStrength::Close(str_pos) => { render_string(&str_pos.str, 182, text, true) },
                    RevealStrength::Distant(str_pos) => { render_string(&str_pos.str, 122, text, true) },
                }
            })
            .collect::<Html>()
    }

    fn reveal(&mut self, word_res: &WordResult) {
        let vec_matches: Vec<_> =
            self.text.clone()
            .into_iter()
            .map(|string_hidden| {
                let string_hidden_lowercase = string_hidden.to_lowercase();
                let word_lowercase = word_res.word.to_lowercase();
                if word_lowercase == string_hidden_lowercase {
                    RevealStrength::Revealed
                } else {
                    match word_res.close_words.iter().position(|candidate| candidate.str.to_lowercase() == string_hidden_lowercase) {
                        None => RevealStrength::NotRevealed,
                        Some(position) => {
                            let str_pos = StringAndPos{str:word_lowercase, pos: position};
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
        // log::info!("vec_new_revelation: {:?}", vec_new_revelation);
        let revealed = vec_matches.into_iter()
            .zip(self.revealed.iter())
            .map(|(reveal_new, reveal_old)| {
                if &reveal_new < reveal_old {
                    reveal_new
                } else {
                    reveal_old.clone()
                }
            })
            .collect();
        self.revealed = revealed;
        self.new_revelations = vec_new_revelation;
    }
}

impl Clone for HiddenText {
    fn clone(&self) -> HiddenText {
        HiddenText{is_title: self.is_title, text: self.text.clone(), revealed: self.revealed.clone(), new_revelations: self.new_revelations.clone()}
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
}

#[derive(PartialEq)]
struct ArticleState {
    opt_page: Option<Page>,
}

impl Default for ArticleState {
    fn default() -> Self {
        Self{ opt_page: None }
    }
}

impl Reducible for ArticleState {
    type Action = ArticleAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let new_page = match action {
            ArticleAction::Render(page) => {
                Some(page.clone())
            },
            ArticleAction::SetInput(input) => {
                let mut page_clone = self.opt_page.clone().expect("There should be a page now..");
                page_clone.input = input;
                Some(page_clone)
            },
            ArticleAction::Reveal(word_res) => {
                // TODO(leo): check word exists
                // TODO(leo): check words close
                let mut page_clone = self.opt_page.clone().expect("There should be a page now..");
                // log::info!("Reveal called !");
                page_clone.reveal(&word_res);
                page_clone.input = "".to_string();
                page_clone.reveal_called_once = true;
                Some(page_clone)
            },
        };
        Self { opt_page: new_page }.into()
    }
}
#[derive(Properties, Clone, PartialEq)]
pub struct ArticleProps {
    pub id: i32,
}
#[function_component(App)]
pub fn app() -> Html {

    let state = use_reducer(move || ArticleState{opt_page:None});

    use_effect_with_deps(
        {
            let state = state.clone();
            move |_| {
                let future = async move { get_one_article().await };
                handle_future(future, move |data: Result<Article, Status>| {
                    match data {
                        Ok(article) => {
                            let state = state.clone();
                            // log::info!("Article: {:?}", article);
                            let page = page_from_json(article);
                            state.dispatch(ArticleAction::Render(page));
                        }
                        Err(_) => {
                            log::info!("Error loading the data !");
                        },
                    };
                });
                || {}
            }
        },
        (),
    );

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
                <input type="text" value={page.input.clone()} {oninput} {onkeypress} id="input_reveal" name="input_reveal" size=10/>
                <br/>
                {
                if num_found + num_close == 0 {
                    if page.reveal_called_once {
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
            </div>
            }
        },
    }
}

// <!-- <span class="w"style="background-color: rgb(51, 51, 51); min-width: 35px; color: rgb(255, 191, 0);">&nbsp;le&nbsp; id="4" </span> -->

fn page_from_json(article: Article) -> Page {
    let title = String::from(article.title + " ");
    let content = String::from(article.content + " ");

    // log::info!("Constructing vector title");
    let title_vec = create_string_vector(title);
    // log::info!("Constructing vector content");
    let content_vec = create_string_vector(content);
    // log::info!("Constructing reveal title");
    let revealed_title = initialize_revealed_vector(&title_vec);
    // log::info!("Constructing reveal content");
    let revealed_content = initialize_revealed_vector(&content_vec);
    // log::info!("All constructed");
    // set length = 1 words to Revealed
    let title_vec_len = title_vec.len();
    let content_vec_len = content_vec.len();
    let hidden_title = HiddenText {
        is_title: true,
        text: title_vec,
        revealed: revealed_title,
        new_revelations: vec![RevealStrength::NotRevealed; title_vec_len],
    };
    let hidden_content = HiddenText {
        is_title: false,
        text: content_vec,
        revealed: revealed_content,
        new_revelations: vec![RevealStrength::NotRevealed; content_vec_len],
    };
    Page {
        title: hidden_title,
        content: hidden_content,
        input: "".to_string(),
        reveal_called_once: false,
        victory: false
    }
}
fn initialize_revealed_vector(vec_text: &VString) -> VIndex {
    //TODO(léo): handle all pre_revealed words
    let determinants = vec!["le", "la", "les", "un", "une", "des"];
    let pronoms = vec!["ce", "ces", "de", "du"];
    let avoir_conj = vec!["eu", "aura"];
    let etre_conj = vec!["était", "sera"];
    let conjonction_coord = vec!["et", "en"];
    let pre_revealed: Vec<_> = [determinants,pronoms,avoir_conj,etre_conj, conjonction_coord].concat();
    vec_text
        .iter()
        .map(|str| {
            match str.chars().count() {
                1 => RevealStrength::Revealed,
                _ => {
                    if let Some(_) = pre_revealed.iter().position(|candidate| candidate == str) {
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
    let separators = [' ', '\'', '.', '(', ')', ',', '!', '?', ';', ':', '/', '§', '%', '*', '€', ']', '[', '-'];
    let separator_indexes: Vec<_> = [0].into_iter().chain(
        text
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
    // log::info!("Constructed separators: {:?}", separator_indexes);
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
                    // log::info!("query result: {:?}", word_res);
                    state.dispatch(ArticleAction::Reveal(word_res));
                }
                Err(_) => {
                    log::info!("Error loading the data !");
                },
            };
        });
    }
}
