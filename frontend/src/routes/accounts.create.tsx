import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/accounts/create')({
  component: CreateAccount,
})

function CreateAccount() {
  return (
    <main>
      <h1>Create account</h1>
      <form>
        <label htmlFor="email">
          Email
          <input id="email" name="email" type="email" autoComplete="email" />
        </label>
        <label htmlFor="password">
          Password
          <input
            id="password"
            name="password"
            type="password"
            autoComplete="new-password"
          />
        </label>
        <button type="submit">Create account</button>
      </form>
    </main>
  )
}
