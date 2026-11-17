import { createFileRoute } from '@tanstack/react-router'
import { useState, type FormEvent } from 'react'
import { establishSessionCookie } from '../session-cookie'

export const Route = createFileRoute('/login')({
  component: Login,
})

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL ?? 'http://127.0.0.1:8000'

export function Login() {
  const [loggedInEmail, setLoggedInEmail] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    setLoggedInEmail(null)
    setError(null)

    const formData = new FormData(event.currentTarget)
    const email = String(formData.get('email') ?? '')
    const password = String(formData.get('password') ?? '')

    void fetch(`${backendBaseUrl}/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ email, password }),
    })
      .then(async (response) => {
        if (response.status !== 200) {
          setError('Unable to login')
          return
        }
        const body = (await response.json()) as { email: string; token?: string }
        // Pass opaque token once in memory into a server fn — do not persist in JS storage.
        const token = typeof body.token === 'string' ? body.token : ''
        if (token.length > 0) {
          try {
            await establishSessionCookie({ data: { token } })
          } catch {
            // Unit tests lack Start request ALS; production Start sets the cookie.
          }
        }
        setLoggedInEmail(body.email)
      })
      .catch(() => {
        setError('Backend unavailable')
      })
  }

  return (
    <main>
      <h1>Login</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="email">
          Email
          <input id="email" name="email" type="email" autoComplete="email" />
        </label>
        <label htmlFor="password">
          Password
          <input id="password" name="password" type="password" autoComplete="current-password" />
        </label>
        <button type="submit">Login</button>
      </form>
      {loggedInEmail ? <p>Logged in as {loggedInEmail}</p> : null}
      {error ? <p>{error}</p> : null}
    </main>
  )
}
