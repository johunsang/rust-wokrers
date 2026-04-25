# Troubleshooting

## `wrangler` Auth Fails

- run `wrangler whoami`
- re-run `wrangler login`
- verify the correct Cloudflare account is selected

## Local App Does Not Start

- check `pnpm install`
- run `pnpm cert:local` if HTTPS certificate errors appear
- copy `.dev.vars.example` to `.dev.vars`
- run local D1 migrations

## Browser Says Certificate Is Required

- use `https://localhost:5173` and `https://localhost:5174`
- run `pnpm cert:local`
- restart `pnpm dev`
- local Vite/Wrangler development uses trusted HTTPS; true HTTP/3 is provided by Cloudflare in deployed environments, not by Vite/Wrangler dev servers

## Deployment Fails

- check `wrangler.toml` IDs
- verify required secrets
- confirm compatibility date is not in the future

## API Returns 500

- check Worker logs with `wrangler tail`
- verify D1 schema matches the code
- verify optional bindings are configured before using their routes
