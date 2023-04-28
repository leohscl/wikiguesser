use yew::prelude::*;

#[function_component(InformationPage)]
pub fn information_page() -> Html {
    let section_introduction = "Vous trouverez ici les informations par rapport au fonctionnement du site : Sur quels critères
        les pages sont choisies, comment fonctionne la similarité entre les mots, etc..";
    let section_page = "Cette section n'est pas encore prête";
    let section_algorithme = "Cette section n'est pas encore prête";
    html! {
        <div class="row">
            <div class="wikitty" title="Informations">
                <img class="logo" src="https://upload.wikimedia.org/wikipedia/commons/6/61/Wikipedia-logo-transparent.png"/>
            </div>
            <div class="welcome">
                <h1 class="welcome_title">{"Informations"}</h1>
                <p class="text">
                    {section_introduction}
                </p>
                <h1 class="welcome_title">{"Sélection des pages"}</h1>
                <p class="text">
                    {section_page}
                </p>
                <h1 class="welcome_title">{"Similarité"}</h1>
                <p class="text">
                    {section_algorithme}
                </p>
            </div>
        </div>
    }
}
