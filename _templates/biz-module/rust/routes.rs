use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use worker::{Env, Response, Result};

use crate::com::db::{d1_all, d1_first};
use crate::com::env::iso_now;
use crate::com::http::{json_error, json_response};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct __ITEM__Record {
    pub id: i64,
    pub title: String,
    pub status: String,
    #[serde(alias = "created_at")]
    pub created_at: String,
    #[serde(alias = "updated_at")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Create__ITEM__Input {
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Update__ITEM__Input {
    pub title: Option<String>,
}

pub async fn list(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let items = d1_all::<__ITEM__Record>(
        &db,
        "SELECT id, title, status, created_at, updated_at FROM __items__ ORDER BY id DESC LIMIT 50",
        &[],
    )
    .await?;
    json_response(200, &items)
}

pub async fn get(env: &Env, id: i64) -> Result<Response> {
    match find(env, id).await? {
        Some(item) => json_response(200, &item),
        None => json_error(404, "__ITEM__ not found"),
    }
}

pub async fn create(env: &Env, input: Create__ITEM__Input) -> Result<Response> {
    if input.title.trim().is_empty() {
        return json_error(400, "title is required");
    }
    let db = env.d1("DB")?;
    let now = iso_now();
    db.prepare(
        "INSERT INTO __items__ (title, status, created_at, updated_at) VALUES (?, 'active', ?, ?)",
    )
    .bind(&[
        JsValue::from_str(&input.title),
        JsValue::from_str(&now),
        JsValue::from_str(&now),
    ])?
    .run()
    .await?;
    json_response(201, &serde_json::json!({ "ok": true }))
}

pub async fn update(env: &Env, id: i64, input: Update__ITEM__Input) -> Result<Response> {
    let Some(existing) = find(env, id).await? else {
        return json_error(404, "__ITEM__ not found");
    };
    let db = env.d1("DB")?;
    db.prepare("UPDATE __items__ SET title = ?, updated_at = ? WHERE id = ?")
        .bind(&[
            JsValue::from_str(input.title.as_deref().unwrap_or(&existing.title)),
            JsValue::from_str(&iso_now()),
            JsValue::from_f64(id as f64),
        ])?
        .run()
        .await?;
    get(env, id).await
}

pub async fn delete(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare("DELETE FROM __items__ WHERE id = ?")
        .bind(&[JsValue::from_f64(id as f64)])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

async fn find(env: &Env, id: i64) -> Result<Option<__ITEM__Record>> {
    let db = env.d1("DB")?;
    d1_first::<__ITEM__Record>(
        &db,
        "SELECT id, title, status, created_at, updated_at FROM __items__ WHERE id = ?",
        &[JsValue::from_f64(id as f64)],
    )
    .await
}
