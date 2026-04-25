# rust-wokrers — Rust Cloudflare Workers SaaS Starter

`rust-wokrers` is an open-source Cloudflare Workers SaaS starter with a Rust Worker backend, Vite frontends, D1 migrations, admin operations screens, CMS pages, and benchmark tooling.

Build production-ready edge SaaS apps with Rust, Cloudflare Workers, D1, Vite, React, admin auth, CMS pages, API-only mode, and benchmark-ready deployment workflows.

## Keywords

`cloudflare-workers`, `rust`, `worker-rs`, `vite`, `react`, `saas-boilerplate`, `edge-computing`, `cloudflare-d1`, `cloudflare-kv`, `cloudflare-r2`, `workers-ai`, `vectorize`, `admin-dashboard`, `api-starter`, `serverless-rust`

## Stack

- `rust-workers`: Cloudflare Worker written in Rust using `worker::Router`
- `apps/landing`: public Vite + React frontend
- `apps/admin`: Vite + React operations console
- `packages/com`: TypeScript contracts shared by the Vite apps
- `rust-workers/migrations`: D1 schema and seed SQL
- `bench`: performance benchmark suites for deployed Worker targets

## Modes

`APP_MODE` selects the runtime surface:

| Mode | Behavior |
| --- | --- |
| `full` | Serves landing, admin, CMS pages, and APIs. |
| `api-only` | Serves `/api/*` only; non-API routes return JSON service information or JSON 404. |

## Quick Start

```bash
pnpm install
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm cert:local
pnpm db:migrate:local
pnpm dev
```

Local URLs:

- Landing: `https://localhost:5173`
- Admin: `https://localhost:5174`
- Rust Worker API: `https://localhost:8787`

## Scripts

```bash
pnpm dev                         # Rust Worker + both Vite apps
pnpm cert:local                  # install local HTTPS certificate
pnpm dev:rust-workers            # Worker only
pnpm dev:apps                    # Vite apps only
pnpm db:migrate:local            # local D1 migrations
pnpm check                       # Rust + Vite type checks
pnpm build                       # Rust check + Vite production builds
pnpm deploy:prod                 # remote D1 migration + Worker deploy
pnpm perf:workers                # deployed Worker benchmark ladder
```

## Runtime Features

- Rust API routing with shared security headers and API logging
- Full mode and API-only mode
- Session login cookies and admin API protection
- D1-backed settings, leads, media, pages, users, emails, logs
- Cloudflare Images direct-upload flow
- AI Gateway copy generation
- Workers AI text and embedding examples
- Vectorize reindex/search API
- KV and R2 example APIs
- KV-backed operations task API
- Dynamic `robots.txt`, `sitemap.xml`, and public CMS pages

## Documentation

- [Quick Start](./docs/00-quick-start.md)
- [Cloudflare Setup](./docs/01-cloudflare-setup.md)
- [Deployments](./docs/02-deployments.md)
- [Security](./docs/03-security-auth.md)
- [Cloudflare Examples](./docs/04-cloudflare-examples.md)
- [CLI Scaffolding](./docs/05-cli-scaffolding.md)
- [Architecture](./docs/06-architecture.md)
- [API Reference](./docs/07-api-reference.md)
- [Module Guide](./docs/08-module-guide.md)
- [Testing](./docs/09-testing.md)
- [Frontend](./docs/10-frontend.md)
- [Templates](./docs/11-templates.md)
- [Troubleshooting](./docs/12-troubleshooting.md)
- [Complete Usage Guide](./docs/13-complete-usage-guide.md)
- [Performance Benchmark](./docs/14-performance-benchmark.md)
- [Open Source Release Checklist](./docs/15-open-source-release.md)

## GitHub Actions Templates

Workflow templates are stored in [`_templates/github-workflows`](./_templates/github-workflows). Copy them into `.github/workflows/` after you configure repository secrets and deployment targets.

## License

MIT
