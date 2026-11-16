import { createFileRoute, redirect } from '@tanstack/react-router'

export const Route = createFileRoute('/me')({
  beforeLoad: () => {
    if (typeof sessionStorage === 'undefined' || !sessionStorage.getItem('token')) {
      throw redirect({ to: '/login' })
    }
  },
  component: Me,
})

function Me() {
  return (
    <main>
      <h1>Profile</h1>
    </main>
  )
}
