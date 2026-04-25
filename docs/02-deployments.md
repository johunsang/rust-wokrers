# Deployments

This project supports local development, staging, and production deployment flows.

## Local

```bash
pnpm dev
```

## Build

```bash
pnpm build
pnpm check
pnpm test
```

## Staging

```bash
pnpm deploy:staging
```

## Production

```bash
pnpm deploy:prod
```

## Deployment Checklist

- Cloudflare login is valid
- `wrangler.toml` contains real IDs
- `APP_MODE` is set to `full` or `api-only` for the deployment target
- required secrets are configured
- D1 migrations are ready
- `pnpm build`, `pnpm check`, and `pnpm test` pass

## Notes

- Run remote D1 migrations intentionally
- Keep staging and production database IDs separate
- Verify `/api/health` after every deployment
