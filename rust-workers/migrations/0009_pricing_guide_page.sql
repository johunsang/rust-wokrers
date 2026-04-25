-- Cloudflare pricing guide CMS page.
INSERT INTO pages (slug, title, content_md, content_html, status, published_at, created_at, updated_at)
VALUES (
  'pricing-guide',
  'Cloudflare Pricing Guide',
  '# Cloudflare Pricing Guide

This page gives rust-wokrers operators a practical checklist for estimating Cloudflare cost. Verify current numbers on Cloudflare before making budget or customer commitments.

## Main cost areas

- Workers: request volume, CPU time, subrequests, logs, builds, Durable Objects, and cron triggers.
- D1: database storage, read rows, write rows, and backup/export workflows.
- KV: reads, writes, list operations, and stored data.
- R2: object storage plus Class A and Class B operations. R2 is often attractive because egress is designed differently from traditional object stores.
- Images: stored images and transformations.
- Workers AI and AI Gateway: model usage, token volume, logging, caching, and rate control.
- Vectorize: stored vectors and query volume.

## Estimation workflow

1. Estimate monthly requests for public pages, admin operations, and API traffic.
2. Estimate average D1 reads and writes per request.
3. Estimate media storage and image transformations.
4. Estimate AI calls separately by feature.
5. Add a buffer for logs, retries, background jobs, and staging.

## Production recommendation

Run staging with realistic seed data before launch. Review Cloudflare analytics after one week, then update the budget model with measured request, storage, and AI usage.',
  '<h1>Cloudflare Pricing Guide</h1><p>This page gives rust-wokrers operators a practical checklist for estimating Cloudflare cost. Verify current numbers on Cloudflare before making budget or customer commitments.</p><h2>Main cost areas</h2><ul><li>Workers: request volume, CPU time, subrequests, logs, builds, Durable Objects, and cron triggers.</li><li>D1: database storage, read rows, write rows, and backup/export workflows.</li><li>KV: reads, writes, list operations, and stored data.</li><li>R2: object storage plus Class A and Class B operations. R2 is often attractive because egress is designed differently from traditional object stores.</li><li>Images: stored images and transformations.</li><li>Workers AI and AI Gateway: model usage, token volume, logging, caching, and rate control.</li><li>Vectorize: stored vectors and query volume.</li></ul><h2>Estimation workflow</h2><ol><li>Estimate monthly requests for public pages, admin operations, and API traffic.</li><li>Estimate average D1 reads and writes per request.</li><li>Estimate media storage and image transformations.</li><li>Estimate AI calls separately by feature.</li><li>Add a buffer for logs, retries, background jobs, and staging.</li></ol><h2>Production recommendation</h2><p>Run staging with realistic seed data before launch. Review Cloudflare analytics after one week, then update the budget model with measured request, storage, and AI usage.</p>',
  'published',
  datetime('now'),
  datetime('now'),
  datetime('now')
)
ON CONFLICT(slug) DO UPDATE SET
  title = excluded.title,
  content_md = excluded.content_md,
  content_html = excluded.content_html,
  status = excluded.status,
  published_at = excluded.published_at,
  updated_at = datetime('now');
