# Module Guide

Backend modules are Rust modules under `rust-workers/src/biz/{code}`.

## Add a Module

```bash
mkdir -p rust-workers/src/biz/ntf
cp _templates/biz-module/rust/mod.rs rust-workers/src/biz/ntf/mod.rs
cp _templates/biz-module/rust/routes.rs rust-workers/src/biz/ntf/routes.rs
```

Then:

1. Replace template placeholders.
2. Export the module in `rust-workers/src/biz/mod.rs`.
3. Register routes in `rust-workers/src/lib.rs`.
4. Add shared frontend types to `packages/com` if needed.
5. Add Vite UI under `apps/admin/src/biz/{code}` if needed.
6. Add a D1 migration under `rust-workers/migrations`.

## Rules

- Use D1 prepared statements.
- Keep route handlers small.
- Put reusable backend helpers in `rust-workers/src/com`.
- Keep frontend hooks/components in Vite apps.
