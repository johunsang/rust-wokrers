use worker::{Env, Response, Result};

use crate::com::db::get_site_settings;
use crate::com::env::env_string;
use crate::com::http::json_error;
use crate::com::net::send_json_request;
use crate::com::types::{AiCopySuggestion, AiCopySuggestionRequest};

pub async fn copy(env: &Env, payload: AiCopySuggestionRequest) -> Result<Response> {
    let api_key = match env_string(env, "AI_PROVIDER_API_KEY") {
        Some(value) => value,
        None => return json_error(503, "AI Gateway is not configured"),
    };
    let account_id = env_string(env, "AI_GATEWAY_ACCOUNT_ID")
        .or_else(|| env_string(env, "CLOUDFLARE_ACCOUNT_ID"))
        .unwrap_or_default();
    let gateway_id = env_string(env, "AI_GATEWAY_ID").unwrap_or_else(|| "default".to_string());
    let provider = env_string(env, "AI_PROVIDER").unwrap_or_else(|| "workers-ai".to_string());
    let model =
        env_string(env, "AI_MODEL").unwrap_or_else(|| "@cf/meta/llama-3.1-8b-instruct".to_string());
    let settings = get_site_settings(&env.d1("DB")?).await?;
    let url = format!(
        "https://gateway.ai.cloudflare.com/v1/{account_id}/{gateway_id}/compat/chat/completions"
    );
    let response = send_json_request(
        &url,
        worker::Method::Post,
        Some(serde_json::json!({
            "model": format!("{provider}/{model}"),
            "temperature": 0.8,
            "messages": [
                {
                    "role": "system",
                    "content": "You write conversion-focused SaaS landing copy. Return strict JSON with heroTitle, heroSubtitle, ctaPrimary, rationale."
                },
                {
                    "role": "user",
                    "content": serde_json::json!({
                        "current": settings,
                        "objective": payload.objective,
                        "audience": payload.audience,
                        "tone": payload.tone
                    }).to_string()
                }
            ],
            "response_format": { "type": "json_object" }
        })),
        &[
            ("content-type", "application/json"),
            ("authorization", &format!("Bearer {api_key}")),
        ],
    )
    .await?;
    let content = response
        .get("choices")
        .and_then(|v| v.as_array())
        .and_then(|v| v.first())
        .and_then(|v| v.get("message"))
        .and_then(|v| v.get("content"))
        .and_then(|v| v.as_str());
    let Some(content) = content else {
        return json_error(503, "AI Gateway returned no content");
    };
    let suggestion: AiCopySuggestion = serde_json::from_str(content)
        .map_err(|e| worker::Error::RustError(format!("Invalid AI response: {e}")))?;
    crate::com::http::json_response(200, &suggestion)
}
