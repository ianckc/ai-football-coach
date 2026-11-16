import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '../..')
const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#34):
 * Document how to run the frontend login flow against a local backend (README
 * and/or `docs/docs/frontend.mdx`), including creating an account first if
 * needed and the default backend base URL (`http://127.0.0.1:8000`).
 */
describe('login local backend docs', () => {
  it('documents running the frontend login flow against a local backend', () => {
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

    // Docs must cover the login page in the local frontend→backend setup
    // (README and/or frontend.mdx — issue allows either or both).
    const pageDocs = [frontendReadme, frontendDocs].join('\n')
    expect(pageDocs).toMatch(/\/login/)
    expect(pageDocs).toMatch(/POST\s+\/login|\/login/)

    // Creating an account first if needed (no account-creation UI on this ticket).
    expect(pageDocs).toMatch(/account|\/accounts\/create|POST\s+\/accounts/i)
  })
})
