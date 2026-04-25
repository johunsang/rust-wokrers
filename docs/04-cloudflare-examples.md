# Cloudflare Examples

Examples are implemented in Rust under `rust-workers/src/biz/ext`.

## APIs

- `GET /api/admin/ext/img/example`
- `GET /api/admin/ext/ai/example`
- `GET /api/admin/ext/ai/workers`
- `POST /api/admin/ext/ai/text`
- `GET /api/admin/ext/agt/example`
- `GET /api/admin/ext/vec/example`
- `GET /api/admin/ext/kv`
- `GET/PUT/DELETE /api/admin/ext/kv/:key`
- `GET/PUT/DELETE /api/admin/ext/r2/:key`

R2 requires enabling the `MEDIA_R2` binding in `rust-workers/wrangler.toml`.
