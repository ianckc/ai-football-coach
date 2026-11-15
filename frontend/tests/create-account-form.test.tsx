import { cleanup, render, screen } from '@testing-library/react'
import { afterEach, describe, expect, it } from 'vitest'
import { CreateAccount } from '../src/routes/accounts.create'

/**
 * Acceptance slice (ianckc/ai-football-coach#49):
 * On that page, show a form with `email` and `password` fields and a submit control.
 */
describe('create account form', () => {
  afterEach(() => {
    cleanup()
  })

  it('shows a form with email and password fields and a submit control', () => {
    render(<CreateAccount />)

    const form = document.querySelector('form')
    expect(form, 'page must include a form').not.toBeNull()
    if (!form) {
      throw new Error('page must include a form')
    }

    const email = screen.getByLabelText(/email/i)
    expect(email).toBeInTheDocument()
    expect(email.tagName).toMatch(/INPUT|TEXTAREA/i)

    const password = screen.getByLabelText(/password/i)
    expect(password).toBeInTheDocument()
    expect(password).toHaveAttribute('type', 'password')

    const submit =
      form.querySelector('button[type="submit"], input[type="submit"]') ??
      screen.queryByRole('button', { name: /create account|submit|create/i })
    expect(submit, 'form must include a submit control').toBeTruthy()
  })
})
