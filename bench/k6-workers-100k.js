import http from 'k6/http'
import { check, sleep } from 'k6'

const targetBase = (__ENV.TARGET_BASE || 'https://rust-wokrers.example.workers.dev').replace(/\/+$/, '')
const targetLabel = __ENV.TARGET_LABEL || 'Rust'
const thinkTimeSeconds = Number(__ENV.THINK_TIME_SECONDS || '10')

const endpoints = [
  { name: 'health-json', path: '/api/health', weight: 10, expectedStatus: 200 },
  { name: 'public-bootstrap-d1', path: '/api/public/bootstrap', weight: 30, expectedStatus: 200 },
  { name: 'public-pages-d1', path: '/api/public/pages', weight: 30, expectedStatus: 200 },
  { name: 'landing-html', path: '/', weight: 20, expectedStatus: 200 },
  { name: 'cms-page-html', path: '/getting-started', weight: 10, expectedStatus: 200 },
]

const totalWeight = endpoints.reduce((sum, endpoint) => sum + endpoint.weight, 0)

export const options = {
  scenarios: {
    hundred_k_vu_ramp: {
      executor: 'ramping-vus',
      gracefulRampDown: '2m',
      stages: [
        { duration: '5m', target: 1000 },
        { duration: '5m', target: 5000 },
        { duration: '10m', target: 10000 },
        { duration: '10m', target: 25000 },
        { duration: '10m', target: 50000 },
        { duration: '10m', target: 75000 },
        { duration: '10m', target: 100000 },
        { duration: '10m', target: 100000 },
        { duration: '10m', target: 0 },
      ],
    },
  },
  thresholds: {
    http_req_failed: ['rate<0.01'],
    http_req_duration: ['p(95)<1000', 'p(99)<2000'],
  },
  summaryTrendStats: ['avg', 'min', 'med', 'p(75)', 'p(90)', 'p(95)', 'p(99)', 'max'],
  tags: {
    target: targetLabel,
  },
}

function pickEndpoint() {
  let cursor = Math.random() * totalWeight
  for (const endpoint of endpoints) {
    cursor -= endpoint.weight
    if (cursor <= 0) return endpoint
  }
  return endpoints[endpoints.length - 1]
}

export default function () {
  const endpoint = pickEndpoint()
  const res = http.get(`${targetBase}${endpoint.path}`, {
    headers: {
      accept: '*/*',
      'cache-control': 'no-cache',
      pragma: 'no-cache',
    },
    tags: {
      endpoint: endpoint.name,
      target: targetLabel,
    },
  })

  check(res, {
    [`${targetLabel} ${endpoint.name} status ${endpoint.expectedStatus}`]: (response) =>
      response.status === endpoint.expectedStatus,
  })

  sleep(thinkTimeSeconds)
}
