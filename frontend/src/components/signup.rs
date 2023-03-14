use crate::entities::interfaces::Status;
use crate::entities::interfaces::{InputUser, User};
use crate::service::future::handle_future;
use crate::service::users::create_user;
use regex::Regex;
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;

struct SignupState {
    email: String,
    password: String,
}

impl SignupState {
    fn is_valid(self: &Self) -> bool {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        let email_valid = email_regex.is_match(&self.email);
        let password_valid = self.password.len() >= 8;
        email_valid && password_valid
    }
}

#[function_component(Signup)]
pub fn signup() -> Html {
    let state = use_state(|| SignupState {
        email: "".to_string(),
        password: "".to_string(),
    });
    let oninput_email = {
        let state = state.clone();
        Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_unchecked_into();
            let value = target.value();
            state.set(SignupState {
                email: value.clone(),
                password: state.password.clone(),
            });
        })
    };

    let oninput_password = {
        let state = state.clone();
        Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_unchecked_into();
            let value = target.value();
            state.set(SignupState {
                email: state.email.clone(),
                password: value.clone(),
            });
        })
    };

    let validate_signup = {
        let state = state.clone();
        Callback::from(move |_| {
            let user = InputUser {
                email: state.email.clone(),
                password: state.password.clone(),
            };
            let future = async move { create_user(&user).await };
            handle_future(future, move |data: Result<User, Status>| {
                match data {
                    Ok(user) => {
                        log::info!("User: {:?}", user)
                    }
                    Err(_) => {
                        log::info!("Error loading the data !");
                    }
                };
            });
        })
    };

    html! {
        <div class="container">
          <div class="row">
            <div class="signin-form">
              <div class="form-group ">
                <input type="email" value={state.clone().email.clone()} oninput={oninput_email} class="form-control" placeholder="Email " id="email"/>
                <i class="fa fa-envelope"></i>
              </div>
              <div class="form-group">
                <input type="password" value={state.clone().password.clone()} oninput={oninput_password} class="form-control" placeholder="Password" id="Password"/>
                <i class="fa fa-lock"></i>
              </div>
              <button onclick={validate_signup} disabled={!state.clone().is_valid()} type="button" class="signup-btn" >{"Create free account"}</button>
              <div class="btm-text">
                 <span>{"Have an account already?"}</span><a href="">{"Sign in"}</a>
              </div>
            </div>
          </div>
        </div>
    }
}
