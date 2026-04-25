# Frontend Guide

The frontend is Vite + React.

## Apps

- `apps/landing` runs on port `5173`
- `apps/admin` runs on port `5174`

Both apps proxy `/api` to the Rust Worker on port `8787`.

## Contracts

Use TypeScript types from `@rust-wokrers/com`.

When an API shape changes:

1. Update Rust structs in `rust-workers/src/com/types.rs`.
2. Update frontend contracts in `packages/com/src/contracts.ts`.
3. Update consuming Vite components/hooks.

## API Ownership

- Landing should call `/api/public/*`.
- Admin should call `/api/auth/*` and `/api/admin/*`.
- Do not put backend logic in Vite apps.
