import { cleanup, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Route } from '../src/routes/me'

/**
 * Acceptance slice (ianckc/ai-football-coach#61):
 * On success (`200` with `{ "id", "email" }`), display the returned profile
 * data (`id` and `email`) on the page.
 */
describe('me success', () => {
  afterEach(() => {
    cleanup()
    sessionStorage.clear()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('on 200 displays the returned profile id and email', async () => {
    const id = '00000000-0000-4000-8000-000000000001'
    const email = 'coach@example.com'
    const token = 'opaque-session-token'
    sessionStorage.setItem('token', token)

    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve(
          new Response(JSON.stringify({ id, email }), {
            status: 200,
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
        screen.getByText(new RegExp(id.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'))),
        'profile page must display the returned id',
      ).toBeInTheDocument()
    })

    expect(
      screen.getByText(new RegExp(email.replace('.', '\\.'), 'i')),
      'profile page must display the returned email',
    ).toBeInTheDocument()
  })
})
