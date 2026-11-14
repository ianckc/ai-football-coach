import { createFileRoute } from '@tanstack/react-router'
import { useEffect } from 'react'

export const Route = createFileRoute('/health')({ component: Health })

const backendBaseUrl =
  import.meta.env.VITE_BACKEND_BASE_URL ?? 'http://127.0.0.1:8000'

function Health() {
  useEffect(() => {
    void fetch(`${backendBaseUrl}/health`)
  }, [])

  return (
    <main>
      <h1>Health</h1>
    </main>
  )
}
