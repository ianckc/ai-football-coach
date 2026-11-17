import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Login } from '../src/routes/login'

/**
 * Acceptance slice (ianckc/ai-football-coach#71):
 * Frontend `/login` no longer persists the session `token` in `sessionStorage`
 * (or any JS-readable storage) for the auth session; use cookie-based session
 * (credentials / same-site requests) instead.
 */
describe('login cookie session', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    localStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('on 200 does not persist token in JS-readable storage and POSTs login with credentials', async () => {
    const email = 'coach@example.com'
    const password = 'secret123'
    const token = 'opaque-session-token'

    const fetchMock = vi.fn(
      (..._args: Parameters<typeof fetch>): Promise<Response> =>
        Promise.resolve(
          new Response(
            JSON.stringify({
              id: '00000000-0000-4000-8000-000000000001',
              email,
              token,
            }),
            {
              status: 200,
              headers: { 'Content-Type': 'application/json' },
            },
          ),
        ),
    )
    vi.stubGlobal('fetch', fetchMock)

    render(<Login />)

    fireEvent.change(screen.getByLabelText(/email/i), {
      target: { value: email },
    })
    fireEvent.change(screen.getByLabelText(/password/i), {
      target: { value: password },
    })

    const form = document.querySelector('form')
    expect(form, 'page must include a form to submit').not.toBeNull()
    if (!form) {
      throw new Error('page must include a form to submit')
    }
    fireEvent.submit(form)

    await waitFor(() => {
      expect(fetchMock, 'submit must call fetch').toHaveBeenCalled()
    })

    const firstCall = fetchMock.mock.calls[0]
    expect(firstCall, 'submit must call fetch').toBeDefined()
    if (!firstCall) {
      throw new Error('submit must call fetch')
    }

    const [, init] = firstCall
    expect(
      init?.credentials,
      'POST /login must include credentials so the browser stores the httpOnly session cookie',
    ).toBe('include')

    await waitFor(() => {
      expect(
        screen.getByText(/logged in|welcome/i),
        'must still show a logged-in state on success',
      ).toBeInTheDocument()
    })

    expect(
      sessionStorage.getItem('token'),
      'must not persist the session token in sessionStorage',
    ).toBeNull()
    expect(
      localStorage.getItem('token'),
      'must not persist the session token in localStorage',
    ).toBeNull()
  })
})
