use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::{JsCast, JsValue};
use ::web_sys::{Request, RequestInit, RequestMode, RequestRedirect, Response};
// use yew::web_sys;

pub enum Method {
    Get,
}

pub async fn fetch(url: String, method:String) -> Result<JsValue, JsValue> {
    log::info!("building request");
    let mut opts = RequestInit::new();
    opts.method(&method);
    opts.mode(RequestMode::Cors);
    opts.redirect(RequestRedirect::Follow);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set(
        "Access-Control-Request-Headers",
        "Content-Type, Authorization",
    )?;
    request
        .headers()
        .set("Access-Control-Request-Method", &method)?;

    log::info!("request_url {:?}", request.url());
    log::info!("request_headers {:?}", request.headers());
    log::info!("request_mode {:?}", request.mode());
    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    log::info!("resp_value: {:?}", resp_value);

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert a JS Promise into a Rust Future
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}

pub struct Fetch();

impl Fetch {
    async fn fetch(
        url: String,
        method: Method,
    ) -> Result<JsValue, JsValue> {
        let method = match method {
            Method::Get => "GET",
        };
        fetch(url, method.to_string()).await
    }

    pub async fn get(url: String) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::Get).await
    }
}


