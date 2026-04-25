# Open Source Release Checklist

Use this checklist before making the repository public.

## Required

- `LICENSE` exists and matches `package.json`.
- `README.md` explains the project, stack, quick start, scripts, and docs.
- `CONTRIBUTING.md` explains local development and PR expectations.
- `SECURITY.md` explains vulnerability reporting and secret handling.
- `.gitignore` excludes local secrets, Wrangler state, certificates, build output, and dependency folders.
- `rust-workers/wrangler.toml` contains only placeholder IDs and example domains.
- `rust-workers/.dev.vars.example` contains placeholder secret values only.
- `pnpm check` passes.

## Recommended

- Run `rg "REPLACE_WITH|admin@example.com|example.com"` and confirm placeholders are intentional.
- Search for private emails, tokens, secrets, account IDs, and database IDs before publishing.
- Run the quick benchmark only against owned test deployments.
- Remove generated benchmark result files if they include private domains or account details.

## API-Only Mode

`APP_MODE` controls whether the Worker serves both UI and API routes.

| Mode | Behavior |
| --- | --- |
| `full` | Serves landing, admin, CMS pages, and APIs. |
| `api-only` | Serves `/api/*`; non-API routes return JSON service information or a JSON 404. |

Set the mode in `rust-workers/wrangler.toml`:

```toml
[vars]
APP_MODE = "api-only"
```

## GitHub Actions

Workflow templates live in `_templates/github-workflows/` by default. Copy the files into `.github/workflows/` only after repository secrets and deployment targets are configured.
