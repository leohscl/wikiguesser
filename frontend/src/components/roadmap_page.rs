// use crate::components::app::Route;
use yew::prelude::*;
// use yew_router::prelude::*;

/*#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeProps {
    pub cb_route: Callback<Route>,
    pub route: Route,
}
*/

/*pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_height(20);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
*/
#[function_component(RoadMap)]
pub fn challenge_page() -> Html {
    /*
    if props.route != Route::ChallengePage {
        props.cb_route.emit(Route::ChallengePage);
    }
    */

    /*let history = use_history().unwrap();
    let onclick_launch_challenge = {
        let history = history.clone();
        Callback::from(move |_| {
            history.push(Route::Challenge {
                opt_str: super::app::StringWrap {
                    cat_or_id: "challenge".to_string(),
                },
            });
        })
    };
    */
    let div_defi_x = html! {
        <div class="roadmap_defi">
            <button>{"X"}</button>
        </div>
    };
    let div_defi_histoire = html! {
        <div class="roadmap_defi">
            <button>{"Histoire"}</button>
        </div>
    };
    let div_defi_geographie = html! {
        <div class="roadmap_defi">
            <button>{"Geographie"}</button>
        </div>
    };

    let div_defi_histoire2 = html! {<div class="roadmap_defi"><button>{"Histoire"}</button></div>};
    let div_defi_x2 = html! {<div class="roadmap_defi"><button>{"X"}</button></div>};

    // let div_fleche_hampe_vertical = html! {<div class="fleche hampe vertical"></div>};
    html! {
        <div class="row">
            <div class="wikitty" title="Page aléatoire">
                <img class="logo" src="https://upload.wikimedia.org/wikipedia/commons/6/61/Wikipedia-logo-transparent.png"/>
            </div>
            <div class="page">
                <h1 class="introduction_title">{"Challenge"}</h1>
                <p class="instructions">
                    {"Effectue des choix et tire ton épingle du jeu avec l'aide des bonus afin de passer la ligne d'arrivée !"}
                </p>
                <div id="roadmap">
                    <div class="roadmap_etape">
                        {div_defi_x}
                        {div_defi_histoire}
                        {div_defi_geographie}
                    </div>
                    <canvas id="canvas" class="arrows"></canvas>
                    <div class="roadmap_etape">
                        {div_defi_x2}
                        {div_defi_histoire2}
                    </div>
                </div>
            </div>
        </div>
    }
}
/*
*/
