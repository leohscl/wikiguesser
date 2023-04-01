use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LoadingProps {
    pub percent: u32,
}

#[function_component(LoadingBar)]
pub fn loading_bar(props: &LoadingProps) -> Html {
    let string_percent = props.percent.to_string();
    html! {
        <div class="progress-outer">
            <p> {"Chargement de la page.."} </p>
            <progress id="file" class="progress" value={string_percent.clone()} max="100"> {string_percent + "%"} </progress>
        </div>
    }
}
