import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Login } from '../src/routes/login'

async function submitLoginForm() {
  render(<Login />)

  fireEvent.change(screen.getByLabelText(/email/i), {
    target: { value: 'coach@example.com' },
  })
  fireEvent.change(screen.getByLabelText(/password/i), {
    target: { value: 'wrong-password' },
  })

  const form = document.querySelector('form')
  expect(form, 'page must include a form to submit').not.toBeNull()
  if (!form) {
    throw new Error('page must include a form to submit')
  }
  fireEvent.submit(form)
}

/**
 * Acceptance slice (ianckc/ai-football-coach#34):
 * On `401` or other failure, show a clear error state and leave the user logged out.
 */
describe('login error', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it.each([
    {
      label: '401',
      fetchImpl: () =>
        Promise.resolve(
          new Response(JSON.stringify({ error: 'unauthorized' }), {
            status: 401,
            headers: { 'Content-Type': 'application/json' },
          }),
        ),
    },
    {
      label: '500',
      fetchImpl: () =>
        Promise.resolve(
          new Response(JSON.stringify({ error: 'server' }), {
            status: 500,
            headers: { 'Content-Type': 'application/json' },
          }),
        ),
    },
    {
      label: 'unreachable backend',
      fetchImpl: () => Promise.reject(new TypeError('Failed to fetch')),
    },
  ])(
    'shows a clear error state when the backend returns $label and leaves the user logged out',
    async ({ fetchImpl }) => {
      vi.stubGlobal('fetch', vi.fn(fetchImpl))

      await submitLoginForm()

      await waitFor(() => {
        expect(
          screen.getByText(/error|failed|unable|unavailable|could not|invalid/i),
          'must show a clear error state',
        ).toBeInTheDocument()
      })

      expect(
        screen.queryByText(/logged in|welcome/i),
        'must leave the user logged out',
      ).not.toBeInTheDocument()

      expect(sessionStorage.getItem('token'), 'must not persist a token on failure').toBeNull()
    },
  )
})
