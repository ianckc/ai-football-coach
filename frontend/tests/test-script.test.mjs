import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#17):
 * `frontend/package.json` includes a `test` script that quality-tests can run
 * (`npm test -- --run`).
 */
describe('test script', () => {
  it('frontend package.json includes a test script for quality-tests', () => {
    const pkg = JSON.parse(
      readFileSync(join(frontendRoot, 'package.json'), 'utf8'),
    )
    const scripts = pkg.scripts ?? {}

    expect(typeof scripts.test).toBe('string')
    expect(scripts.test.trim().length).toBeGreaterThan(0)
  })
})
