use crate::components::guessing_page::RevealStrength;
use crate::components::hidden_field::HiddenField;
use crate::entities::interfaces::StringAndPos;
use crate::entities::interfaces::WordResult;
use crate::utils::similar_word::same_root;
use yew::prelude::*;

#[derive(PartialEq)]
pub struct HiddenText {
    pub is_title: bool,
    pub text: Vec<String>,
    pub revealed: Vec<RevealStrength>,
    pub new_revelations: Vec<RevealStrength>,
    pub fully_revealed: bool,
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

impl HiddenText {
    pub fn render(&self) -> Html {
        let html_in = self
            .text
            .iter()
            .zip(&self.revealed)
            .zip(&self.new_revelations)
            .map(|((text, revealed), new_reveal)| {
                if text == &"\n" {
                    html! {<p><div/><div/></p>}
                } else if text == "" {
                    html! {}
                } else {
                    match new_reveal {
                        RevealStrength::NotRevealed => match revealed {
                            RevealStrength::Revealed => html! {<span>{text}</span>},
                            RevealStrength::VeryClose(str_pos) => {
                                render_string(&str_pos.str, 230, text, false)
                            }
                            RevealStrength::Close(str_pos) => {
                                render_string(&str_pos.str, 196, text, false)
                            }
                            RevealStrength::Distant(str_pos) => {
                                render_string(&str_pos.str, 132, text, false)
                            }
                            _ => render_string("", 0, text, false),
                        },
                        RevealStrength::Revealed => {
                            let green_style = format!(
                                "background-color: rgb(100, {}, 100);color: rgb(0, {}, 0);",
                                250, 50
                            );
                            html! {<span style={green_style}> {text}</span>}
                        }
                        RevealStrength::VeryClose(str_pos) => {
                            render_string(&str_pos.str, 232, text, true)
                        }
                        RevealStrength::Close(str_pos) => {
                            render_string(&str_pos.str, 182, text, true)
                        }
                        RevealStrength::Distant(str_pos) => {
                            render_string(&str_pos.str, 122, text, true)
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
                            if position == 0 {
                                RevealStrength::Revealed
                            } else if position < 10 {
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

fn render_string(str_to_render: &str, rgb_num: u8, true_word: &str, is_new: bool) -> Html {
    html! {
        <HiddenField
            hidden_string={true_word.to_string()}
            close_word={str_to_render.to_string()}
            rgb_num={rgb_num}
            is_new={is_new}
        />
    }
}
