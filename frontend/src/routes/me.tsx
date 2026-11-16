import { createFileRoute, redirect } from '@tanstack/react-router'
import { useEffect } from 'react'

export const Route = createFileRoute('/me')({
  beforeLoad: () => {
    if (typeof sessionStorage === 'undefined' || !sessionStorage.getItem('token')) {
      throw redirect({ to: '/login' })
    }
  },
  component: Me,
})

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL ?? 'http://127.0.0.1:8000'

function Me() {
  useEffect(() => {
    const token = sessionStorage.getItem('token')
    if (!token) {
      return
    }

    void fetch(`${backendBaseUrl}/me`, {
      method: 'GET',
      headers: { Authorization: `Bearer ${token}` },
    })
  }, [])

  return (
    <main>
      <h1>Profile</h1>
    </main>
  )
}
