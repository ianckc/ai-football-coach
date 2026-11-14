import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/health')({ component: Health })

function Health() {
  return (
    <main>
      <h1>Health</h1>
    </main>
  )
}
