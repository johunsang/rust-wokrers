// Panel component template for a new business module.
// Replace __ITEM__ / __item__ / __items__ with the real entity name.

import { useState } from 'react'
import { Panel } from '../../../com/ui/Panel'

type __ITEM__Record = {
  id: number
  title: string
  status: string
  createdAt: string
}

type Props = {
  items: __ITEM__Record[]
  onCreate: (input: { title: string }) => Promise<void>
  onUpdate: (id: number, input: { title: string }) => Promise<void>
  onRemove: (id: number) => Promise<void>
}

export function __ITEM__Panel({ items, onCreate, onUpdate, onRemove }: Props) {
  const [newTitle, setNewTitle] = useState('')

  const handleCreate = async () => {
    if (!newTitle.trim()) return
    await onCreate({ title: newTitle.trim() })
    setNewTitle('')
  }

  return (
    <Panel eyebrow="biz/__item__" title="Manage __ITEM__">
      {/* Create form */}
      <div style={{ display: 'flex', gap: 8, marginBottom: 16 }}>
        <input
          value={newTitle}
          onChange={(e) => setNewTitle(e.target.value)}
          placeholder="New __item__ title"
          onKeyDown={(e) => e.key === 'Enter' && handleCreate()}
        />
        <button onClick={handleCreate}>Add</button>
      </div>

      {/* List */}
      {items.length === 0 && <p>No __items__ yet.</p>}

      {items.map((item) => (
        <div key={item.id} style={{ display: 'flex', gap: 8, marginBottom: 8, alignItems: 'center' }}>
          <span style={{ flex: 1 }}>
            <strong>{item.title}</strong>
            <small style={{ marginLeft: 8, opacity: 0.6 }}>{item.status}</small>
          </span>
          <button onClick={() => onUpdate(item.id, { title: `${item.title} (edited)` })}>Edit</button>
          <button onClick={() => onRemove(item.id)}>Delete</button>
        </div>
      ))}
    </Panel>
  )
}
