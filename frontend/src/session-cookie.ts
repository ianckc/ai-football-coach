import { createServerFn } from '@tanstack/react-start'
import { deleteCookie, setCookie } from '@tanstack/react-start/server'

/** Frontend-origin httpOnly session cookie (SSR-visible on the Vite host). */
const SESSION_COOKIE = 'session'

const frontendSessionCookieOptions = {
  httpOnly: true,
  path: '/',
  sameSite: 'lax',
} as const

/**
 * Mirror the opaque API session token onto the frontend origin so SSR
 * `getCookie('session')` on localhost can see a logged-in user. The API
 * cookie on 127.0.0.1 remains the credential for `GET /me`.
 */
export const establishSessionCookie = createServerFn({ method: 'POST' })
  .validator((data: unknown) => {
    if (
      typeof data !== 'object' ||
      data === null ||
      !('token' in data) ||
      typeof (data as { token: unknown }).token !== 'string' ||
      (data as { token: string }).token.length === 0
    ) {
      throw new Error('token is required')
    }
    return { token: (data as { token: string }).token }
  })
  .handler(({ data }) => {
    setCookie(SESSION_COOKIE, data.token, frontendSessionCookieOptions)
    return { ok: true as const }
  })

/** Clear the frontend-origin session cookie (pair with backend POST /logout). */
export const clearSessionCookie = createServerFn({ method: 'POST' }).handler(() => {
  deleteCookie(SESSION_COOKIE, frontendSessionCookieOptions)
  return { ok: true as const }
})
