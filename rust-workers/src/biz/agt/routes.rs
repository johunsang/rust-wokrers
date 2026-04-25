use serde_json::json;
use worker::{Env, Response, Result};

use crate::com::env::iso_now;
use crate::com::http::{json_error, json_response};
use crate::com::types::{AgentNoteInput, AgentTask, AgentTaskInput, OpsAgentState};

const STATE_KEY: &str = "ops-agent:state";

pub async fn snapshot(env: &Env) -> Result<Response> {
    json_response(200, &load_state(env).await?)
}

pub async fn create_task(env: &Env, input: AgentTaskInput) -> Result<Response> {
    if input.title.trim().len() < 2 {
        return json_error(400, "Task title is too short");
    }
    let mut state = load_state(env).await?;
    let task = AgentTask {
        id: format!("task-{}", js_sys::Date::now().round() as i64),
        title: input.title,
        done: false,
        created_at: iso_now(),
        completed_at: None,
    };
    state.tasks.push(task.clone());
    save_state(env, &state).await?;
    json_response(201, &task)
}

pub async fn complete_task(env: &Env, id: &str) -> Result<Response> {
    let mut state = load_state(env).await?;
    let now = iso_now();
    for task in &mut state.tasks {
        if task.id == id {
            task.done = true;
            task.completed_at = Some(now.clone());
        }
    }
    save_state(env, &state).await?;
    json_response(200, &json!({ "tasks": state.tasks }))
}

pub async fn add_note(env: &Env, input: AgentNoteInput) -> Result<Response> {
    if input.note.trim().len() < 2 {
        return json_error(400, "Note is too short");
    }
    let mut state = load_state(env).await?;
    state.notes.push(input.note);
    save_state(env, &state).await?;
    json_response(200, &json!({ "notes": state.notes }))
}

pub async fn summarize(env: &Env) -> Result<Response> {
    let state = load_state(env).await?;
    let open = state.tasks.iter().filter(|task| !task.done).count();
    let done = state.tasks.iter().filter(|task| task.done).count();
    let latest = state
        .tasks
        .last()
        .map(|task| task.title.clone())
        .unwrap_or_else(|| "No tasks yet".to_string());
    json_response(
        200,
        &json!({
            "summary": format!("{open} open tasks, {done} completed tasks. Latest: {latest}.")
        }),
    )
}

async fn load_state(env: &Env) -> Result<OpsAgentState> {
    let kv = env.kv("APP_KV")?;
    let Some(raw) = kv.get(STATE_KEY).text().await? else {
        return Ok(OpsAgentState::default());
    };
    serde_json::from_str(&raw)
        .map_err(|error| worker::Error::RustError(format!("Invalid OpsAgent state: {error}")))
}

async fn save_state(env: &Env, state: &OpsAgentState) -> Result<()> {
    let raw = serde_json::to_string(state)
        .map_err(|error| worker::Error::RustError(format!("Invalid OpsAgent state: {error}")))?;
    env.kv("APP_KV")?.put(STATE_KEY, raw)?.execute().await?;
    Ok(())
}
