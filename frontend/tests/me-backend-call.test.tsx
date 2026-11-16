import { cleanup, render, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/me'

/**
 * Acceptance slice (ianckc/ai-football-coach#61):
 * When logged in, call backend `GET /me` with `Authorization: Bearer <token>`.
 */
describe('me backend call', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('when logged in, GETs /me with Authorization Bearer token using the backend base URL', async () => {
    const token = 'opaque-session-token'
    sessionStorage.setItem('token', token)

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
      expect(fetchMock, 'logged-in /me must call fetch').toHaveBeenCalled()
    })

    const firstCall = fetchMock.mock.calls[0]
    expect(firstCall, 'logged-in /me must call fetch').toBeDefined()
    if (!firstCall) {
      throw new Error('logged-in /me must call fetch')
    }

    const [url, init] = firstCall
    expect(String(url)).toBe('http://127.0.0.1:8000/me')
    expect(String(init?.method ?? 'GET').toUpperCase()).toBe('GET')

    const headers = new Headers(init?.headers)
    expect(headers.get('Authorization')).toBe(`Bearer ${token}`)
  })
})
