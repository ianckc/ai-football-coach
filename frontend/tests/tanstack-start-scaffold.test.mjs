import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#17):
 * Create a `frontend/` app using TanStack Start.
 */
describe('TanStack Start scaffold', () => {
  it('frontend is a TanStack Start app', () => {
    const pkgPath = join(frontendRoot, 'package.json')
    expect(existsSync(pkgPath)).toBe(true)

    const pkg = JSON.parse(readFileSync(pkgPath, 'utf8'))
    const deps = { ...pkg.dependencies, ...pkg.devDependencies }

    expect(deps['@tanstack/react-start']).toBeTruthy()
  })
})
