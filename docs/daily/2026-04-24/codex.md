# 2026-04-24 Codex Work Log

## Rust Worker Router Refactor

- Changed `rust-workers/src/lib.rs` from a manual `match` dispatcher to the `worker::Router` framework API.
- Kept shared fetch-layer behavior intact: admin API authentication, API request logging, and common security headers.
- Added route parameter helpers for typed IDs and string params.
- Updated `rust-workers/README.md` to document the Router-based structure.

## Verification

- `pnpm --filter @rust-wokrers/rust-workers check` passed.

## Full Rust Backend Migration

- Kept frontend as Vite/React and restored concrete `apps/landing` and `apps/admin` Vite apps.
- Removed the legacy TypeScript Worker directory after moving D1 migrations and local env template ownership to `rust-workers/`.
- Added Rust API modules for previously missing TS-only surfaces:
  - `pub`: public pages and releases
  - `vec`: Vectorize reindex/search
  - `agt`: KV-backed operations task API
  - `ext`: KV, R2, AI, Vectorize, image, and agent examples
- Updated root scripts so `pnpm dev` runs Rust Worker plus both Vite apps.
- Set `rust-workers/wrangler.toml` compatibility date to `2026-04-08` so the installed Wrangler runtime can start locally.
- Updated workspace membership to `apps/*`, `packages/*`, and `rust-workers`.
- Replaced backend templates with Rust module templates while keeping Vite admin hook/component templates.
- Updated `AGENTS.md`, `CLAUDE.md`, README, and focused docs for the Rust backend + Vite frontend architecture.

## Verification

- `pnpm install` passed.
- `pnpm check` passed.
- `pnpm build` passed.
- `pnpm dev` started successfully.
- `curl http://localhost:8787/api/health` returned `{"ok":true,"runtime":"cloudflare-workers-rust"}`.

## Local HTTPS

- Added `pnpm cert:local` to generate and trust a local `mkcert` certificate.
- Configured Vite landing/admin dev servers to use `.certs/local.pem` and `.certs/local-key.pem`.
- Configured Wrangler dev to run with `--local-protocol https` on `https://localhost:8787`.
- Updated docs to use `https://localhost:5173`, `https://localhost:5174`, and `https://localhost:8787`.
- Verified HTTPS responses for landing, admin, and `/api/health`.

## English Documentation And GitHub Landing

- Replaced Korean CMS seed content with English pages for fresh installs.
- Added D1 migrations `0011_english_docs_refresh.sql` and `0012_github_landing_settings.sql` so existing local databases get the refreshed English CMS pages and GitHub-first landing copy.
- Converted remaining Korean template comments and sample UI labels to English.
- Reworked the Vite landing app into a GitHub-first project home that links visitors to the repository, releases, CMS guides, and admin console.
- Updated GitHub Actions workflows to use `@rust-wokrers/rust-workers` instead of the removed TypeScript Worker package.
- Updated local proxy documentation from HTTP to HTTPS.

## Verification

- `pnpm db:migrate:local` applied `0011_english_docs_refresh.sql` and `0012_github_landing_settings.sql`.
- `pnpm check` passed.
- `pnpm build` passed, then generated Vite `dist` folders were removed from the workspace.
- `curl -k -I https://localhost:5173` returned `HTTP/2 200`.
- `curl -k https://localhost:8787/api/public/bootstrap` returned the GitHub-first English settings.
- A Korean text scan across README, agent docs, docs, templates, and Rust Worker sources returned no content matches.

## Worker Performance Benchmark

- Added `scripts/bench-workers.mjs` and `pnpm bench:workers` for dependency-free HTTP latency and throughput comparisons.
- Benchmarked the current Hono domain against the existing Rust workers.dev deployment on `/api/health`.
- Documented the baseline in `docs/14-performance-benchmark.md`.
- Fixed root deploy scripts and GitHub Actions deploy commands to use `pnpm --filter @rust-wokrers/rust-workers run deploy`.
- `pnpm build` produced a release-ready Rust Worker bundle, but remote D1 migration and deploy were blocked by Cloudflare `Invalid access token [code: 9109]`.
- `pnpm check` passed after the benchmark/deploy script changes.

## Professional Performance Suite

- Added `autocannon` as a dev dependency.
- Added `bench/workers-suite.json` with deployed Hono and Rust targets, endpoint definitions, and quick/standard/soak profiles.
- Added `scripts/perf-workers.mjs` and `pnpm perf:workers` for connection-ladder benchmarks.
- Refreshed Wrangler OAuth login, applied remote D1 migrations `0011` and `0012`, and deployed a Rust Worker version.
- Verified both deployed runtimes return `200` for `/api/health`, `/api/public/bootstrap`, `/api/public/pages`, and `/getting-started`.
- Ran quick connection-ladder benchmarks after redeploy and wrote:
  - `bench/results/workers-quick-after-deploy-2026-04-24.json`
  - `bench/results/workers-quick-after-deploy-2026-04-24.md`
- Updated `docs/14-performance-benchmark.md` with the same-endpoint results and interpretation.

## 100k User Performance Plan

- Added `bench/k6-workers-100k.js` for a distributed k6 Cloud 100,000 virtual-user ramp.
- Added `bench/README.md` with 100k execution instructions, safety gates, and Hono/Rust commands.
- Added `.github/workflows/perf-100k.yml` as a manual GitHub Actions workflow for audited 100k runs using `K6_CLOUD_TOKEN`.
- Added `pnpm perf:100k:hono` and `pnpm perf:100k:rust` shortcuts for authenticated k6 environments.
- Updated `docs/14-performance-benchmark.md` with 100k ramp stages, think-time guidance, abort conditions, and required evidence.
- Verified JSON/script syntax and `pnpm check`.

## Sample Page Verification

- Audited deployed sample API and CMS routes on the Rust Worker.
- Fixed GET `/contact` returning 405 by adding an explicit GET route that serves the published CMS contact page while preserving POST `/contact` for lead form submissions.
- Redeployed the Rust Worker after the route fix.
- Verified Rust Worker returns 200 for `/api/health`, `/api/public/bootstrap`, `/api/public/pages`, `/`, `/getting-started`, `/about`, `/features`, `/pricing`, and `/contact`.
- `pnpm check` passed after the route fix.
