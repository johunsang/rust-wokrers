use serde_json::json;
use wasm_bindgen::JsValue;
use worker::{Env, Request, Response, Result};

use crate::com::db::{count_query, d1_all, get_site_settings, site_settings_dto};
use crate::com::env::{env_string, iso_now, js_opt_string};
use crate::com::http::{
    escape_html, form_value, html_response, json_response, optional_form_value, parse_form,
    query_value,
};
use crate::com::net::send_json_request;
use crate::com::types::{
    LeadSubmissionInput, Page, PageSummary, PublicBootstrap, PublicMetrics, ReleaseAsset,
    ReleaseSummary,
};

pub async fn landing_page(req: &Request, env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let settings = get_site_settings(&db).await?;
    let metrics = PublicMetrics {
        total_leads: count_query(&db, "SELECT COUNT(*) AS count FROM leads").await?,
        total_media: count_query(&db, "SELECT COUNT(*) AS count FROM media_assets").await?,
        updated_at: settings.updated_at.clone(),
    };
    let pages = d1_all::<PageSummary>(
        &db,
        "SELECT id, slug, title, status, published_at, updated_at FROM pages WHERE status = 'published' ORDER BY updated_at DESC LIMIT 6",
        &[],
    )
    .await
    .unwrap_or_default();
    let success = query_value(req, "submitted").is_some();

    let page_cards = if pages.is_empty() {
        "<p class=\"muted\">No published pages yet.</p>".to_string()
    } else {
        pages.into_iter()
            .map(|page| {
                format!(
                    "<article class=\"card\"><h3><a href=\"/{slug}\">{title}</a></h3><p class=\"muted\">Updated {updated}</p></article>",
                    slug = escape_html(&page.slug),
                    title = escape_html(&page.title),
                    updated = escape_html(&page.updated_at)
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };
    let notice = if success {
        "<div class=\"notice success\">Lead submitted successfully.</div>"
    } else {
        ""
    };

    let body = format!(
        r##"
<section class="hero">
  <div>
    <span class="eyebrow">{hero_label}</span>
    <h1>{hero_title}</h1>
    <p class="lede">{hero_subtitle}</p>
    <div class="hero-actions">
      <a class="button" href="#contact">{cta_primary}</a>
      <a class="button secondary" href="/admin">{cta_secondary}</a>
    </div>
  </div>
  <div class="panel">
    <h2>Runtime snapshot</h2>
    <dl class="stats">
      <div><dt>Leads</dt><dd>{total_leads}</dd></div>
      <div><dt>Media</dt><dd>{total_media}</dd></div>
      <div><dt>Updated</dt><dd>{updated_at}</dd></div>
      <div><dt>Stack</dt><dd>Cloudflare Workers + Rust + D1</dd></div>
    </dl>
  </div>
</section>
{notice}
<section class="grid two">
  <article class="panel">
    <h2>Rust-first structure</h2>
    <ul class="list">
      <li>Landing and admin pages are rendered by the Rust worker.</li>
      <li>D1 is reused for settings, leads, and published pages.</li>
      <li>Cloudflare bindings stay in place for future extensions.</li>
    </ul>
  </article>
  <article class="panel" id="contact">
    <h2>Create a lead</h2>
    <form method="post" action="/contact" class="stack">
      <label>Name<input name="name" required /></label>
      <label>Email<input type="email" name="email" required /></label>
      <label>Company<input name="company" /></label>
      <label>Message<textarea name="message" rows="5"></textarea></label>
      <button type="submit" class="button">Submit</button>
    </form>
  </article>
</section>
<section class="panel">
  <h2>Published pages</h2>
  <div class="grid three">{page_cards}</div>
</section>
"##,
        hero_label = escape_html(&settings.hero_label),
        hero_title = escape_html(&settings.hero_title),
        hero_subtitle = escape_html(&settings.hero_subtitle),
        cta_primary = escape_html(&settings.cta_primary),
        cta_secondary = escape_html(&settings.cta_secondary),
        total_leads = metrics.total_leads,
        total_media = metrics.total_media,
        updated_at = escape_html(&metrics.updated_at),
        notice = notice,
        page_cards = page_cards,
    );
    html_response(
        "rust-wokrers",
        &settings.brand,
        "Rust-rendered Cloudflare Workers starter.",
        &body,
    )
}

pub async fn contact_submit(req: &mut Request, env: &Env) -> Result<Response> {
    let form = parse_form(req).await?;
    let payload = LeadSubmissionInput {
        name: form_value(&form, "name"),
        email: form_value(&form, "email"),
        company: optional_form_value(&form, "company"),
        message: optional_form_value(&form, "message"),
    };
    validate_lead_submission(&payload)?;
    insert_lead(env, &payload).await?;
    Response::redirect(req.url()?.join("/?submitted=1")?)
}

pub async fn public_bootstrap(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let settings = get_site_settings(&db).await?;
    let bootstrap = PublicBootstrap {
        settings: site_settings_dto(settings.clone()),
        metrics: PublicMetrics {
            total_leads: count_query(&db, "SELECT COUNT(*) AS count FROM leads").await?,
            total_media: count_query(&db, "SELECT COUNT(*) AS count FROM media_assets").await?,
            updated_at: settings.updated_at,
        },
    };
    json_response(200, &bootstrap)
}

pub async fn public_lead(req: &mut Request, env: &Env) -> Result<Response> {
    let payload = req.json::<LeadSubmissionInput>().await?;
    validate_lead_submission(&payload)?;
    insert_lead(env, &payload).await?;
    json_response(201, &json!({ "ok": true }))
}

pub async fn public_pages(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let pages = d1_all::<PageSummary>(
        &db,
        "SELECT id, slug, title, status, published_at, updated_at FROM pages WHERE status = 'published' ORDER BY updated_at DESC",
        &[],
    )
    .await?;
    json_response(200, &pages)
}

pub async fn public_page_by_slug(env: &Env, slug: &str) -> Result<Response> {
    let db = env.d1("DB")?;
    let page = crate::com::db::d1_first::<Page>(
        &db,
        "SELECT id, slug, title, content_md, content_html, status, published_at, created_at, updated_at FROM pages WHERE slug = ?",
        &[JsValue::from_str(slug)],
    )
    .await?;
    match page.filter(|page| page.status == "published") {
        Some(page) => json_response(200, &page),
        None => crate::com::http::json_error(404, "Page not found"),
    }
}

pub async fn releases(env: &Env) -> Result<Response> {
    let repo = env_string(env, "GITHUB_RELEASES_REPO")
        .unwrap_or_else(|| "your-org/rust-wokrers".to_string());
    let url = format!("https://api.github.com/repos/{repo}/releases");
    let payload = send_json_request(
        &url,
        worker::Method::Get,
        None,
        &[
            ("user-agent", "rust-wokrers"),
            ("accept", "application/vnd.github+json"),
        ],
    )
    .await?;
    let releases = payload
        .as_array()
        .map(|items| {
            items
                .iter()
                .take(5)
                .map(|release| {
                    let assets = release
                        .get("assets")
                        .and_then(|value| value.as_array())
                        .map(|items| {
                            items
                                .iter()
                                .map(|asset| ReleaseAsset {
                                    name: string_field(asset, "name"),
                                    url: string_field(asset, "browser_download_url"),
                                    size: asset.get("size").and_then(|v| v.as_i64()).unwrap_or(0),
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    ReleaseSummary {
                        tag: string_field(release, "tag_name"),
                        name: string_field(release, "name"),
                        published_at: string_field(release, "published_at"),
                        body: string_field(release, "body"),
                        assets,
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    json_response(200, &releases)
}

pub fn robots(req: &Request) -> Result<Response> {
    let domain = req.url()?.host_str().unwrap_or("localhost").to_string();
    let content = format!(
        "User-agent: *\nAllow: /\nDisallow: /admin/\nDisallow: /api/admin/\n\nSitemap: https://{domain}/sitemap.xml"
    );
    let mut response = Response::ok(content)?;
    response
        .headers_mut()
        .set("content-type", "text/plain; charset=utf-8")?;
    Ok(response)
}

pub async fn sitemap(env: &Env) -> Result<Response> {
    let db = env.d1("DB")?;
    let pages = d1_all::<PageSummary>(
        &db,
        "SELECT id, slug, title, status, published_at, updated_at FROM pages WHERE status = 'published' ORDER BY updated_at DESC LIMIT 50",
        &[],
    )
    .await
    .unwrap_or_default();
    let domain = env_string(env, "APP_DOMAIN").unwrap_or_else(|| "example.com".to_string());
    let mut xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n  <url><loc>https://{domain}</loc></url>\n"
    );
    for page in pages {
        xml.push_str(&format!(
            "  <url><loc>https://{domain}/{slug}</loc></url>\n",
            slug = escape_html(&page.slug)
        ));
    }
    xml.push_str("</urlset>");
    let mut response = Response::ok(xml)?;
    response
        .headers_mut()
        .set("content-type", "application/xml; charset=utf-8")?;
    Ok(response)
}

async fn insert_lead(env: &Env, payload: &LeadSubmissionInput) -> Result<()> {
    let db = env.d1("DB")?;
    db.prepare(
        "INSERT INTO leads (name, email, company, message, status, created_at) VALUES (?, ?, ?, ?, 'new', ?)",
    )
    .bind(&[
        JsValue::from_str(&payload.name),
        JsValue::from_str(&payload.email),
        js_opt_string(payload.company.as_deref()),
        js_opt_string(payload.message.as_deref()),
        JsValue::from_str(&iso_now()),
    ])?
    .run()
    .await?;
    Ok(())
}

fn validate_lead_submission(payload: &LeadSubmissionInput) -> Result<()> {
    if payload.name.trim().is_empty() {
        return Err(worker::Error::RustError("Name is required".to_string()));
    }
    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        return Err(worker::Error::RustError(
            "Valid email is required".to_string(),
        ));
    }
    Ok(())
}

fn string_field(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or_default()
        .to_string()
}
