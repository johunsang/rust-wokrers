use wasm_bindgen::JsValue;
use worker::{Env, Response, Result};

use crate::com::db::d1_all;
use crate::com::http::json_response;
use crate::com::types::{SearchLead, SearchMedia, SearchResponse};

pub async fn search(env: &Env, query: &str) -> Result<Response> {
    if query.trim().is_empty() {
        return json_response(
            200,
            &SearchResponse {
                leads: vec![],
                media: vec![],
            },
        );
    }
    let db = env.d1("DB")?;
    let escaped = query.replace('%', "\\%").replace('_', "\\_");
    let keyword = format!("%{escaped}%");
    let leads = d1_all::<SearchLead>(
        &db,
        "SELECT id, name, email, company FROM leads WHERE name LIKE ? OR email LIKE ? OR company LIKE ? ORDER BY id DESC LIMIT 10",
        &[JsValue::from_str(&keyword), JsValue::from_str(&keyword), JsValue::from_str(&keyword)],
    )
    .await?;
    let media = d1_all::<SearchMedia>(
        &db,
        "SELECT image_id, title, alt FROM media_assets WHERE title LIKE ? OR alt LIKE ? ORDER BY id DESC LIMIT 10",
        &[JsValue::from_str(&keyword), JsValue::from_str(&keyword)],
    )
    .await
    .unwrap_or_default();
    json_response(200, &SearchResponse { leads, media })
}
