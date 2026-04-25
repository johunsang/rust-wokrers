# API Reference

The API is implemented in Rust under `rust-workers/src/biz`.

## Public

- `GET /api/health`
- `GET /api/public/bootstrap`
- `POST /api/public/leads`
- `GET /api/public/pages`
- `GET /api/public/pages/:slug`
- `GET /api/public/releases`
- `GET /robots.txt`
- `GET /sitemap.xml`

## Auth

- `GET /api/auth/me`
- `POST /api/auth/login`
- `POST /api/auth/logout`

## Admin

- `GET /api/admin/dashboard`
- `GET/PUT /api/admin/settings`
- `GET /api/admin/leads`
- `GET /api/admin/leads/:id`
- `PUT /api/admin/leads/:id/status`
- `POST /api/admin/leads/:id/tags`
- `DELETE /api/admin/leads/:id/tags/:tag`
- `GET/POST /api/admin/leads/:id/notes`
- `GET/POST /api/admin/pages`
- `GET/PUT/DELETE /api/admin/pages/:id`
- `POST /api/admin/pages/:id/publish`
- `POST /api/admin/pages/:id/unpublish`
- `GET /api/admin/images`
- `POST /api/admin/images/direct-upload`
- `POST /api/admin/images/:image_id/refresh`
- `PUT/DELETE /api/admin/images/:image_id`
- `POST /api/admin/ai/copy`
- `POST /api/admin/vec/reindex`
- `POST /api/admin/vec/search`
- `GET/POST /api/admin/email/templates`
- `GET/PUT/DELETE /api/admin/email/templates/:id`
- `POST /api/admin/email/send`
- `GET /api/admin/email/logs`
- `GET/POST /api/admin/users`
- `GET/PUT/DELETE /api/admin/users/:id`
- `PUT /api/admin/users/:id/toggle`
- `GET /api/admin/logs/access`
- `GET /api/admin/logs/api`
- `GET /api/admin/logs/stats`
- `GET /api/admin/agt`
- `POST /api/admin/agt/tasks`
- `POST /api/admin/agt/tasks/:id/complete`
- `POST /api/admin/agt/notes`
- `POST /api/admin/agt/summarize`
- `GET/PUT/DELETE /api/admin/ext/kv/:key`
- `GET/PUT/DELETE /api/admin/ext/r2/:key`
