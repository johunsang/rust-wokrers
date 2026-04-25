use wasm_bindgen::JsValue;
use worker::{D1Database, Env, Response, Result};

use crate::biz::usr::routes::{active_user_count, user_count};
use crate::com::db::{count_query, d1_all};
use crate::com::http::json_response;
use crate::com::types::{AccessLogRow, ApiLogRow, SystemStats};

pub async fn list_access(env: &Env, limit: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    let logs = d1_all::<AccessLogRow>(
        &db,
        "SELECT * FROM access_logs ORDER BY created_at DESC LIMIT ?",
        &[JsValue::from_f64(limit as f64)],
    )
    .await?;
    json_response(200, &logs)
}

pub async fn list_api(env: &Env, limit: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    let logs = d1_all::<ApiLogRow>(
        &db,
        "SELECT * FROM api_logs ORDER BY created_at DESC LIMIT ?",
        &[JsValue::from_f64(limit as f64)],
    )
    .await?;
    json_response(200, &logs)
}

pub async fn stats(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let response = SystemStats {
        total_users: user_count(env).await?,
        active_users: active_user_count(env).await?,
        total_leads: count_query(&db, "SELECT COUNT(*) AS count FROM leads").await?,
        total_media: count_query(&db, "SELECT COUNT(*) AS count FROM media_assets").await?,
        total_pages: count_query(&db, "SELECT COUNT(*) AS count FROM pages").await?,
        total_emails: count_query(&db, "SELECT COUNT(*) AS count FROM email_logs")
            .await
            .unwrap_or(0),
        total_api_requests: count_query(&db, "SELECT COUNT(*) AS count FROM api_logs").await?,
        recent_access_logs: d1_all::<AccessLogRow>(
            &db,
            "SELECT * FROM access_logs ORDER BY created_at DESC LIMIT 10",
            &[],
        )
        .await?,
        recent_api_logs: d1_all::<ApiLogRow>(
            &db,
            "SELECT * FROM api_logs ORDER BY created_at DESC LIMIT 10",
            &[],
        )
        .await?,
    };
    json_response(200, &response)
}

pub async fn log_api_request(
    db: &D1Database,
    method: &str,
    path: &str,
    status_code: i64,
    duration_ms: i64,
) -> Result<()> {
    db.prepare("INSERT INTO api_logs (method, path, status_code, duration_ms, request_body, response_size, ip_address, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))")
        .bind(&[
            JsValue::from_str(method),
            JsValue::from_str(path),
            JsValue::from_f64(status_code as f64),
            JsValue::from_f64(duration_ms as f64),
            JsValue::NULL,
            JsValue::NULL,
            JsValue::NULL,
        ])?
        .run()
        .await?;
    Ok(())
}
