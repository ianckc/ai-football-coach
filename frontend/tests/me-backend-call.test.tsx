import { cleanup, render, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/me'

/**
 * Acceptance slice (ianckc/ai-football-coach#61), superseded for auth transport by #71:
 * When logged in, call backend `GET /me` using the backend base URL with
 * cookie credentials (not `Authorization: Bearer` from `sessionStorage`).
 */
describe('me backend call', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('GETs /me with credentials using the backend base URL and without Authorization Bearer', async () => {
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
    expect(init?.credentials).toBe('include')

    const headers = new Headers(init?.headers)
    expect(headers.get('Authorization')).toBeNull()
  })
})
