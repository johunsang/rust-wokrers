# Security, Login, and Access

This project ships with secure defaults for admin access and API usage.

## Auth Options

- JWT cookie session
- GitHub OAuth
- Cloudflare Access

## Required Secrets

- `ADMIN_LOGIN_PASSWORD`
- `ADMIN_JWT_SECRET`

Optional:

- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`
- `GITHUB_ALLOWED_USERS`

## Security Features

- CSP, HSTS, `X-Frame-Options`, `nosniff`
- rate limiting for sensitive endpoints
- RBAC for admin APIs
- prepared statements for D1 queries
- admin route protection middleware

## Review Checklist

- no hardcoded secrets
- no unvalidated input on write endpoints
- no unprotected admin route
- no direct SQL string interpolation
- no unnecessary public bindings

## Next Docs

- [API Reference](./07-api-reference.md)
- [Testing](./09-testing.md)
