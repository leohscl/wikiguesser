use crate::components::hidden_field::Category;
use crate::components::hidden_field::HiddenField;
use crate::entities::interfaces::StringAndPos;
use crate::entities::interfaces::WordResult;
use crate::utils::reveal_strength::RevealStrength;
use crate::utils::similar_word::get_class_variants;
use crate::utils::similar_word::same_class;
use crate::utils::similar_word::same_root;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct HiddenText {
    pub is_title: bool,
    pub text: Vec<String>,
    pub revealed: Vec<RevealStrength>,
    pub new_revelations: Vec<RevealStrength>,
    pub categories: Vec<Category>,
    pub fully_revealed: bool,
}

#[derive(PartialEq, Clone)]
pub enum RevealState {
    Plain,
    NewReveal,
    NewHint(u8),
    OldHint(u8),
    Hidden,
}

impl RevealState {
    pub fn get_class(&self) -> String {
        match self {
            RevealState::Plain => "revealed".to_string(),
            RevealState::NewReveal => "hidden_revealed".to_string(),
            _ => "hidden_field".to_string(),
        }
    }
    pub fn get_style(&self) -> String {
        match self {
            // RevealState::Plain => format!("color: rgb(255, 255, 255);"),
            RevealState::Plain => format!("color: rgb(0, 0, 0);"),
            RevealState::NewReveal => format!("color: rgb(0, 0, 0);"),
            RevealState::NewHint(code) => format!("color: rgb(255, {}, 0);", code),
            RevealState::OldHint(code) => format!("color: rgb({}, {}, {});", code, code, code),
            RevealState::Hidden => format!(""),
        }
    }
}

impl HiddenText {
    pub fn render(self) -> Html {
        let html_in = self
            .text
            .iter()
            .zip(&self.revealed)
            .zip(&self.new_revelations)
            .zip(self.categories)
            .map(|(((text, revealed), new_reveal), cat)| {
                if text == &"\n" {
                    html! {<p><div/><div/></p>}
                } else if text == "" {
                    html! {}
                } else {
                    match new_reveal {
                        RevealStrength::NotRevealed => match revealed {
                            RevealStrength::Revealed => {
                                let style = RevealState::Plain;
                                render_string(text, text, style, cat)
                            }
                            RevealStrength::VeryClose(str_pos) => {
                                let style = RevealState::OldHint(230);
                                render_string(text, &str_pos.str, style, cat)
                            }
                            RevealStrength::Close(str_pos) => {
                                let style = RevealState::OldHint(196);
                                render_string(text, &str_pos.str, style, cat)
                            }
                            RevealStrength::Distant(str_pos) => {
                                let style = RevealState::OldHint(132);
                                render_string(text, &str_pos.str, style, cat)
                            }
                            RevealStrength::NotRevealed => {
                                let style = RevealState::Hidden;
                                render_string(text, "", style, cat)
                            }
                        },
                        RevealStrength::Revealed => {
                            let style = RevealState::NewReveal;
                            render_string(text, "", style, cat)
                        }
                        RevealStrength::VeryClose(str_pos) => {
                            let style = RevealState::NewHint(230);
                            render_string(text, &str_pos.str, style, cat)
                        }
                        RevealStrength::Close(str_pos) => {
                            let style = RevealState::NewHint(180);
                            render_string(text, &str_pos.str, style, cat)
                        }
                        RevealStrength::Distant(str_pos) => {
                            let style = RevealState::NewHint(122);
                            render_string(text, &str_pos.str, style, cat)
                        }
                    }
                }
            })
            .collect::<Html>();
        if self.is_title {
            html! {
                <h2 class="title">
                    {html_in}
                </h2>
            }
        } else {
            html! {
                <p align="justified" >
                    {html_in}
                </p>
            }
        }
    }

    pub fn reveal_all(&mut self) {
        self.revealed = std::iter::repeat(RevealStrength::Revealed)
            .take(self.revealed.len())
            .collect();
        self.new_revelations = vec![RevealStrength::NotRevealed; self.revealed.len()];
        self.fully_revealed = true;
    }
    pub fn get_word_match_with_engine(
        &mut self,
        word: &str,
        result_engine: &Vec<StringAndPos>,
    ) -> Vec<RevealStrength> {
        let word_class_variants = get_class_variants(word);
        self.text
            .clone()
            .into_iter()
            .map(|string_hidden| {
                let string_hidden_lowercase = string_hidden.to_lowercase();
                let word_lowercase = word.to_lowercase();
                if word_lowercase == string_hidden_lowercase {
                    RevealStrength::Revealed
                } else if same_root(&word_lowercase, &string_hidden_lowercase) {
                    RevealStrength::Revealed
                } else if same_class(&word_class_variants, &string_hidden_lowercase) {
                    RevealStrength::Revealed
                } else {
                    let opt_index = result_engine
                        .iter()
                        .position(|str_and_pos| str_and_pos.str == string_hidden);
                    match opt_index {
                        None => RevealStrength::NotRevealed,
                        Some(index) => {
                            let mut str_pos = result_engine[index].clone();
                            str_pos.str = word.to_string();
                            let position = str_pos.pos;
                            // if it is a variant
                            match position {
                                0 => RevealStrength::Revealed,
                                1..=9 => RevealStrength::VeryClose(str_pos),
                                10..=100 => RevealStrength::Close(str_pos),
                                _ => RevealStrength::Distant(str_pos),
                            }
                        }
                    }
                }
            })
            .collect()
    }

    pub fn reveal_with_engine(&mut self, word: &str, result_engine: &Vec<StringAndPos>) -> bool {
        let vec_matches = self.get_word_match_with_engine(word, result_engine);
        self.update_revelations(&vec_matches)
    }

    pub fn get_word_match(&mut self, word_res: &WordResult) -> Vec<RevealStrength> {
        self.text
            .clone()
            .into_iter()
            .map(|string_hidden| {
                let string_hidden_lowercase = string_hidden.to_lowercase();
                let word_lowercase = word_res.word.to_lowercase();
                if word_lowercase == string_hidden_lowercase {
                    RevealStrength::Revealed
                } else if same_root(&word_lowercase, &string_hidden_lowercase) {
                    RevealStrength::Revealed
                } else {
                    match word_res.close_words.iter().position(|candidate| {
                        candidate.str.to_lowercase() == string_hidden_lowercase
                    }) {
                        None => RevealStrength::NotRevealed,
                        Some(position) => {
                            let str_pos = StringAndPos {
                                str: word_lowercase,
                                pos: position,
                            };
                            // log::info!("position: {}", position);
                            if position < 10 {
                                RevealStrength::VeryClose(str_pos)
                            } else if position < 100 {
                                RevealStrength::Close(str_pos)
                            } else {
                                RevealStrength::Distant(str_pos)
                            }
                        }
                    }
                }
            })
            .collect()
    }
    pub fn update_revelations(&mut self, vec_matches: &Vec<RevealStrength>) -> bool {
        let vec_new_revelation: Vec<_> = vec_matches
            .clone()
            .into_iter()
            .zip(self.revealed.iter())
            .map(|(reveal_new, reveal_old)| {
                if &reveal_new <= reveal_old {
                    reveal_new
                } else {
                    RevealStrength::NotRevealed
                }
            })
            .collect();
        let revealed: Vec<RevealStrength> = vec_matches
            .into_iter()
            .zip(self.revealed.iter())
            .map(|(reveal_new, reveal_old)| {
                if reveal_new <= reveal_old {
                    reveal_new.clone()
                } else {
                    reveal_old.clone()
                }
            })
            .collect();
        self.new_revelations = vec_new_revelation;
        let all_revealed = revealed
            .iter()
            .all(|rev_strength| matches!(rev_strength, RevealStrength::Revealed));
        self.revealed = revealed;
        all_revealed
    }
    pub fn reveal(&mut self, word_res: &WordResult) -> bool {
        let vec_matches = self.get_word_match(word_res);
        self.update_revelations(&vec_matches)
    }
}

impl ToString for HiddenText {
    fn to_string(&self) -> String {
        self.text
            .iter()
            .zip(self.revealed.clone())
            .map(|(text, revealed)| match revealed {
                RevealStrength::Revealed => text.clone(),
                _ => std::iter::repeat("*").take(text.len()).collect(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn render_string(
    true_word: &str,
    str_to_render: &str,
    reveal_state: RevealState,
    category: Category,
) -> Html {
    html! {
        <HiddenField
            hidden_string={true_word.to_string()}
            reveal_state={reveal_state}
            close_word={str_to_render.to_string()}
            category={category}
        />
    }
}
