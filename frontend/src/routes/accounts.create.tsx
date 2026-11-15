import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/accounts/create')({
  component: CreateAccount,
})

function CreateAccount() {
  return (
    <main>
      <h1>Create account</h1>
    </main>
  )
}
