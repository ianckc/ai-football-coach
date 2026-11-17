import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/me'

/**
 * Acceptance slice (ianckc/ai-football-coach#71):
 * Encode frontend Vitest coverage for logout clearing the cookie session
 * (`POST /logout` with credentials / same-site requests).
 */
describe('logout cookie session', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    localStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('POSTs /logout with credentials to clear the session cookie', async () => {
    const fetchMock = vi.fn((input: RequestInfo | URL, init?: RequestInit) => {
      const url = String(input)
      if (url.endsWith('/me') || url.includes('/me')) {
        return Promise.resolve(
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
        )
      }
      if (url.endsWith('/logout') || url.includes('/logout')) {
        return Promise.resolve(new Response(null, { status: 200 }))
      }
      return Promise.resolve(new Response(null, { status: 404 }))
    })
    vi.stubGlobal('fetch', fetchMock)

    const Me = Route.options.component
    expect(Me, 'expected /me route component').toBeTypeOf('function')
    if (typeof Me !== 'function') {
      throw new Error('expected /me route component')
    }

    render(<Me />)

    const logoutControl = await screen.findByRole('button', { name: /logout/i })
    fireEvent.click(logoutControl)

    await waitFor(() => {
      const logoutCall = fetchMock.mock.calls.find(([url]) => String(url).includes('/logout'))
      expect(logoutCall, 'logout must call fetch against /logout').toBeDefined()
    })

    const logoutCall = fetchMock.mock.calls.find(([url]) => String(url).includes('/logout'))
    expect(logoutCall, 'logout must call fetch against /logout').toBeDefined()
    if (!logoutCall) {
      throw new Error('logout must call fetch against /logout')
    }

    const [url, init] = logoutCall
    expect(String(url)).toBe('http://127.0.0.1:8000/logout')
    expect(String(init?.method ?? 'GET').toUpperCase()).toBe('POST')
    expect(
      init?.credentials,
      'POST /logout must include credentials so the browser sends/clears the httpOnly session cookie',
    ).toBe('include')
  })
})
