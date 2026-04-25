# Benchmark Suite

This directory contains two benchmark levels.

## Local Ladder

`pnpm perf:workers` uses `autocannon` from this machine against deployed Cloudflare URLs. It is good for quick comparison and regression checks up to modest connection counts.

```bash
pnpm perf:workers --profile=standard
```

## 100k Virtual Users

Use `bench/k6-workers-100k.js` for a professional 100,000 virtual-user test. Do not run this from a local laptop against production. Use k6 Cloud or a controlled distributed runner, and notify the platform/provider before the test window.

### GitHub Actions

The repository includes a manual workflow:

- Workflow: `.github/workflows/perf-100k.yml`
- Required secret: `K6_CLOUD_TOKEN`
- Inputs: target (`both`, `Hono`, or `Rust`), think time, and operator note

Use the workflow for the real 100k test window so execution is repeatable and auditable.

### Local Command Form

Run Hono and Rust separately with the same script:

```bash
k6 cloud \
  -e TARGET_LABEL=Hono \
  -e TARGET_BASE=https://example.com \
  -e THINK_TIME_SECONDS=10 \
  bench/k6-workers-100k.js

k6 cloud \
  -e TARGET_LABEL=Rust \
  -e TARGET_BASE=https://rust-wokrers.example.workers.dev \
  -e THINK_TIME_SECONDS=10 \
  bench/k6-workers-100k.js
```

The script ramps through `1k -> 5k -> 10k -> 25k -> 50k -> 75k -> 100k` virtual users, holds 100k, then ramps down. The default 10-second think time models users who make roughly one request every 10 seconds.

Shortcut scripts are also available when `k6` is installed and authenticated:

```bash
pnpm perf:100k:hono
pnpm perf:100k:rust
```

## Safety Gates

Abort the run if any of these happen:

- `http_req_failed` reaches 1% or higher.
- p95 latency is above 1000 ms for more than 5 minutes.
- p99 latency is above 2000 ms for more than 5 minutes.
- Cloudflare analytics show throttling, elevated 5xx, or D1 saturation.
- Unexpected status codes appear for any endpoint.
