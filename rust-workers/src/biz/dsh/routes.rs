use worker::{Env, Request, Response, Result};

use crate::biz::aut::routes::current_admin;
use crate::com::db::{count_query, d1_all, get_site_settings};
use crate::com::http::{escape_html, html_response, json_error, json_response, query_value};
use crate::com::security::login_configured;
use crate::com::types::{DashboardResponse, DashboardStats, LeadRow, PageSummary};

pub async fn admin_page(req: &Request, env: &Env) -> Result<Response> {
    if let Some(email) = current_admin(req, env)? {
        return admin_dashboard(env, &email).await;
    }
    admin_login_page(req, env)
}

pub async fn dashboard_api(req: &Request, env: &Env) -> Result<Response> {
    if current_admin(req, env)?.is_none() {
        return json_error(401, "Authentication required");
    }
    let db = env.d1("DB")?;
    let settings = get_site_settings(&db).await?;
    let response = DashboardResponse {
        stats: DashboardStats {
            total_leads: count_query(&db, "SELECT COUNT(*) AS count FROM leads").await?,
            total_media: count_query(&db, "SELECT COUNT(*) AS count FROM media_assets").await?,
            total_pages: count_query(&db, "SELECT COUNT(*) AS count FROM pages")
                .await
                .unwrap_or(0),
            latest_update_at: settings.updated_at,
        },
        recent_leads: d1_all::<LeadRow>(
            &db,
            "SELECT id, name, email, company, message, status, created_at FROM leads ORDER BY id DESC LIMIT 8",
            &[],
        )
        .await
        .unwrap_or_default(),
        recent_pages: d1_all::<PageSummary>(
            &db,
            "SELECT id, slug, title, status, published_at, updated_at FROM pages ORDER BY id DESC LIMIT 8",
            &[],
        )
        .await
        .unwrap_or_default(),
    };
    json_response(200, &response)
}

fn admin_login_page(req: &Request, env: &Env) -> Result<Response> {
    let message = query_value(req, "error")
        .map(|value| format!("<div class=\"notice error\">{}</div>", escape_html(&value)))
        .unwrap_or_default();
    let body = format!(
        r##"
<section class="hero compact">
  <div>
    <span class="eyebrow">Admin</span>
    <h1>Rust worker sign-in</h1>
    <p class="lede">The admin shell is rendered directly by the worker. Existing session auth remains available.</p>
  </div>
</section>
{message}
<section class="grid two">
  <article class="panel">
    <h2>Password login</h2>
    <form method="post" action="/admin/login" class="stack">
      <label>Email<input type="email" name="email" required /></label>
      <label>Password<input type="password" name="password" required /></label>
      <button type="submit" class="button">Sign in</button>
    </form>
  </article>
  <article class="panel">
    <h2>Status</h2>
    <p class="muted">Session login configured: {configured}</p>
  </article>
</section>
"##,
        message = message,
        configured = if login_configured(env) { "yes" } else { "no" },
    );
    html_response(
        "Admin",
        "rust-wokrers Admin",
        "Rust-rendered admin sign-in.",
        &body,
    )
}

async fn admin_dashboard(env: &Env, email: &str) -> Result<Response> {
    let db = env.d1("DB")?;
    let settings = get_site_settings(&db).await?;
    let recent_leads = d1_all::<LeadRow>(
        &db,
        "SELECT id, name, email, company, message, status, created_at FROM leads ORDER BY id DESC LIMIT 8",
        &[],
    )
    .await
    .unwrap_or_default();
    let recent_pages = d1_all::<PageSummary>(
        &db,
        "SELECT id, slug, title, status, published_at, updated_at FROM pages ORDER BY id DESC LIMIT 8",
        &[],
    )
    .await
    .unwrap_or_default();
    let stats = DashboardStats {
        total_leads: count_query(&db, "SELECT COUNT(*) AS count FROM leads").await?,
        total_media: count_query(&db, "SELECT COUNT(*) AS count FROM media_assets").await?,
        total_pages: count_query(&db, "SELECT COUNT(*) AS count FROM pages")
            .await
            .unwrap_or(0),
        latest_update_at: settings.updated_at.clone(),
    };

    let leads_html = if recent_leads.is_empty() {
        "<tr><td colspan=\"4\" class=\"muted\">No leads yet.</td></tr>".to_string()
    } else {
        recent_leads
            .into_iter()
            .map(|lead| {
                format!(
                    "<tr><td>{name}</td><td>{email}</td><td>{status}</td><td>{created}</td></tr>",
                    name = escape_html(&lead.name),
                    email = escape_html(&lead.email),
                    status = escape_html(&lead.status),
                    created = escape_html(&lead.created_at),
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };
    let pages_html = if recent_pages.is_empty() {
        "<tr><td colspan=\"3\" class=\"muted\">No pages yet.</td></tr>".to_string()
    } else {
        recent_pages
            .into_iter()
            .map(|page| {
                format!(
                    "<tr><td><a href=\"/{slug}\">{title}</a></td><td>{status}</td><td>{updated}</td></tr>",
                    slug = escape_html(&page.slug),
                    title = escape_html(&page.title),
                    status = escape_html(&page.status),
                    updated = escape_html(&page.updated_at),
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };

    let body = format!(
        r##"
<section class="hero compact">
  <div>
    <span class="eyebrow">Admin</span>
    <h1>{brand}</h1>
    <p class="lede">Signed in as {email}.</p>
  </div>
  <form method="post" action="/admin/logout">
    <button type="submit" class="button secondary">Log out</button>
  </form>
</section>
<section class="grid four">
  <article class="card stat"><span>Leads</span><strong>{total_leads}</strong></article>
  <article class="card stat"><span>Media</span><strong>{total_media}</strong></article>
  <article class="card stat"><span>Pages</span><strong>{total_pages}</strong></article>
  <article class="card stat"><span>Updated</span><strong>{updated_at}</strong></article>
</section>
<section class="grid two">
  <article class="panel">
    <h2>Recent leads</h2>
    <table>
      <thead><tr><th>Name</th><th>Email</th><th>Status</th><th>Created</th></tr></thead>
      <tbody>{leads_html}</tbody>
    </table>
  </article>
  <article class="panel">
    <h2>Pages</h2>
    <table>
      <thead><tr><th>Title</th><th>Status</th><th>Updated</th></tr></thead>
      <tbody>{pages_html}</tbody>
    </table>
  </article>
</section>
"##,
        brand = escape_html(&settings.brand),
        email = escape_html(email),
        total_leads = stats.total_leads,
        total_media = stats.total_media,
        total_pages = stats.total_pages,
        updated_at = escape_html(&stats.latest_update_at),
        leads_html = leads_html,
        pages_html = pages_html,
    );
    html_response(
        "Admin",
        "rust-wokrers Admin",
        "Rust-rendered admin dashboard.",
        &body,
    )
}
