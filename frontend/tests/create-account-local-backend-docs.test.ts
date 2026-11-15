import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '../..')
const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#49):
 * Document how to run the frontend against a local backend for this page
 * (README and/or `docs/docs/frontend.mdx`), including the default backend
 * base URL (`http://127.0.0.1:8000`).
 */
describe('create account local backend docs', () => {
  it('documents running the frontend against a local backend for /accounts/create', () => {
    const repoReadmePath = join(repoRoot, 'README.md')
    const frontendReadmePath = join(frontendRoot, 'README.md')
    const frontendDocsPath = join(repoRoot, 'docs/docs/frontend.mdx')

    expect(existsSync(repoReadmePath)).toBe(true)
    expect(existsSync(frontendReadmePath)).toBe(true)
    expect(existsSync(frontendDocsPath)).toBe(true)

    const frontendReadme = readFileSync(frontendReadmePath, 'utf8')
    const frontendDocs = readFileSync(frontendDocsPath, 'utf8')
    const docs = [readFileSync(repoReadmePath, 'utf8'), frontendReadme, frontendDocs].join('\n')

    // Default backend base URL required by the acceptance criterion.
    expect(docs).toMatch(/http:\/\/127\.0\.0\.1:8000/)

    // Env var used to configure that base URL for local dev.
    expect(docs).toMatch(/VITE_BACKEND_BASE_URL/)

    // Docs must cover the create-account page in the local frontend→backend setup
    // (README and/or frontend.mdx — issue allows either or both).
    const pageDocs = [frontendReadme, frontendDocs].join('\n')
    expect(pageDocs).toMatch(/\/accounts\/create/)
    expect(pageDocs).toMatch(/POST\s+\/accounts|\/accounts/)
  })
})
