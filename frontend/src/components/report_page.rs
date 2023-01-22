use yew::prelude::*;
use yew_router::prelude::*;
use std::str::FromStr;
use web_sys::{InputEvent, HtmlInputElement, HtmlSelectElement};
use crate::service::future::handle_future;
use crate::entities::interfaces::{Status, InputReport};
use crate::service::reports::create_report;
use super::app::Route;

#[derive(Clone)]
enum ReportCategory {
    Bug,
    BadContent,
    Other,
}

impl ToString for ReportCategory {
    fn to_string(&self) -> String {
        match self {
            ReportCategory::Bug => "Bug".to_string(),
            ReportCategory::BadContent => "BadContent".to_string(),
            ReportCategory::Other => "Other".to_string(),
        }
    }
}


impl std::str::FromStr for ReportCategory {
    type Err = ();
    fn from_str(input: &str) -> Result<ReportCategory, Self::Err> {
        match input {
            "Bug"         => Ok(ReportCategory::Bug),
            "BadContent"  => Ok(ReportCategory::BadContent),
            "Other"       => Ok(ReportCategory::Other),
             _            => Err(()),
        }
    }
}

struct ReportState {
    category: ReportCategory,
    description: String,
}

impl ReportState {
    fn to_report_input(&self, article_id: i32) -> InputReport {
        InputReport { article_id, report_cat: self.category.to_string(), description: self.description.to_string()}
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ReportProps {
    pub article_id: i32,
}



#[function_component(ReportPage)]
pub fn report_page(props: &ReportProps) -> Html {
    let state = use_state(|| ReportState{category: ReportCategory::Bug, description: "".to_string()});
    let history = use_history().unwrap();

    let onclick_submit = {
        let state = state.clone();
        let props = props.clone();
        Callback::from( move |_| {
            let state = state.clone();
            let history = history.clone();
            let future_user = async move { create_report(&state.to_report_input(props.article_id)).await };
            handle_future(future_user, move |data: Result<(), Status>| {
                match data {
                    Ok(_) => {
                        log::info!("Report submitted");
                        history.push(Route::LaunchPage)
                    }
                    Err(_) => {
                        log::info!("Report failed");
                    },
                };
            });
        })
    };
    let on_select_cat = {
        let state = state.clone();
        Callback::from( move |e: Event | {
            let target: HtmlSelectElement = e.target_unchecked_into();
            let value = target.value();
            let category = ReportCategory::from_str(&value).expect("Selected values should not panic");
            state.set(ReportState { category, description: state.description.clone() });
        })
    };

    let oninput_description = {
        let state = state.clone();
        Callback::from( move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target_unchecked_into();
            let value = target.value();
            state.set(ReportState{category: state.category.clone(), description: value.clone()});
        })
    };
    let bug_string = ReportCategory::Bug.to_string();
    let content_string = ReportCategory::BadContent.to_string();
    let other_string = ReportCategory::Other.to_string();
    html! {
        <div>
            <select onchange={on_select_cat}>
                <option value={bug_string.clone()} selected=true>{bug_string}</option>
                <option value={content_string.clone()}>{content_string}</option>
                <option value={other_string.clone()}>{other_string}</option>
            </select>
            <div class="form-group ">
                <input type="text" value={state.description.clone()} oninput={oninput_description} class="form-control" placeholder="" id="report_description"/>
                <i class="fa fa-envelope"></i>
            </div>
            <button onclick={onclick_submit}>
                { "Submit report" }
            </button>
        </div>
    }
}
