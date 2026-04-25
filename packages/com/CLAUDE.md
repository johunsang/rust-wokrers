# Shared Contracts

`packages/com` is a TypeScript-only contract package for the Vite frontends.

- Backend runtime types live in Rust under `rust-workers/src/com/types.rs`.
- Frontend API response/input shapes live here and are imported as `@rust-wokrers/com`.
- Keep these TypeScript types aligned with Rust JSON serialization.
- When an API response changes, update Rust structs, this package, and the consuming Vite app together.
