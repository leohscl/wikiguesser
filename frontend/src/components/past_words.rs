use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PastWordsProps {
    pub past_words: Vec<String>,
}

#[function_component(PastWords)]
pub fn past_words(props: &PastWordsProps) -> Html {
    // display up to 5 words
    let iter_string = props.past_words.iter().rev().take(5);
    html! {
        <div class="past_word">
            <h3> {"Mots"}</h3>
            {
                iter_string.map(|word| {
                    html!{<p> {word} </p>}
                }).collect::<Html>()
            }
        </div>
    }
}
