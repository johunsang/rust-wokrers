# Architecture

The project is a pnpm workspace with Vite frontends and a Rust Cloudflare Worker backend.

## Parts

- `apps/landing`: public Vite + React app
- `apps/admin`: authenticated Vite + React console
- `packages/com`: TypeScript contracts for frontend code
- `rust-workers`: Rust Worker API/runtime
- `rust-workers/migrations`: D1 schema and seed data

## Request Flow

1. Browser loads a Vite app.
2. The Vite app calls `/api/*`.
3. In local dev, Vite proxies `/api/*` to `https://localhost:8787`.
4. Cloudflare Worker runs Rust `worker::Router`.
5. Admin APIs pass session auth.
6. Route handlers use D1, KV, R2, Workers AI, Vectorize, or external APIs.
7. API logging and security headers are applied before response.

## Backend Module Convention

- `rust-workers/src/com`: shared infrastructure helpers
- `rust-workers/src/biz/{code}`: business modules
- `rust-workers/src/com/types.rs`: shared Rust response/input structs
- `packages/com/src/contracts.ts`: frontend TypeScript contracts
