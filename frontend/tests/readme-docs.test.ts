import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '../..')

/**
 * Acceptance slice (ianckc/ai-football-coach#17):
 * Document how to install and run the frontend in the repo README.
 */
describe('README docs', () => {
  it('repo README documents how to install and run the frontend', () => {
    const readmePath = join(repoRoot, 'README.md')
    expect(existsSync(readmePath)).toBe(true)

    const readme = readFileSync(readmePath, 'utf8')
    expect(readme).toMatch(/cd frontend/)
    expect(readme).toMatch(/npm install/)
    expect(readme).toMatch(/npm run dev/)
  })
})
