import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')
const loginRoutePath = join(frontendRoot, 'src/routes/login.tsx')

/**
 * Acceptance slice (ianckc/ai-football-coach#34):
 * Add a frontend route (e.g. `/login`) that shows a login form with `email` and
 * `password` fields and a submit control.
 */
describe('login page', () => {
  it('registers /login with a form that has email, password, and a submit control', () => {
    expect(
      existsSync(loginRoutePath),
      'expected a route file for /login at src/routes/login.tsx',
    ).toBe(true)
    if (!existsSync(loginRoutePath)) {
      throw new Error('expected a route file for /login at src/routes/login.tsx')
    }

    const source = readFileSync(loginRoutePath, 'utf8')
    expect(source).toContain("createFileRoute('/login')")
    expect(source, 'page must include a form').toMatch(/<form[\s>]/)
    expect(source, 'form must include an email field').toMatch(
      /htmlFor=["']email["']|id=["']email["']|name=["']email["']|label[^>]*>\s*email/i,
    )
    expect(source, 'form must include a password field').toMatch(/type=["']password["']/)
    expect(source, 'form must include a submit control').toMatch(/type=["']submit["']/)
  })
})
