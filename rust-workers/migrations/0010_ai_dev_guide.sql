-- AI development guide CMS page.
INSERT INTO pages (slug, title, content_md, content_html, status, published_at, created_at, updated_at)
VALUES (
  'ai-dev-guide',
  'AI Development Guide: Claude Code and Codex',
  '# AI Development Guide: Claude Code and Codex

rust-wokrers is structured so coding agents can read the project, make scoped changes, run checks, and document the result.

## 1. Agent documents

- `AGENTS.md` and `CLAUDE.md` are the root operating guides and must stay identical.
- Subdirectory agent documents can add narrower rules for a package or app.
- `docs/daily/YYYY-MM-DD/` stores daily implementation notes.
- `_templates/` contains module, migration, and theme templates.

## 2. Useful prompts

Good prompts include the feature, the target module, and the expected validation.

Example:

```text
Add priority to leads. Update shared contracts, Rust routes, D1 migrations, admin UI, and tests. Run pnpm check.
```

## 3. Development flow

1. Read the relevant guide and module files.
2. Update shared contracts before frontend or API consumers need them.
3. Add D1 migrations for schema or seed-data changes.
4. Implement Rust Worker routes and repositories.
5. Implement Vite admin or landing UI.
6. Run `pnpm check`, `pnpm test`, and targeted API or browser checks.
7. Record important changes in the daily log.

## 4. Local HTTPS workflow

The current development stack uses HTTPS locally:

- Landing: https://localhost:5173
- Admin: https://localhost:5174
- Worker API: https://localhost:8787

Generate certificates with `pnpm cert:local` and initialize local D1 with `pnpm db:migrate:local`.

## 5. Collaboration rules

- Do not revert unrelated user changes.
- Keep `AGENTS.md` and `CLAUDE.md` synchronized.
- Update docs whenever behavior, commands, structure, or deployment assumptions change.
- Prefer focused edits that match existing module boundaries.',
  '<h1>AI Development Guide: Claude Code and Codex</h1><p>rust-wokrers is structured so coding agents can read the project, make scoped changes, run checks, and document the result.</p><h2>1. Agent documents</h2><ul><li><code>AGENTS.md</code> and <code>CLAUDE.md</code> are the root operating guides and must stay identical.</li><li>Subdirectory agent documents can add narrower rules for a package or app.</li><li><code>docs/daily/YYYY-MM-DD/</code> stores daily implementation notes.</li><li><code>_templates/</code> contains module, migration, and theme templates.</li></ul><h2>2. Useful prompts</h2><p>Good prompts include the feature, the target module, and the expected validation.</p><pre><code>Add priority to leads. Update shared contracts, Rust routes, D1 migrations, admin UI, and tests. Run pnpm check.</code></pre><h2>3. Development flow</h2><ol><li>Read the relevant guide and module files.</li><li>Update shared contracts before frontend or API consumers need them.</li><li>Add D1 migrations for schema or seed-data changes.</li><li>Implement Rust Worker routes and repositories.</li><li>Implement Vite admin or landing UI.</li><li>Run <code>pnpm check</code>, <code>pnpm test</code>, and targeted API or browser checks.</li><li>Record important changes in the daily log.</li></ol><h2>4. Local HTTPS workflow</h2><p>The current development stack uses HTTPS locally: Landing https://localhost:5173, Admin https://localhost:5174, Worker API https://localhost:8787.</p><p>Generate certificates with <code>pnpm cert:local</code> and initialize local D1 with <code>pnpm db:migrate:local</code>.</p><h2>5. Collaboration rules</h2><ul><li>Do not revert unrelated user changes.</li><li>Keep <code>AGENTS.md</code> and <code>CLAUDE.md</code> synchronized.</li><li>Update docs whenever behavior, commands, structure, or deployment assumptions change.</li><li>Prefer focused edits that match existing module boundaries.</li></ul>',
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
