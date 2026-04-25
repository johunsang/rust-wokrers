# 2026-04-25 Codex Work Log

## Open Source Release Preparation

- Renamed the workspace, packages, Rust crate, Wrangler app, docs, and examples to `rust-wokrers`.
- Added open source project files: `LICENSE`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, and `SECURITY.md`.
- Added `docs/15-open-source-release.md` with release readiness, repository publishing, secret hygiene, and API-only mode notes.
- Replaced personal domains, account IDs, database IDs, repository names, and emails in public config with placeholders.
- Removed generated benchmark result artifacts from `bench/results/` so the repository can start clean.

## API-Only Runtime Mode

- Added `APP_MODE=full | api-only`.
- `APP_MODE=full` serves API plus UI routes.
- `APP_MODE=api-only` serves `/api/*` only, returns a JSON discovery response at `/`, and returns JSON 404 responses for disabled UI routes.
- Documented the mode in `README.md`, Cloudflare setup docs, deployment docs, open source release docs, `AGENTS.md`, and `CLAUDE.md`.

## Verification

- `AGENTS.md` and `CLAUDE.md` are synchronized.
- `pnpm install` passed.
- `pnpm db:migrate:local` passed.
- `pnpm check` passed.
- `pnpm build` passed.
- Generated Vite `dist` folders were removed after build verification.
- Scans for old private project names, personal identifiers, and accidental broken URL/path replacements passed. Remaining `example.com`, `admin@example.com`, `your-org`, and `REPLACE_WITH_*` values are intentional placeholders.

## Personal Data Cleanup

- Removed local-only secret and certificate files: `rust-workers/.dev.vars` and `.certs/`.
- Removed personal local agent configuration: `.mcp.json`, `.codex/config.toml`, and `.claude/settings.local.json`.
- Added local MCP/Codex config paths to `.gitignore`.
- Removed local `.agents/` and `.claude/` tool configuration folders from the public repository surface.
- Sanitized previous deployment version IDs from daily logs.
- Updated scaffold output so generated projects can choose `APP_MODE=full` or `APP_MODE=api-only`.

## GitHub Public Release Preparation

- Added SEO-focused README title, description, and repository keywords.
- Added landing Open Graph, Twitter card, description, keyword, and indexing metadata.
- Added `noindex` metadata to the private admin app HTML.
- Added Open Graph and Twitter metadata to Rust-rendered HTML pages.
- Changed staging and production deploy workflows to manual `workflow_dispatch` with confirmation inputs.
- Moved GitHub Actions files into `_templates/github-workflows/` so publishing the repository does not require a GitHub token with `workflow` scope.
- Replaced the `autocannon` benchmark dependency with a Node fetch-based connection ladder to avoid vulnerable transitive dependencies.
- Added a `postcss` patched-version override.
- `pnpm audit --audit-level moderate` passed with no known vulnerabilities.

## Design Theme Verification

- Connected landing and admin CSS to the theme preset variables so design options visibly affect backgrounds, text, surfaces, borders, radii, shadows, and accents.
- Added `scripts/test-themes.mjs` and `pnpm test:themes`.
- Verified all 30 theme presets by injecting each theme into both Vite apps and building landing/admin for every option.
- `pnpm test:themes`, `pnpm check`, `pnpm audit --audit-level moderate`, and `pnpm build` passed.
