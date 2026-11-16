import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/login')({
  component: Login,
})

export function Login() {
  return (
    <main>
      <h1>Login</h1>
      <form>
        <label htmlFor="email">
          Email
          <input id="email" name="email" type="email" autoComplete="email" />
        </label>
        <label htmlFor="password">
          Password
          <input id="password" name="password" type="password" autoComplete="current-password" />
        </label>
        <button type="submit">Login</button>
      </form>
    </main>
  )
}
