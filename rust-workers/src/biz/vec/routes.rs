use js_sys::{Function, Promise};
use serde::Serialize;
use serde_json::{json, Value};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker::{Env, Response, Result};

use crate::com::db::{d1_all, get_site_settings};
use crate::com::env::env_string;
use crate::com::http::{json_error, json_response};
use crate::com::types::{
    LeadRow, MediaAsset, SiteSettings, VectorReindexResponse, VectorSearchInput,
};

#[derive(Debug, Clone, Serialize)]
struct VectorDocument {
    id: String,
    text: String,
    metadata: Value,
}

pub async fn reindex(env: &Env) -> Result<Response> {
    let Some(index) = vectorize_binding(env) else {
        return json_error(503, "Vectorize binding is not configured");
    };
    let docs = build_documents(env).await?;
    let mut vectors = Vec::with_capacity(docs.len());
    for doc in &docs {
        vectors.push(json!({
            "id": doc.id,
            "values": create_embedding(env, &doc.text).await?,
            "metadata": {
                "snippet": doc.text.chars().take(220).collect::<String>(),
                "source": doc.metadata
            },
            "namespace": "app"
        }));
    }
    call_method_one(&index, "upsert", serde_wasm_bindgen::to_value(&vectors)?).await?;
    json_response(
        200,
        &VectorReindexResponse {
            count: vectors.len(),
            ids: docs.into_iter().map(|doc| doc.id).collect(),
        },
    )
}

pub async fn search(env: &Env, input: VectorSearchInput) -> Result<Response> {
    if input.query.trim().len() < 3 {
        return json_error(400, "query must be at least 3 characters");
    }
    let Some(index) = vectorize_binding(env) else {
        return json_error(503, "Vectorize binding is not configured");
    };
    let query_vector = create_embedding(env, &input.query).await?;
    let result = call_method_two(
        &index,
        "query",
        serde_wasm_bindgen::to_value(&query_vector)?,
        serde_wasm_bindgen::to_value(&json!({
            "topK": 8,
            "namespace": "app",
            "returnMetadata": "all"
        }))?,
    )
    .await?;
    json_response(200, &result)
}

async fn build_documents(env: &Env) -> Result<Vec<VectorDocument>> {
    let db = env.d1("DB")?;
    let settings = get_site_settings(&db).await?;
    let leads = d1_all::<LeadRow>(
        &db,
        "SELECT id, name, email, company, message, status, created_at FROM leads ORDER BY id DESC LIMIT 50",
        &[],
    )
    .await?;
    let media = d1_all::<MediaAsset>(
        &db,
        "SELECT id, image_id, title, alt, status, delivery_url, preview_url, uploaded_at FROM media_assets ORDER BY id DESC LIMIT 50",
        &[],
    )
    .await?;

    let mut docs = vec![settings_document(settings)];
    for lead in leads {
        docs.push(VectorDocument {
            id: format!("lead:{}", lead.id),
            text: [
                lead.name.clone(),
                lead.email.clone(),
                lead.company.clone().unwrap_or_default(),
                lead.message.clone().unwrap_or_default(),
            ]
            .join("\n"),
            metadata: json!({
                "type": "lead",
                "title": lead.name,
                "email": lead.email,
                "company": lead.company.unwrap_or_default(),
                "createdAt": lead.created_at
            }),
        });
    }
    for asset in media {
        docs.push(VectorDocument {
            id: format!("media:{}", asset.image_id),
            text: [
                asset.title.clone(),
                asset.alt.clone().unwrap_or_default(),
                asset.image_id.clone(),
            ]
            .join("\n"),
            metadata: json!({
                "type": "media",
                "title": asset.title,
                "imageId": asset.image_id,
                "status": asset.status
            }),
        });
    }
    Ok(docs)
}

fn settings_document(settings: SiteSettings) -> VectorDocument {
    VectorDocument {
        id: "settings:1".to_string(),
        text: [
            settings.brand.clone(),
            settings.hero_label.clone(),
            settings.hero_title.clone(),
            settings.hero_subtitle.clone(),
            settings.cta_primary.clone(),
            settings.cta_secondary.clone(),
        ]
        .join("\n"),
        metadata: json!({
            "type": "settings",
            "title": settings.hero_title,
            "updatedAt": settings.updated_at
        }),
    }
}

async fn create_embedding(env: &Env, text: &str) -> Result<Vec<f64>> {
    let model = env_string(env, "AI_EMBED_MODEL")
        .unwrap_or_else(|| "@cf/baai/bge-small-en-v1.5".to_string());
    let ai = env.ai("AI")?;
    let output: Value = ai.run(model, json!({ "text": text })).await?;
    parse_embedding(output)
}

fn parse_embedding(output: Value) -> Result<Vec<f64>> {
    let data = output.get("data").ok_or_else(|| {
        worker::Error::RustError("Workers AI embedding returned no data".to_string())
    })?;
    let data_array = data.as_array().cloned().unwrap_or_default();
    let vector = if data_array
        .first()
        .and_then(|value| value.as_array())
        .is_some()
    {
        data_array
            .first()
            .and_then(|value| value.as_array())
            .cloned()
            .unwrap_or_default()
    } else {
        data_array
    };
    let values = vector
        .into_iter()
        .filter_map(|value| value.as_f64())
        .collect::<Vec<_>>();
    if values.is_empty() {
        return Err(worker::Error::RustError(
            "Workers AI embedding returned an empty vector".to_string(),
        ));
    }
    Ok(values)
}

fn vectorize_binding(env: &Env) -> Option<JsValue> {
    js_sys::Reflect::get(env, &JsValue::from_str("DOC_INDEX"))
        .ok()
        .filter(|value| !value.is_undefined() && !value.is_null())
}

async fn call_method_one(binding: &JsValue, method: &str, arg: JsValue) -> Result<Value> {
    let function =
        js_sys::Reflect::get(binding, &JsValue::from_str(method))?.dyn_into::<Function>()?;
    let promise = function.call1(binding, &arg)?.dyn_into::<Promise>()?;
    let value = JsFuture::from(promise).await?;
    serde_wasm_bindgen::from_value(value)
        .map_err(|error| worker::Error::RustError(format!("Invalid Vectorize response: {error}")))
}

async fn call_method_two(
    binding: &JsValue,
    method: &str,
    arg_a: JsValue,
    arg_b: JsValue,
) -> Result<Value> {
    let function =
        js_sys::Reflect::get(binding, &JsValue::from_str(method))?.dyn_into::<Function>()?;
    let promise = function
        .call2(binding, &arg_a, &arg_b)?
        .dyn_into::<Promise>()?;
    let value = JsFuture::from(promise).await?;
    serde_wasm_bindgen::from_value(value)
        .map_err(|error| worker::Error::RustError(format!("Invalid Vectorize response: {error}")))
}
