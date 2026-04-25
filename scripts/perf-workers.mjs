#!/usr/bin/env node

import { mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { performance } from 'node:perf_hooks'

const args = new Map(
  process.argv.slice(2).map((arg) => {
    const [key, value = 'true'] = arg.replace(/^--/, '').split('=')
    return [key, value]
  }),
)

if (args.has('help') || args.has('h')) {
  console.log(`Usage:
  pnpm perf:workers
  pnpm perf:workers -- --profile=quick
  pnpm perf:workers -- --targets=Rust --endpoints=health-json

Options:
  --suite=bench/workers-suite.json
  --profile=quick|standard|soak
  --targets=Hono,Rust
  --endpoints=health-json,public-bootstrap-d1
  --out=bench/results/workers-quick.json
  --markdown=bench/results/workers-quick.md`)
  process.exit(0)
}

const suitePath = resolve(args.get('suite') ?? 'bench/workers-suite.json')
const profileName = args.get('profile') ?? 'quick'
const endpointFilter = new Set((args.get('endpoints') ?? '').split(',').filter(Boolean))
const targetFilter = new Set((args.get('targets') ?? '').split(',').filter(Boolean))
const outputPath = resolve(args.get('out') ?? `bench/results/workers-${profileName}.json`)
const markdownPath = resolve(args.get('markdown') ?? `bench/results/workers-${profileName}.md`)

const suite = JSON.parse(readFileSync(suitePath, 'utf8'))
const profile = suite.profiles[profileName]

if (!profile) {
  console.error(`Unknown profile "${profileName}". Available: ${Object.keys(suite.profiles).join(', ')}`)
  process.exit(1)
}

const targets = suite.targets.filter((target) => targetFilter.size === 0 || targetFilter.has(target.label))
const endpoints = suite.endpoints.filter(
  (endpoint) => endpointFilter.size === 0 || endpointFilter.has(endpoint.name),
)

if (targets.length === 0 || endpoints.length === 0) {
  console.error('No targets or endpoints selected.')
  process.exit(1)
}

function joinUrl(baseUrl, path) {
  return `${baseUrl.replace(/\/+$/, '')}${path.startsWith('/') ? path : `/${path}`}`
}

function sleep(ms) {
  return new Promise((resolveSleep) => setTimeout(resolveSleep, ms))
}

function percentile(sorted, pct) {
  if (sorted.length === 0) return 0
  const index = Math.min(sorted.length - 1, Math.ceil((pct / 100) * sorted.length) - 1)
  return sorted[index]
}

async function hit(url, method, headers) {
  const startedAt = performance.now()
  const res = await fetch(url, { method, headers, cache: 'no-store' })
  const bytes = (await res.arrayBuffer()).byteLength
  return {
    statusCode: res.status,
    bytes,
    latencyMs: performance.now() - startedAt,
  }
}

async function warmup(url, endpoint, headers) {
  const endAt = performance.now() + profile.warmupSeconds * 1000
  while (performance.now() < endAt) {
    await hit(url, endpoint.method, headers).catch(() => undefined)
  }
}

async function runStep({ url, endpoint, target, connections }) {
  await warmup(url, endpoint, target.headers ?? {})

  const durationMs = profile.durationSeconds * 1000
  const endAt = performance.now() + durationMs
  const latencies = []
  const statusCodeStats = {}
  let requestsTotal = 0
  let expectedStatusCount = 0
  let errors = 0
  let responseBytes = 0

  async function runner() {
    while (performance.now() < endAt) {
      try {
        const result = await hit(url, endpoint.method, target.headers ?? {})
        requestsTotal += 1
        responseBytes += result.bytes
        latencies.push(result.latencyMs)
        statusCodeStats[result.statusCode] = (statusCodeStats[result.statusCode] ?? 0) + 1
        if (result.statusCode === endpoint.expectedStatus) {
          expectedStatusCount += 1
        }
      } catch {
        errors += 1
      }
    }
  }

  const startedAt = performance.now()
  await Promise.all(Array.from({ length: connections }, () => runner()))
  const elapsedSeconds = (performance.now() - startedAt) / 1000
  latencies.sort((a, b) => a - b)
  const badStatus = Math.max(0, requestsTotal - expectedStatusCount)

  return {
    connections,
    durationSeconds: Number(elapsedSeconds.toFixed(2)),
    expectedStatus: endpoint.expectedStatus,
    requestsTotal,
    expectedStatusCount,
    errors,
    timeouts: 0,
    badStatus,
    statusCodeStats,
    latency: {
      average: Number((latencies.reduce((sum, value) => sum + value, 0) / Math.max(1, latencies.length)).toFixed(2)),
      min: Number((latencies[0] ?? 0).toFixed(2)),
      max: Number((latencies.at(-1) ?? 0).toFixed(2)),
      p50: Number(percentile(latencies, 50).toFixed(2)),
      p75: Number(percentile(latencies, 75).toFixed(2)),
      p90: Number(percentile(latencies, 90).toFixed(2)),
      p97_5: Number(percentile(latencies, 97.5).toFixed(2)),
      p99: Number(percentile(latencies, 99).toFixed(2)),
    },
    requests: {
      average: Number((requestsTotal / Math.max(0.001, elapsedSeconds)).toFixed(2)),
      total: requestsTotal,
    },
    throughput: {
      average: Number((responseBytes / Math.max(0.001, elapsedSeconds)).toFixed(2)),
      total: responseBytes,
    },
  }
}

function avg(values) {
  if (values.length === 0) return 0
  return values.reduce((sum, value) => sum + value, 0) / values.length
}

function summarize(runs) {
  return {
    repetitions: runs.length,
    requestsTotal: runs.reduce((sum, run) => sum + run.requestsTotal, 0),
    expectedStatusCount: runs.reduce((sum, run) => sum + run.expectedStatusCount, 0),
    errors: runs.reduce((sum, run) => sum + run.errors, 0),
    timeouts: runs.reduce((sum, run) => sum + run.timeouts, 0),
    badStatus: runs.reduce((sum, run) => sum + run.badStatus, 0),
    rpsAvg: Number(avg(runs.map((run) => run.requests.average)).toFixed(2)),
    latencyAvgMs: Number(avg(runs.map((run) => run.latency.average)).toFixed(2)),
    p50Ms: Number(avg(runs.map((run) => run.latency.p50)).toFixed(2)),
    p75Ms: Number(avg(runs.map((run) => run.latency.p75)).toFixed(2)),
    p90Ms: Number(avg(runs.map((run) => run.latency.p90)).toFixed(2)),
    p97_5Ms: Number(avg(runs.map((run) => run.latency.p97_5)).toFixed(2)),
    p99Ms: Number(avg(runs.map((run) => run.latency.p99)).toFixed(2)),
    maxMs: Number(avg(runs.map((run) => run.latency.max)).toFixed(2)),
    throughputBytesPerSec: Number(avg(runs.map((run) => run.throughput.average)).toFixed(2)),
  }
}

function renderMarkdown(report) {
  const lines = [
    '# Worker Performance Benchmark Results',
    '',
    `Generated at: ${report.generatedAt}`,
    '',
    `Suite: \`${report.suitePath}\``,
    `Profile: \`${report.profileName}\``,
    '',
    `Methodology: Node fetch runners, warmup=${report.profile.warmupSeconds}s, duration=${report.profile.durationSeconds}s per run, repetitions=${report.profile.repetitions}, connection ladder=${report.profile.connections.join(' -> ')}`,
    '',
    '> Load is generated from this local machine against deployed Cloudflare URLs. It is a client-observed benchmark, not an internal Cloudflare isolate CPU profile.',
    '',
  ]

  for (const endpoint of report.endpoints) {
    lines.push(`## ${endpoint.name}`)
    lines.push('')
    lines.push(`${endpoint.method} \`${endpoint.path}\` - ${endpoint.description}`)
    lines.push('')
    lines.push('| Runtime | Users | RPS avg | p50 | p75 | p90 | p97.5 | p99 | max | errors | bad status |')
    lines.push('| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |')

    for (const result of endpoint.results) {
      for (const step of result.steps) {
        const s = step.summary
        lines.push(
          `| ${result.target.label} | ${step.connections} | ${s.rpsAvg} | ${s.p50Ms} ms | ${s.p75Ms} ms | ${s.p90Ms} ms | ${s.p97_5Ms} ms | ${s.p99Ms} ms | ${s.maxMs} ms | ${s.errors + s.timeouts} | ${s.badStatus} |`,
        )
      }
    }
    lines.push('')
  }

  lines.push('## Reading The Ladder')
  lines.push('')
  lines.push('- Healthy scaling: RPS rises as users increase while p95/p99 stay mostly flat and errors remain zero.')
  lines.push('- Saturation: RPS stops rising or falls while p95/p99 and max latency climb.')
  lines.push('- Failure: errors, timeouts, or unexpected status codes appear.')
  lines.push('')

  return `${lines.join('\n')}\n`
}

const report = {
  generatedAt: new Date().toISOString(),
  suitePath,
  profileName,
  profile,
  targets,
  endpoints: [],
}

console.log(
  `Professional benchmark: profile=${profileName} duration=${profile.durationSeconds}s warmup=${profile.warmupSeconds}s repetitions=${profile.repetitions} ladder=${profile.connections.join(',')}`,
)

for (const endpoint of endpoints) {
  console.log(`\nEndpoint: ${endpoint.name} ${endpoint.method} ${endpoint.path}`)
  const endpointReport = { ...endpoint, results: [] }

  for (const target of targets) {
    const url = joinUrl(target.baseUrl, endpoint.path)
    const targetReport = { target, url, steps: [] }

    for (const connections of profile.connections) {
      const runs = []
      for (let i = 0; i < profile.repetitions; i += 1) {
        const run = await runStep({ url, endpoint, target, connections })
        runs.push(run)
        console.log(
          `  ${target.label} users=${connections} run=${i + 1}/${profile.repetitions} rps=${run.requests.average} p99=${run.latency.p99}ms errors=${run.errors} bad=${run.badStatus}`,
        )
        await sleep(profile.pauseSeconds * 1000)
      }

      targetReport.steps.push({
        connections,
        runs,
        summary: summarize(runs),
      })
    }

    endpointReport.results.push(targetReport)
  }

  report.endpoints.push(endpointReport)
}

mkdirSync(dirname(outputPath), { recursive: true })
writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`)
writeFileSync(markdownPath, renderMarkdown(report))
console.log(`\nWrote ${outputPath}`)
console.log(`Wrote ${markdownPath}`)
