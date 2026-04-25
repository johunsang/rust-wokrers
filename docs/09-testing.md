# Testing

Current verification is centered on compilation and type checking.

```bash
pnpm check
pnpm build
```

This runs:

- Rust Worker check for `wasm32-unknown-unknown`
- Vite app TypeScript checks
- Vite production builds when running `pnpm build`

When adding backend logic, prefer small pure helper functions that can later be covered by Rust unit tests. For Worker binding behavior, verify with local Wrangler and D1 migrations.
