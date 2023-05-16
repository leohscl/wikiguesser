use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TimerProps {
    pub cb_time_up: Callback<()>,
    pub num_secs: u32,
    pub stop_count: bool,
}

#[derive(Debug, PartialEq, Clone)]
enum CountdownStatus {
    Running(u32),
    #[allow(dead_code)]
    Stopped(u32),
}

#[derive(Debug, Clone)]
struct TimerError;

impl CountdownStatus {
    fn get_left(&self) -> u32 {
        match self {
            Self::Running(left) => *left,
            Self::Stopped(left) => *left,
        }
    }
    fn get_min_sec(&self) -> (u32, u32) {
        let time_left = self.get_left();
        let min = time_left / 60;
        let secs = time_left % 60;
        (min, secs)
    }

    fn remove_one(&self) -> Result<Self, TimerError> {
        match self {
            Self::Running(left) => {
                if left > &0 {
                    Ok(Self::Running(left - 1))
                } else {
                    Ok(Self::Stopped(0))
                }
            }
            Self::Stopped(_) => Err(TimerError {}),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct TimerState {
    countdown_status: CountdownStatus,
}

#[function_component(Timer)]
pub fn timer(props: &TimerProps) -> Html {
    let state = use_state(|| TimerState {
        countdown_status: CountdownStatus::Running(props.num_secs),
    });

    let state_count = state.clone();
    let cb_time_up = props.cb_time_up.clone();

    let remove_sec = move || {
        if state_count.countdown_status.get_left() != 0 {
            state_count.set(TimerState {
                countdown_status: state_count
                    .countdown_status
                    .remove_one()
                    .expect("The time should be running"),
            })
        } else {
            cb_time_up.emit(());
        }
    };
    if !props.stop_count {
        gloo::timers::callback::Timeout::new(1000, remove_sec).forget();
    }

    let (min, secs) = state.countdown_status.get_min_sec();
    let time_left = format!("{:02}:{:02}", min, secs);
    html! {
        <h2 class="countdown"> {time_left} </h2>
    }
}
