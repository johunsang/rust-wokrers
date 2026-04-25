use wasm_bindgen::JsValue;
use worker::{Env, Response, Result};

use crate::com::db::{get_site_settings, site_settings_dto};
use crate::com::env::iso_now;
use crate::com::http::json_response;
use crate::com::types::SiteSettings;

pub async fn get_settings(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    json_response(200, &site_settings_dto(get_site_settings(&db).await?))
}

pub async fn update_settings(env: &Env, payload: SiteSettings) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare(
        "UPDATE site_settings SET brand = ?, hero_label = ?, hero_title = ?, hero_subtitle = ?, cta_primary = ?, cta_secondary = ?, updated_at = ? WHERE id = 1",
    )
    .bind(&[
        JsValue::from_str(&payload.brand),
        JsValue::from_str(&payload.hero_label),
        JsValue::from_str(&payload.hero_title),
        JsValue::from_str(&payload.hero_subtitle),
        JsValue::from_str(&payload.cta_primary),
        JsValue::from_str(&payload.cta_secondary),
        JsValue::from_str(&iso_now()),
    ])?
    .run()
    .await?;
    json_response(200, &site_settings_dto(get_site_settings(&db).await?))
}
