use serde_json::json;
use worker::{Response, Result};

use crate::com::http::json_response;

pub fn health() -> Result<Response> {
    json_response(
        200,
        &json!({ "ok": true, "runtime": "cloudflare-workers-rust" }),
    )
}
