import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/accounts.create'

const CreateAccount = Route.options.component

async function submitCreateAccountForm() {
  expect(CreateAccount, 'create-account route must define a component').toBeTypeOf('function')

  render(<CreateAccount />)

  fireEvent.change(screen.getByLabelText(/email/i), {
    target: { value: 'coach@example.com' },
  })
  fireEvent.change(screen.getByLabelText(/password/i), {
    target: { value: 'secret123' },
  })

  const form = document.querySelector('form')
  expect(form, 'page must include a form to submit').not.toBeNull()
  fireEvent.submit(form!)
}

/**
 * Acceptance slice (ianckc/ai-football-coach#49):
 * When the backend returns `400` or `409` (or the request fails / backend is
 * unreachable), show a clear error state (do not pretend the account was created).
 */
describe('create account error', () => {
  afterEach(() => {
    cleanup()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it.each([
    {
      label: '400',
      fetchImpl: () =>
        Promise.resolve(
          new Response(JSON.stringify({ error: 'invalid' }), {
            status: 400,
            headers: { 'Content-Type': 'application/json' },
          }),
        ),
    },
    {
      label: '409',
      fetchImpl: () =>
        Promise.resolve(
          new Response(JSON.stringify({ error: 'conflict' }), {
            status: 409,
            headers: { 'Content-Type': 'application/json' },
          }),
        ),
    },
    {
      label: 'unreachable backend',
      fetchImpl: () => Promise.reject(new TypeError('Failed to fetch')),
    },
  ])(
    'shows a clear error state when the backend returns $label (and does not show success)',
    async ({ fetchImpl }) => {
      vi.stubGlobal('fetch', vi.fn(fetchImpl))

      await submitCreateAccountForm()

      await waitFor(() => {
        expect(
          screen.getByText(/error|failed|unable|unavailable|could not/i),
          'must show a clear error state',
        ).toBeInTheDocument()
      })

      expect(
        screen.queryByText(/success|account created/i),
        'must not pretend the account was created',
      ).not.toBeInTheDocument()
    },
  )
})
