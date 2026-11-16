import { createFileRoute, redirect } from '@tanstack/react-router'
import { useEffect, useState } from 'react'

export const Route = createFileRoute('/me')({
  beforeLoad: () => {
    if (typeof sessionStorage === 'undefined' || !sessionStorage.getItem('token')) {
      throw redirect({ to: '/login' })
    }
  },
  component: Me,
})

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL ?? 'http://127.0.0.1:8000'

function Me() {
  const [id, setId] = useState<string | null>(null)
  const [email, setEmail] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const token = sessionStorage.getItem('token')
    if (!token) {
      return
    }

    void fetch(`${backendBaseUrl}/me`, {
      method: 'GET',
      headers: { Authorization: `Bearer ${token}` },
    })
      .then(async (response) => {
        if (response.status !== 200) {
          setId(null)
          setEmail(null)
          setError('Unable to load profile')
          return
        }
        const body = (await response.json()) as { id: string; email: string }
        setError(null)
        setId(body.id)
        setEmail(body.email)
      })
      .catch(() => {
        setId(null)
        setEmail(null)
        setError('Backend unavailable')
      })
  }, [])

  return (
    <main>
      <h1>Profile</h1>
      {id ? <p>{id}</p> : null}
      {email ? <p>{email}</p> : null}
      {error ? <p>{error}</p> : null}
    </main>
  )
}
