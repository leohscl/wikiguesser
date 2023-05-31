// use crate::components::app::Route;

use std::f64::consts::PI;

use gloo_console::log;
use sorted_vec::partial::SortedSet;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, Path2d};
use yew::prelude::*;
use yew_canvas::{Canvas, WithRander};
// use yew_router::prelude::*;

/*#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeProps {
    pub cb_route: Callback<Route>,
    pub route: Route,
}
*/

fn get_x_value_for_arrow(element: &Element) -> f64 {
    let bounding_rect = element.get_bounding_client_rect(); //get_bounding_client_rect();

    let x_start_arrow = bounding_rect.x() + bounding_rect.width() / 2.0;
    /*
    log!(
        "x position : {} rect width : {} x_of_arrow : {}",
        bounding_rect.x(),
        bounding_rect.width(),
        x_start_arrow
    );
    */
    x_start_arrow
}
fn _create_arrow_straight(path: &Path2d, x_from: f64, y_from: f64, x_to: f64, y_to: f64) {
    log!(
        "x_from: {}, y_from: {}, x_to: {}, y_to: {}",
        x_from,
        y_from,
        x_to,
        y_to
    );

    path.move_to(x_from, y_from);
    path.line_to(x_to, y_to);
    let angle = { y_to - y_from }.atan2(x_to - x_from);
    let arrow_end_size = 19.0;
    let angle_offset_modifier = (PI / 2.0 - { angle + PI / 2.0 }.abs()).sqrt();
    log!(format!("angle_offset_modifier {}", angle_offset_modifier));
    let angle_offset = (PI / 15.0) + angle_offset_modifier / 4.0;
    log!(format!("angle_offset {}", angle_offset));
    log!(format!("angle : {}", angle));
    log!(format!(
        "x_off : {}",
        arrow_end_size * { angle + PI + angle_offset }.cos()
    ));
    log!(format!(
        "y_off : {}",
        arrow_end_size * { angle + PI + angle_offset }.sin()
    ));
    path.move_to(x_to, y_to);
    path.line_to(
        x_to + arrow_end_size * { angle + PI + angle_offset }.cos(),
        y_to + arrow_end_size * { angle + PI + angle_offset }.sin(),
    );
    path.move_to(x_to, y_to);
    path.line_to(
        x_to + arrow_end_size * { angle + PI - angle_offset }.cos(),
        y_to + arrow_end_size * { angle + PI - angle_offset }.sin(),
    );
}

fn create_arrow(path: &Path2d, x_from: f64, mut y_from: f64, x_to: f64, y_to: f64) {
    log!(
        "x_from: {}, y_from: {}, x_to: {}, y_to: {}",
        x_from,
        y_from,
        x_to,
        y_to
    );

    path.move_to(x_from, y_from);
    y_from = y_from - y_from / 20.0;
    path.line_to(x_from, y_from);
    path.move_to(x_from, y_from);
    path.bezier_curve_to(x_to, y_from, x_to, y_to, x_to, y_to);

    let mut angle = { y_to - y_from }.atan2(x_to - x_from);
    let delta_angle = -PI / 2.0 - angle;
    angle = -PI / 2.0 - delta_angle / 7.0;
    log!(format!("angle : {}", angle));
    log!(format!("delta_angle : {}", delta_angle));
    let angle_offset = PI / 6.0;
    let arrow_end_size = 20.0;
    path.move_to(x_to, y_to);
    path.line_to(
        x_to + arrow_end_size * { PI + angle + angle_offset }.cos(),
        y_to + arrow_end_size * { PI + angle + angle_offset }.sin(),
    );
    path.move_to(x_to, y_to);
    path.line_to(
        x_to + arrow_end_size * { PI + angle - angle_offset }.cos(),
        y_to + arrow_end_size * { PI + angle - angle_offset }.sin(),
    );
}

#[derive(Clone, PartialEq)]
struct Rander {
    element_selected_id: String,
}

impl WithRander for Rander {
    fn rand(self, canvas: &HtmlCanvasElement) {
        if self.element_selected_id.ne("None") {
            log!(format!("{}", self.element_selected_id));
            let bounding_rect = canvas.get_bounding_client_rect();

            let next = canvas.next_element_sibling().unwrap();
            let next_children = next.children();

            let mut myset = SortedSet::<f64>::new();
            for children_index in 0..next_children.length() {
                let arrow_x_value = match next_children.get_with_index(children_index) {
                    Some(child_exists) => {
                        let arrow_absolute_x = get_x_value_for_arrow(&child_exists);
                        let arrow_relative_x = arrow_absolute_x - bounding_rect.x();
                        assert!(arrow_relative_x >= 0.0);
                        arrow_relative_x
                    }
                    None => 0.0,
                };
                myset.insert(arrow_x_value);
            }

            log!("{}", self.element_selected_id.clone());
            let interface: CanvasRenderingContext2d = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            interface.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            interface.set_fill_style(&JsValue::from_str("#fe5c5a"));
            interface.set_stroke_style(&JsValue::from_str("#00ffea;"));
            interface.set_line_width(3.0);

            /*
            let previous = canvas.previous_element_sibling().unwrap();
            let prev_children = previous.children();
            */

            let path = Path2d::new().unwrap();

            let id_selected_div: String = "rdmap_defi_".to_owned() + &self.element_selected_id;
            log!(format!("{}", id_selected_div));
            let window = web_sys::window().expect("global window does not exists");
            let document = window.document().expect("expecting a document on window");
            let selected_element = match document.get_element_by_id(&id_selected_div) {
                Some(element) => element,
                None => {
                    log!(format!(
                        "{} {} {}",
                        "rdmap_defi_Start_0",
                        &id_selected_div,
                        id_selected_div.eq("rdmap_defi_Start_0")
                    ));
                    document.get_element_by_id(&id_selected_div).unwrap() // pour pas avoir
                                                                          // d'erreurs
                }
            };

            let selected_defi = match selected_element.dyn_into::<web_sys::HtmlElement>() {
                Ok(defi) => defi,
                Err(element) => {
                    log!(format!(
                        "{} {} {}",
                        "rdmap_defi_Start_0",
                        &id_selected_div,
                        id_selected_div.eq("rdmap_defi_Start_0")
                    ));
                    element.dyn_into::<web_sys::HtmlElement>().unwrap() // ne marchera jamais
                }
            };

            //for children_index in 0..prev_children.length() {

            let arrow_x_value = {
                let arrow_absolute_x = get_x_value_for_arrow(&selected_defi);
                let arrow_relative_x = arrow_absolute_x - bounding_rect.x();
                assert!(arrow_relative_x >= 0.0);
                arrow_relative_x
            };
            log!(arrow_x_value);
            let mut arrows_to_create: Vec<f64> = Vec::<f64>::new();
            let mut inserted_false_value = false;
            let index_inserted = match myset.find_or_insert(arrow_x_value) {
                Ok(ind) => {
                    log!("inserting : {}", arrow_x_value);
                    arrows_to_create.push(arrow_x_value);
                    ind
                }
                Err(ind) => {
                    inserted_false_value = true;
                    ind
                }
            };

            match myset.get(index_inserted - 1) {
                Some(val) => {
                    log!("inserting : {}", *val);
                    arrows_to_create.push(*val)
                }
                None => {}
            }
            match myset.get(index_inserted + 1) {
                Some(val) => {
                    log!("inserting : {}", *val);
                    arrows_to_create.push(*val)
                }
                None => {}
            }

            if inserted_false_value {
                myset.remove_index(index_inserted);
            }

            for arrow_to_create in arrows_to_create {
                create_arrow(
                    &path,
                    arrow_x_value,
                    f64::from(canvas.height()),
                    arrow_to_create,
                    0.0,
                );

                interface.stroke_with_path(&path);
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct RoadMapDefiProps {
    pub defi_name: String,
    pub defi_etape_num: usize,
}

#[function_component(RoadMapDefi)]
pub fn rdmap_defi(props: &RoadMapDefiProps) -> Html {
    let id_rdmap_defi: String = "rdmap_defi_".to_owned()
        + &String::from(props.defi_etape_num.to_string())
        + "_"
        + &props.defi_name;
    html! {<div id={id_rdmap_defi} class="roadmap_defi" ><button>{&props.defi_name}</button></div>}
}

#[derive(Properties, PartialEq)]
pub struct ArrowsProps {
    pub element_id: String,
}

#[function_component(Arrows)]
pub fn arrows(props: &ArrowsProps) -> Html {
    html! {
        <Canvas<CanvasRenderingContext2d, Rander>
        style="
            height: 100px;
        "
        rander={Box::new(Rander{element_selected_id: props.element_id.clone()})}
        >
        </Canvas<CanvasRenderingContext2d, Rander>>
    }
}

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
    let sakara_state = use_state(|| 0);
    let defi = html!(<RoadMapDefi defi_etape_num=0 defi_name={"Start".to_string()}/>);

    let _onhover = {
        let sakara_state = sakara_state.clone();
        Callback::from(move |_: usize| sakara_state.set(*sakara_state + 1))
    };

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
                        {defi.clone()}
                    </div>
                    <Arrows element_id="0_Start" />
                    <div class="roadmap_etape">
                        <RoadMapDefi defi_etape_num=1 defi_name={"foo".to_string()} />
                        <RoadMapDefi defi_etape_num=1 defi_name={"Histoire".to_string()} />
                        <RoadMapDefi defi_etape_num=1 defi_name={"Geographie".to_string()} />
                    </div>
                    <Arrows element_id="None"/>

                    <div class="roadmap_etape">
                        <RoadMapDefi defi_etape_num=2 defi_name={"Histoire".to_string()} />
                        <RoadMapDefi defi_etape_num=2 defi_name={"Geographie".to_string()} />
                    </div>
                </div>
            </div>
        </div>
    }
}
