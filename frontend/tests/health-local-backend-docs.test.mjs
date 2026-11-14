import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '../..')

/**
 * Acceptance slice (ianckc/ai-football-coach#32):
 * Document how to run the frontend against a local backend for the `/health`
 * page (README and/or `docs/docs/frontend.mdx`), including the default backend
 * base URL (`http://127.0.0.1:8000`).
 */
describe('health local backend docs', () => {
  it('documents running the frontend against a local backend for /health', () => {
    const readmePath = join(repoRoot, 'README.md')
    const frontendDocsPath = join(repoRoot, 'docs/docs/frontend.mdx')

    expect(existsSync(readmePath)).toBe(true)
    expect(existsSync(frontendDocsPath)).toBe(true)

    const docs = [readFileSync(readmePath, 'utf8'), readFileSync(frontendDocsPath, 'utf8')].join(
      '\n',
    )

    // Default backend base URL required by the acceptance criterion.
    expect(docs).toMatch(/http:\/\/127\.0\.0\.1:8000/)

    // Env var used to configure that base URL for local dev.
    expect(docs).toMatch(/VITE_BACKEND_BASE_URL/)

    // Docs must cover the /health page in the local frontend→backend setup.
    expect(docs).toMatch(/\/health/)
  })
})
