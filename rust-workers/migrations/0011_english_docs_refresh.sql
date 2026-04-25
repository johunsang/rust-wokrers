-- Refresh existing local and deployed CMS rows after the English documentation update.
INSERT INTO pages (slug, title, content_md, content_html, status, published_at, created_at, updated_at) VALUES
(
  'about',
  'About rust-wokrers',
  '# About rust-wokrers

rust-wokrers is a Cloudflare Workers SaaS boilerplate with a Rust API worker, React 19 Vite frontends, D1 storage, and production-oriented operations patterns.

## Stack

- API: Rust Worker
- Frontend: React 19 + Vite
- Database: Cloudflare D1
- Optional services: KV, R2, Cloudflare Images, Workers AI, AI Gateway, Vectorize
- Deployment: Wrangler and GitHub Actions',
  '<h1>About rust-wokrers</h1><p>rust-wokrers is a Cloudflare Workers SaaS boilerplate with a Rust API worker, React 19 Vite frontends, D1 storage, and production-oriented operations patterns.</p><h2>Stack</h2><ul><li>API: Rust Worker</li><li>Frontend: React 19 + Vite</li><li>Database: Cloudflare D1</li><li>Optional services: KV, R2, Cloudflare Images, Workers AI, AI Gateway, Vectorize</li><li>Deployment: Wrangler and GitHub Actions</li></ul>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'features',
  'Core Features',
  '# Core Features

- Public landing app with CMS pages and lead capture.
- Authenticated admin app with dashboard, leads, CMS, settings, media, AI, email, users, logs, and ops panels.
- Rust Worker API with security middleware and D1 repositories.
- Optional Cloudflare integrations for media, AI, search, realtime, KV, and R2.',
  '<h1>Core Features</h1><ul><li>Public landing app with CMS pages and lead capture.</li><li>Authenticated admin app with dashboard, leads, CMS, settings, media, AI, email, users, logs, and ops panels.</li><li>Rust Worker API with security middleware and D1 repositories.</li><li>Optional Cloudflare integrations for media, AI, search, realtime, KV, and R2.</li></ul>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'pricing',
  'Pricing Notes',
  '# Pricing Notes

rust-wokrers itself is MIT licensed. Runtime cost depends on Cloudflare usage.

Use the official Cloudflare pricing pages for final numbers. Estimate Workers, D1, KV, R2, Images, AI Gateway, Workers AI, and Vectorize separately.',
  '<h1>Pricing Notes</h1><p>rust-wokrers itself is MIT licensed. Runtime cost depends on Cloudflare usage.</p><p>Use the official Cloudflare pricing pages for final numbers. Estimate Workers, D1, KV, R2, Images, AI Gateway, Workers AI, and Vectorize separately.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'contact',
  'Contact',
  '# Contact

Replace this placeholder with your own support, sales, and enterprise contact channels.

- Email: hello@example.com
- GitHub: https://github.com/your-org/rust-wokrers',
  '<h1>Contact</h1><p>Replace this placeholder with your own support, sales, and enterprise contact channels.</p><ul><li>Email: hello@example.com</li><li>GitHub: https://github.com/your-org/rust-wokrers</li></ul>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'getting-started',
  'Getting Started',
  '# Getting Started

```bash
pnpm install
pnpm cert:local
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm db:migrate:local
pnpm dev
```

Local URLs:

- Landing: https://localhost:5173
- Admin: https://localhost:5174
- Worker API: https://localhost:8787',
  '<h1>Getting Started</h1><pre><code>pnpm install
pnpm cert:local
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm db:migrate:local
pnpm dev</code></pre><p>Local URLs:</p><ul><li>Landing: https://localhost:5173</li><li>Admin: https://localhost:5174</li><li>Worker API: https://localhost:8787</li></ul>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'guide',
  'Cloudflare Workers Practical Guide',
  '# Cloudflare Workers Practical Guide

This guide covers the current rust-wokrers workflow: Rust Worker API, Vite frontends, D1 migrations, local HTTPS, and Cloudflare deployment.

## Local flow

```bash
pnpm install
pnpm cert:local
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm db:migrate:local
pnpm dev
```

## Validation

```bash
pnpm check
pnpm test
pnpm build
```

## Deployment

Configure `rust-workers/wrangler.toml`, set secrets with `wrangler secret put`, then run `pnpm deploy:staging` or `pnpm deploy:prod`.',
  '<h1>Cloudflare Workers Practical Guide</h1><p>This guide covers the current rust-wokrers workflow: Rust Worker API, Vite frontends, D1 migrations, local HTTPS, and Cloudflare deployment.</p><h2>Local flow</h2><pre><code>pnpm install
pnpm cert:local
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm db:migrate:local
pnpm dev</code></pre><h2>Validation</h2><pre><code>pnpm check
pnpm test
pnpm build</code></pre><h2>Deployment</h2><p>Configure <code>rust-workers/wrangler.toml</code>, set secrets with <code>wrangler secret put</code>, then run <code>pnpm deploy:staging</code> or <code>pnpm deploy:prod</code>.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'pricing-guide',
  'Cloudflare Pricing Guide',
  '# Cloudflare Pricing Guide

Verify current Cloudflare prices before using this guide for budgets.

Estimate Workers, D1, KV, R2, Images, Workers AI, AI Gateway, Vectorize, logs, and staging separately. Review analytics after launch and update the model with measured usage.',
  '<h1>Cloudflare Pricing Guide</h1><p>Verify current Cloudflare prices before using this guide for budgets.</p><p>Estimate Workers, D1, KV, R2, Images, Workers AI, AI Gateway, Vectorize, logs, and staging separately. Review analytics after launch and update the model with measured usage.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'ai-dev-guide',
  'AI Development Guide: Claude Code and Codex',
  '# AI Development Guide: Claude Code and Codex

Agents should read `AGENTS.md` and `CLAUDE.md`, keep them synchronized, make scoped changes, run checks, and record important work in `docs/daily/YYYY-MM-DD/`.

Recommended validation:

```bash
pnpm check
pnpm test
pnpm build
```',
  '<h1>AI Development Guide: Claude Code and Codex</h1><p>Agents should read <code>AGENTS.md</code> and <code>CLAUDE.md</code>, keep them synchronized, make scoped changes, run checks, and record important work in <code>docs/daily/YYYY-MM-DD/</code>.</p><h2>Recommended validation</h2><pre><code>pnpm check
pnpm test
pnpm build</code></pre>',
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
