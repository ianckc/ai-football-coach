import { cleanup, render, screen } from '@testing-library/react'
import { afterEach, describe, expect, it } from 'vitest'
import { Route } from '../src/routes/accounts.create'

const CreateAccount = Route.options.component

/**
 * Acceptance slice (ianckc/ai-football-coach#49):
 * On that page, show a form with `email` and `password` fields and a submit control.
 */
describe('create account form', () => {
  afterEach(() => {
    cleanup()
  })

  it('shows a form with email and password fields and a submit control', () => {
    expect(CreateAccount, 'create-account route must define a component').toBeTypeOf('function')

    render(<CreateAccount />)

    expect(document.querySelector('form'), 'page must include a form').not.toBeNull()

    const email = screen.getByLabelText(/email/i)
    expect(email).toBeInTheDocument()
    expect(email.tagName).toMatch(/INPUT|TEXTAREA/i)

    const password = screen.getByLabelText(/password/i)
    expect(password).toBeInTheDocument()
    expect(password).toHaveAttribute('type', 'password')

    const form = document.querySelector('form')!
    const submit =
      form.querySelector('button[type="submit"], input[type="submit"]') ??
      screen.queryByRole('button', { name: /create account|submit|create/i })
    expect(submit, 'form must include a submit control').toBeTruthy()
  })
})
