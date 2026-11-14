import { createFileRoute } from '@tanstack/react-router'
import { useEffect, useState } from 'react'

export const Route = createFileRoute('/health')({ component: Health })

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL ?? 'http://127.0.0.1:8000'

function Health() {
  const [status, setStatus] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    void fetch(`${backendBaseUrl}/health`)
      .then((response) => {
        if (!response.ok) {
          throw new Error('Health request failed')
        }
        return response.json()
      })
      .then((body: { status: string }) => {
        setError(null)
        setStatus(body.status)
      })
      .catch(() => {
        setStatus(null)
        setError('Backend unavailable')
      })
  }, [])

  return (
    <main>
      <h1>Health</h1>
      {error !== null ? <p>{error}</p> : null}
      {status !== null ? <p>Backend status: {status}</p> : null}
    </main>
  )
}
