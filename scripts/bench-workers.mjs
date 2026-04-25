#!/usr/bin/env node

import { performance } from 'node:perf_hooks'
import { writeFileSync } from 'node:fs'

const args = process.argv.slice(2)

const defaultCases = [
  { name: 'health-json', path: '/api/health', expect: [200] },
  { name: 'public-bootstrap-d1', path: '/api/public/bootstrap', expect: [200] },
  { name: 'public-pages-d1', path: '/api/public/pages', expect: [200] },
  { name: 'landing-html', path: '/', expect: [200] },
  { name: 'cms-page-html', path: '/getting-started', expect: [200] },
  { name: 'missing-route', path: '/__bench_missing__', expect: [404] },
]

const legacyArgs = args.filter((arg) => !arg.startsWith('--'))
const flags = new Map(
  args
    .filter((arg) => arg.startsWith('--'))
    .map((arg) => {
      const [key, value = 'true'] = arg.slice(2).split('=')
      return [key, value]
    }),
)

const baseArgs = flags.get('bases')
const caseArgs = flags.get('cases')
const labelArgs = flags.get('labels')
const outFile = flags.get('out')
const markdownFile = flags.get('markdown')

const durationMs = Number(process.env.BENCH_DURATION_MS ?? flags.get('duration-ms') ?? 10000)
const concurrency = Number(process.env.BENCH_CONCURRENCY ?? flags.get('concurrency') ?? 20)
const warmupMs = Number(process.env.BENCH_WARMUP_MS ?? flags.get('warmup-ms') ?? 2000)
const rounds = Number(process.env.BENCH_ROUNDS ?? flags.get('rounds') ?? 1)

function usage() {
  console.error(`Usage:
  pnpm bench:workers <url> [url...]
  pnpm bench:workers --bases=https://hono.example,https://rust.example

Options:
  --labels=Hono,Rust
  --cases=health-json,public-bootstrap-d1
  --duration-ms=10000
  --warmup-ms=2000
  --concurrency=20
  --rounds=3
  --out=benchmark.json
  --markdown=benchmark.md`)
}

function normalizeBase(base) {
  return base.replace(/\/+$/, '')
}

function toUrl(base, path) {
  if (/^https?:\/\//.test(path)) return path
  return `${normalizeBase(base)}${path.startsWith('/') ? path : `/${path}`}`
}

function percentile(values, pct) {
  if (values.length === 0) return 0
  const index = Math.min(values.length - 1, Math.ceil((pct / 100) * values.length) - 1)
  return values[index]
}

async function hit(url) {
  const start = performance.now()
  const res = await fetch(url, { cache: 'no-store' })
  const bytes = (await res.arrayBuffer()).byteLength
  return {
    status: res.status,
    contentType: res.headers.get('content-type') ?? '',
    bytes,
    ms: performance.now() - start,
  }
}

async function probe(url) {
  try {
    return await hit(url)
  } catch (error) {
    return {
      status: 0,
      contentType: '',
      bytes: 0,
      ms: 0,
      error: error instanceof Error ? error.message : String(error),
    }
  }
}

async function warmup(url) {
  const endAt = performance.now() + warmupMs
  while (performance.now() < endAt) {
    await hit(url).catch(() => undefined)
  }
}

async function benchUrl(url, expectedStatuses) {
  await warmup(url)

  const latencies = []
  const statuses = new Map()
  let ok = 0
  let errors = 0
  let total = 0
  let responseBytes = 0
  let stop = false
  const startedAt = performance.now()

  setTimeout(() => {
    stop = true
  }, durationMs)

  async function worker() {
    while (!stop) {
      total += 1
      try {
        const result = await hit(url)
        responseBytes += result.bytes
        latencies.push(result.ms)
        statuses.set(result.status, (statuses.get(result.status) ?? 0) + 1)
        if (expectedStatuses.includes(result.status)) ok += 1
        else errors += 1
      } catch {
        errors += 1
      }
    }
  }

  await Promise.all(Array.from({ length: concurrency }, () => worker()))
  const elapsedSec = (performance.now() - startedAt) / 1000
  latencies.sort((a, b) => a - b)

  return {
    durationSec: Number(elapsedSec.toFixed(2)),
    total,
    ok,
    errors,
    statuses: Object.fromEntries(statuses),
    rps: Number((ok / elapsedSec).toFixed(2)),
    avgBytes: total > 0 ? Number((responseBytes / total).toFixed(1)) : 0,
    minMs: Number((latencies[0] ?? 0).toFixed(2)),
    p50Ms: Number(percentile(latencies, 50).toFixed(2)),
    p90Ms: Number(percentile(latencies, 90).toFixed(2)),
    p95Ms: Number(percentile(latencies, 95).toFixed(2)),
    p99Ms: Number(percentile(latencies, 99).toFixed(2)),
    maxMs: Number((latencies.at(-1) ?? 0).toFixed(2)),
  }
}

function summarizeRounds(roundsForTarget) {
  const numericFields = ['rps', 'p50Ms', 'p90Ms', 'p95Ms', 'p99Ms', 'maxMs']
  const summary = { rounds: roundsForTarget.length }
  for (const field of numericFields) {
    const values = roundsForTarget.map((round) => round[field]).sort((a, b) => a - b)
    const avg = values.reduce((sum, value) => sum + value, 0) / values.length
    summary[field] = {
      avg: Number(avg.toFixed(2)),
      min: Number(values[0].toFixed(2)),
      max: Number(values.at(-1).toFixed(2)),
    }
  }
  summary.ok = roundsForTarget.reduce((sum, round) => sum + round.ok, 0)
  summary.errors = roundsForTarget.reduce((sum, round) => sum + round.errors, 0)
  return summary
}

function renderMarkdown(report) {
  const lines = [
    '# Worker Performance Benchmark Results',
    '',
    `Generated at: ${report.generatedAt}`,
    '',
    `Settings: duration=${report.settings.durationMs}ms, warmup=${report.settings.warmupMs}ms, concurrency=${report.settings.concurrency}, rounds=${report.settings.rounds}`,
    '',
  ]

  for (const testCase of report.cases) {
    lines.push(`## ${testCase.name}`)
    lines.push('')
    lines.push(`Path: \`${testCase.path}\``)
    lines.push('')
    lines.push('| Target | OK | Errors | RPS avg | p50 avg | p95 avg | p99 avg | max avg | Statuses |')
    lines.push('| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |')
    for (const target of testCase.targets) {
      const s = target.summary
      const statuses = target.rounds.map((round) => JSON.stringify(round.statuses)).join('<br>')
      lines.push(
        `| ${target.label} | ${s.ok} | ${s.errors} | ${s.rps.avg} | ${s.p50Ms.avg} ms | ${s.p95Ms.avg} ms | ${s.p99Ms.avg} ms | ${s.maxMs.avg} ms | ${statuses} |`,
      )
    }
    lines.push('')
  }

  return `${lines.join('\n')}\n`
}

let bases = []
let labels = []
let cases = defaultCases

if (baseArgs) {
  bases = baseArgs.split(',').filter(Boolean)
  labels = labelArgs ? labelArgs.split(',') : bases.map((base) => base.replace(/^https?:\/\//, ''))
} else if (legacyArgs.length > 0) {
  bases = legacyArgs
  labels = legacyArgs.map((url) => url)
  cases = [{ name: 'custom-url', path: '', expect: [200] }]
} else {
  usage()
  process.exit(1)
}

if (caseArgs) {
  const selected = new Set(caseArgs.split(','))
  cases = defaultCases.filter((testCase) => selected.has(testCase.name))
}

if (bases.length !== labels.length) {
  console.error('The number of labels must match the number of bases.')
  process.exit(1)
}

const report = {
  generatedAt: new Date().toISOString(),
  settings: { durationMs, warmupMs, concurrency, rounds },
  bases: bases.map((base, index) => ({ label: labels[index], base })),
  cases: [],
}

console.log(
  `Benchmark: duration=${durationMs}ms warmup=${warmupMs}ms concurrency=${concurrency} rounds=${rounds}`,
)

for (const testCase of cases) {
  const caseResult = { ...testCase, targets: [] }
  console.log(`\nCase: ${testCase.name} ${testCase.path}`)

  for (const [baseIndex, base] of bases.entries()) {
    const label = labels[baseIndex]
    const url = testCase.path ? toUrl(base, testCase.path) : base
    const probeResult = await probe(url)
    const roundsResult = []
    console.log(`  Target: ${label} ${url}`)
    console.log(
      `    Probe: status=${probeResult.status} type=${probeResult.contentType} bytes=${probeResult.bytes}`,
    )

    for (let round = 1; round <= rounds; round += 1) {
      const result = await benchUrl(url, testCase.expect)
      roundsResult.push(result)
      console.log(
        `    Round ${round}: ok=${result.ok} errors=${result.errors} rps=${result.rps} p50=${result.p50Ms}ms p95=${result.p95Ms}ms p99=${result.p99Ms}ms max=${result.maxMs}ms`,
      )
    }

    caseResult.targets.push({
      label,
      base,
      url,
      probe: probeResult,
      rounds: roundsResult,
      summary: summarizeRounds(roundsResult),
    })
  }

  report.cases.push(caseResult)
}

if (outFile) {
  writeFileSync(outFile, `${JSON.stringify(report, null, 2)}\n`)
}

if (markdownFile) {
  writeFileSync(markdownFile, renderMarkdown(report))
}
