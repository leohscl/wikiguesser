use gloo::timers::callback::Timeout;
use yew::prelude::*;

use crate::entities::hidden_text::RevealState;

#[derive(PartialEq, Clone)]
pub enum Category {
    Normal,
    Important,
}

#[derive(PartialEq, Properties)]
pub struct HiddenFieldProps {
    pub hidden_string: String,
    pub close_word: String,
    pub reveal_state: RevealState,
    pub category: Category,
}

struct HiddenFieldState {
    show_num: bool,
}

impl HiddenFieldState {
    fn new() -> Self {
        Self { show_num: false }
    }
}

#[function_component(HiddenField)]
pub fn hidden_field(props: &HiddenFieldProps) -> Html {
    let state = use_state(|| HiddenFieldState::new());
    let style_string = props.reveal_state.get_style();
    let style_number = format!("color: rgb({}, {}, {});", 255, 150, 150);
    let string_with_padding = get_string_with_padding(props);
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let _ = {
                let state = state.clone();
                Timeout::new(2000, move || {
                    state.set(HiddenFieldState { show_num: false });
                })
            };
            state.set(HiddenFieldState { show_num: true });
        })
    };
    let string_reveal = get_number_with_padding(
        props.hidden_string.chars().count(),
        string_with_padding.chars().count(),
    );
    let hidden_class = props.reveal_state.get_class();
    html! {
        <span
        class={hidden_class}
        style={
            if state.show_num {
                style_number
            } else {
                style_string
            }
        }
        onclick={onclick}>
            {
                if state.show_num {
                    string_reveal
                } else {
                    string_with_padding
                }
            }
        </span>
    }
}

fn get_number_with_padding(hidden_word_length: usize, str_with_padding_size: usize) -> String {
    let size_number = hidden_word_length.to_string().chars().count();
    let size_blank = str_with_padding_size - size_number;
    if size_blank % 2 == 0 {
        std::iter::repeat('\u{00a0}')
            .take(size_blank / 2)
            .chain(hidden_word_length.to_string().chars())
            .chain(std::iter::repeat('\u{00a0}').take(size_blank / 2))
            .collect::<String>()
    } else {
        std::iter::repeat('\u{00a0}')
            .take(size_blank / 2)
            .chain(hidden_word_length.to_string().chars())
            .chain(std::iter::repeat('\u{00a0}').take(size_blank / 2 + 1))
            .collect::<String>()
    }
}

fn get_string_with_padding(props: &HiddenFieldProps) -> String {
    let len_hidden = props.hidden_string.len();
    let len_close = props.close_word.len();
    let str_to_render = &props.close_word;
    match props.reveal_state {
        RevealState::Plain => str_to_render.to_string(),
        RevealState::NewReveal => {
            let padding = 1;
            std::iter::repeat('\u{00a0}')
                .take(padding)
                .chain(props.hidden_string.chars())
                .chain(std::iter::repeat('\u{00a0}').take(padding))
                .collect::<String>()
        }
        _ => {
            if str_to_render == "" {
                std::iter::repeat('\u{00a0}')
                    .take(len_hidden * 2)
                    .collect::<String>()
            } else {
                let padding = {
                    // Add some padding when the true word is a lot bigger
                    // than the close word
                    if len_hidden + 1 > len_close {
                        1 + (len_hidden - len_close) / 2
                    } else {
                        1
                    }
                };
                std::iter::repeat('\u{00a0}')
                    .take(padding)
                    .chain(str_to_render.chars())
                    .chain(std::iter::repeat('\u{00a0}').take(padding))
                    .collect::<String>()
            }
        }
    }
}
