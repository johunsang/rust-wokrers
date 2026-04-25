# Performance Benchmark

This project uses an `autocannon`-based benchmark suite to compare the deployed legacy Hono Worker with the deployed Rust Worker under the same request paths and the same connection ladder.

## Targets

Both targets are real Cloudflare-hosted servers. Localhost is not used as a benchmark target.

| Label | URL | Runtime |
| --- | --- | --- |
| Hono | `https://example.com` | `cloudflare-workers` |
| Rust | `https://rust-wokrers.example.workers.dev` | `cloudflare-workers-rust` |

## Methodology

- Tool: `autocannon`
- Suite config: `bench/workers-suite.json`
- Quick profile: users `1 -> 5 -> 20`, warmup `2s`, run duration `8s`, repetitions `1`
- Standard profile: users `1 -> 5 -> 10 -> 25 -> 50 -> 100`, warmup `5s`, run duration `30s`, repetitions `3`
- Headers: `cache-control: no-cache`, `pragma: no-cache`
- Metrics: RPS, p50, p75, p90, p97.5, p99, max latency, errors, timeouts, and unexpected status counts
- Important rule: if one runtime returns a different status code, that case is a functional mismatch, not a performance win.

## Commands

Quick smoke benchmark:

```bash
pnpm perf:workers --profile=quick
```

Standard benchmark for publishable numbers:

```bash
pnpm perf:workers --profile=standard \
  --out=bench/results/workers-standard-$(date +%Y-%m-%d).json \
  --markdown=bench/results/workers-standard-$(date +%Y-%m-%d).md
```

Focused API benchmark:

```bash
pnpm perf:workers --profile=standard \
  --endpoints=health-json,public-bootstrap-d1,public-pages-d1
```

## 2026-04-24 Quick Result After Rust Redeploy

Result files:

- `bench/results/workers-quick-after-deploy-2026-04-24.json`
- `bench/results/workers-quick-after-deploy-2026-04-24.md`

### API Endpoints

#### `GET /api/health`

| Runtime | Users | RPS | p50 | p99 | Max | Errors | Bad Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Hono | 1 | 7.63 | 129 ms | 210 ms | 210 ms | 0 | 0 |
| Hono | 5 | 30.13 | 165 ms | 275 ms | 298 ms | 0 | 0 |
| Hono | 20 | 112.75 | 165 ms | 325 ms | 577 ms | 0 | 0 |
| Rust | 1 | 6.75 | 143 ms | 246 ms | 246 ms | 0 | 0 |
| Rust | 5 | 31.38 | 152 ms | 300 ms | 323 ms | 0 | 0 |
| Rust | 20 | 117.63 | 161 ms | 282 ms | 412 ms | 0 | 0 |

#### `GET /api/public/bootstrap`

| Runtime | Users | RPS | p50 | p99 | Max | Errors | Bad Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Hono | 1 | 3.50 | 276 ms | 377 ms | 377 ms | 0 | 0 |
| Hono | 5 | 14.13 | 342 ms | 442 ms | 453 ms | 0 | 0 |
| Hono | 20 | 40.63 | 467 ms | 727 ms | 910 ms | 0 | 0 |
| Rust | 1 | 3.50 | 272 ms | 382 ms | 382 ms | 0 | 0 |
| Rust | 5 | 14.50 | 321 ms | 514 ms | 520 ms | 0 | 0 |
| Rust | 20 | 40.88 | 464 ms | 784 ms | 1004 ms | 0 | 0 |

#### `GET /api/public/pages`

| Runtime | Users | RPS | p50 | p99 | Max | Errors | Bad Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Hono | 1 | 4.63 | 199 ms | 455 ms | 455 ms | 0 | 0 |
| Hono | 5 | 19.88 | 247 ms | 499 ms | 507 ms | 0 | 0 |
| Hono | 20 | 60.75 | 291 ms | 571 ms | 868 ms | 0 | 0 |
| Rust | 1 | 6.00 | 163 ms | 242 ms | 242 ms | 0 | 0 |
| Rust | 5 | 21.25 | 230 ms | 343 ms | 519 ms | 0 | 0 |
| Rust | 20 | 71.88 | 268 ms | 350 ms | 440 ms | 0 | 0 |

### HTML Routes

HTML routes are useful for user-visible behavior, but they are not pure framework comparisons because payload size and static asset handling differ.

#### `GET /`

| Runtime | Users | RPS | p50 | p99 | Max | Response Size |
| --- | --- | --- | --- | --- | --- | --- |
| Hono | 1 | 13.25 | 72 ms | 90 ms | 202 ms | 3029 B |
| Hono | 5 | 78.88 | 63 ms | 103 ms | 175 ms | 3029 B |
| Hono | 20 | 329.13 | 58 ms | 147 ms | 398 ms | 3029 B |
| Rust | 1 | 4.63 | 210 ms | 289 ms | 289 ms | 7008 B |
| Rust | 5 | 19.13 | 245 ms | 423 ms | 426 ms | 7008 B |
| Rust | 20 | 56.75 | 330 ms | 445 ms | 497 ms | 7008 B |

#### `GET /getting-started`

| Runtime | Users | RPS | p50 | p99 | Max | Response Size |
| --- | --- | --- | --- | --- | --- | --- |
| Hono | 1 | 12.25 | 78 ms | 191 ms | 191 ms | 3029 B |
| Hono | 5 | 82.25 | 58 ms | 108 ms | 264 ms | 3029 B |
| Hono | 20 | 355.50 | 54 ms | 123 ms | 361 ms | 3029 B |
| Rust | 1 | 8.63 | 112 ms | 220 ms | 220 ms | 5058 B |
| Rust | 5 | 47.00 | 105 ms | 188 ms | 220 ms | 5058 B |
| Rust | 20 | 185.13 | 104 ms | 190 ms | 241 ms | 5058 B |

## Interpretation

- For tiny API routing (`/api/health`), Rust and Hono are close at low users. At 20 users, Rust has slightly higher RPS and lower p99/max latency in this run.
- For D1-backed public bootstrap, both runtimes are effectively tied. D1/network latency dominates more than framework overhead.
- For D1-backed public page lists, Rust is ahead in this quick run: higher RPS and much lower p99/max at 20 users.
- For HTML routes, Hono is faster in this run, but the response bodies are not equivalent. The Rust deployment serves larger HTML responses, so this should be optimized or normalized before drawing framework conclusions.
- No errors or unexpected statuses occurred after redeploy. Before redeploy, Rust returned 404 for `/api/public/pages`; that was a deployment mismatch and was fixed before the final run.

## Professional Follow-Up

Run the standard profile before using results externally:

1. Run from a stable machine and network.
2. Run at least three full standard passes.
3. Record Cloudflare colo, time of day, and whether Workers analytics show throttling.
4. Compare only paths with equivalent status codes and similar payload sizes.
5. For HTML route comparison, normalize payload size or benchmark API-only paths separately.

## 100k User Test Plan

Do not run 100,000 concurrent users from a single local machine. It creates unreliable client-side bottlenecks and can look like abusive traffic. The professional setup is a distributed load generator such as k6 Cloud or a controlled multi-region runner.

The repository includes a k6 script for this:

- Script: `bench/k6-workers-100k.js`
- Instructions: `bench/README.md`
- Manual GitHub Actions workflow: `.github/workflows/perf-100k.yml`
- Required GitHub secret: `K6_CLOUD_TOKEN`

The 100k profile ramps gradually:

| Stage | Virtual Users |
| --- | --- |
| Ramp 1 | 1,000 |
| Ramp 2 | 5,000 |
| Ramp 3 | 10,000 |
| Ramp 4 | 25,000 |
| Ramp 5 | 50,000 |
| Ramp 6 | 75,000 |
| Ramp 7 | 100,000 |
| Hold | 100,000 |
| Ramp down | 0 |

Run Hono and Rust separately with the same script and the same think time:

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

Or use the package scripts when `k6` is installed and authenticated:

```bash
pnpm perf:100k:hono
pnpm perf:100k:rust
```

### Why Think Time Matters

“100,000 users” is not the same as “100,000 requests per second.” With a 10-second think time, 100,000 active virtual users produce roughly 10,000 requests per second before network latency and endpoint mix are considered. With a 1-second think time, the same virtual-user count can approach roughly 100,000 requests per second.

### Abort Conditions

Stop the run if:

- Error rate reaches 1%.
- p95 stays above 1000 ms for more than 5 minutes.
- p99 stays above 2000 ms for more than 5 minutes.
- Unexpected status codes appear.
- Cloudflare analytics show throttling, elevated 5xx, or D1 saturation.

### Required Evidence

For a publishable 100k report, save:

- k6 summary JSON for Hono and Rust.
- Cloudflare Workers analytics for both scripts.
- D1 analytics for database-backed endpoints.
- Test window time, source regions, think time, endpoint mix, and Cloudflare colo distribution.