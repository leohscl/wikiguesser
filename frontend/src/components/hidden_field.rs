use gloo::timers::callback::Timeout;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HiddenFieldProps {
    pub hidden_string: String,
    pub close_word: String,
    pub rgb_num: u8,
    pub is_new: bool,
    pub is_revealed: bool,
}

struct HiddenFieldState {
    show_num: bool,
    _timeout: Option<Timeout>,
}

impl HiddenFieldState {
    fn new() -> Self {
        Self {
            show_num: false,
            _timeout: None,
        }
    }
}

#[function_component(HiddenField)]
pub fn hidden_field(props: &HiddenFieldProps) -> Html {
    let state = use_state(|| HiddenFieldState::new());
    let is_revealed = props.is_revealed;
    let style_string = if is_revealed {
        format!("color: rgb({}, {}, {});", 0, 0, 0)
    } else {
        get_style(&props)
    };
    let style_number = format!("color: rgb({}, {}, {});", 255, 150, 150);
    let string_with_padding = get_string_with_padding(props);
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let handle = {
                let state = state.clone();
                Timeout::new(2000, move || {
                    state.set(HiddenFieldState {
                        show_num: false,
                        _timeout: None,
                    });
                })
            };
            state.set(HiddenFieldState {
                show_num: true,
                _timeout: Some(handle),
            });
        })
    };
    let string_reveal = get_number_with_padding(
        props.hidden_string.chars().count(),
        string_with_padding.chars().count(),
    );
    let hidden_class = if is_revealed {
        "hidden_revealed"
    } else {
        "hidden_field"
    };
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

fn get_style(props: &HiddenFieldProps) -> String {
    let red = {
        if props.is_new {
            255
        } else {
            props.rgb_num
        }
    };
    let green = props.rgb_num;
    let blue = {
        if props.is_new {
            0
        } else {
            props.rgb_num
        }
    };
    format!("color: rgb({}, {}, {});", red, green, blue)
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
    if props.is_revealed {
        let padding = 1;
        std::iter::repeat('\u{00a0}')
            .take(padding)
            .chain(props.hidden_string.chars())
            .chain(std::iter::repeat('\u{00a0}').take(padding))
            .collect::<String>()
    } else {
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
