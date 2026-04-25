-- Sample English CMS pages for fresh local and production databases.
INSERT INTO pages (slug, title, content_md, content_html, status, published_at, created_at, updated_at) VALUES
(
  'about',
  'About rust-wokrers',
  '# About rust-wokrers

rust-wokrers is a Cloudflare Workers SaaS boilerplate with a Rust API worker, React 19 Vite frontends, D1 storage, and production-oriented operations patterns.

## Why use it

- Full-stack edge runtime for API, static assets, database access, media, and optional AI.
- Production baseline with authentication, security headers, CORS, CSP, and rate limiting.
- Modular business structure using short three-letter module codes.
- Staging and production deployment through Wrangler and GitHub Actions.

## Stack

| Layer | Technology |
| --- | --- |
| API | Rust Worker |
| Frontend | React 19 + Vite |
| Database | Cloudflare D1 |
| Media | Cloudflare Images and optional R2 |
| AI | Workers AI, AI Gateway, and optional Vectorize |
| Deployment | Wrangler and GitHub Actions |

Start with the quick-start guide, then customize the modules you need.',
  '<h1>About rust-wokrers</h1><p>rust-wokrers is a Cloudflare Workers SaaS boilerplate with a Rust API worker, React 19 Vite frontends, D1 storage, and production-oriented operations patterns.</p><h2>Why use it</h2><ul><li>Full-stack edge runtime for API, static assets, database access, media, and optional AI.</li><li>Production baseline with authentication, security headers, CORS, CSP, and rate limiting.</li><li>Modular business structure using short three-letter module codes.</li><li>Staging and production deployment through Wrangler and GitHub Actions.</li></ul><h2>Stack</h2><p>API: Rust Worker / Frontend: React 19 + Vite / Database: Cloudflare D1 / Media: Cloudflare Images and optional R2 / AI: Workers AI, AI Gateway, and optional Vectorize / Deployment: Wrangler and GitHub Actions.</p><p>Start with the quick-start guide, then customize the modules you need.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'features',
  'Core Features',
  '# Core Features

## Landing app
The public Vite app includes a marketing home page, feature sections, public CMS page rendering, and lead capture.

## Admin app
The admin Vite app includes login, dashboard metrics, lead CRM, CMS page management, site settings, media tools, AI copy generation, email tools, users, logs, and ops panels.

## Rust API
The backend is a Rust Cloudflare Worker using `worker::Router`. It exposes public APIs, admin APIs, security middleware, D1 repositories, and optional Cloudflare service integrations.

## Database
D1 migrations live in `rust-workers/migrations`. All runtime data access should use prepared statements.

## Media and AI
Cloudflare Images, R2, Workers AI, AI Gateway, and Vectorize can be enabled when a product needs them.',
  '<h1>Core Features</h1><h2>Landing app</h2><p>The public Vite app includes a marketing home page, feature sections, public CMS page rendering, and lead capture.</p><h2>Admin app</h2><p>The admin Vite app includes login, dashboard metrics, lead CRM, CMS page management, site settings, media tools, AI copy generation, email tools, users, logs, and ops panels.</p><h2>Rust API</h2><p>The backend is a Rust Cloudflare Worker using <code>worker::Router</code>. It exposes public APIs, admin APIs, security middleware, D1 repositories, and optional Cloudflare service integrations.</p><h2>Database</h2><p>D1 migrations live in <code>rust-workers/migrations</code>. All runtime data access should use prepared statements.</p><h2>Media and AI</h2><p>Cloudflare Images, R2, Workers AI, AI Gateway, and Vectorize can be enabled when a product needs them.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'pricing',
  'Pricing Notes',
  '# Pricing Notes

rust-wokrers itself is an MIT-licensed boilerplate. Runtime cost depends on Cloudflare usage.

## Common starting point

- Workers Free can cover early prototypes and demos.
- Workers Paid starts at the monthly Workers base plan and unlocks higher limits.
- D1, KV, R2, Images, Workers AI, AI Gateway, and Vectorize each have separate quotas and overage rules.

## Recommendation

Begin with the smallest Cloudflare plan that matches your traffic. Move to paid Workers before production launch if you need higher CPU limits, more requests, Durable Objects, or advanced observability.

Always confirm current Cloudflare pricing before quoting customers or estimating a production budget.',
  '<h1>Pricing Notes</h1><p>rust-wokrers itself is an MIT-licensed boilerplate. Runtime cost depends on Cloudflare usage.</p><h2>Common starting point</h2><ul><li>Workers Free can cover early prototypes and demos.</li><li>Workers Paid starts at the monthly Workers base plan and unlocks higher limits.</li><li>D1, KV, R2, Images, Workers AI, AI Gateway, and Vectorize each have separate quotas and overage rules.</li></ul><h2>Recommendation</h2><p>Begin with the smallest Cloudflare plan that matches your traffic. Move to paid Workers before production launch if you need higher CPU limits, more requests, Durable Objects, or advanced observability.</p><p>Always confirm current Cloudflare pricing before quoting customers or estimating a production budget.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'contact',
  'Contact',
  '# Contact

Use this page as a placeholder for your own support and sales channels.

## General contact
- Email: hello@example.com
- GitHub: https://github.com/your-org/rust-wokrers

## Technical support
- Track bugs and feature requests in GitHub Issues.
- Keep deployment and incident notes in `docs/daily/YYYY-MM-DD/`.

## Enterprise work
Add your consulting, onboarding, SLA, and dedicated support details here.',
  '<h1>Contact</h1><p>Use this page as a placeholder for your own support and sales channels.</p><h2>General contact</h2><ul><li>Email: hello@example.com</li><li>GitHub: https://github.com/your-org/rust-wokrers</li></ul><h2>Technical support</h2><ul><li>Track bugs and feature requests in GitHub Issues.</li><li>Keep deployment and incident notes in <code>docs/daily/YYYY-MM-DD/</code>.</li></ul><h2>Enterprise work</h2><p>Add your consulting, onboarding, SLA, and dedicated support details here.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
),
(
  'getting-started',
  'Getting Started',
  '# Getting Started

## 1. Install dependencies

```bash
pnpm install
```

## 2. Create local secrets

```bash
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
```

Fill in `ADMIN_LOGIN_PASSWORD` and `ADMIN_JWT_SECRET`.

## 3. Prepare HTTPS certificates

```bash
pnpm cert:local
```

The local stack uses HTTPS by default because the frontends proxy API requests to the Worker.

## 4. Initialize D1

```bash
pnpm db:migrate:local
```

## 5. Start development

```bash
pnpm dev
```

- Landing: https://localhost:5173
- Admin: https://localhost:5174
- Worker API: https://localhost:8787

## 6. Validate

```bash
pnpm check
pnpm test
pnpm build
```',
  '<h1>Getting Started</h1><h2>1. Install dependencies</h2><pre><code>pnpm install</code></pre><h2>2. Create local secrets</h2><pre><code>cp rust-workers/.dev.vars.example rust-workers/.dev.vars</code></pre><p>Fill in <code>ADMIN_LOGIN_PASSWORD</code> and <code>ADMIN_JWT_SECRET</code>.</p><h2>3. Prepare HTTPS certificates</h2><pre><code>pnpm cert:local</code></pre><p>The local stack uses HTTPS by default because the frontends proxy API requests to the Worker.</p><h2>4. Initialize D1</h2><pre><code>pnpm db:migrate:local</code></pre><h2>5. Start development</h2><pre><code>pnpm dev</code></pre><ul><li>Landing: https://localhost:5173</li><li>Admin: https://localhost:5174</li><li>Worker API: https://localhost:8787</li></ul><h2>6. Validate</h2><pre><code>pnpm check
pnpm test
pnpm build</code></pre>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
);
