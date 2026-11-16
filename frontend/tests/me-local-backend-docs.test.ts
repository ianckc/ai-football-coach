import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '../..')
const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#61):
 * Document how to run the frontend profile flow against a local backend (README
 * and/or `docs/docs/frontend.mdx`), including logging in first and the default
 * backend base URL (`http://127.0.0.1:8000`).
 */
describe('me local backend docs', () => {
  it('documents running the frontend /me profile flow against a local backend', () => {
    const repoReadmePath = join(repoRoot, 'README.md')
    const frontendReadmePath = join(frontendRoot, 'README.md')
    const frontendDocsPath = join(repoRoot, 'docs/docs/frontend.mdx')

    expect(existsSync(repoReadmePath)).toBe(true)
    expect(existsSync(frontendReadmePath)).toBe(true)
    expect(existsSync(frontendDocsPath)).toBe(true)

    const frontendReadme = readFileSync(frontendReadmePath, 'utf8')
    const frontendDocs = readFileSync(frontendDocsPath, 'utf8')
    const docs = [readFileSync(repoReadmePath, 'utf8'), frontendReadme, frontendDocs].join('\n')

    expect(docs).toMatch(/http:\/\/127\.0\.0\.1:8000/)
    expect(docs).toMatch(/VITE_BACKEND_BASE_URL/)

    const pageDocs = [frontendReadme, frontendDocs].join('\n')
    expect(pageDocs).toMatch(/\/me/)
    expect(pageDocs).toMatch(/GET\s+\/me|\/me/)
    expect(pageDocs).toMatch(/login|\/login/i)
  })
})
