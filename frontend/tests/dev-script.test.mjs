import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#17):
 * App starts locally with a documented npm (or package-manager) script.
 */
describe('dev script', () => {
  it('frontend has a documented npm script to start locally', () => {
    const pkg = JSON.parse(
      readFileSync(join(frontendRoot, 'package.json'), 'utf8'),
    )
    const scripts = pkg.scripts ?? {}

    expect(typeof scripts.dev).toBe('string')
    expect(scripts.dev.trim().length).toBeGreaterThan(0)
  })
})
