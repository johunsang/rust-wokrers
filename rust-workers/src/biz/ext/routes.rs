use serde_json::json;
use worker::{Env, Response, Result};

use crate::com::env::env_string;
use crate::com::http::{json_error, json_response};
use crate::com::types::KvValueInput;

pub fn image_example() -> Result<Response> {
    json_response(
        200,
        &json!({
            "image": {
                "title": "Product screenshot",
                "alt": "A clean product dashboard screenshot"
            },
            "api": {
                "create": "/api/admin/images/direct-upload",
                "refresh": "/api/admin/images/:imageId/refresh",
                "update": "/api/admin/images/:imageId",
                "delete": "/api/admin/images/:imageId"
            }
        }),
    )
}

pub fn ai_example() -> Result<Response> {
    json_response(
        200,
        &json!({
            "ai": {
                "objective": "Launch a Cloudflare-native SaaS starter",
                "audience": "Indie hackers and small product teams",
                "tone": "confident and practical"
            },
            "api": "/api/admin/ai/copy"
        }),
    )
}

pub fn ai_workers(env: &Env) -> Result<Response> {
    json_response(
        200,
        &json!({
            "configured": env.ai("AI").is_ok(),
            "textModel": env_string(env, "AI_TEXT_MODEL"),
            "embedModel": env_string(env, "AI_EMBED_MODEL"),
            "vectorizeBound": vectorize_bound(env),
        }),
    )
}

pub async fn ai_text(env: &Env, prompt: Option<String>) -> Result<Response> {
    let prompt = prompt
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            "Give me three concise launch ideas for a Cloudflare-native SaaS product.".to_string()
        });
    let model = env_string(env, "AI_TEXT_MODEL")
        .unwrap_or_else(|| "@cf/meta/llama-3.1-8b-instruct-fast".to_string());
    let ai = match env.ai("AI") {
        Ok(ai) => ai,
        Err(_) => return json_error(503, "Workers AI binding is not configured"),
    };
    let output: serde_json::Value = ai
        .run(
            model,
            json!({
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a product strategist. Return concise, actionable answers."
                    },
                    {
                        "role": "user",
                        "content": prompt
                    }
                ]
            }),
        )
        .await?;
    json_response(200, &json!({ "prompt": prompt, "output": output }))
}

pub fn agent_example() -> Result<Response> {
    json_response(
        200,
        &json!({
            "agent": "OpsAgent",
            "agentPath": "/api/admin/agt",
            "adminApi": {
                "snapshot": "/api/admin/agt",
                "createTask": "/api/admin/agt/tasks",
                "completeTask": "/api/admin/agt/tasks/:id/complete",
                "addNote": "/api/admin/agt/notes",
                "summarize": "/api/admin/agt/summarize"
            },
            "callableMethods": ["addTask", "completeTask", "addNote", "listTasks", "summarizeTasks"]
        }),
    )
}

pub fn vector_example() -> Result<Response> {
    json_response(
        200,
        &json!({
            "vectorize": {
                "query": "Which leads mention enterprise onboarding?"
            },
            "apis": {
                "reindex": "/api/admin/vec/reindex",
                "search": "/api/admin/vec/search"
            }
        }),
    )
}

pub async fn kv_list(env: &Env) -> Result<Response> {
    let kv = env.kv("APP_KV")?;
    let list = kv
        .list()
        .prefix("example:".to_string())
        .limit(20)
        .execute()
        .await?;
    let keys = list
        .keys
        .into_iter()
        .map(|item| item.name)
        .collect::<Vec<_>>();
    json_response(200, &json!({ "keys": keys }))
}

pub async fn kv_get(env: &Env, raw_key: &str) -> Result<Response> {
    let key = example_key(raw_key);
    let value = env.kv("APP_KV")?.get(&key).text().await?;
    json_response(200, &json!({ "key": key, "value": value }))
}

pub async fn kv_put(env: &Env, raw_key: &str, input: KvValueInput) -> Result<Response> {
    if input.value.trim().is_empty() {
        return json_error(400, "value is required");
    }
    let key = example_key(raw_key);
    let mut put = env.kv("APP_KV")?.put(&key, input.value)?;
    if let Some(ttl) = input.expiration_ttl {
        put = put.expiration_ttl(ttl);
    }
    put.execute().await?;
    json_response(200, &json!({ "ok": true, "key": key }))
}

pub async fn kv_delete(env: &Env, raw_key: &str) -> Result<Response> {
    let key = example_key(raw_key);
    env.kv("APP_KV")?.delete(&key).await?;
    json_response(200, &json!({ "ok": true, "key": key }))
}

pub async fn r2_get(env: &Env, raw_key: &str) -> Result<Response> {
    let key = r2_key(raw_key);
    let bucket = match env.bucket("MEDIA_R2") {
        Ok(bucket) => bucket,
        Err(_) => return json_error(503, "MEDIA_R2 binding is not configured"),
    };
    let value = match bucket.get(&key).execute().await? {
        Some(object) => match object.body() {
            Some(body) => Some(body.text().await?),
            None => None,
        },
        None => None,
    };
    json_response(200, &json!({ "key": key, "value": value }))
}

pub async fn r2_put(env: &Env, raw_key: &str, input: KvValueInput) -> Result<Response> {
    if input.value.trim().is_empty() {
        return json_error(400, "value is required");
    }
    let key = r2_key(raw_key);
    let bucket = match env.bucket("MEDIA_R2") {
        Ok(bucket) => bucket,
        Err(_) => return json_error(503, "MEDIA_R2 binding is not configured"),
    };
    bucket.put(&key, input.value).execute().await?;
    json_response(200, &json!({ "ok": true, "key": key }))
}

pub async fn r2_delete(env: &Env, raw_key: &str) -> Result<Response> {
    let key = r2_key(raw_key);
    let bucket = match env.bucket("MEDIA_R2") {
        Ok(bucket) => bucket,
        Err(_) => return json_error(503, "MEDIA_R2 binding is not configured"),
    };
    bucket.delete(&key).await?;
    json_response(200, &json!({ "ok": true, "key": key }))
}

fn vectorize_bound(env: &Env) -> bool {
    js_sys::Reflect::get(env, &wasm_bindgen::JsValue::from_str("DOC_INDEX"))
        .map(|value| !value.is_undefined() && !value.is_null())
        .unwrap_or(false)
}

fn example_key(raw: &str) -> String {
    format!("example:{raw}")
}

fn r2_key(raw: &str) -> String {
    format!("example/{raw}")
}
