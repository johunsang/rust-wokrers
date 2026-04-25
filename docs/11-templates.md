# Templates

Templates live under `_templates`.

## Backend Module

Use `_templates/biz-module/rust`.

```bash
mkdir -p rust-workers/src/biz/ntf
cp _templates/biz-module/rust/mod.rs rust-workers/src/biz/ntf/mod.rs
cp _templates/biz-module/rust/routes.rs rust-workers/src/biz/ntf/routes.rs
```

Register the module in `rust-workers/src/biz/mod.rs` and `rust-workers/src/lib.rs`.

## Frontend

Use `_templates/biz-module/admin` for Vite admin hooks/components.

## Migration

Use `_templates/migration/0000_template.sql` and place the copied file in `rust-workers/migrations`.
