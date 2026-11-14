import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#32):
 * On that page, call the backend `GET /health` endpoint.
 */
describe('health page backend call', () => {
  it('calls the backend GET /health endpoint', () => {
    const healthPath = join(frontendRoot, 'src/routes/health.tsx')
    expect(existsSync(healthPath)).toBe(true)

    const source = readFileSync(healthPath, 'utf8')
    // Must issue an HTTP request — registering the app route alone is not enough.
    expect(source).toMatch(/\bfetch\s*\(/)

    // `/health` must appear as a request path outside createFileRoute('/health').
    const withoutFileRoute = source.replace(/createFileRoute\(\s*['"]\/health['"]\s*\)/g, '')
    expect(withoutFileRoute).toMatch(/\/health/)
  })
})
