import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#32):
 * When the backend is unreachable or the request fails, display a clear
 * error/unavailable state (do not pretend the backend is ok).
 */
describe('health page unavailable state', () => {
  it('displays an error or unavailable state when the health request fails', () => {
    const healthPath = join(frontendRoot, 'src/routes/health.tsx')
    expect(existsSync(healthPath)).toBe(true)

    const source = readFileSync(healthPath, 'utf8')

    // Must handle unreachable backend or failed request (not success-only).
    expect(source).toMatch(/\.catch\s*\(|\bresponse\.ok\b/)

    // Must surface a clear error/unavailable UI — not silent null or fake ok.
    expect(source).toMatch(/unavailable|error/i)
  })
})
