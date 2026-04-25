mod biz;
mod com;

use worker::{event, Context, Env, Method, Request, Response, Result, RouteContext, Router};

#[event(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let path = req.path();
    let method = req.method().to_string();
    let started = js_sys::Date::now();

    if is_api_only(&env) && !path.starts_with("/api/") {
        let mut response = api_only_response(&path)?;
        com::security::apply_common_headers(&path, &mut response)?;
        return Ok(response);
    }

    if path.starts_with("/api/admin/") && biz::aut::routes::current_admin(&req, &env)?.is_none() {
        let mut response = com::http::json_error(401, "Authentication required")?;
        com::security::apply_common_headers(&path, &mut response)?;
        return Ok(response);
    }

    let mut response = app().run(req, env.clone()).await?;

    if path.starts_with("/api/") {
        let duration_ms = (js_sys::Date::now() - started).round() as i64;
        if let Ok(db) = env.d1("DB") {
            let _ = biz::log::routes::log_api_request(
                &db,
                &method,
                &path,
                response.status_code() as i64,
                duration_ms,
            )
            .await;
        }
    }

    com::security::apply_common_headers(&path, &mut response)?;
    Ok(response)
}

fn app() -> Router<'static, ()> {
    Router::new()
        .get_async("/", |req, ctx| async move {
            biz::r#pub::routes::landing_page(&req, &ctx.env).await
        })
        .get_async("/contact", |req, ctx| async move {
            match biz::pag::routes::published_page(&req, &ctx.env).await? {
                Some(response) => Ok(response),
                None => biz::r#pub::routes::landing_page(&req, &ctx.env).await,
            }
        })
        .post_async("/contact", |mut req, ctx| async move {
            biz::r#pub::routes::contact_submit(&mut req, &ctx.env).await
        })
        .get_async("/admin", |req, ctx| async move {
            biz::dsh::routes::admin_page(&req, &ctx.env).await
        })
        .get_async("/admin/", |req, ctx| async move {
            biz::dsh::routes::admin_page(&req, &ctx.env).await
        })
        .post_async("/admin/login", |mut req, ctx| async move {
            biz::aut::routes::admin_login_form(&mut req, &ctx.env).await
        })
        .post("/admin/logout", |req, _ctx| {
            biz::aut::routes::admin_logout_form(&req)
        })
        .get("/robots.txt", |req, _ctx| biz::r#pub::routes::robots(&req))
        .get_async("/sitemap.xml", |_req, ctx| async move {
            biz::r#pub::routes::sitemap(&ctx.env).await
        })
        .get("/api/health", |_req, _ctx| biz::hlt::routes::health())
        .get_async("/api/public/bootstrap", |_req, ctx| async move {
            biz::r#pub::routes::public_bootstrap(&ctx.env).await
        })
        .post_async("/api/public/leads", |mut req, ctx| async move {
            biz::r#pub::routes::public_lead(&mut req, &ctx.env).await
        })
        .get_async("/api/public/pages", |_req, ctx| async move {
            biz::r#pub::routes::public_pages(&ctx.env).await
        })
        .get_async("/api/public/pages/:slug", |_req, ctx| async move {
            biz::r#pub::routes::public_page_by_slug(&ctx.env, param(&ctx, "slug")?).await
        })
        .get_async("/api/public/releases", |_req, ctx| async move {
            biz::r#pub::routes::releases(&ctx.env).await
        })
        .get("/api/auth/me", |req, ctx| {
            biz::aut::routes::auth_me(&req, &ctx.env)
        })
        .post_async("/api/auth/login", |mut req, ctx| async move {
            biz::aut::routes::auth_login(&mut req, &ctx.env).await
        })
        .post("/api/auth/logout", |req, _ctx| {
            biz::aut::routes::auth_logout(&req)
        })
        .get_async("/api/admin/dashboard", |req, ctx| async move {
            biz::dsh::routes::dashboard_api(&req, &ctx.env).await
        })
        .get_async("/api/admin/settings", |_req, ctx| async move {
            biz::set::routes::get_settings(&ctx.env).await
        })
        .put_async("/api/admin/settings", |mut req, ctx| async move {
            biz::set::routes::update_settings(&ctx.env, req.json().await?).await
        })
        .get_async("/api/admin/leads", |_req, ctx| async move {
            biz::led::routes::list_leads(&ctx.env).await
        })
        .get_async("/api/admin/leads/:id", |_req, ctx| async move {
            biz::led::routes::get_lead(&ctx.env, param_id(&ctx, "id")?).await
        })
        .put_async("/api/admin/leads/:id/status", |mut req, ctx| async move {
            let payload: com::types::StatusInput = req.json().await?;
            biz::led::routes::update_status(&ctx.env, param_id(&ctx, "id")?, &payload.status).await
        })
        .post_async("/api/admin/leads/:id/tags", |mut req, ctx| async move {
            let payload: com::types::TagInput = req.json().await?;
            biz::led::routes::add_tag(&ctx.env, param_id(&ctx, "id")?, &payload.tag).await
        })
        .delete_async("/api/admin/leads/:id/tags/:tag", |_req, ctx| async move {
            biz::led::routes::delete_tag(&ctx.env, param_id(&ctx, "id")?, param(&ctx, "tag")?).await
        })
        .get_async("/api/admin/leads/:id/notes", |_req, ctx| async move {
            biz::led::routes::list_notes(&ctx.env, param_id(&ctx, "id")?).await
        })
        .post_async("/api/admin/leads/:id/notes", |mut req, ctx| async move {
            let payload: com::types::NoteInput = req.json().await?;
            let created_by = payload.created_by.unwrap_or_else(|| "admin".to_string());
            biz::led::routes::add_note(
                &ctx.env,
                param_id(&ctx, "id")?,
                &payload.content,
                &created_by,
            )
            .await
        })
        .get_async("/api/admin/users", |_req, ctx| async move {
            biz::usr::routes::list_users(&ctx.env).await
        })
        .get_async("/api/admin/users/:id", |_req, ctx| async move {
            biz::usr::routes::get_user(&ctx.env, param_id(&ctx, "id")?).await
        })
        .post_async("/api/admin/users", |mut req, ctx| async move {
            biz::usr::routes::create_user(&ctx.env, req.json().await?).await
        })
        .put_async("/api/admin/users/:id", |mut req, ctx| async move {
            biz::usr::routes::update_user(&ctx.env, param_id(&ctx, "id")?, req.json().await?).await
        })
        .put_async("/api/admin/users/:id/toggle", |_req, ctx| async move {
            biz::usr::routes::toggle_user(&ctx.env, param_id(&ctx, "id")?).await
        })
        .delete_async("/api/admin/users/:id", |_req, ctx| async move {
            biz::usr::routes::delete_user(&ctx.env, param_id(&ctx, "id")?).await
        })
        .get_async("/api/admin/logs/access", |req, ctx| async move {
            biz::log::routes::list_access(&ctx.env, query_limit(&req, 50, 200)).await
        })
        .get_async("/api/admin/logs/api", |req, ctx| async move {
            biz::log::routes::list_api(&ctx.env, query_limit(&req, 50, 200)).await
        })
        .get_async("/api/admin/logs/stats", |_req, ctx| async move {
            biz::log::routes::stats(&ctx.env).await
        })
        .get_async("/api/admin/search", |req, ctx| async move {
            let query = com::http::query_value(&req, "q").unwrap_or_default();
            biz::srh::routes::search(&ctx.env, &query).await
        })
        .get_async("/api/admin/images", |_req, ctx| async move {
            biz::med::routes::list_media(&ctx.env).await
        })
        .post_async(
            "/api/admin/images/direct-upload",
            |mut req, ctx| async move {
                biz::med::routes::direct_upload(&ctx.env, req.json().await?).await
            },
        )
        .post_async(
            "/api/admin/images/:image_id/refresh",
            |_req, ctx| async move {
                biz::med::routes::refresh_media(&ctx.env, param(&ctx, "image_id")?).await
            },
        )
        .put_async("/api/admin/images/:image_id", |mut req, ctx| async move {
            biz::med::routes::update_media(&ctx.env, param(&ctx, "image_id")?, req.json().await?)
                .await
        })
        .delete_async("/api/admin/images/:image_id", |_req, ctx| async move {
            biz::med::routes::delete_media(&ctx.env, param(&ctx, "image_id")?).await
        })
        .get_async("/api/admin/pages", |_req, ctx| async move {
            biz::pag::routes::list_pages(&ctx.env).await
        })
        .get_async("/api/admin/pages/:id", |_req, ctx| async move {
            biz::pag::routes::get_page_by_id(&ctx.env, param_id(&ctx, "id")?).await
        })
        .post_async("/api/admin/pages", |mut req, ctx| async move {
            biz::pag::routes::create_page(&ctx.env, req.json().await?).await
        })
        .put_async("/api/admin/pages/:id", |mut req, ctx| async move {
            biz::pag::routes::update_page(&ctx.env, param_id(&ctx, "id")?, req.json().await?).await
        })
        .post_async("/api/admin/pages/:id/publish", |_req, ctx| async move {
            biz::pag::routes::publish_page(&ctx.env, param_id(&ctx, "id")?).await
        })
        .post_async("/api/admin/pages/:id/unpublish", |_req, ctx| async move {
            biz::pag::routes::unpublish_page(&ctx.env, param_id(&ctx, "id")?).await
        })
        .delete_async("/api/admin/pages/:id", |_req, ctx| async move {
            biz::pag::routes::delete_page(&ctx.env, param_id(&ctx, "id")?).await
        })
        .get_async("/api/admin/email/templates", |_req, ctx| async move {
            biz::eml::routes::list_templates(&ctx.env).await
        })
        .get_async("/api/admin/email/templates/:id", |_req, ctx| async move {
            biz::eml::routes::get_template(&ctx.env, param_id(&ctx, "id")?).await
        })
        .post_async("/api/admin/email/templates", |mut req, ctx| async move {
            biz::eml::routes::create_template(&ctx.env, req.json().await?).await
        })
        .put_async(
            "/api/admin/email/templates/:id",
            |mut req, ctx| async move {
                biz::eml::routes::update_template(
                    &ctx.env,
                    param_id(&ctx, "id")?,
                    req.json().await?,
                )
                .await
            },
        )
        .delete_async("/api/admin/email/templates/:id", |_req, ctx| async move {
            biz::eml::routes::delete_template(&ctx.env, param_id(&ctx, "id")?).await
        })
        .get_async("/api/admin/email/logs", |_req, ctx| async move {
            biz::eml::routes::list_logs(&ctx.env).await
        })
        .post_async("/api/admin/email/send", |mut req, ctx| async move {
            biz::eml::routes::send_email(&ctx.env, req.json().await?).await
        })
        .post_async("/api/admin/ai/copy", |mut req, ctx| async move {
            biz::aid::routes::copy(&ctx.env, req.json().await?).await
        })
        .post_async("/api/admin/vec/reindex", |_req, ctx| async move {
            biz::vec::routes::reindex(&ctx.env).await
        })
        .post_async("/api/admin/vec/search", |mut req, ctx| async move {
            biz::vec::routes::search(&ctx.env, req.json().await?).await
        })
        .get_async("/api/admin/agt", |_req, ctx| async move {
            biz::agt::routes::snapshot(&ctx.env).await
        })
        .post_async("/api/admin/agt/tasks", |mut req, ctx| async move {
            biz::agt::routes::create_task(&ctx.env, req.json().await?).await
        })
        .post_async(
            "/api/admin/agt/tasks/:id/complete",
            |_req, ctx| async move {
                biz::agt::routes::complete_task(&ctx.env, param(&ctx, "id")?).await
            },
        )
        .post_async("/api/admin/agt/notes", |mut req, ctx| async move {
            biz::agt::routes::add_note(&ctx.env, req.json().await?).await
        })
        .post_async("/api/admin/agt/summarize", |_req, ctx| async move {
            biz::agt::routes::summarize(&ctx.env).await
        })
        .get("/api/admin/ext/img/example", |_req, _ctx| {
            biz::ext::routes::image_example()
        })
        .get("/api/admin/ext/ai/example", |_req, _ctx| {
            biz::ext::routes::ai_example()
        })
        .get("/api/admin/ext/ai/workers", |_req, ctx| {
            biz::ext::routes::ai_workers(&ctx.env)
        })
        .post_async("/api/admin/ext/ai/text", |mut req, ctx| async move {
            let payload = req.json::<serde_json::Value>().await.unwrap_or_default();
            let prompt = payload
                .get("prompt")
                .and_then(|value| value.as_str())
                .map(str::to_string);
            biz::ext::routes::ai_text(&ctx.env, prompt).await
        })
        .get("/api/admin/ext/agt/example", |_req, _ctx| {
            biz::ext::routes::agent_example()
        })
        .get("/api/admin/ext/vec/example", |_req, _ctx| {
            biz::ext::routes::vector_example()
        })
        .get_async("/api/admin/ext/kv", |_req, ctx| async move {
            biz::ext::routes::kv_list(&ctx.env).await
        })
        .get_async("/api/admin/ext/kv/:key", |_req, ctx| async move {
            biz::ext::routes::kv_get(&ctx.env, param(&ctx, "key")?).await
        })
        .put_async("/api/admin/ext/kv/:key", |mut req, ctx| async move {
            biz::ext::routes::kv_put(&ctx.env, param(&ctx, "key")?, req.json().await?).await
        })
        .delete_async("/api/admin/ext/kv/:key", |_req, ctx| async move {
            biz::ext::routes::kv_delete(&ctx.env, param(&ctx, "key")?).await
        })
        .get_async("/api/admin/ext/r2/:key", |_req, ctx| async move {
            biz::ext::routes::r2_get(&ctx.env, param(&ctx, "key")?).await
        })
        .put_async("/api/admin/ext/r2/:key", |mut req, ctx| async move {
            biz::ext::routes::r2_put(&ctx.env, param(&ctx, "key")?, req.json().await?).await
        })
        .delete_async("/api/admin/ext/r2/:key", |_req, ctx| async move {
            biz::ext::routes::r2_delete(&ctx.env, param(&ctx, "key")?).await
        })
        .or_else_any_method_async("/*path", |req, ctx| async move {
            if req.method() == Method::Get {
                if let Some(response) = biz::pag::routes::published_page(&req, &ctx.env).await? {
                    return Ok(response);
                }
            }
            com::http::json_error(404, "Not found")
        })
}

fn param<'a>(ctx: &'a RouteContext<()>, key: &str) -> Result<&'a str> {
    ctx.param(key)
        .map(String::as_str)
        .ok_or_else(|| worker::Error::RustError(format!("Missing route parameter: {key}")))
}

fn param_id(ctx: &RouteContext<()>, key: &str) -> Result<i64> {
    param(ctx, key)?
        .parse::<i64>()
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| worker::Error::RustError(format!("Invalid route parameter: {key}")))
}

fn query_limit(req: &Request, default: i64, max: i64) -> i64 {
    com::http::query_value(req, "limit")
        .and_then(|value| value.parse::<i64>().ok())
        .map(|value| value.clamp(1, max))
        .unwrap_or(default)
}

fn is_api_only(env: &Env) -> bool {
    com::env::env_string(env, "APP_MODE")
        .map(|value| value.eq_ignore_ascii_case("api-only"))
        .unwrap_or(false)
}

fn api_only_response(path: &str) -> Result<Response> {
    if path == "/" {
        return com::http::json_response(
            200,
            &serde_json::json!({
                "ok": true,
                "mode": "api-only",
                "health": "/api/health",
                "public": ["/api/public/bootstrap", "/api/public/pages"],
                "admin": "/api/admin/*"
            }),
        );
    }

    com::http::json_response(
        404,
        &serde_json::json!({
            "error": "UI routes are disabled because APP_MODE is api-only",
            "mode": "api-only",
            "path": path
        }),
    )
}
