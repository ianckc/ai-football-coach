import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

type RouteCandidate = {
  path: string
  fileRoute: string
}

const candidates: RouteCandidate[] = [
  {
    path: join(frontendRoot, 'src/routes/accounts.create.tsx'),
    fileRoute: '/accounts/create',
  },
  {
    path: join(frontendRoot, 'src/routes/accounts/create.tsx'),
    fileRoute: '/accounts/create',
  },
  {
    path: join(frontendRoot, 'src/routes/create-account.tsx'),
    fileRoute: '/create-account',
  },
]

/**
 * Acceptance slice (ianckc/ai-football-coach#49):
 * Add a frontend route (e.g. `/accounts/create` or `/create-account`) that loads in the app.
 */
describe('create account route', () => {
  it('registers a create-account route that loads in the app', () => {
    const match = candidates.find((candidate) => existsSync(candidate.path))
    expect(
      match,
      'expected a route file for /accounts/create or /create-account under src/routes/',
    ).toBeDefined()

    const source = readFileSync(match!.path, 'utf8')
    expect(source).toContain(`createFileRoute('${match!.fileRoute}')`)
  })
})
