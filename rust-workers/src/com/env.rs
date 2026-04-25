use js_sys::Date;
use wasm_bindgen::JsValue;
use worker::Env;

pub fn env_string(env: &Env, key: &str) -> Option<String> {
    env.var(key).ok().map(|value| value.to_string())
}

pub fn js_opt_string(value: Option<&str>) -> JsValue {
    value.map(JsValue::from_str).unwrap_or(JsValue::NULL)
}

pub fn now_unix() -> i64 {
    (Date::now() / 1000.0).floor() as i64
}

pub fn iso_now() -> String {
    Date::new_0().to_iso_string().into()
}
