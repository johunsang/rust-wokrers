// React hook template for a new business module.
// Replace __ITEM__ / __item__ / __items__ with the real entity name.
// Adjust import paths to match the target module location.

import { useEffect, useState, useCallback } from 'react'
import { apiFetch } from '../../../com/api/client'

// import type { __ITEM__Record } from '@rust-wokrers/com'

type __ITEM__Record = {
  id: number
  title: string
  status: string
  createdAt: string
}

export function use__ITEM__s() {
  const [items, setItems] = useState<__ITEM__Record[]>([])
  const [loading, setLoading] = useState(true)

  const reload = useCallback(async () => {
    setLoading(true)
    try {
      const data = await apiFetch<__ITEM__Record[]>('/api/admin/__items__')
      setItems(data)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    reload()
  }, [reload])

  const create = async (input: { title: string }) => {
    await apiFetch('/api/admin/__items__', {
      method: 'POST',
      body: JSON.stringify(input),
    })
    await reload()
  }

  const update = async (id: number, input: { title?: string }) => {
    await apiFetch(`/api/admin/__items__/${id}`, {
      method: 'PUT',
      body: JSON.stringify(input),
    })
    await reload()
  }

  const remove = async (id: number) => {
    await apiFetch(`/api/admin/__items__/${id}`, {
      method: 'DELETE',
    })
    await reload()
  }

  return { items, loading, create, update, remove, reload }
}
