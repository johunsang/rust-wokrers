use wasm_bindgen::JsValue;
use worker::{Env, Response, Result};

use crate::com::db::d1_all;
use crate::com::env::{env_string, iso_now};
use crate::com::http::{json_error, json_response};
use crate::com::net::{send_empty_request, send_json_request};
use crate::com::types::{DirectUploadInput, MediaAsset, UpdateMediaInput};

pub async fn list_media(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let rows = d1_all::<MediaAsset>(
        &db,
        "SELECT id, image_id, title, alt, status, delivery_url, preview_url, uploaded_at FROM media_assets ORDER BY id DESC LIMIT 20",
        &[],
    )
    .await?;
    json_response(200, &rows)
}

pub async fn direct_upload(env: &Env, input: DirectUploadInput) -> Result<Response> {
    let account_id = match env_string(env, "CLOUDFLARE_ACCOUNT_ID") {
        Some(value) => value,
        None => return json_error(503, "Cloudflare Images is not configured"),
    };
    let token = match env_string(env, "CLOUDFLARE_IMAGES_API_TOKEN") {
        Some(value) => value,
        None => return json_error(503, "Cloudflare Images is not configured"),
    };
    let response = send_json_request(
        &format!(
            "https://api.cloudflare.com/client/v4/accounts/{account_id}/images/v2/direct_upload"
        ),
        worker::Method::Post,
        Some(serde_json::json!({
            "requireSignedURLs": false,
            "metadata": { "title": input.title, "alt": input.alt.clone().unwrap_or_default() }
        })),
        &[
            ("authorization", &format!("Bearer {token}")),
            ("content-type", "application/json"),
        ],
    )
    .await?;
    let image_id = response
        .get("result")
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let upload_url = response
        .get("result")
        .and_then(|v| v.get("uploadURL"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let db = env.d1("DB")?;
    db.prepare("INSERT OR IGNORE INTO media_assets (image_id, title, alt, status, uploaded_at) VALUES (?, ?, ?, 'draft', ?)")
        .bind(&[
            JsValue::from_str(&image_id),
            JsValue::from_str(&input.title),
            input.alt.as_deref().map(JsValue::from_str).unwrap_or(JsValue::NULL),
            JsValue::from_str(&iso_now()),
        ])?
        .run()
        .await?;
    json_response(
        201,
        &serde_json::json!({ "imageId": image_id, "uploadURL": upload_url }),
    )
}

pub async fn refresh_media(env: &Env, image_id: &str) -> Result<Response> {
    let account_id = match env_string(env, "CLOUDFLARE_ACCOUNT_ID") {
        Some(value) => value,
        None => return json_error(503, "Cloudflare Images is not configured"),
    };
    let token = match env_string(env, "CLOUDFLARE_IMAGES_API_TOKEN") {
        Some(value) => value,
        None => return json_error(503, "Cloudflare Images is not configured"),
    };
    let response = send_json_request(
        &format!("https://api.cloudflare.com/client/v4/accounts/{account_id}/images/v1/{image_id}"),
        worker::Method::Get,
        None,
        &[
            ("authorization", &format!("Bearer {token}")),
            ("content-type", "application/json"),
        ],
    )
    .await?;
    let preview_url = response
        .get("result")
        .and_then(|v| v.get("variants"))
        .and_then(|v| v.as_array())
        .and_then(|v| v.first())
        .and_then(|v| v.as_str())
        .map(|v| v.to_string());
    let status = if preview_url.is_some() {
        "ready"
    } else {
        "draft"
    };
    let db = env.d1("DB")?;
    db.prepare(
        "UPDATE media_assets SET status = ?, delivery_url = ?, preview_url = ? WHERE image_id = ?",
    )
    .bind(&[
        JsValue::from_str(status),
        preview_url
            .as_deref()
            .map(JsValue::from_str)
            .unwrap_or(JsValue::NULL),
        preview_url
            .as_deref()
            .map(JsValue::from_str)
            .unwrap_or(JsValue::NULL),
        JsValue::from_str(image_id),
    ])?
    .run()
    .await?;
    json_response(
        200,
        &serde_json::json!({
            "imageId": image_id,
            "status": status,
            "deliveryUrl": preview_url,
            "previewUrl": preview_url
        }),
    )
}

pub async fn update_media(env: &Env, image_id: &str, input: UpdateMediaInput) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare("UPDATE media_assets SET title = ?, alt = ? WHERE image_id = ?")
        .bind(&[
            JsValue::from_str(&input.title),
            input
                .alt
                .as_deref()
                .map(JsValue::from_str)
                .unwrap_or(JsValue::NULL),
            JsValue::from_str(image_id),
        ])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn delete_media(env: &Env, image_id: &str) -> Result<Response> {
    if let (Some(account_id), Some(token)) = (
        env_string(env, "CLOUDFLARE_ACCOUNT_ID"),
        env_string(env, "CLOUDFLARE_IMAGES_API_TOKEN"),
    ) {
        let _ = send_empty_request(
            &format!(
                "https://api.cloudflare.com/client/v4/accounts/{account_id}/images/v1/{image_id}"
            ),
            worker::Method::Delete,
            &[("authorization", &format!("Bearer {token}"))],
        )
        .await;
    }
    let db = env.d1("DB")?;
    db.prepare("DELETE FROM media_assets WHERE image_id = ?")
        .bind(&[JsValue::from_str(image_id)])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}
