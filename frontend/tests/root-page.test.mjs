import { existsSync, readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const frontendRoot = join(dirname(fileURLToPath(import.meta.url)), '..')

/**
 * Acceptance slice (ianckc/ai-football-coach#17):
 * Root route renders a simple page that identifies the product.
 */
describe('root page', () => {
  it('root route identifies the product as AI Football Coach', () => {
    const indexPath = join(frontendRoot, 'src/routes/index.tsx')
    expect(existsSync(indexPath)).toBe(true)

    const source = readFileSync(indexPath, 'utf8')
    expect(source).toContain('AI Football Coach')
  })
})
