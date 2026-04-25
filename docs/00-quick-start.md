# Quick Start

## Prerequisites

- Node.js 20+
- pnpm 9+
- Rust toolchain with `wasm32-unknown-unknown`
- Cloudflare account for remote resources

## Local Setup

```bash
rustup target add wasm32-unknown-unknown
pnpm install
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm cert:local
pnpm db:migrate:local
pnpm dev
```

## Local URLs

- Landing Vite app: `https://localhost:5173`
- Admin Vite app: `https://localhost:5174`
- Rust Worker API: `https://localhost:8787`

The Vite apps proxy `/api` to the HTTPS Rust Worker during development.
