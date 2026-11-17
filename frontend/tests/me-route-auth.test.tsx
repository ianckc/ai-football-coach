import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')
const meRoutePath = join(frontendRoot, 'src/routes/me.tsx')

/**
 * Acceptance slice (ianckc/ai-football-coach#61), superseded for storage by #71:
 * Add a frontend route (e.g. `/me`) that is only reachable when a session is
 * present; if the user is not logged in, redirect to `/login`.
 * #71: auth is the httpOnly session cookie (SSR-readable), not sessionStorage.
 */
describe('me route auth guard', () => {
  it('registers /me only reachable with a session; logged-out users go to /login', () => {
    expect(existsSync(meRoutePath), 'expected a route file for /me at src/routes/me.tsx').toBe(true)
    if (!existsSync(meRoutePath)) {
      throw new Error('expected a route file for /me at src/routes/me.tsx')
    }

    const source = readFileSync(meRoutePath, 'utf8')
    expect(source).toContain("createFileRoute('/me')")
    expect(source, 'protected /me must declare a beforeLoad auth guard').toMatch(/\bbeforeLoad\b/)
    expect(source, 'route must check the session cookie from login (not sessionStorage)').toMatch(
      /\bgetCookie\b|\bcookie\b|\bCookie\b/i,
    )
    expect(
      source,
      'SSR-safe guard must not rely on browser sessionStorage for the auth session',
    ).not.toMatch(/sessionStorage/)
    expect(
      source,
      'when logged out, must redirect to /login or otherwise refuse profile content',
    ).toMatch(/['"]\/login['"]|to:\s*['"]\/login['"]|href=["']\/login["']/)
  })
})
