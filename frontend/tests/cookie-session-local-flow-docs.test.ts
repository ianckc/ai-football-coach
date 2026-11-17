import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '../..')
const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#71):
 * Document the local login → `/me` → logout flow (and CORS / cookie topology for
 * Vite `localhost:3000` → backend `127.0.0.1:8000`) in README and/or
 * `docs/docs/` (frontend + backend-api).
 */
describe('cookie session local flow docs', () => {
  it('documents local login → /me → logout with cookie CORS for Vite → backend', () => {
    const repoReadmePath = join(repoRoot, 'README.md')
    const frontendReadmePath = join(frontendRoot, 'README.md')
    const frontendDocsPath = join(repoRoot, 'docs/docs/frontend.mdx')
    const backendDocsPath = join(repoRoot, 'docs/docs/backend-api.mdx')

    expect(existsSync(repoReadmePath)).toBe(true)
    expect(existsSync(frontendReadmePath)).toBe(true)
    expect(existsSync(frontendDocsPath)).toBe(true)
    expect(existsSync(backendDocsPath)).toBe(true)

    const repoReadme = readFileSync(repoReadmePath, 'utf8')
    const frontendReadme = readFileSync(frontendReadmePath, 'utf8')
    const frontendDocs = readFileSync(frontendDocsPath, 'utf8')
    const backendDocs = readFileSync(backendDocsPath, 'utf8')

    const frontendSurface = [repoReadme, frontendReadme, frontendDocs].join('\n')
    const allDocs = [frontendSurface, backendDocs].join('\n')
    const lower = allDocs.toLowerCase()

    expect(frontendSurface).toMatch(/http:\/\/localhost:3000/)
    expect(frontendSurface).toMatch(/http:\/\/127\.0\.0\.1:8000/)

    expect(
      lower,
      'docs must describe credentialed / cookie-based browser calls (not only Bearer from sessionStorage)',
    ).toMatch(/credentials|httponly|session cookie|cookie session/)

    expect(
      lower,
      'docs must cover CORS (or cookie SameSite) for local Vite → backend cross-origin sessions',
    ).toMatch(/cors|samesite|same-site|allow-credentials|access-control-allow-credentials/)

    // Local flow: login → /me → logout (issue allows README and/or docs/docs/).
    expect(frontendSurface).toMatch(/\/login/)
    expect(frontendSurface).toMatch(/\/me/)
    expect(
      frontendSurface,
      'frontend docs surface must document logout in the local cookie session flow',
    ).toMatch(/\/logout|logout/i)

    expect(
      backendDocs,
      'backend-api docs must document logout in the local cookie session flow',
    ).toMatch(/POST\s+\/logout|\/logout/)

    const frontendLower = [frontendReadme, frontendDocs].join('\n').toLowerCase()
    expect(
      frontendLower,
      'frontend docs must not still instruct storing the session token in sessionStorage for the browser auth session',
    ).not.toMatch(
      /token` in `sessionstorage|token in sessionstorage|stores the `token` in `sessionstorage/,
    )

    const flowDocs = [frontendReadme, frontendDocs, backendDocs, repoReadme]
      .join('\n')
      .toLowerCase()
    expect(
      flowDocs.includes('login') && flowDocs.includes('/me') && flowDocs.includes('logout'),
      'docs must cover the local login → /me → logout flow',
    ).toBe(true)
  })
})
