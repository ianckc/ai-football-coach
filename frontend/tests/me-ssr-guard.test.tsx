import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')
const meRoutePath = join(frontendRoot, 'src/routes/me.tsx')

/**
 * Acceptance slice (ianckc/ai-football-coach#71):
 * Frontend route guards for protected pages (at least `/me`) are SSR-safe:
 * unauthenticated users are sent to `/login` without relying on browser
 * `sessionStorage`, and logged-in users are not bounced on full page loads.
 */
describe('me SSR-safe route guard', () => {
  it('guards /me without sessionStorage; unauthenticated go to /login; logged-in full page loads are not bounced', () => {
    expect(existsSync(meRoutePath), 'expected a route file for /me at src/routes/me.tsx').toBe(true)
    if (!existsSync(meRoutePath)) {
      throw new Error('expected a route file for /me at src/routes/me.tsx')
    }

    const source = readFileSync(meRoutePath, 'utf8')
    expect(source).toContain("createFileRoute('/me')")
    expect(source, 'protected /me must declare a beforeLoad auth guard').toMatch(/\bbeforeLoad\b/)

    expect(
      source,
      'SSR-safe guard must not rely on browser sessionStorage for the auth session',
    ).not.toMatch(/sessionStorage/)
    expect(
      source,
      'SSR-safe guard must not rely on browser localStorage for the auth session',
    ).not.toMatch(/localStorage/)

    expect(
      source,
      'unauthenticated users must be sent to /login',
    ).toMatch(/redirect\s*\(|to:\s*['"]\/login['"]|['"]\/login['"]/)

    expect(
      source,
      'must not skip the auth guard on the server (typeof window === "undefined" early return) — that either leaves SSR ungarded or bounces logged-in users after hydrate when sessionStorage is empty',
    ).not.toMatch(
      /typeof\s+window\s*===\s*['"]undefined['"][\s\S]{0,80}?\breturn\b/,
    )

    expect(
      source,
      'SSR-safe guard must use the session cookie (readable on the server) rather than JS-only storage',
    ).toMatch(/\bgetCookie\b|\bcookie\b|\bCookie\b|document\.cookie|headers\.get\(\s*['"]cookie['"]/i)
  })
})
