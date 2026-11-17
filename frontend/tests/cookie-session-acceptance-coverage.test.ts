import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '../..')
const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

type RequiredCoverage = {
  area: string
  path: string
  mustMatch: RegExp
}

/**
 * Acceptance slice (ianckc/ai-football-coach#71):
 * Encode acceptance with backend cargo tests and frontend Vitest so
 * `quality-tests` can go red/green (login sets cookie; `/me` works with cookie
 * and rejects without; logout clears session; SSR/guard behavior covered).
 */
describe('cookie session acceptance coverage', () => {
  it('encodes login cookie, /me cookie accept/reject, logout, and SSR guard in cargo + vitest', () => {
    const required: RequiredCoverage[] = [
      {
        area: 'login sets cookie (backend)',
        path: join(repoRoot, 'backend/tests/login.rs'),
        mustMatch: /login_post_sets_httponly_session_cookie_with_opaque_token/,
      },
      {
        area: '/me works with cookie (backend)',
        path: join(repoRoot, 'backend/tests/sessions.rs'),
        mustMatch: /me_get_with_session_cookie_returns_matching_account/,
      },
      {
        area: '/me rejects without cookie (backend)',
        path: join(repoRoot, 'backend/tests/sessions.rs'),
        mustMatch: /me_get_rejects_missing_unknown_or_invalid_session_cookie/,
      },
      {
        area: 'logout clears session (backend)',
        path: join(repoRoot, 'backend/tests/logout.rs'),
        mustMatch: /logout_post_clears_session_cookie_and_deletes_sessions_row/,
      },
      {
        area: 'login cookie credentials (frontend)',
        path: join(frontendRoot, 'tests/login-cookie-session.test.tsx'),
        mustMatch: /credentials:\s*['"]include['"]|credentials['"]?\s*,?\s*['"]include['"]|\.toBe\(['"]include['"]\)/,
      },
      {
        area: '/me works with cookie credentials (frontend)',
        path: join(frontendRoot, 'tests/me-cookie-session.test.tsx'),
        mustMatch: /credentials/,
      },
      {
        area: '/me rejects without valid session (frontend)',
        path: join(frontendRoot, 'tests/me-rejects-without-cookie.test.tsx'),
        mustMatch: /401/,
      },
      {
        area: 'logout clears session (frontend)',
        path: join(frontendRoot, 'tests/logout-cookie-session.test.tsx'),
        mustMatch: /\/logout/,
      },
      {
        area: 'SSR/guard behavior (frontend)',
        path: join(frontendRoot, 'tests/me-ssr-guard.test.tsx'),
        mustMatch: /beforeLoad|sessionStorage|getCookie|SSR/,
      },
    ]

    const missing: string[] = []

    for (const item of required) {
      if (!existsSync(item.path)) {
        missing.push(`${item.area}: missing file ${item.path}`)
        continue
      }
      const source = readFileSync(item.path, 'utf8')
      if (!item.mustMatch.test(source)) {
        missing.push(`${item.area}: ${item.path} must match ${item.mustMatch}`)
      }
    }

    expect(missing, `quality-tests acceptance coverage gaps:\n${missing.join('\n')}`).toEqual([])
  })
})
