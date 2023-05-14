use crate::components::app::Route;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InfoProps {
    pub cb_route: Callback<Route>,
    pub route: Route,
}

#[function_component(InformationPage)]
pub fn information_page(props: &InfoProps) -> Html {
    let section_introduction = "Vous trouverez ici les informations par rapport au fonctionnement du site : Sur quels critères
        les pages sont choisies, comment fonctionne la similarité entre les mots, etc..";
    let section_page = "Cette section n'est pas encore prête";
    let section_algorithme = "Cette section n'est pas encore prête";
    if props.route != Route::Information {
        props.cb_route.emit(Route::Information);
    }
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
