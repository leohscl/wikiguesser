pub mod components;
pub mod entities;
pub mod service;
pub mod utils;
// use crate::guessing_page::GuessingPage;
use components::app::App;
use dotenv_codegen::dotenv;

// Constants
const API_URL: &str = dotenv!("API_URL");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // yew::start_app::<GuessingPage>();
    yew::start_app::<App>();
}
