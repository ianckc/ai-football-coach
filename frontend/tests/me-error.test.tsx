import { cleanup, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/me'

/**
 * Acceptance slice (ianckc/ai-football-coach#61):
 * On `401` or other failure, show a clear error state and do not display
 * another user’s data or invent profile fields.
 */
describe('me error', () => {
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
    'shows a clear error state when the backend returns $label and does not invent profile fields',
    async ({ fetchImpl }) => {
      const token = 'opaque-session-token'
      sessionStorage.setItem('token', token)

      vi.stubGlobal('fetch', vi.fn(fetchImpl))

      const Me = Route.options.component
      expect(Me, 'expected /me route component').toBeTypeOf('function')
      if (typeof Me !== 'function') {
        throw new Error('expected /me route component')
      }

      render(<Me />)

      await waitFor(() => {
        expect(
          screen.getByText(/error|failed|unable|unavailable|could not|unauthorized|invalid/i),
          'must show a clear error state',
        ).toBeInTheDocument()
      })

      expect(
        screen.queryByText(/coach@example\.com/i),
        'must not invent a profile email on failure',
      ).not.toBeInTheDocument()
      expect(
        screen.queryByText(/00000000-0000-4000-8000-000000000001/),
        'must not invent a profile id on failure',
      ).not.toBeInTheDocument()
    },
  )
})
