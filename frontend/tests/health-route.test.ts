import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#32):
 * Add a frontend route (e.g. `/health`) that loads in the app.
 */
describe('health route', () => {
  it('registers a /health route that loads in the app', () => {
    const healthPath = join(frontendRoot, 'src/routes/health.tsx')
    expect(existsSync(healthPath)).toBe(true)

    const source = readFileSync(healthPath, 'utf8')
    expect(source).toContain("createFileRoute('/health')")
  })
})
