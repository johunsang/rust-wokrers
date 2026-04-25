# Cloudflare Setup

Configure Cloudflare resources in `rust-workers/wrangler.toml`.

## Required

- Account ID
- D1 database bound as `DB`
- KV namespace bound as `APP_KV`

## Optional

- Workers AI bound as `AI`
- Vectorize index bound as `DOC_INDEX`
- R2 bucket bound as `MEDIA_R2`
- Cloudflare Images token and delivery hash
- Resend API key

## Local Secrets

```bash
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
```

## Runtime Mode

Set `APP_MODE` in `rust-workers/wrangler.toml`:

- `full`: serve landing, admin, CMS pages, and APIs.
- `api-only`: serve `/api/*`; non-API routes return JSON service information or JSON 404.

## Migrations

```bash
pnpm db:migrate:local
pnpm db:migrate:remote
```
