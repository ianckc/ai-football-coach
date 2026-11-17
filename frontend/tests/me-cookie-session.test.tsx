import { cleanup, render, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/me'

/**
 * Acceptance slice (ianckc/ai-football-coach#71):
 * `/me` (and its route guard) use cookie-based session (credentials /
 * same-site requests) instead of `Authorization: Bearer` from `sessionStorage`.
 */
describe('me cookie session', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    localStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('GETs /me with credentials and without Authorization Bearer from sessionStorage', async () => {
    expect(
      sessionStorage.getItem('token'),
      'setup: cookie-based /me must not rely on a sessionStorage token',
    ).toBeNull()

    const fetchMock = vi.fn(
      (..._args: Parameters<typeof fetch>): Promise<Response> =>
        Promise.resolve(
          new Response(
            JSON.stringify({
              id: '00000000-0000-4000-8000-000000000001',
              email: 'coach@example.com',
            }),
            {
              status: 200,
              headers: { 'Content-Type': 'application/json' },
            },
          ),
        ),
    )
    vi.stubGlobal('fetch', fetchMock)

    const Me = Route.options.component
    expect(Me, 'expected /me route component').toBeTypeOf('function')
    if (typeof Me !== 'function') {
      throw new Error('expected /me route component')
    }

    render(<Me />)

    await waitFor(() => {
      expect(fetchMock, 'cookie-based /me must call fetch without a sessionStorage token').toHaveBeenCalled()
    })

    const firstCall = fetchMock.mock.calls[0]
    expect(firstCall, 'cookie-based /me must call fetch').toBeDefined()
    if (!firstCall) {
      throw new Error('cookie-based /me must call fetch')
    }

    const [url, init] = firstCall
    expect(String(url)).toBe('http://127.0.0.1:8000/me')
    expect(String(init?.method ?? 'GET').toUpperCase()).toBe('GET')
    expect(
      init?.credentials,
      'GET /me must include credentials so the httpOnly session cookie is sent',
    ).toBe('include')

    const headers = new Headers(init?.headers)
    expect(
      headers.get('Authorization'),
      'browser /me must not send Authorization: Bearer from sessionStorage',
    ).toBeNull()
  })
})
