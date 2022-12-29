use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{InputEvent, HtmlInputElement};
use regex::Regex;
use crate::service::future::handle_future;
use crate::service::users::get_user;
use crate::entities::interfaces::Status;
use crate::entities::interfaces::User;
use crate::utils::hashing::verify_password;
use crate::components::app::Route;

const PASSWORD_MIN_LENGTH: usize = 8;

struct LoginState {
    email: String,
    password: String,
}

impl LoginState {
    fn is_valid(self: &Self) -> bool {
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        let email_valid = email_regex.is_match(&self.email);
        let password_valid = self.password.len() >= PASSWORD_MIN_LENGTH;
        email_valid && password_valid
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct LoginProps {
    pub cb_user_login: Callback<User>,
}

#[function_component(Login)]
pub fn signup(props: &LoginProps) -> Html {
    let state = use_state(|| LoginState{email: "".to_string(), password: "".to_string()});
    let oninput_email = {
        let state = state.clone();
        Callback::from( move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_unchecked_into();
            let value = target.value();
            state.set(LoginState{email: value.clone(), password: state.password.clone()});
        })
    };

    let oninput_password = {
        let state = state.clone();
        Callback::from( move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_unchecked_into();
            let value = target.value();
            state.set(LoginState{email: state.email.clone(), password: value.clone()});
        })
    };
    
    let history = use_history().unwrap();
    let try_login = {
        let state = state.clone();
        let props = props.clone();
        Callback::from( move |_| {
            let history = history.clone();
            let props = props.clone();
            // Step 1: check email exists in database, pull user
            let email_copy = state.email.clone();
            let password_copy = state.password.clone();
            let future_user = async move { get_user(&email_copy).await };
            handle_future(future_user, move |data: Result<User, Status>| {
                match data {
                    Ok(user) => {
                        log::info!("User: {:?}", user);
                        // Step 2: check password hash match
                        let password_hash = user.t_password.clone();
                        if let Ok(is_valid) = verify_password(&password_copy, &password_hash) {
                            match is_valid {
                                true => {
                                    props.cb_user_login.emit(user.clone());
                                    log::info!("match !");
                                    history.push(Route::GuessingPageDefault);
                                },
                                false => log::info!("no match !"),
                            }
                        } else {
                            log::info!("Error verifying password");
                        }
                    }
                    Err(_) => {
                        log::info!("No username with this email. Please try again");
                    },
                };
            });
        })
    };

    let is_valid = state.clone().is_valid();
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
              <button onclick={try_login} disabled={!is_valid} type="button" class="signup-btn" >{"Login"}</button>
                {
                    if !is_valid {
                        html!{
                            <span>{format!("Please enter a valid email adress and a password with at least {} characters", PASSWORD_MIN_LENGTH)}</span>
                        }
                    } else {
                        html!{}
                    }
                }
            </div>
          </div>
        </div>
    }
}
