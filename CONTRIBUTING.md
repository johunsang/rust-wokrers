# Contributing

Thanks for helping improve `rust-wokrers`.

## Development

```bash
pnpm install
cp rust-workers/.dev.vars.example rust-workers/.dev.vars
pnpm cert:local
pnpm db:migrate:local
pnpm dev
```

Run checks before opening a pull request:

```bash
pnpm check
pnpm test
pnpm build
```

## Project Rules

- Keep backend runtime code in `rust-workers/`.
- Keep frontend apps in `apps/landing` and `apps/admin`.
- Keep shared frontend contracts in `packages/com`.
- Add D1 migrations under `rust-workers/migrations/`.
- Update docs when behavior, commands, config, or deployment assumptions change.
- Keep `AGENTS.md` and `CLAUDE.md` synchronized.

## Pull Requests

Include:

- What changed
- Why it changed
- How it was tested
- Any migration or deployment notes
