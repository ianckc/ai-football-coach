import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')
const meRoutePath = join(frontendRoot, 'src/routes/me.tsx')

/**
 * Acceptance slice (ianckc/ai-football-coach#61):
 * Add a frontend route (e.g. `/me`) that is only reachable when a session
 * `token` is present; if the user is not logged in, redirect to `/login`
 * (or otherwise refuse to show profile content).
 */
describe('me route auth guard', () => {
  it('registers /me only reachable with a session token; logged-out users go to /login or see no profile', () => {
    expect(existsSync(meRoutePath), 'expected a route file for /me at src/routes/me.tsx').toBe(true)
    if (!existsSync(meRoutePath)) {
      throw new Error('expected a route file for /me at src/routes/me.tsx')
    }

    const source = readFileSync(meRoutePath, 'utf8')
    expect(source).toContain("createFileRoute('/me')")
    expect(source, 'route must check the session token from login').toMatch(
      /sessionStorage\.getItem\(\s*['"]token['"]\s*\)/,
    )
    expect(
      source,
      'auth guard must not treat Node’s empty sessionStorage as the browser (check window)',
    ).toMatch(/typeof\s+window\s*===\s*['"]undefined['"]|typeof\s+window\s*!==\s*['"]undefined['"]/)
    expect(
      source,
      'when logged out, must redirect to /login or otherwise refuse profile content',
    ).toMatch(/['"]\/login['"]|to:\s*['"]\/login['"]|href=["']\/login["']/)
  })
})
