# rust-workers

Cloudflare Workers backend written in Rust.

## Responsibilities

- API runtime for Vite frontends
- Session auth and admin API protection
- D1 reads/writes and migrations
- Cloudflare Images, Workers AI, Vectorize, KV, and optional R2 integrations
- Dynamic public CMS pages, `robots.txt`, and `sitemap.xml`

## Structure

```text
src/lib.rs       # fetch entrypoint, worker::Router, auth gate, logs, headers
src/com/         # shared Rust helpers and JSON types
src/biz/*/       # 3-letter business modules
migrations/      # D1 SQL migrations
wrangler.toml    # Cloudflare bindings
```

## Development

```bash
rustup target add wasm32-unknown-unknown
pnpm install
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm cert:local
pnpm db:migrate:local
pnpm dev:rust-workers
```

The default Worker URL is `https://localhost:8787`.

## Route Pattern

Add handlers in `src/biz/{code}/routes.rs`, export the module in `src/biz/mod.rs`, then register routes in `app()` inside `src/lib.rs`.
