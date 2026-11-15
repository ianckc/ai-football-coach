import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '../..')
const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance (ianckc/ai-football-coach#32 + #39):
 * Document how to run the frontend against a local backend for the `/health`
 * page, including the default backend base URL (`http://127.0.0.1:8000`).
 * `frontend/README.md` must be discoverable for local `/health` setup.
 */
describe('health local backend docs', () => {
  it('documents running the frontend against a local backend for /health', () => {
    const repoReadmePath = join(repoRoot, 'README.md')
    const frontendReadmePath = join(frontendRoot, 'README.md')
    const frontendDocsPath = join(repoRoot, 'docs/docs/frontend.mdx')

    expect(existsSync(repoReadmePath)).toBe(true)
    expect(existsSync(frontendReadmePath)).toBe(true)
    expect(existsSync(frontendDocsPath)).toBe(true)

    const frontendReadme = readFileSync(frontendReadmePath, 'utf8')
    const docs = [
      readFileSync(repoReadmePath, 'utf8'),
      frontendReadme,
      readFileSync(frontendDocsPath, 'utf8'),
    ].join('\n')

    // Default backend base URL required by the acceptance criterion.
    expect(docs).toMatch(/http:\/\/127\.0\.0\.1:8000/)
    expect(frontendReadme).toMatch(/http:\/\/127\.0\.0\.1:8000/)

    // Env var used to configure that base URL for local dev.
    expect(docs).toMatch(/VITE_BACKEND_BASE_URL/)
    expect(frontendReadme).toMatch(/VITE_BACKEND_BASE_URL/)

    // Docs must cover the /health page in the local frontend→backend setup.
    expect(docs).toMatch(/\/health/)
    expect(frontendReadme).toMatch(/\/health/)

    // Point to product docs and configurable local CORS.
    expect(frontendReadme).toMatch(/docs\/docs\/frontend\.mdx/)
    expect(frontendReadme).toMatch(/CORS_ALLOWED_ORIGINS/)
  })
})
