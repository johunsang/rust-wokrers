use wasm_bindgen::JsValue;
use worker::{Env, Request, Response, Result};

use crate::com::db::{d1_all, d1_first};
use crate::com::env::iso_now;
use crate::com::http::{escape_html, html_response, json_error, json_response};
use crate::com::types::{Page, PageInput, PageSummary};

pub async fn list_pages(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let pages = d1_all::<PageSummary>(
        &db,
        "SELECT id, slug, title, status, published_at, updated_at FROM pages ORDER BY id DESC",
        &[],
    )
    .await?;
    json_response(200, &pages)
}

pub async fn get_page_by_id(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    let page = d1_first::<Page>(
        &db,
        "SELECT id, slug, title, content_md, content_html, status, published_at, created_at, updated_at FROM pages WHERE id = ?",
        &[JsValue::from_f64(id as f64)],
    )
    .await?;
    match page {
        Some(page) => json_response(200, &page),
        None => json_error(404, "Page not found"),
    }
}

pub async fn create_page(env: &Env, input: PageInput) -> Result<Response> {
    let db = env.d1("DB")?;
    let html = markdown_to_html(&input.content_md);
    let now = iso_now();
    db.prepare("INSERT INTO pages (slug, title, content_md, content_html, status, created_at, updated_at) VALUES (?, ?, ?, ?, 'draft', ?, ?)")
        .bind(&[
            JsValue::from_str(&input.slug),
            JsValue::from_str(&input.title),
            JsValue::from_str(&input.content_md),
            JsValue::from_str(&html),
            JsValue::from_str(&now),
            JsValue::from_str(&now),
        ])?
        .run()
        .await?;
    json_response(201, &serde_json::json!({ "ok": true }))
}

pub async fn update_page(env: &Env, id: i64, input: PageInput) -> Result<Response> {
    let db = env.d1("DB")?;
    let html = markdown_to_html(&input.content_md);
    db.prepare("UPDATE pages SET slug = ?, title = ?, content_md = ?, content_html = ?, updated_at = ? WHERE id = ?")
        .bind(&[
            JsValue::from_str(&input.slug),
            JsValue::from_str(&input.title),
            JsValue::from_str(&input.content_md),
            JsValue::from_str(&html),
            JsValue::from_str(&iso_now()),
            JsValue::from_f64(id as f64),
        ])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn publish_page(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    let now = iso_now();
    db.prepare(
        "UPDATE pages SET status = 'published', published_at = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&[
        JsValue::from_str(&now),
        JsValue::from_str(&now),
        JsValue::from_f64(id as f64),
    ])?
    .run()
    .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn unpublish_page(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare("UPDATE pages SET status = 'draft', updated_at = ? WHERE id = ?")
        .bind(&[JsValue::from_str(&iso_now()), JsValue::from_f64(id as f64)])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn delete_page(env: &Env, id: i64) -> Result<Response> {
    let db = env.d1("DB")?;
    db.prepare("DELETE FROM pages WHERE id = ?")
        .bind(&[JsValue::from_f64(id as f64)])?
        .run()
        .await?;
    json_response(200, &serde_json::json!({ "ok": true }))
}

pub async fn published_page(req: &Request, env: &Env) -> Result<Option<Response>> {
    let path = req.path();
    if path == "/" || path.starts_with("/api/") || path.starts_with("/admin") || path.contains('.')
    {
        return Ok(None);
    }

    let slug = path.trim_start_matches('/').trim_end_matches('/');
    if slug.is_empty() {
        return Ok(None);
    }

    let db = env.d1("DB")?;
    let page = d1_first::<Page>(
        &db,
        "SELECT id, slug, title, content_md, content_html, status, published_at, created_at, updated_at FROM pages WHERE slug = ?",
        &[JsValue::from_str(slug)],
    )
    .await?;
    let Some(page) = page else {
        return Ok(None);
    };
    if page.status != "published" {
        return Ok(None);
    }

    let body = format!(
        r##"
<section class="hero compact">
  <div>
    <span class="eyebrow">Published page</span>
    <h1>{title}</h1>
    <p class="lede">Updated {updated}</p>
  </div>
</section>
<article class="panel prose">{content}</article>
"##,
        title = escape_html(&page.title),
        updated = escape_html(&page.updated_at),
        content = page.content_html,
    );
    html_response(
        &page.title,
        "rust-wokrers",
        "Rust-rendered published page.",
        &body,
    )
    .map(Some)
}

fn markdown_to_html(md: &str) -> String {
    let mut html = escape_html(md);
    html = html.replace("```", "");
    html = html.replace("\n\n", "</p><p>");
    html = html.replace('\n', "<br />");
    format!("<p>{html}</p>")
}
