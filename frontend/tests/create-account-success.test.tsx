import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/accounts.create'

const CreateAccount = Route.options.component

/**
 * Acceptance slice (ianckc/ai-football-coach#49):
 * When the backend returns `201` with `{ "id", "email" }`, show a clear success
 * state that includes the created account `email` (do not show the password).
 */
describe('create account success', () => {
  afterEach(() => {
    cleanup()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('shows a success state with the created account email and not the password', async () => {
    expect(CreateAccount, 'create-account route must define a component').toBeTypeOf('function')

    const email = 'coach@example.com'
    const password = 'secret123'

    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve(
          new Response(
            JSON.stringify({
              id: '00000000-0000-4000-8000-000000000001',
              email,
            }),
            {
              status: 201,
              headers: { 'Content-Type': 'application/json' },
            },
          ),
        ),
      ),
    )

    render(<CreateAccount />)

    fireEvent.change(screen.getByLabelText(/email/i), {
      target: { value: email },
    })
    fireEvent.change(screen.getByLabelText(/password/i), {
      target: { value: password },
    })

    const form = document.querySelector('form')
    expect(form, 'page must include a form to submit').not.toBeNull()
    fireEvent.submit(form!)

    await waitFor(() => {
      expect(
        screen.getByText(/success|created|account created/i),
        'must show a clear success state',
      ).toBeInTheDocument()
    })

    expect(
      screen.getByText(new RegExp(email.replace('.', '\\.'), 'i')),
      'success state must include the created account email',
    ).toBeInTheDocument()

    expect(
      screen.queryByText(password),
      'success state must not show the password',
    ).not.toBeInTheDocument()
  })
})
