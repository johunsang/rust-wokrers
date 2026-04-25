// Add this to rust-workers/src/biz/mod.rs:
// pub mod __item__;

// Add this to rust-workers/src/lib.rs app() router chain:
// .get_async("/api/admin/__items__", |_req, ctx| async move {
//     biz::__item__::routes::list(&ctx.env).await
// })
// .get_async("/api/admin/__items__/:id", |_req, ctx| async move {
//     biz::__item__::routes::get(&ctx.env, param_id(&ctx, "id")?).await
// })
// .post_async("/api/admin/__items__", |mut req, ctx| async move {
//     biz::__item__::routes::create(&ctx.env, req.json().await?).await
// })
// .put_async("/api/admin/__items__/:id", |mut req, ctx| async move {
//     biz::__item__::routes::update(&ctx.env, param_id(&ctx, "id")?, req.json().await?).await
// })
// .delete_async("/api/admin/__items__/:id", |_req, ctx| async move {
//     biz::__item__::routes::delete(&ctx.env, param_id(&ctx, "id")?).await
// })
