-- Refresh the practical guide with concise pricing guidance.
UPDATE pages
SET
  content_md = content_md || '

## 8. Cost planning

Cloudflare pricing changes over time, so treat this section as operational guidance rather than a fixed quote.

- Prototype with free quotas when possible.
- Use Workers Paid before production when you need higher request, CPU, subrequest, Durable Object, or observability limits.
- Estimate D1, KV, R2, Images, Workers AI, AI Gateway, and Vectorize separately.
- Re-check the official Cloudflare pricing pages before committing to customer-facing pricing or annual budgets.

## 9. Production checklist

- D1 migrations applied to the target environment.
- Secrets set with `wrangler secret put`.
- Custom domains and routes configured in `rust-workers/wrangler.toml`.
- HTTPS verified for landing, admin, and API hosts.
- `pnpm check`, `pnpm test`, and `pnpm build` passing.',
  content_html = content_html || '<h2>8. Cost planning</h2><p>Cloudflare pricing changes over time, so treat this section as operational guidance rather than a fixed quote.</p><ul><li>Prototype with free quotas when possible.</li><li>Use Workers Paid before production when you need higher request, CPU, subrequest, Durable Object, or observability limits.</li><li>Estimate D1, KV, R2, Images, Workers AI, AI Gateway, and Vectorize separately.</li><li>Re-check the official Cloudflare pricing pages before committing to customer-facing pricing or annual budgets.</li></ul><h2>9. Production checklist</h2><ul><li>D1 migrations applied to the target environment.</li><li>Secrets set with <code>wrangler secret put</code>.</li><li>Custom domains and routes configured in <code>rust-workers/wrangler.toml</code>.</li><li>HTTPS verified for landing, admin, and API hosts.</li><li><code>pnpm check</code>, <code>pnpm test</code>, and <code>pnpm build</code> passing.</li></ul>',
  updated_at = datetime('now')
WHERE slug = 'guide';
