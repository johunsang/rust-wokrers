use wasm_bindgen::JsValue;
use worker::{Env, Response, Result};

use crate::com::db::{count_query, d1_all, d1_first};
use crate::com::env::iso_now;
use crate::com::http::{json_error, json_response};
use crate::com::types::{AdminUserInput, AdminUserRow, AdminUserUpdateInput};

pub async fn list_users(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let users = d1_all::<AdminUserRow>(
        &db,
        "SELECT * FROM admin_users ORDER BY created_at DESC",
        &[],
    )
    .await?;
    json_response(200, &users)
}

pub async fn get_user(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    let user = d1_first::<AdminUserRow>(
        &db,
        "SELECT * FROM admin_users WHERE id = ?",
        &[JsValue::from_f64(id as f64)],
    )
    .await?;
    match user {
        Some(user) => json_response(200, &user),
        None => json_error(404, "User not found"),
    }
}

pub async fn create_user(env: &Env, payload: AdminUserInput) -> Result<Response> {
    let db = env.d1("DB")?;
    let now = iso_now();
    db.prepare("INSERT INTO admin_users (email, name, role, is_active, created_at, updated_at) VALUES (?, ?, ?, 1, ?, ?)")
        .bind(&[
            JsValue::from_str(&payload.email),
            JsValue::from_str(&payload.name),
            JsValue::from_str(&payload.role),
            JsValue::from_str(&now),
            JsValue::from_str(&now),
        ])?
        .run()
        .await?;
    json_response(201, &serde_json::json!({ "ok": true }))
}

pub async fn update_user(env: &Env, id: i64, payload: AdminUserUpdateInput) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare("UPDATE admin_users SET name = ?, role = ?, updated_at = ? WHERE id = ?")
        .bind(&[
            JsValue::from_str(&payload.name),
            JsValue::from_str(&payload.role),
            JsValue::from_str(&iso_now()),
            JsValue::from_f64(id as f64),
        ])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn toggle_user(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    let user = d1_first::<AdminUserRow>(
        &db,
        "SELECT * FROM admin_users WHERE id = ?",
        &[JsValue::from_f64(id as f64)],
    )
    .await?;
    let Some(user) = user else {
        return json_error(404, "User not found");
    };
    let next = if user.is_active == 1 { 0 } else { 1 };
    db.prepare("UPDATE admin_users SET is_active = ?, updated_at = ? WHERE id = ?")
        .bind(&[
            JsValue::from_f64(next as f64),
            JsValue::from_str(&iso_now()),
            JsValue::from_f64(id as f64),
        ])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn delete_user(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare("DELETE FROM admin_users WHERE id = ?")
        .bind(&[JsValue::from_f64(id as f64)])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn user_count(env: &Env) -> Result<i64> {
    let db = env.d1("DB")?;
    count_query(&db, "SELECT COUNT(*) AS count FROM admin_users").await
}

pub async fn active_user_count(env: &Env) -> Result<i64> {
    let db = env.d1("DB")?;
    count_query(
        &db,
        "SELECT COUNT(*) AS count FROM admin_users WHERE is_active = 1",
    )
    .await
}
