import { useEffect, useState } from 'react'
import { createRoot } from 'react-dom/client'
import type { PageSummary, PublicBootstrap } from '@rust-wokrers/com'
import './styles.css'

const GITHUB_URL = 'https://github.com/your-org/rust-wokrers'
const RELEASES_URL = 'https://github.com/your-org/rust-wokrers/releases'

async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(path, {
    headers: { 'content-type': 'application/json', ...(init?.headers ?? {}) },
    ...init,
  })
  if (!res.ok) throw new Error(await res.text())
  return res.json() as Promise<T>
}

function App() {
  const [bootstrap, setBootstrap] = useState<PublicBootstrap | null>(null)
  const [pages, setPages] = useState<PageSummary[]>([])

  useEffect(() => {
    void apiFetch<PublicBootstrap>('/api/public/bootstrap').then(setBootstrap)
    void apiFetch<PageSummary[]>('/api/public/pages').then(setPages)
  }, [])

  const settings = bootstrap?.settings

  return (
    <main>
      <section className="hero">
        <div className="hero-copy">
          <p className="eyebrow">{settings?.heroLabel ?? 'GitHub-first SaaS boilerplate'}</p>
          <h1>{settings?.heroTitle ?? 'Build from the repository, deploy at the edge.'}</h1>
          <p className="lede">
            {settings?.heroSubtitle ??
              'rust-wokrers keeps the public landing page focused on GitHub: source, docs, releases, and the admin console stay one click away.'}
          </p>
          <div className="actions" aria-label="Primary actions">
            <a href={GITHUB_URL}>{settings?.ctaPrimary ?? 'Open GitHub'}</a>
            <a href="/admin" className="secondary">
              {settings?.ctaSecondary ?? 'Open admin'}
            </a>
          </div>
        </div>

        <aside className="repo-card" aria-label="GitHub repository summary">
          <div className="repo-header">
            <span className="mark">GH</span>
            <div>
              <strong>rust-wokrers</strong>
              <span>Rust Worker + Vite SaaS kit</span>
            </div>
          </div>
          <div className="repo-lines">
            <span />
            <span />
            <span />
          </div>
          <dl>
            <div>
              <dt>Leads</dt>
              <dd>{bootstrap?.metrics.totalLeads ?? '-'}</dd>
            </div>
            <div>
              <dt>Media</dt>
              <dd>{bootstrap?.metrics.totalMedia ?? '-'}</dd>
            </div>
          </dl>
        </aside>
      </section>

      <section className="content-grid">
        <article>
          <p className="eyebrow">Repository workflow</p>
          <h2>Use GitHub as the front door.</h2>
          <p>
            Point visitors to the repository for source code, issues, releases, setup docs, and
            contribution history. Keep the admin app private for operations.
          </p>
          <div className="link-list">
            <a href={GITHUB_URL}>Repository</a>
            <a href={RELEASES_URL}>Releases</a>
            <a href="/getting-started">Getting started</a>
          </div>
        </article>

        <article>
          <p className="eyebrow">Published docs</p>
          <h2>Guides from the CMS</h2>
          <div className="pages">
            {pages.slice(0, 6).map((page) => (
              <a key={page.id} href={`/${page.slug}`}>
                <strong>{page.title}</strong>
                <span>{page.updatedAt}</span>
              </a>
            ))}
            {pages.length === 0 && <p className="muted">No published pages yet.</p>}
          </div>
        </article>
      </section>
    </main>
  )
}

createRoot(document.getElementById('root')!).render(<App />)
