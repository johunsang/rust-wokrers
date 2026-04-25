# Templates

Templates cover the current architecture: Rust Worker backend and Vite frontends.

## Add a Business Module

Example module code: `ntf` for notifications.

```bash
mkdir -p rust-workers/src/biz/ntf
cp _templates/biz-module/rust/mod.rs rust-workers/src/biz/ntf/mod.rs
cp _templates/biz-module/rust/routes.rs rust-workers/src/biz/ntf/routes.rs
```

Then:

1. Replace `__ITEM__`, `__item__`, and `__items__`.
2. Add `pub mod ntf;` to `rust-workers/src/biz/mod.rs`.
3. Copy the snippets from `_templates/biz-module/rust/router-snippet.rs` into `rust-workers/src/lib.rs`.
4. Add frontend types to `packages/com/src/contracts.ts` if Vite apps consume the API.
5. Add Vite hook/component files under `apps/admin/src/biz/ntf/` when the admin UI needs the module.
6. Add a D1 migration under `rust-workers/migrations/`.

## Add a Migration

```bash
cp _templates/migration/0000_template.sql rust-workers/migrations/0011_add_notifications.sql
pnpm db:migrate:local
```

## Theme Presets

Theme presets remain CSS snippets under `_templates/themes/`. Apply them to Vite app stylesheets, for example:

- `apps/landing/src/styles.css`
- `apps/admin/src/styles.css`
