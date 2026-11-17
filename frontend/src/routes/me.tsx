import { createFileRoute, redirect } from '@tanstack/react-router'
import { createServerFn } from '@tanstack/react-start'
import { getCookie } from '@tanstack/react-start/server'
import { useEffect, useState } from 'react'

/** SSR-readable session check via the httpOnly `session` cookie. */
const hasSessionCookie = createServerFn({ method: 'GET' }).handler(() => {
  const session = getCookie('session')
  return Boolean(session)
})

export const Route = createFileRoute('/me')({
  beforeLoad: async () => {
    const loggedIn = await hasSessionCookie()
    if (!loggedIn) {
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
    void fetch(`${backendBaseUrl}/me`, {
      method: 'GET',
      credentials: 'include',
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

  function handleLogout() {
    void fetch(`${backendBaseUrl}/logout`, {
      method: 'POST',
      credentials: 'include',
    })
      .then((response) => {
        if (response.status !== 200) {
          setError('Unable to logout')
          return
        }
        setId(null)
        setEmail(null)
        setError(null)
        window.location.replace('/login')
      })
      .catch(() => {
        setError('Backend unavailable')
      })
  }

  return (
    <main>
      <h1>Profile</h1>
      {id ? <p>{id}</p> : null}
      {email ? <p>{email}</p> : null}
      {error ? <p>{error}</p> : null}
      <button type="button" onClick={handleLogout}>
        Logout
      </button>
    </main>
  )
}
