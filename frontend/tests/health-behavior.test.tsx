import { cleanup, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { Health } from '../src/routes/health'

/**
 * Acceptance (ianckc/ai-football-coach#39):
 * Behavioral checks for `/health` — mock fetch, assert ok status,
 * unavailable/error UI, and a clear loading state while in flight.
 */
describe('health page behavior', () => {
  afterEach(() => {
    cleanup()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('shows a loading state while the health request is in flight', async () => {
    let resolveFetch!: (value: Response) => void
    const pending = new Promise<Response>((resolve) => {
      resolveFetch = resolve
    })
    vi.stubGlobal(
      'fetch',
      vi.fn(() => pending),
    )

    render(<Health />)

    expect(screen.getByText(/loading health status/i)).toBeInTheDocument()
    expect(screen.queryByText(/backend status:/i)).not.toBeInTheDocument()
    expect(screen.queryByText(/backend unavailable/i)).not.toBeInTheDocument()

    resolveFetch(
      new Response(JSON.stringify({ status: 'ok' }), {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
      }),
    )

    await waitFor(() => {
      expect(screen.getByText(/backend status:\s*ok/i)).toBeInTheDocument()
    })
    expect(screen.queryByText(/loading health status/i)).not.toBeInTheDocument()
  })

  it('displays the status value from a successful health response', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve(
          new Response(JSON.stringify({ status: 'ok' }), {
            status: 200,
            headers: { 'Content-Type': 'application/json' },
          }),
        ),
      ),
    )

    render(<Health />)

    await waitFor(() => {
      expect(screen.getByText(/backend status:\s*ok/i)).toBeInTheDocument()
    })
    expect(screen.queryByText(/backend unavailable/i)).not.toBeInTheDocument()
  })

  it('displays an unavailable state when the health request fails', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(() => Promise.reject(new TypeError('Failed to fetch'))),
    )

    render(<Health />)

    await waitFor(() => {
      expect(screen.getByText(/backend unavailable/i)).toBeInTheDocument()
    })
    expect(screen.queryByText(/backend status:/i)).not.toBeInTheDocument()
  })

  it('displays an unavailable state when the health response is not ok', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(() =>
        Promise.resolve(
          new Response('error', {
            status: 500,
            headers: { 'Content-Type': 'text/plain' },
          }),
        ),
      ),
    )

    render(<Health />)

    await waitFor(() => {
      expect(screen.getByText(/backend unavailable/i)).toBeInTheDocument()
    })
  })
})
