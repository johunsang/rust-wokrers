use serde_json::json;
use worker::{Env, Request, Response, Result};

use crate::com::db::d1_first;
use crate::com::env::{iso_now, now_unix};
use crate::com::http::{form_value, json_error, json_response, parse_form};
use crate::com::security::{
    authenticate_admin, clear_cookie, login_configured, read_session, set_session_cookie,
    validate_credentials,
};
use crate::com::types::{LoginInput, SessionClaims, SessionUser};

pub fn auth_me(req: &Request, env: &Env) -> Result<Response> {
    match read_session(req, env)? {
        Some(claims) => json_response(
            200,
            &SessionUser {
                email: claims.email,
            },
        ),
        None => json_error(401, "Not authenticated"),
    }
}

pub async fn auth_login(req: &mut Request, env: &Env) -> Result<Response> {
    let payload = req.json::<LoginInput>().await?;
    if !login_configured(env) {
        return json_error(503, "Admin login is not configured");
    }
    if !validate_credentials(env, &payload.email, &payload.password) {
        return json_error(401, "Invalid credentials");
    }
    ensure_admin_user(env, &payload.email).await?;
    let claims = SessionClaims {
        email: payload.email.clone(),
        exp: now_unix() + 60 * 60 * 12,
    };
    let mut response = json_response(200, &json!({ "ok": true, "email": payload.email }))?;
    set_session_cookie(req, env, &mut response, &claims)?;
    Ok(response)
}

pub fn auth_logout(req: &Request) -> Result<Response> {
    let mut response = json_response(200, &json!({ "ok": true }))?;
    clear_cookie(req, &mut response)?;
    Ok(response)
}

pub async fn admin_login_form(req: &mut Request, env: &Env) -> Result<Response> {
    let form = parse_form(req).await?;
    let email = form_value(&form, "email");
    let password = form_value(&form, "password");

    if !login_configured(env) {
        return Response::redirect(req.url()?.join("/admin?error=Login%20not%20configured")?);
    }
    if !validate_credentials(env, &email, &password) {
        return Response::redirect(req.url()?.join("/admin?error=Invalid%20credentials")?);
    }

    ensure_admin_user(env, &email).await?;
    let claims = SessionClaims {
        email,
        exp: now_unix() + 60 * 60 * 12,
    };
    let mut response = Response::redirect(req.url()?.join("/admin")?)?;
    set_session_cookie(req, env, &mut response, &claims)?;
    Ok(response)
}

pub fn admin_logout_form(req: &Request) -> Result<Response> {
    let mut response = Response::redirect(req.url()?.join("/admin")?)?;
    clear_cookie(req, &mut response)?;
    Ok(response)
}

pub fn current_admin(req: &Request, env: &Env) -> Result<Option<String>> {
    authenticate_admin(req, env)
}

async fn ensure_admin_user(env: &Env, email: &str) -> Result<()> {
    let db = env.d1("DB")?;
    let existing = d1_first::<ExistingUser>(
        &db,
        "SELECT email FROM admin_users WHERE email = ?",
        &[email.into()],
    )
    .await?;
    if existing.is_some() {
        return Ok(());
    }
    let now = iso_now();
    db.prepare(
        "INSERT INTO admin_users (email, name, role, is_active, created_at, updated_at) VALUES (?, ?, 'super_admin', 1, ?, ?)",
    )
    .bind(&[
        email.into(),
        email.into(),
        now.clone().into(),
        now.into(),
    ])?
    .run()
    .await?;
    Ok(())
}

#[derive(serde::Deserialize)]
struct ExistingUser {
    _email: String,
}
