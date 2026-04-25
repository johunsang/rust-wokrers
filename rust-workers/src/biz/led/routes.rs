use wasm_bindgen::JsValue;
use worker::{Env, Response, Result};

use crate::com::db::{d1_all, d1_first};
use crate::com::env::iso_now;
use crate::com::http::{json_error, json_response};
use crate::com::types::{LeadDetail, LeadNote, LeadRow, LeadTag};

pub async fn list_leads(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let leads = d1_all::<LeadRow>(
        &db,
        "SELECT id, name, email, company, message, status, created_at FROM leads ORDER BY id DESC LIMIT 20",
        &[],
    )
    .await?;
    json_response(200, &leads)
}

pub async fn get_lead(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    let lead = d1_first::<LeadRow>(
        &db,
        "SELECT id, name, email, company, message, status, created_at FROM leads WHERE id = ?",
        &[JsValue::from_f64(id as f64)],
    )
    .await?;
    let Some(lead) = lead else {
        return json_error(404, "Lead not found");
    };
    let tags = d1_all::<LeadTag>(
        &db,
        "SELECT id, lead_id, tag, created_at FROM lead_tags WHERE lead_id = ? ORDER BY id",
        &[JsValue::from_f64(id as f64)],
    )
    .await?;
    let notes = d1_all::<LeadNote>(
        &db,
        "SELECT id, lead_id, content, created_by, created_at FROM lead_notes WHERE lead_id = ? ORDER BY id DESC",
        &[JsValue::from_f64(id as f64)],
    )
    .await?;
    json_response(200, &LeadDetail { lead, tags, notes })
}

pub async fn update_status(env: &Env, id: i64, status: &str) -> Result<Response> {
    let db = env.d1("DB")?;
    if !lead_exists(&db, id).await? {
        return json_error(404, "Lead not found");
    }
    db.prepare("UPDATE leads SET status = ? WHERE id = ?")
        .bind(&[JsValue::from_str(status), JsValue::from_f64(id as f64)])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn add_tag(env: &Env, id: i64, tag: &str) -> Result<Response> {
    let db = env.d1("DB")?;
    if !lead_exists(&db, id).await? {
        return json_error(404, "Lead not found");
    }
    db.prepare("INSERT OR IGNORE INTO lead_tags (lead_id, tag, created_at) VALUES (?, ?, ?)")
        .bind(&[
            JsValue::from_f64(id as f64),
            JsValue::from_str(tag),
            JsValue::from_str(&iso_now()),
        ])?
        .run()
        .await?;
    json_response(201, &serde_json::json!({ "ok": true }))
}

pub async fn delete_tag(env: &Env, id: i64, tag: &str) -> Result<Response> {
    let db = env.d1("DB")?;
    if !lead_exists(&db, id).await? {
        return json_error(404, "Lead not found");
    }
    db.prepare("DELETE FROM lead_tags WHERE lead_id = ? AND tag = ?")
        .bind(&[JsValue::from_f64(id as f64), JsValue::from_str(tag)])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn list_notes(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    if !lead_exists(&db, id).await? {
        return json_error(404, "Lead not found");
    }
    let notes = d1_all::<LeadNote>(
        &db,
        "SELECT id, lead_id, content, created_by, created_at FROM lead_notes WHERE lead_id = ? ORDER BY id DESC",
        &[JsValue::from_f64(id as f64)],
    )
    .await?;
    json_response(200, &notes)
}

pub async fn add_note(env: &Env, id: i64, content: &str, created_by: &str) -> Result<Response> {
    let db = env.d1("DB")?;
    if !lead_exists(&db, id).await? {
        return json_error(404, "Lead not found");
    }
    db.prepare(
        "INSERT INTO lead_notes (lead_id, content, created_by, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&[
        JsValue::from_f64(id as f64),
        JsValue::from_str(content),
        JsValue::from_str(created_by),
        JsValue::from_str(&iso_now()),
    ])?
    .run()
    .await?;
    json_response(201, &serde_json::json!({ "ok": true }))
}

async fn lead_exists(db: &worker::D1Database, id: i64) -> Result<bool> {
    Ok(d1_first::<LeadRow>(
        db,
        "SELECT id, name, email, company, message, status, created_at FROM leads WHERE id = ?",
        &[JsValue::from_f64(id as f64)],
    )
    .await?
    .is_some())
}
