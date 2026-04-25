import { FormEvent, useEffect, useMemo, useState } from 'react'
import { createRoot } from 'react-dom/client'
import type {
  AdminUser,
  AdminUserInput,
  AiCopySuggestion,
  ApiLog,
  DashboardData,
  EmailLog,
  EmailTemplate,
  EmailTemplateInput,
  LeadDetail,
  LeadRecord,
  LeadStatus,
  MediaAsset,
  Page,
  PageInput,
  PageSummary,
  SiteSettings,
  SystemStats,
} from '@rust-wokrers/com'
import './styles.css'

type Session = { email: string }
type Tab =
  | 'dashboard'
  | 'leads'
  | 'pages'
  | 'settings'
  | 'media'
  | 'ai'
  | 'email'
  | 'users'
  | 'logs'
  | 'ops'

type OpsState = {
  tasks: Array<{ id: string; title: string; done: boolean; createdAt: string; completedAt: string | null }>
  notes: string[]
}

const tabs: Array<{ id: Tab; label: string }> = [
  { id: 'dashboard', label: 'Dashboard' },
  { id: 'leads', label: 'Leads' },
  { id: 'pages', label: 'Pages' },
  { id: 'settings', label: 'Settings' },
  { id: 'media', label: 'Media' },
  { id: 'ai', label: 'AI' },
  { id: 'email', label: 'Email' },
  { id: 'users', label: 'Users' },
  { id: 'logs', label: 'Logs' },
  { id: 'ops', label: 'Ops' },
]

async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(path, {
    credentials: 'include',
    headers: { 'content-type': 'application/json', ...(init?.headers ?? {}) },
    ...init,
  })
  if (!res.ok) throw new Error(await res.text())
  return res.json() as Promise<T>
}

function App() {
  const [session, setSession] = useState<Session | null>(null)
  const [active, setActive] = useState<Tab>('dashboard')
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [notice, setNotice] = useState<string | null>(null)
  const [dashboard, setDashboard] = useState<DashboardData | null>(null)
  const [stats, setStats] = useState<SystemStats | null>(null)
  const [settings, setSettings] = useState<SiteSettings | null>(null)
  const [leads, setLeads] = useState<LeadRecord[]>([])
  const [leadDetail, setLeadDetail] = useState<LeadDetail | null>(null)
  const [pages, setPages] = useState<PageSummary[]>([])
  const [pageDetail, setPageDetail] = useState<Page | null>(null)
  const [media, setMedia] = useState<MediaAsset[]>([])
  const [templates, setTemplates] = useState<EmailTemplate[]>([])
  const [emailLogs, setEmailLogs] = useState<EmailLog[]>([])
  const [users, setUsers] = useState<AdminUser[]>([])
  const [apiLogs, setApiLogs] = useState<ApiLog[]>([])
  const [aiCopy, setAiCopy] = useState<AiCopySuggestion | null>(null)
  const [ops, setOps] = useState<OpsState | null>(null)

  async function run(action: () => Promise<void>, success?: string) {
    setBusy(true)
    setError(null)
    setNotice(null)
    try {
      await action()
      if (success) setNotice(success)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Request failed')
    } finally {
      setBusy(false)
    }
  }

  async function loadCore() {
    const me = await apiFetch<Session>('/api/auth/me')
    setSession(me)
    await Promise.all([loadDashboard(), loadSettings()])
  }

  async function loadDashboard() {
    const [nextDashboard, nextStats] = await Promise.all([
      apiFetch<DashboardData>('/api/admin/dashboard'),
      apiFetch<SystemStats>('/api/admin/logs/stats'),
    ])
    setDashboard(nextDashboard)
    setStats(nextStats)
  }

  async function loadSettings() {
    setSettings(await apiFetch<SiteSettings>('/api/admin/settings'))
  }

  async function loadLeads(selectId?: number) {
    const next = await apiFetch<LeadRecord[]>('/api/admin/leads')
    setLeads(next)
    const id = selectId ?? leadDetail?.id ?? next[0]?.id
    if (id) setLeadDetail(await apiFetch<LeadDetail>(`/api/admin/leads/${id}`))
  }

  async function loadPages(selectId?: number) {
    const next = await apiFetch<PageSummary[]>('/api/admin/pages')
    setPages(next)
    const id = selectId ?? pageDetail?.id ?? next[0]?.id
    if (id) setPageDetail(await apiFetch<Page>(`/api/admin/pages/${id}`))
  }

  async function loadMedia() {
    setMedia(await apiFetch<MediaAsset[]>('/api/admin/images'))
  }

  async function loadEmail() {
    const [nextTemplates, nextLogs] = await Promise.all([
      apiFetch<EmailTemplate[]>('/api/admin/email/templates'),
      apiFetch<EmailLog[]>('/api/admin/email/logs'),
    ])
    setTemplates(nextTemplates)
    setEmailLogs(nextLogs)
  }

  async function loadUsers() {
    setUsers(await apiFetch<AdminUser[]>('/api/admin/users'))
  }

  async function loadLogs() {
    setApiLogs(await apiFetch<ApiLog[]>('/api/admin/logs/api?limit=80'))
  }

  async function loadOps() {
    setOps(await apiFetch<OpsState>('/api/admin/agt'))
  }

  useEffect(() => {
    void loadCore().catch(() => setSession(null))
  }, [])

  useEffect(() => {
    if (!session) return
    const loaders: Record<Tab, () => Promise<void>> = {
      dashboard: loadDashboard,
      leads: () => loadLeads(),
      pages: () => loadPages(),
      settings: loadSettings,
      media: loadMedia,
      ai: async () => {},
      email: loadEmail,
      users: loadUsers,
      logs: loadLogs,
      ops: loadOps,
    }
    void run(loaders[active])
  }, [active, session])

  async function login(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    const form = new FormData(event.currentTarget)
    await run(async () => {
      await apiFetch('/api/auth/login', {
        method: 'POST',
        body: JSON.stringify({
          email: form.get('email'),
          password: form.get('password'),
        }),
      })
      await loadCore()
    })
  }

  if (!session) {
    return (
      <main className="login">
        <form className="login-card" onSubmit={login}>
          <div>
            <p className="eyebrow">Secure console</p>
            <h1>Admin</h1>
          </div>
          {error && <p className="error">{error}</p>}
          <Field label="Email" name="email" type="email" required />
          <Field label="Password" name="password" type="password" required />
          <button disabled={busy}>{busy ? 'Signing in' : 'Login'}</button>
        </form>
      </main>
    )
  }

  return (
    <main className="shell">
      <aside className="sidebar">
        <div>
          <strong>rust-wokrers</strong>
          <span>{session.email}</span>
        </div>
        <nav>
          {tabs.map((tab) => (
            <button
              className={active === tab.id ? 'active' : ''}
              key={tab.id}
              onClick={() => setActive(tab.id)}
            >
              {tab.label}
            </button>
          ))}
        </nav>
      </aside>

      <section className="content">
        <header>
          <div>
            <p className="eyebrow">Operations</p>
            <h1>{tabs.find((tab) => tab.id === active)?.label}</h1>
          </div>
          <div className="header-actions">
            <button className="secondary" onClick={() => void run(loadDashboard, 'Refreshed')} disabled={busy}>
              Refresh
            </button>
            <button
              onClick={() =>
                void run(async () => {
                  await apiFetch('/api/auth/logout', { method: 'POST' })
                  setSession(null)
                })
              }
            >
              Logout
            </button>
          </div>
        </header>

        {error && <p className="error">{error}</p>}
        {notice && <p className="notice">{notice}</p>}

        {active === 'dashboard' && <DashboardView dashboard={dashboard} stats={stats} />}
        {active === 'leads' && (
          <LeadsView
            leads={leads}
            detail={leadDetail}
            onSelect={(id) => void run(async () => setLeadDetail(await apiFetch(`/api/admin/leads/${id}`)))}
            onStatus={(id, status) =>
              void run(async () => {
                await apiFetch(`/api/admin/leads/${id}/status`, {
                  method: 'PUT',
                  body: JSON.stringify({ status }),
                })
                await loadLeads(id)
              }, 'Lead updated')
            }
            onAddTag={(id, tag) =>
              void run(async () => {
                await apiFetch(`/api/admin/leads/${id}/tags`, {
                  method: 'POST',
                  body: JSON.stringify({ tag }),
                })
                await loadLeads(id)
              }, 'Tag added')
            }
            onAddNote={(id, content) =>
              void run(async () => {
                await apiFetch(`/api/admin/leads/${id}/notes`, {
                  method: 'POST',
                  body: JSON.stringify({ content, createdBy: session.email }),
                })
                await loadLeads(id)
              }, 'Note added')
            }
          />
        )}
        {active === 'pages' && (
          <PagesView
            pages={pages}
            detail={pageDetail}
            onSelect={(id) => void run(async () => setPageDetail(await apiFetch(`/api/admin/pages/${id}`)))}
            onSave={(input, id) =>
              void run(async () => {
                if (id) {
                  await apiFetch(`/api/admin/pages/${id}`, { method: 'PUT', body: JSON.stringify(input) })
                  await loadPages(id)
                } else {
                  await apiFetch('/api/admin/pages', { method: 'POST', body: JSON.stringify(input) })
                  await loadPages()
                }
              }, 'Page saved')
            }
            onAction={(id, action) =>
              void run(async () => {
                await apiFetch(`/api/admin/pages/${id}/${action}`, { method: 'POST' })
                await loadPages(id)
              }, `Page ${action}ed`)
            }
          />
        )}
        {active === 'settings' && settings && (
          <SettingsView
            settings={settings}
            onSave={(input) =>
              void run(async () => {
                await apiFetch('/api/admin/settings', { method: 'PUT', body: JSON.stringify(input) })
                await loadSettings()
              }, 'Settings saved')
            }
          />
        )}
        {active === 'media' && (
          <MediaView
            media={media}
            onCreate={(input) =>
              void run(async () => {
                const payload = await apiFetch<{ imageId: string; uploadURL: string }>(
                  '/api/admin/images/direct-upload',
                  { method: 'POST', body: JSON.stringify(input) },
                )
                window.prompt('Upload URL', payload.uploadURL)
                await loadMedia()
              }, 'Upload URL created')
            }
            onRefresh={(id) =>
              void run(async () => {
                await apiFetch(`/api/admin/images/${id}/refresh`, { method: 'POST' })
                await loadMedia()
              }, 'Media refreshed')
            }
          />
        )}
        {active === 'ai' && (
          <AiView
            copy={aiCopy}
            onGenerate={(input) =>
              void run(async () => {
                setAiCopy(
                  await apiFetch<AiCopySuggestion>('/api/admin/ai/copy', {
                    method: 'POST',
                    body: JSON.stringify(input),
                  }),
                )
              }, 'Copy generated')
            }
          />
        )}
        {active === 'email' && (
          <EmailView
            templates={templates}
            logs={emailLogs}
            onCreate={(input) =>
              void run(async () => {
                await apiFetch('/api/admin/email/templates', { method: 'POST', body: JSON.stringify(input) })
                await loadEmail()
              }, 'Template created')
            }
          />
        )}
        {active === 'users' && (
          <UsersView
            users={users}
            onCreate={(input) =>
              void run(async () => {
                await apiFetch('/api/admin/users', { method: 'POST', body: JSON.stringify(input) })
                await loadUsers()
              }, 'User created')
            }
            onToggle={(id) =>
              void run(async () => {
                await apiFetch(`/api/admin/users/${id}/toggle`, { method: 'PUT' })
                await loadUsers()
              }, 'User toggled')
            }
          />
        )}
        {active === 'logs' && <LogsView logs={apiLogs} stats={stats} />}
        {active === 'ops' && (
          <OpsView
            ops={ops}
            onTask={(title) =>
              void run(async () => {
                await apiFetch('/api/admin/agt/tasks', { method: 'POST', body: JSON.stringify({ title }) })
                await loadOps()
              }, 'Task created')
            }
            onComplete={(id) =>
              void run(async () => {
                await apiFetch(`/api/admin/agt/tasks/${id}/complete`, { method: 'POST' })
                await loadOps()
              }, 'Task completed')
            }
            onNote={(note) =>
              void run(async () => {
                await apiFetch('/api/admin/agt/notes', { method: 'POST', body: JSON.stringify({ note }) })
                await loadOps()
              }, 'Note added')
            }
          />
        )}
      </section>
    </main>
  )
}

function DashboardView({ dashboard, stats }: { dashboard: DashboardData | null; stats: SystemStats | null }) {
  return (
    <>
      <div className="metrics">
        <Metric label="Leads" value={dashboard?.stats.totalLeads} />
        <Metric label="Media" value={dashboard?.stats.totalMedia} />
        <Metric label="Pages" value={dashboard?.stats.totalPages} />
        <Metric label="API requests" value={stats?.totalApiRequests} />
      </div>
      <div className="panels two">
        <Panel title="Recent leads">
          {dashboard?.recentLeads.map((lead) => <LeadRowView key={lead.id} lead={lead} />)}
        </Panel>
        <Panel title="Recent pages">
          {dashboard?.recentPages.map((page) => (
            <div className="row" key={page.id}>
              <strong>{page.title}</strong>
              <span>{page.status}</span>
            </div>
          ))}
        </Panel>
      </div>
    </>
  )
}

function LeadsView({
  leads,
  detail,
  onSelect,
  onStatus,
  onAddTag,
  onAddNote,
}: {
  leads: LeadRecord[]
  detail: LeadDetail | null
  onSelect: (id: number) => void
  onStatus: (id: number, status: LeadStatus) => void
  onAddTag: (id: number, tag: string) => void
  onAddNote: (id: number, content: string) => void
}) {
  const [tag, setTag] = useState('')
  const [note, setNote] = useState('')

  return (
    <div className="panels split">
      <Panel title="Lead queue">
        {leads.map((lead) => (
          <button className="list-button" key={lead.id} onClick={() => onSelect(lead.id)}>
            <span>{lead.name}</span>
            <small>{lead.email}</small>
          </button>
        ))}
      </Panel>
      <Panel title={detail ? detail.name : 'Lead detail'}>
        {detail && (
          <>
            <div className="detail-grid">
              <span>Email</span>
              <strong>{detail.email}</strong>
              <span>Status</span>
              <select value={detail.status} onChange={(event) => onStatus(detail.id, event.target.value as LeadStatus)}>
                {['new', 'contacted', 'qualified', 'converted', 'lost'].map((status) => (
                  <option key={status}>{status}</option>
                ))}
              </select>
              <span>Company</span>
              <strong>{detail.company ?? '-'}</strong>
            </div>
            <p className="message">{detail.message ?? 'No message'}</p>
            <div className="chips">{detail.tags.map((item) => <span key={item.id}>{item.tag}</span>)}</div>
            <InlineForm
              placeholder="Tag"
              value={tag}
              setValue={setTag}
              onSubmit={() => {
                onAddTag(detail.id, tag)
                setTag('')
              }}
            />
            <InlineForm
              placeholder="Note"
              value={note}
              setValue={setNote}
              onSubmit={() => {
                onAddNote(detail.id, note)
                setNote('')
              }}
            />
            <div className="stack">
              {detail.notes.map((item) => (
                <div className="row" key={item.id}>
                  <strong>{item.createdBy}</strong>
                  <span>{item.content}</span>
                </div>
              ))}
            </div>
          </>
        )}
      </Panel>
    </div>
  )
}

function PagesView({
  pages,
  detail,
  onSelect,
  onSave,
  onAction,
}: {
  pages: PageSummary[]
  detail: Page | null
  onSelect: (id: number) => void
  onSave: (input: PageInput, id?: number) => void
  onAction: (id: number, action: 'publish' | 'unpublish') => void
}) {
  const initial = detail ?? { slug: '', title: '', contentMd: '' }
  return (
    <div className="panels split">
      <Panel title="Pages">
        {pages.map((page) => (
          <button className="list-button" key={page.id} onClick={() => onSelect(page.id)}>
            <span>{page.title}</span>
            <small>{page.status}</small>
          </button>
        ))}
      </Panel>
      <Panel title="Editor">
        <JsonForm
          key={detail?.id ?? 'new'}
          fields={[
            ['slug', initial.slug],
            ['title', initial.title],
            ['contentMd', initial.contentMd],
          ]}
          multiline="contentMd"
          submitLabel={detail ? 'Save page' : 'Create page'}
          onSubmit={(input) => onSave(input as PageInput, detail?.id)}
        />
        {detail && (
          <div className="button-row">
            <button onClick={() => onAction(detail.id, 'publish')}>Publish</button>
            <button className="secondary" onClick={() => onAction(detail.id, 'unpublish')}>
              Unpublish
            </button>
          </div>
        )}
      </Panel>
    </div>
  )
}

function SettingsView({ settings, onSave }: { settings: SiteSettings; onSave: (input: SiteSettings) => void }) {
  return (
    <Panel title="Site settings">
      <JsonForm
        fields={[
          ['brand', settings.brand],
          ['heroLabel', settings.heroLabel],
          ['heroTitle', settings.heroTitle],
          ['heroSubtitle', settings.heroSubtitle],
          ['ctaPrimary', settings.ctaPrimary],
          ['ctaSecondary', settings.ctaSecondary],
        ]}
        submitLabel="Save settings"
        onSubmit={(input) => onSave({ ...settings, ...input })}
      />
    </Panel>
  )
}

function MediaView({
  media,
  onCreate,
  onRefresh,
}: {
  media: MediaAsset[]
  onCreate: (input: { title: string; alt?: string }) => void
  onRefresh: (id: string) => void
}) {
  return (
    <div className="panels split">
      <Panel title="Direct upload">
        <JsonForm
          fields={[
            ['title', 'Product screenshot'],
            ['alt', 'Dashboard screenshot'],
          ]}
          submitLabel="Create upload URL"
          onSubmit={(input) => onCreate(input as { title: string; alt?: string })}
        />
      </Panel>
      <Panel title="Assets">
        {media.map((asset) => (
          <div className="row with-action" key={asset.id}>
            <span>
              <strong>{asset.title}</strong>
              <small>{asset.status}</small>
            </span>
            <button className="secondary" onClick={() => onRefresh(asset.imageId)}>
              Refresh
            </button>
          </div>
        ))}
      </Panel>
    </div>
  )
}

function AiView({
  copy,
  onGenerate,
}: {
  copy: AiCopySuggestion | null
  onGenerate: (input: { objective: string; audience: string; tone: string }) => void
}) {
  return (
    <div className="panels split">
      <Panel title="Copy prompt">
        <JsonForm
          fields={[
            ['objective', 'Launch the Rust Worker SaaS boilerplate'],
            ['audience', 'Small product teams'],
            ['tone', 'clear and confident'],
          ]}
          submitLabel="Generate"
          onSubmit={(input) => onGenerate(input as { objective: string; audience: string; tone: string })}
        />
      </Panel>
      <Panel title="Suggestion">
        {copy && (
          <div className="stack">
            <strong>{copy.heroTitle}</strong>
            <span>{copy.heroSubtitle}</span>
            <span>{copy.ctaPrimary}</span>
            <p className="message">{copy.rationale}</p>
          </div>
        )}
      </Panel>
    </div>
  )
}

function EmailView({
  templates,
  logs,
  onCreate,
}: {
  templates: EmailTemplate[]
  logs: EmailLog[]
  onCreate: (input: EmailTemplateInput) => void
}) {
  return (
    <div className="panels split">
      <Panel title="New template">
        <JsonForm
          fields={[
            ['name', 'Welcome'],
            ['subject', 'Thanks for reaching out'],
            ['bodyHtml', '<p>Hello, thanks for your interest.</p>'],
            ['bodyText', 'Hello, thanks for your interest.'],
          ]}
          multiline="bodyHtml"
          submitLabel="Create template"
          onSubmit={(input) => onCreate(input as EmailTemplateInput)}
        />
      </Panel>
      <Panel title="Templates and logs">
        {templates.map((template) => (
          <div className="row" key={template.id}>
            <strong>{template.name}</strong>
            <span>{template.subject}</span>
          </div>
        ))}
        {logs.slice(0, 8).map((log) => (
          <div className="row" key={`log-${log.id}`}>
            <strong>{log.subject}</strong>
            <span>{log.status}</span>
          </div>
        ))}
      </Panel>
    </div>
  )
}

function UsersView({
  users,
  onCreate,
  onToggle,
}: {
  users: AdminUser[]
  onCreate: (input: AdminUserInput) => void
  onToggle: (id: number) => void
}) {
  return (
    <div className="panels split">
      <Panel title="Create user">
        <JsonForm
          fields={[
            ['email', 'editor@example.com'],
            ['name', 'Editor'],
            ['role', 'editor'],
          ]}
          submitLabel="Create user"
          onSubmit={(input) => onCreate(input as AdminUserInput)}
        />
      </Panel>
      <Panel title="Users">
        {users.map((user) => (
          <div className="row with-action" key={user.id}>
            <span>
              <strong>{user.email}</strong>
              <small>{user.role}</small>
            </span>
            <button className="secondary" onClick={() => onToggle(user.id)}>
              {user.isActive ? 'Disable' : 'Enable'}
            </button>
          </div>
        ))}
      </Panel>
    </div>
  )
}

function LogsView({ logs, stats }: { logs: ApiLog[]; stats: SystemStats | null }) {
  return (
    <>
      <div className="metrics">
        <Metric label="Users" value={stats?.totalUsers} />
        <Metric label="Active users" value={stats?.activeUsers} />
        <Metric label="Emails" value={stats?.totalEmails} />
        <Metric label="API requests" value={stats?.totalApiRequests} />
      </div>
      <Panel title="API logs">
        {logs.map((log) => (
          <div className="row code-row" key={log.id}>
            <strong>
              {log.method} {log.path}
            </strong>
            <span>
              {log.statusCode} / {log.durationMs ?? 0}ms
            </span>
          </div>
        ))}
      </Panel>
    </>
  )
}

function OpsView({
  ops,
  onTask,
  onComplete,
  onNote,
}: {
  ops: OpsState | null
  onTask: (title: string) => void
  onComplete: (id: string) => void
  onNote: (note: string) => void
}) {
  const [task, setTask] = useState('')
  const [note, setNote] = useState('')
  return (
    <div className="panels split">
      <Panel title="Tasks">
        <InlineForm
          placeholder="Task title"
          value={task}
          setValue={setTask}
          onSubmit={() => {
            onTask(task)
            setTask('')
          }}
        />
        {ops?.tasks.map((item) => (
          <div className="row with-action" key={item.id}>
            <span>
              <strong>{item.title}</strong>
              <small>{item.done ? 'done' : 'open'}</small>
            </span>
            {!item.done && (
              <button className="secondary" onClick={() => onComplete(item.id)}>
                Complete
              </button>
            )}
          </div>
        ))}
      </Panel>
      <Panel title="Notes">
        <InlineForm
          placeholder="Note"
          value={note}
          setValue={setNote}
          onSubmit={() => {
            onNote(note)
            setNote('')
          }}
        />
        {ops?.notes.map((item, index) => (
          <div className="row" key={`${item}-${index}`}>
            <span>{item}</span>
          </div>
        ))}
      </Panel>
    </div>
  )
}

function JsonForm({
  fields,
  multiline,
  submitLabel,
  onSubmit,
}: {
  fields: Array<[string, string | number | null | undefined]>
  multiline?: string
  submitLabel: string
  onSubmit: (input: Record<string, string>) => void
}) {
  const defaults = useMemo(
    () => Object.fromEntries(fields.map(([key, value]) => [key, value == null ? '' : String(value)])),
    [fields],
  )
  const [values, setValues] = useState(defaults)
  return (
    <form
      className="form-grid"
      onSubmit={(event) => {
        event.preventDefault()
        onSubmit(values)
      }}
    >
      {fields.map(([key]) => (
        <label key={key}>
          {labelize(key)}
          {multiline === key ? (
            <textarea rows={8} value={values[key]} onChange={(event) => setValues({ ...values, [key]: event.target.value })} />
          ) : (
            <input value={values[key]} onChange={(event) => setValues({ ...values, [key]: event.target.value })} />
          )}
        </label>
      ))}
      <button>{submitLabel}</button>
    </form>
  )
}

function InlineForm({
  placeholder,
  value,
  setValue,
  onSubmit,
}: {
  placeholder: string
  value: string
  setValue: (value: string) => void
  onSubmit: () => void
}) {
  return (
    <form
      className="inline-form"
      onSubmit={(event) => {
        event.preventDefault()
        if (value.trim()) onSubmit()
      }}
    >
      <input placeholder={placeholder} value={value} onChange={(event) => setValue(event.target.value)} />
      <button>Add</button>
    </form>
  )
}

function Field({ label, name, type = 'text', required = false }: { label: string; name: string; type?: string; required?: boolean }) {
  return (
    <label>
      {label}
      <input name={name} type={type} required={required} />
    </label>
  )
}

function Metric({ label, value }: { label: string; value?: number }) {
  return (
    <article className="metric">
      <span>{label}</span>
      <strong>{value ?? '-'}</strong>
    </article>
  )
}

function Panel({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <article className="panel">
      <h2>{title}</h2>
      <div className="stack">{children}</div>
    </article>
  )
}

function LeadRowView({ lead }: { lead: LeadRecord }) {
  return (
    <div className="row">
      <strong>{lead.name}</strong>
      <span>{lead.email}</span>
    </div>
  )
}

function labelize(key: string) {
  return key.replace(/([A-Z])/g, ' $1').replace(/^./, (value) => value.toUpperCase())
}

createRoot(document.getElementById('root')!).render(<App />)
