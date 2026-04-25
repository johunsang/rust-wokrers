# rust-wokrers Agent Guide

This repository is now a Rust Worker backend with Vite frontends.

## Project Structure

- `/rust-workers/` — Cloudflare Worker backend written in Rust
- `/rust-workers/src/lib.rs` — `worker::Router` registration, fetch entrypoint, auth gate, API logging, security headers
- `/rust-workers/src/com/` — shared Rust infrastructure helpers
- `/rust-workers/src/biz/{3-letter-code}/` — Rust business modules
- `/rust-workers/migrations/` — D1 migrations
- `/apps/landing/` — public Vite + React app
- `/apps/admin/` — authenticated Vite + React operations console
- `/packages/com/` — TypeScript contracts for Vite apps
- `/_templates/` — Rust module, Vite hook/component, migration, and theme templates
- `/docs/` — operating docs

## Development Rules

- `AGENTS.md` and `CLAUDE.md` must stay identical.
- Backend runtime code belongs in `rust-workers`, not in a TypeScript Worker.
- Frontend code remains Vite/React under `apps/*`.
- Shared frontend API shapes belong in `packages/com`.
- Business backend modules use 3-letter codes under `rust-workers/src/biz/`.
- Common backend helpers belong in `rust-workers/src/com/`.
- D1 schema changes are added as `rust-workers/migrations/NNNN_*.sql`.
- Update docs immediately when runtime structure, scripts, APIs, bindings, or module rules change.
- Record major work in `docs/daily/YYYY-MM-DD/codex.md` or `claude.md`.

## Commands

```bash
pnpm install
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm cert:local
pnpm db:migrate:local
pnpm dev
```

```bash
pnpm dev:rust-workers
pnpm dev:apps
pnpm check
pnpm build
pnpm deploy:staging
pnpm deploy:prod
```

Local URLs:

- Landing: `https://localhost:5173`
- Admin: `https://localhost:5174`
- Rust Worker: `https://localhost:8787`

## Backend Rules

- Use `worker::Router` in `rust-workers/src/lib.rs`.
- Route handlers live in `rust-workers/src/biz/{code}/routes.rs`.
- Use `worker::{Env, Request, Response, Result}`.
- `APP_MODE=full` serves UI and API routes; `APP_MODE=api-only` serves `/api/*` and disables UI routes.
- Use D1 prepared statements only.
- Reuse helpers from `com/db.rs`, `com/http.rs`, `com/env.rs`, `com/security.rs`, and `com/net.rs`.
- JSON response types should be serializable Rust structs in `com/types.rs` when shared across modules.
- Add route registrations in `app()` and keep auth-protected admin APIs under `/api/admin/*`.

## Frontend Rules

- Vite apps are the frontend source of truth.
- Landing reads public APIs under `/api/public/*`.
- Admin reads authenticated APIs under `/api/auth/*` and `/api/admin/*`.
- During Vite dev, proxy `/api` to `https://localhost:8787`.
- Use `packages/com` types instead of duplicating API shapes.
- Keep app UI practical and workflow-focused.

## Cloudflare Bindings

| Binding | Type | Purpose |
| --- | --- | --- |
| `DB` | D1 | settings, leads, media, pages, users, logs |
| `APP_KV` | KV | examples and Ops Agent-compatible state |
| `AI` | Workers AI | text and embeddings |
| `DOC_INDEX` | Vectorize | semantic search |
| `MEDIA_R2` | R2 | optional object storage example |

## Current API Modules

| Code | Area |
| --- | --- |
| `hlt` | health |
| `aut` | session auth |
| `pub` | public bootstrap, leads, pages, releases, SEO |
| `dsh` | dashboard |
| `set` | site settings |
| `led` | lead CRM |
| `med` | Cloudflare Images metadata/direct upload |
| `aid` | AI copy |
| `eml` | email templates/logs/send |
| `pag` | CMS pages |
| `srh` | SQL search |
| `vec` | Vectorize search |
| `agt` | operations task API |
| `usr` | admin users |
| `log` | access/API/system logs |
| `ext` | KV/R2/AI/example APIs |

## Daily Record

- Store work records under `docs/daily/YYYY-MM-DD/`.
- Use `codex.md` for Codex work and `claude.md` for Claude work.
- Include structural changes, file movement/deletion, and verification results.
