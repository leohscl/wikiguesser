use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PastWordsProps {
    pub past_words: Vec<String>,
}

#[function_component(PastWords)]
pub fn past_words(props: &PastWordsProps) -> Html {
    // display up to 5 words
    let string_to_print = props
        .past_words
        .iter()
        .rev()
        .take(5)
        .fold(String::new(), |a, b| a + &b + "\n");
    html! {
        <aside>
            <p>
            {string_to_print}
            </p>
        </aside>
    }
}
// <table class="word_logs">
//     <tbody >
//         <tr>
//             <td colspan="2" style="background-color:#f6f6f6;color:#000000;">{"Wikipédia"}</td>
//         </tr>
//     <th scope="row"><a title="Slogan">{"Slogan"}</a> </th>
//     <td>{"L'encyclopédie libre"}</td>
//     </tbody>
// </table>
