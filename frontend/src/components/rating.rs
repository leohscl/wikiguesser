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
            if !state.rating_sent {
                state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::One) })
            }
        })
    };
    let onclick_2 = {
        let state = state.clone();
        Callback::from( move |_| {
            if !state.rating_sent {
                state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::Two) })
            }
        })
    };
    let onclick_3 = {
        let state = state.clone();
        Callback::from( move |_| {
            if !state.rating_sent {
                state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::Three) })
            }
        })
    };
    let onclick_4 = {
        let state = state.clone();
        Callback::from( move |_| {
            if !state.rating_sent {
                state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::Four) })
            }
        })
    };
    let onclick_5 = {
        let state = state.clone();
        Callback::from( move |_| {
            log::info!("5 clicked!");
            if !state.rating_sent {
                state.set(RatingState { rating_sent: state.rating_sent, opt_rate: Some(Rate::Five) })
            }
        })
    };

    let disabled = state.rating_sent || state.opt_rate.is_none();
    html! {
        <div >
            <div class="rate" >
                <input type="radio" onclick={onclick_5} checked={is_clicked(Rate::Five,state.opt_rate.clone())}  id="star5" name="rate" value="5" />
                <label for="star5" title="text">{"5 stars"}</label>
                <input type="radio" onclick={onclick_4} checked={is_clicked(Rate::Four,state.opt_rate.clone())}  id="star4" name="rate" value="4" />
                <label for="star4" title="text">{"4 stars"}</label>
                <input type="radio" onclick={onclick_3} checked={is_clicked(Rate::Three,state.opt_rate.clone())}  id="star3" name="rate" value="3" />
                <label for="star3" title="text">{"3 stars"}</label>
                <input type="radio" onclick={onclick_2} checked={is_clicked(Rate::Two,state.opt_rate.clone())} id="star2" name="rate" value="2" />
                <label for="star2" title="text">{"2 stars"}</label>
                <input type="radio" onclick={onclick_1} checked={is_clicked(Rate::One,state.opt_rate.clone())} id="star1" name="rate" value="1" />
                <label for="star1" title="text">{"1 star"}</label>
              </div>
            <button onclick={onclick_send_rating} disabled={disabled}>
                { "Submit rating" }
            </button>
        </div>
    }
}
fn is_clicked(rate_fixed: Rate, opt_rate: Option<Rate>) -> bool {
    if let Some(rate) = opt_rate {
        rate == rate_fixed
    } else {
        false
    }
}
