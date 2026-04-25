use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use worker::{Env, Request, Response, Result};

use crate::com::env::{env_string, now_unix};
use crate::com::types::SessionClaims;

pub const COOKIE_NAME: &str = "rust-wokrers_admin";
type HmacSha256 = Hmac<Sha256>;

pub fn apply_common_headers(path: &str, response: &mut Response) -> Result<()> {
    response
        .headers_mut()
        .set("Referrer-Policy", "strict-origin-when-cross-origin")?;
    response
        .headers_mut()
        .set("X-Content-Type-Options", "nosniff")?;
    response.headers_mut().set("X-Frame-Options", "DENY")?;
    response.headers_mut().set(
        "Permissions-Policy",
        "camera=(), microphone=(), geolocation=()",
    )?;
    if path.starts_with("/admin") || path.starts_with("/api/admin") {
        response
            .headers_mut()
            .set("Cache-Control", "no-store, max-age=0")?;
    } else {
        response
            .headers_mut()
            .set("Cache-Control", "public, max-age=0, must-revalidate")?;
    }
    Ok(())
}

pub fn login_configured(env: &Env) -> bool {
    env_string(env, "ADMIN_LOGIN_EMAIL").is_some()
        && env_string(env, "ADMIN_LOGIN_PASSWORD").is_some()
        && env_string(env, "ADMIN_JWT_SECRET").is_some()
}

pub fn validate_credentials(env: &Env, email: &str, password: &str) -> bool {
    let Some(expected_email) = env_string(env, "ADMIN_LOGIN_EMAIL") else {
        return false;
    };
    let Some(expected_password) = env_string(env, "ADMIN_LOGIN_PASSWORD") else {
        return false;
    };
    expected_email == email && constant_time_eq(expected_password.as_bytes(), password.as_bytes())
}

pub fn authenticate_admin(req: &Request, env: &Env) -> Result<Option<String>> {
    if let Some(email) = req
        .headers()
        .get("cf-access-authenticated-user-email")?
        .map(|value| value.to_lowercase())
    {
        return Ok(Some(email));
    }
    Ok(read_session(req, env)?.map(|claims| claims.email))
}

pub fn read_session(req: &Request, env: &Env) -> Result<Option<SessionClaims>> {
    let Some(token) = cookie(req, COOKIE_NAME) else {
        return Ok(None);
    };
    let Some(secret) = env_string(env, "ADMIN_JWT_SECRET") else {
        return Ok(None);
    };
    let mut parts = token.split('.');
    let Some(payload_b64) = parts.next() else {
        return Ok(None);
    };
    let Some(signature_b64) = parts.next() else {
        return Ok(None);
    };
    if parts.next().is_some() {
        return Ok(None);
    }

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|error| worker::Error::RustError(format!("HMAC init failed: {error}")))?;
    mac.update(payload_b64.as_bytes());
    let expected = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    if !constant_time_eq(expected.as_bytes(), signature_b64.as_bytes()) {
        return Ok(None);
    }

    let payload = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|error| worker::Error::RustError(format!("Invalid session payload: {error}")))?;
    let claims: SessionClaims = serde_json::from_slice(&payload)
        .map_err(|error| worker::Error::RustError(format!("Invalid session json: {error}")))?;
    if claims.exp <= now_unix() {
        return Ok(None);
    }
    Ok(Some(claims))
}

pub fn set_session_cookie(
    req: &Request,
    env: &Env,
    response: &mut Response,
    claims: &SessionClaims,
) -> Result<()> {
    let secret = env_string(env, "ADMIN_JWT_SECRET")
        .ok_or_else(|| worker::Error::RustError("Missing ADMIN_JWT_SECRET".to_string()))?;
    let payload = serde_json::to_vec(claims)
        .map_err(|error| worker::Error::RustError(format!("Session encoding failed: {error}")))?;
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|error| worker::Error::RustError(format!("HMAC init failed: {error}")))?;
    mac.update(payload_b64.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    let value = format!("{payload_b64}.{signature}");
    set_cookie(req, response, COOKIE_NAME, &value, 60 * 60 * 12)
}

pub fn clear_cookie(req: &Request, response: &mut Response) -> Result<()> {
    let secure = req.url()?.scheme() == "https";
    let mut value = format!("{COOKIE_NAME}=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0");
    if secure {
        value.push_str("; Secure");
    }
    response.headers_mut().append("Set-Cookie", &value)?;
    Ok(())
}

fn set_cookie(
    req: &Request,
    response: &mut Response,
    name: &str,
    value: &str,
    max_age: i64,
) -> Result<()> {
    let secure = req.url()?.scheme() == "https";
    let mut header =
        format!("{name}={value}; Path=/; HttpOnly; SameSite=Strict; Max-Age={max_age}");
    if secure {
        header.push_str("; Secure");
    }
    response.headers_mut().append("Set-Cookie", &header)?;
    Ok(())
}

fn cookie(req: &Request, name: &str) -> Option<String> {
    let header = req.headers().get("Cookie").ok().flatten()?;
    header.split(';').find_map(|chunk| {
        let mut parts = chunk.trim().splitn(2, '=');
        let key = parts.next()?;
        let value = parts.next()?;
        if key == name {
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut diff = 0u8;
    for (a, b) in left.iter().zip(right.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}
