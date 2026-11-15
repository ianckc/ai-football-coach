import { createFileRoute } from '@tanstack/react-router'
import type { FormEvent } from 'react'

export const Route = createFileRoute('/accounts/create')({
  component: CreateAccount,
})

const backendBaseUrl =
  import.meta.env.VITE_BACKEND_BASE_URL ?? 'http://127.0.0.1:8000'

function CreateAccount() {
  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    const formData = new FormData(event.currentTarget)
    const email = String(formData.get('email') ?? '')
    const password = String(formData.get('password') ?? '')

    void fetch(`${backendBaseUrl}/accounts`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password }),
    })
  }

  return (
    <main>
      <h1>Create account</h1>
      <form onSubmit={handleSubmit}>
        <label htmlFor="email">
          Email
          <input id="email" name="email" type="email" autoComplete="email" />
        </label>
        <label htmlFor="password">
          Password
          <input
            id="password"
            name="password"
            type="password"
            autoComplete="new-password"
          />
        </label>
        <button type="submit">Create account</button>
      </form>
    </main>
  )
}
