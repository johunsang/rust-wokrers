use serde_json::Value;
use wasm_bindgen::JsValue;
use worker::{Fetch, Headers, Method, Request, RequestInit, Response, Result};

pub async fn send_json_request(
    url: &str,
    method: Method,
    body: Option<Value>,
    headers: &[(&str, &str)],
) -> Result<Value> {
    let req_headers = Headers::new();
    for (key, value) in headers {
        req_headers.set(key, value)?;
    }

    let mut init = RequestInit::new();
    init.with_method(method);
    init.with_headers(req_headers);
    if let Some(body) = body {
        init.with_body(Some(JsValue::from_str(&body.to_string())));
    }

    let req = Request::new_with_init(url, &init)?;
    let mut res = Fetch::Request(req).send().await?;
    res.json().await
}

pub async fn send_empty_request(
    url: &str,
    method: Method,
    headers: &[(&str, &str)],
) -> Result<Response> {
    let req_headers = Headers::new();
    for (key, value) in headers {
        req_headers.set(key, value)?;
    }
    let mut init = RequestInit::new();
    init.with_method(method);
    init.with_headers(req_headers);
    let req = Request::new_with_init(url, &init)?;
    Fetch::Request(req).send().await
}
