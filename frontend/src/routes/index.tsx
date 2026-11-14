import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({ component: Home })

function Home() {
  return (
    <main>
      <h1>AI Football Coach</h1>
      <p>Help football coaches plan training and improve teams.</p>
    </main>
  )
}
