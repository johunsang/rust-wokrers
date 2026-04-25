use serde::Serialize;
use serde_json::json;
use url::form_urlencoded;
use worker::{Request, Response, Result};

pub fn json_response<T: Serialize>(status: u16, payload: &T) -> Result<Response> {
    Response::from_json(payload).map(|resp| resp.with_status(status))
}

pub fn json_error(status: u16, message: &str) -> Result<Response> {
    json_response(status, &json!({ "error": message }))
}

pub fn html_response(title: &str, brand: &str, description: &str, body: &str) -> Result<Response> {
    let document = format!(
        r##"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{title}</title>
    <meta name="description" content="{description}" />
    <meta name="robots" content="index,follow" />
    <meta property="og:type" content="website" />
    <meta property="og:title" content="{title}" />
    <meta property="og:description" content="{description}" />
    <meta property="og:site_name" content="{brand}" />
    <meta name="twitter:card" content="summary" />
    <meta name="twitter:title" content="{title}" />
    <meta name="twitter:description" content="{description}" />
    <style>{styles}</style>
  </head>
  <body>
    <div class="shell">
      <header class="topbar">
        <a class="brand" href="/">{brand}</a>
        <nav>
          <a href="/">Home</a>
          <a href="/admin">Admin</a>
          <a href="/api/health">Health</a>
        </nav>
      </header>
      {body}
    </div>
  </body>
</html>"##,
        title = escape_html(title),
        description = escape_html(description),
        brand = escape_html(brand),
        styles = styles(),
        body = body,
    );
    Response::from_html(document)
}

pub async fn parse_form(req: &mut Request) -> Result<Vec<(String, String)>> {
    let body = req.text().await?;
    Ok(form_urlencoded::parse(body.as_bytes())
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect())
}

pub fn form_value(form: &[(String, String)], key: &str) -> String {
    form.iter()
        .find(|(candidate, _)| candidate == key)
        .map(|(_, value)| value.trim().to_string())
        .unwrap_or_default()
}

pub fn optional_form_value(form: &[(String, String)], key: &str) -> Option<String> {
    let value = form_value(form, key);
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

pub fn query_value(req: &Request, key: &str) -> Option<String> {
    req.url().ok().and_then(|url| {
        url.query_pairs()
            .find(|(candidate, _)| candidate == key)
            .map(|(_, value)| value.to_string())
    })
}

pub fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn styles() -> &'static str {
    r#"
:root {
  color-scheme: light;
  --bg: #f3efe7;
  --paper: rgba(255, 255, 255, 0.9);
  --ink: #13202b;
  --muted: #61717d;
  --line: rgba(19, 32, 43, 0.12);
  --accent: #0d7c66;
  --shadow: 0 24px 60px rgba(8, 19, 28, 0.08);
  --radius: 24px;
}
* { box-sizing: border-box; }
html, body { margin: 0; padding: 0; }
body {
  font-family: ui-sans-serif, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  color: var(--ink);
  background:
    radial-gradient(circle at top left, rgba(13, 124, 102, 0.15), transparent 32rem),
    radial-gradient(circle at bottom right, rgba(243, 159, 90, 0.12), transparent 28rem),
    var(--bg);
}
a { color: inherit; }
.shell { max-width: 1200px; margin: 0 auto; padding: 24px; }
.topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  padding: 18px 22px;
  margin-bottom: 24px;
  border: 1px solid var(--line);
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.76);
  backdrop-filter: blur(12px);
}
.topbar nav { display: flex; gap: 16px; flex-wrap: wrap; }
.brand { font-weight: 800; text-decoration: none; }
.hero {
  display: grid;
  grid-template-columns: minmax(0, 1.7fr) minmax(280px, 1fr);
  gap: 20px;
  margin-bottom: 24px;
}
.hero.compact { grid-template-columns: minmax(0, 1fr) auto; align-items: end; }
.eyebrow {
  display: inline-block;
  margin-bottom: 12px;
  padding: 6px 10px;
  border-radius: 999px;
  background: rgba(13, 124, 102, 0.1);
  color: var(--accent);
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
}
h1, h2, h3, p { margin-top: 0; }
h1 { font-size: clamp(2.4rem, 5vw, 4.8rem); line-height: 0.95; margin-bottom: 14px; }
h2 { font-size: 1.2rem; margin-bottom: 14px; }
.lede { font-size: 1.08rem; color: var(--muted); max-width: 52rem; }
.panel, .card {
  border: 1px solid var(--line);
  border-radius: var(--radius);
  background: var(--paper);
  box-shadow: var(--shadow);
}
.panel { padding: 24px; }
.card { padding: 20px; }
.stat strong { display: block; margin-top: 10px; font-size: 1.7rem; }
.grid { display: grid; gap: 20px; margin-bottom: 24px; }
.grid.two { grid-template-columns: repeat(2, minmax(0, 1fr)); }
.grid.three { grid-template-columns: repeat(3, minmax(0, 1fr)); }
.grid.four { grid-template-columns: repeat(4, minmax(0, 1fr)); }
.hero-actions { display: flex; gap: 12px; flex-wrap: wrap; margin-top: 24px; }
.button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 12px 18px;
  border: 0;
  border-radius: 999px;
  background: var(--accent);
  color: white;
  font-weight: 700;
  text-decoration: none;
  cursor: pointer;
}
.button.secondary {
  background: rgba(19, 32, 43, 0.08);
  color: var(--ink);
}
.stats {
  display: grid;
  gap: 14px;
  grid-template-columns: repeat(2, minmax(0, 1fr));
}
.stats dt { color: var(--muted); font-size: 0.9rem; }
.stats dd { margin: 8px 0 0; font-size: 1.2rem; font-weight: 800; }
.stack { display: grid; gap: 14px; }
label { display: grid; gap: 8px; font-weight: 600; }
input, textarea {
  width: 100%;
  padding: 12px 14px;
  border: 1px solid var(--line);
  border-radius: 16px;
  background: white;
  color: var(--ink);
  font: inherit;
}
.list { margin: 0; padding-left: 18px; color: var(--muted); }
.muted { color: var(--muted); }
.notice {
  margin-bottom: 24px;
  padding: 14px 16px;
  border-radius: 16px;
  border: 1px solid var(--line);
  background: rgba(255, 255, 255, 0.92);
}
.notice.success { border-color: rgba(13, 124, 102, 0.25); }
.notice.error { border-color: rgba(206, 83, 64, 0.25); }
table { width: 100%; border-collapse: collapse; }
th, td {
  padding: 12px 10px;
  border-bottom: 1px solid var(--line);
  text-align: left;
  vertical-align: top;
}
.prose { line-height: 1.7; }
@media (max-width: 900px) {
  .hero, .hero.compact, .grid.two, .grid.three, .grid.four { grid-template-columns: 1fr; }
  .topbar { border-radius: 24px; }
}
"#
}
