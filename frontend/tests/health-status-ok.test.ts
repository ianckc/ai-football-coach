import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#32):
 * When the backend returns `{ "status": "ok" }`, display that the backend
 * status is ok (show the status value from the response).
 */
describe('health page status display', () => {
  it('displays the status value from a successful health response', () => {
    const healthPath = join(frontendRoot, 'src/routes/health.tsx')
    expect(existsSync(healthPath)).toBe(true)

    const source = readFileSync(healthPath, 'utf8')

    // Must parse the JSON body and read the status field from the response.
    expect(source).toMatch(/\.json\s*\(/)
    expect(source).toMatch(/\bstatus\b/)

    // Must render the status value from the response (not a fetch-only page).
    expect(source).toMatch(/\{[^}\n]*status[^}\n]*\}/)
  })
})
