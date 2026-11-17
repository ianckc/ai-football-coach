import { cleanup, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/me'

/**
 * Acceptance slice (ianckc/ai-football-coach#71):
 * Frontend Vitest: `/me` works with the cookie session and rejects without
 * (401 / missing session → clear error, no invented profile; no sessionStorage).
 */
describe('me rejects without cookie session', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    localStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('on 401 without a JS-readable token shows an error and does not invent profile fields', async () => {
    expect(sessionStorage.getItem('token')).toBeNull()

    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve(
          new Response(JSON.stringify({ error: 'invalid token' }), {
            status: 401,
            headers: { 'Content-Type': 'application/json' },
          }),
        ),
      ),
    )

    const Me = Route.options.component
    expect(Me, 'expected /me route component').toBeTypeOf('function')
    if (typeof Me !== 'function') {
      throw new Error('expected /me route component')
    }

    render(<Me />)

    await waitFor(() => {
      expect(
        screen.getByText(/error|failed|unable|unavailable|could not|unauthorized|invalid/i),
        'must show a clear error when the session cookie is missing/invalid (401)',
      ).toBeInTheDocument()
    })

    expect(
      screen.queryByText(/coach@example\.com/i),
      'must not invent a profile email without a valid session',
    ).not.toBeInTheDocument()
    expect(
      screen.queryByText(/00000000-0000-4000-8000-000000000001/),
      'must not invent a profile id without a valid session',
    ).not.toBeInTheDocument()
  })
})
