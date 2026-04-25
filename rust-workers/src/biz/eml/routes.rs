use wasm_bindgen::JsValue;
use worker::{Env, Response, Result};

use crate::com::db::{d1_all, d1_first};
use crate::com::env::{env_string, iso_now};
use crate::com::http::{json_error, json_response};
use crate::com::net::send_json_request;
use crate::com::types::{EmailLog, EmailTemplate, EmailTemplateInput, SendEmailInput};

pub async fn list_templates(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let rows = d1_all::<EmailTemplate>(
        &db,
        "SELECT id, name, subject, body_html, body_text, created_at, updated_at FROM email_templates ORDER BY id DESC",
        &[],
    )
    .await?;
    json_response(200, &rows)
}

pub async fn get_template(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    let row = d1_first::<EmailTemplate>(
        &db,
        "SELECT id, name, subject, body_html, body_text, created_at, updated_at FROM email_templates WHERE id = ?",
        &[JsValue::from_f64(id as f64)],
    )
    .await?;
    match row {
        Some(row) => json_response(200, &row),
        None => json_error(404, "Template not found"),
    }
}

pub async fn create_template(env: &Env, input: EmailTemplateInput) -> Result<Response> {
    let db = env.d1("DB")?;
    let now = iso_now();
    db.prepare("INSERT INTO email_templates (name, subject, body_html, body_text, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&[
            JsValue::from_str(&input.name),
            JsValue::from_str(&input.subject),
            JsValue::from_str(&input.body_html),
            JsValue::from_str(&input.body_text),
            JsValue::from_str(&now),
            JsValue::from_str(&now),
        ])?
        .run()
        .await?;
    json_response(201, &serde_json::json!({ "ok": true }))
}

pub async fn update_template(env: &Env, id: i64, input: EmailTemplateInput) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare("UPDATE email_templates SET name = ?, subject = ?, body_html = ?, body_text = ?, updated_at = ? WHERE id = ?")
        .bind(&[
            JsValue::from_str(&input.name),
            JsValue::from_str(&input.subject),
            JsValue::from_str(&input.body_html),
            JsValue::from_str(&input.body_text),
            JsValue::from_str(&iso_now()),
            JsValue::from_f64(id as f64),
        ])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn delete_template(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare("DELETE FROM email_templates WHERE id = ?")
        .bind(&[JsValue::from_f64(id as f64)])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn list_logs(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let rows = d1_all::<EmailLog>(
        &db,
        "SELECT el.id, el.lead_id, el.template_id, el.subject, el.status, el.sent_at, l.name AS lead_name, l.email AS lead_email FROM email_logs el LEFT JOIN leads l ON l.id = el.lead_id ORDER BY el.id DESC LIMIT 50",
        &[],
    )
    .await?;
    json_response(200, &rows)
}

pub async fn send_email(env: &Env, input: SendEmailInput) -> Result<Response> {
    let api_key = match env_string(env, "RESEND_API_KEY") {
        Some(value) => value,
        None => return json_error(500, "RESEND_API_KEY not configured"),
    };
    let from =
        env_string(env, "RESEND_FROM_ADDRESS").unwrap_or_else(|| "noreply@example.com".to_string());
    let db = env.d1("DB")?;
    let lead = d1_first::<LeadEmailRow>(
        &db,
        "SELECT email, name FROM leads WHERE id = ?",
        &[JsValue::from_f64(input.lead_id as f64)],
    )
    .await?;
    let Some(lead) = lead else {
        return json_error(404, "Lead not found");
    };

    let mut subject = input.subject.unwrap_or_default();
    let mut body_html = input.body_html.unwrap_or_default();
    let mut body_text = input.body_text.unwrap_or_default();

    if let Some(template_id) = input.template_id {
        let template = d1_first::<EmailTemplate>(
            &db,
            "SELECT id, name, subject, body_html, body_text, created_at, updated_at FROM email_templates WHERE id = ?",
            &[JsValue::from_f64(template_id as f64)],
        )
        .await?;
        let Some(template) = template else {
            return json_error(404, "Template not found");
        };
        if subject.is_empty() {
            subject = template.subject;
        }
        if body_html.is_empty() {
            body_html = template.body_html;
        }
        if body_text.is_empty() {
            body_text = template.body_text;
        }
    }
    if subject.is_empty() {
        return json_error(400, "Subject is required");
    }

    let result = send_json_request(
        "https://api.resend.com/emails",
        worker::Method::Post,
        Some(serde_json::json!({
            "from": from,
            "to": [lead.email],
            "subject": subject,
            "html": body_html,
            "text": body_text
        })),
        &[
            ("authorization", &format!("Bearer {api_key}")),
            ("content-type", "application/json"),
        ],
    )
    .await;

    let status = if result.is_ok() { "sent" } else { "failed" };
    db.prepare("INSERT INTO email_logs (lead_id, template_id, subject, status, sent_at) VALUES (?, ?, ?, ?, ?)")
        .bind(&[
            JsValue::from_f64(input.lead_id as f64),
            input
                .template_id
                .map(|value| JsValue::from_f64(value as f64))
                .unwrap_or(JsValue::NULL),
            JsValue::from_str(&subject),
            JsValue::from_str(status),
            JsValue::from_str(&iso_now()),
        ])?
        .run()
        .await?;

    if result.is_err() {
        return json_error(500, "Email send failed");
    }
    json_response(200, &serde_json::json!({ "ok": true }))
}

#[derive(serde::Deserialize)]
struct LeadEmailRow {
    email: String,
}
