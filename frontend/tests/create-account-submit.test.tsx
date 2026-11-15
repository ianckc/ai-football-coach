import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { afterEach, describe, expect, it, vi } from 'vitest'
import { CreateAccount } from '../src/routes/accounts.create'

/**
 * Acceptance slice (ianckc/ai-football-coach#49):
 * On submit, call the backend `POST /accounts` with JSON `{ "email", "password" }`
 * (`Content-Type: application/json`) using the configured backend base URL.
 */
describe('create account submit', () => {
  afterEach(() => {
    cleanup()
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('on submit POSTs JSON email and password to /accounts using the backend base URL', async () => {
    const fetchMock = vi.fn(
      (..._args: Parameters<typeof fetch>): Promise<Response> =>
        Promise.resolve(
          new Response(
            JSON.stringify({
              id: '00000000-0000-4000-8000-000000000001',
              email: 'coach@example.com',
            }),
            {
              status: 201,
              headers: { 'Content-Type': 'application/json' },
            },
          ),
        ),
    )
    vi.stubGlobal('fetch', fetchMock)

    render(<CreateAccount />)

    fireEvent.change(screen.getByLabelText(/email/i), {
      target: { value: 'coach@example.com' },
    })
    fireEvent.change(screen.getByLabelText(/password/i), {
      target: { value: 'secret123' },
    })

    const form = document.querySelector('form')
    expect(form, 'page must include a form to submit').not.toBeNull()
    if (!form) {
      throw new Error('page must include a form to submit')
    }
    fireEvent.submit(form)

    await waitFor(() => {
      expect(fetchMock, 'submit must call fetch').toHaveBeenCalled()
    })

    const firstCall = fetchMock.mock.calls[0]
    expect(firstCall, 'submit must call fetch').toBeDefined()
    if (!firstCall) {
      throw new Error('submit must call fetch')
    }

    const [url, init] = firstCall
    expect(String(url)).toBe('http://127.0.0.1:8000/accounts')
    expect(String(init?.method ?? 'GET').toUpperCase()).toBe('POST')

    const headers = new Headers(init?.headers)
    expect(headers.get('Content-Type')).toMatch(/application\/json/i)

    expect(JSON.parse(String(init?.body))).toEqual({
      email: 'coach@example.com',
      password: 'secret123',
    })
  })
})
