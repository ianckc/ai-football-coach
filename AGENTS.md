# AI Football Coach — factory agent contract

This file is the **contract** for anyone (human or agent) changing this repo via the Swamp TDD factory. Read it before coding.

## Product

AI Football Coach — help football coaches plan training and improve teams.

- Docs site today: [`docs/`](docs/) (Docusaurus)
- Product apps (Phase 2): `backend/` (Rust) and `frontend/` (TanStack) — not scaffolded yet

## Vocabulary

- Prefer domain words from the ticket (e.g. session, drill, squad). Do not invent parallel synonyms.
- Until apps exist, still encode acceptance as tests/stubs the factory can turn red/green.

## Roles (TDD ping-pong + review)

| Role | May change | Must not | Done when |
|------|------------|----------|-----------|
| **Agent A (tests)** | tests / failing stubs only | product implementation; bookkeeping | One acceptance slice is encoded; suite is **red** |
| **Agent B (implement)** | product code only enough for current failures | tests (except clear ticket contradiction); later slices | Suite is **green** for **current** failures only |
| **Agent C (review)** | findings markdown under `work-orders/` | product code or tests | No unresolved **critical** / **high** findings |

Ownership is enforced by this contract and kickoff prompts — not by filesystem locks.

## Hard rules

1. **Work only from the GitHub Issue** named in the workflow (`issue_ref`). Do not invent scope.
2. **One slice per round.** Agent A encodes the next uncovered acceptance criterion only.
3. **Agent A** does not implement. **Agent B** does not write tests (except the documented contradiction exception). **Agent C** does not change code.
4. **Red mid-round is expected** after A. **Green before review/closeout** after B.
5. **Quality gates are mechanical:** `quality-tests` (red/green) and `quality-static` (after rounds). Do not skip them.
6. When finished: bookkeeping pause → factory comments on the issue and moves Project Status to Done when configured.

## Agent control panel

| Task | Command |
|------|---------|
| Contract | `swamp behavior run show-agents-contract execute` |
| Ready issues | `swamp behavior run list-ready-issues execute` |
| Fetch issue | `ISSUE_REF='owner/repo#N' swamp behavior run fetch-issue execute` |
| Run tests gate | `swamp behavior run quality-tests execute` |
| Run static gate | `swamp behavior run quality-static execute` |
| Start ticket | `swamp workflow run run-ticket --input issue_ref='owner/repo#N'` |
| See pauses | `swamp workflow approvals` |
| Approve / resume | `swamp workflow approve <wf> <step> --run <id>` then `swamp workflow resume <wf> --run <id>` |
| Continue rounds | `swamp workflow signal --workflow tdd-round --key tdd-more-rounds --payload '{"continue":true}'` |

Config: copy [`factory.env.example`](factory.env.example) → `factory.env` (gitignored). See [`docs/factory-howto.md`](docs/factory-howto.md).

### Agent A — paste (backup if approvals unavailable)

```text
You are Agent A (tests only) for ISSUE_REF.
Follow AGENTS.md and agent-constraints/agent-a-tests.md.
Encode ONLY the next uncovered acceptance slice. Leave the suite red.
Do not implement or do bookkeeping.
```

### Agent B — paste

```text
You are Agent B (implement only) for ISSUE_REF.
Follow AGENTS.md and agent-constraints/agent-b-implement.md.
Change code ONLY enough to make current failures pass. Do not anticipate later slices.
```

### Agent C — paste

```text
You are Agent C (review only) for ISSUE_REF.
Follow AGENTS.md and agent-constraints/agent-c-review.md.
Record findings with severities. Do not edit code or tests.
```

## Stub red/green (pre-product)

Until `backend/` / `frontend/` exist:

- Agent A creates `work-orders/.force-red` so `quality-tests` exits non-zero
- Agent B removes `work-orders/.force-red` when the slice is notionally done so green can pass
