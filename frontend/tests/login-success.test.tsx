import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Login } from '../src/routes/login'

/**
 * Acceptance slice (ianckc/ai-football-coach#34):
 * On success (`200` with `{ "id", "email", "token" }`), persist the `token` for
 * the browser session and show a logged-in state that includes the coach `email`
 * (do not display the password or treat failure as success).
 */
describe('login success', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('on 200 persists the token and shows a logged-in state with email but not the password', async () => {
    const email = 'coach@example.com'
    const password = 'secret123'
    const token = 'opaque-session-token'

    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
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
      ),
    )

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
      expect(
        screen.getByText(/logged in|welcome/i),
        'must show a clear logged-in state',
      ).toBeInTheDocument()
    })

    expect(
      screen.getByText(new RegExp(email.replace('.', '\\.'), 'i')),
      'logged-in state must include the coach email',
    ).toBeInTheDocument()

    expect(
      screen.queryByText(password),
      'logged-in state must not show the password',
    ).not.toBeInTheDocument()

    expect(sessionStorage.getItem('token'), 'token must be persisted for the browser session').toBe(
      token,
    )
  })

  it('does not treat a non-200 response as a successful login', async () => {
    const fetchMock = vi.fn(() =>
      Promise.resolve(
        new Response(JSON.stringify({ error: 'unauthorized' }), {
          status: 401,
          headers: { 'Content-Type': 'application/json' },
        }),
      ),
    )
    vi.stubGlobal('fetch', fetchMock)

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

    await waitFor(() => {
      expect(fetchMock, 'submit must call fetch').toHaveBeenCalled()
    })

    expect(
      screen.queryByText(/logged in|welcome/i),
      'must not show a logged-in state on failure',
    ).not.toBeInTheDocument()

    expect(sessionStorage.getItem('token'), 'must not persist a token on failure').toBeNull()
  })
})
