use yew::prelude::*;
use crate::service::future::handle_future;

use crate::entities::interfaces::Status;
use crate::{service::ratings::create_rating, entities::interfaces::InputRatings};

#[derive(PartialEq, Clone)]
enum Rate {
    One = 1,
    Two,
    Three,
    Four,
    Five,
}

#[derive(PartialEq)]
struct RatingState {
    rating_sent: bool,
    opt_rate: Option<Rate>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RatingProps {
    pub article_id: i32,
}

impl RatingState {
    fn to_rating_input(&self, article_id: i32) -> InputRatings {
        let rate = self.opt_rate.clone().expect("There should be a rating at this point");
        let rating = rate as i32;
        InputRatings { article_id, rating}
    }
}

#[function_component(Rating)]
pub fn rating(props: &RatingProps) -> Html {
    let state = use_state(|| RatingState{rating_sent: false, opt_rate: None});
    let onclick_send_rating = {
        let state = state.clone();
        let props = props.clone();
        Callback::from( move |_| {
            state.set(RatingState{rating_sent: true, opt_rate: state.opt_rate.clone()});
            let state = state.clone();
            let future_user = async move { create_rating(&state.to_rating_input(props.article_id)).await };
            handle_future(future_user, move |data: Result<(), Status>| {
                match data {
                    Ok(_) => {
                        log::info!("Rating submitted");
                    }
                    Err(_) => {
                        log::info!("Rating failed");
                    },
                };
            });
        })
    };

    let onclick_1 = {
        let state = state.clone();
        Callback::from( move |_| {
            state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::One) })
        })
    };
    let onclick_2 = {
        let state = state.clone();
        Callback::from( move |_| {
            state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::Two) })
        })
    };
    let onclick_3 = {
        let state = state.clone();
        Callback::from( move |_| {
            state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::Three) })
        })
    };
    let onclick_4 = {
        let state = state.clone();
        Callback::from( move |_| {
            state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::Four) })
        })
    };
    let onclick_5 = {
        let state = state.clone();
        Callback::from( move |_| {
            log::info!("5 clicked!");
            state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::Five) })
        })
    };

    let disabled = state.rating_sent || state.opt_rate.is_none();
    html! {
        <div >
            <div class="rating">
                <input type="radio" name="rating" value="5" id="5" onclick={onclick_5}/>
                <label for="5" value='\u{2606}' />
                <input type="radio" name="rating" value="4" id="4" onclick={onclick_4}/>
                <label for="4" value='\u{2606}' />
                <input type="radio" name="rating" value="3" id="3" onclick={onclick_3}/>
                <label for="3" value='\u{2606}' />
                <input type="radio" name="rating" value="2" id="2" onclick={onclick_2}/>
                <label for="2" value='\u{2606}' />
                <input type="radio" name="rating" value="1" id="1" onclick={onclick_1}/>
                <label for="1" value='\u{2606}' />
            </div>
            <button onclick={onclick_send_rating} disabled={disabled}>
                { "Submit rating" }
            </button>
        </div>
    }
}

