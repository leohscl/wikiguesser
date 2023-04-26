use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PastWordsProps {
    pub past_words: Vec<String>,
}

#[function_component(PastWords)]
pub fn past_words(props: &PastWordsProps) -> Html {
    // display up to 5 words
    let iter_string = props.past_words.iter().enumerate().skip(1).rev().take(5);
    log::info!(
        "Queried words: {:?}, len: {}",
        props.past_words,
        props.past_words.len()
    );
    let text_words = "Mots";
    html! {
        <div class="past_word">
            <h4> {text_words}</h4>
            {
                iter_string.map(|(num, word)| {
                    let string_display = num.to_string() + &" " + word;
                    html!{<p class="p_word"> {string_display} </p>}
                }).collect::<Html>()
            }
        </div>
    }
}
