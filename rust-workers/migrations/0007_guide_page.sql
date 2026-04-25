-- Cloudflare Workers practical guide CMS page.
INSERT INTO pages (slug, title, content_md, content_html, status, published_at, created_at, updated_at)
VALUES (
  'guide',
  'Cloudflare Workers Practical Guide',
  '# Cloudflare Workers Practical Guide

This guide explains the current rust-wokrers development flow: Rust Worker API, Vite frontends, D1 migrations, local HTTPS, and Cloudflare deployment.

## 1. Cloudflare account

- Create or select a Cloudflare account.
- Note the Account ID from the dashboard.
- Create the D1 database, KV namespace, and any optional services you plan to use.
- Keep production secrets in Cloudflare, not in repository files.

## 2. Local tooling

```bash
pnpm install
pnpm cert:local
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm db:migrate:local
pnpm dev
```

The local URLs are HTTPS:

- Landing: https://localhost:5173
- Admin: https://localhost:5174
- Worker API: https://localhost:8787

## 3. Project layout

- `rust-workers/` contains the Rust Cloudflare Worker API and D1 migrations.
- `apps/landing/` contains the public Vite app.
- `apps/admin/` contains the authenticated Vite admin console.
- `packages/com/` contains shared TypeScript contracts for the frontends.
- `docs/` contains operator and developer guides.

## 4. Configuration

Replace placeholder IDs and routes in `rust-workers/wrangler.toml` before staging or production deployment. Add secrets with `wrangler secret put`.

## 5. Development rules

- Keep business modules under `rust-workers/src/biz/{code}/`.
- Add D1 schema changes under `rust-workers/migrations/`.
- Keep shared frontend contracts in `packages/com/src/contracts.ts`.
- Update docs and daily records when behavior, structure, or operations change.

## 6. Validation

Run the full project checks before deployment:

```bash
pnpm check
pnpm test
pnpm build
```

## 7. Deployment

Use `pnpm deploy:staging` for staging and `pnpm deploy:prod` for production after Cloudflare resources, routes, and secrets are ready.',
  '<h1>Cloudflare Workers Practical Guide</h1><p>This guide explains the current rust-wokrers development flow: Rust Worker API, Vite frontends, D1 migrations, local HTTPS, and Cloudflare deployment.</p><h2>1. Cloudflare account</h2><ul><li>Create or select a Cloudflare account.</li><li>Note the Account ID from the dashboard.</li><li>Create the D1 database, KV namespace, and any optional services you plan to use.</li><li>Keep production secrets in Cloudflare, not in repository files.</li></ul><h2>2. Local tooling</h2><pre><code>pnpm install
pnpm cert:local
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm db:migrate:local
pnpm dev</code></pre><p>The local URLs are HTTPS: Landing https://localhost:5173, Admin https://localhost:5174, Worker API https://localhost:8787.</p><h2>3. Project layout</h2><ul><li><code>rust-workers/</code> contains the Rust Cloudflare Worker API and D1 migrations.</li><li><code>apps/landing/</code> contains the public Vite app.</li><li><code>apps/admin/</code> contains the authenticated Vite admin console.</li><li><code>packages/com/</code> contains shared TypeScript contracts for the frontends.</li><li><code>docs/</code> contains operator and developer guides.</li></ul><h2>4. Configuration</h2><p>Replace placeholder IDs and routes in <code>rust-workers/wrangler.toml</code> before staging or production deployment. Add secrets with <code>wrangler secret put</code>.</p><h2>5. Development rules</h2><ul><li>Keep business modules under <code>rust-workers/src/biz/{code}/</code>.</li><li>Add D1 schema changes under <code>rust-workers/migrations/</code>.</li><li>Keep shared frontend contracts in <code>packages/com/src/contracts.ts</code>.</li><li>Update docs and daily records when behavior, structure, or operations change.</li></ul><h2>6. Validation</h2><pre><code>pnpm check
pnpm test
pnpm build</code></pre><h2>7. Deployment</h2><p>Use <code>pnpm deploy:staging</code> for staging and <code>pnpm deploy:prod</code> for production after Cloudflare resources, routes, and secrets are ready.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
)
ON CONFLICT(slug) DO UPDATE SET
  title = excluded.title,
  content_md = excluded.content_md,
  content_html = excluded.content_html,
  status = excluded.status,
  published_at = excluded.published_at,
  updated_at = datetime('now');
