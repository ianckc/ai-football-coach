---
name: deslop
description: >-
  Remove AI-generated code slop and clean up style without changing behavior.
  Use when asked to deslop, remove slop, clean AI noise, tidy a diff, or polish
  recently generated code before commit or review.
---

# Remove AI code slop

Check the diff against the default branch (and uncommitted changes) and remove AI-generated slop introduced in the working tree. Prefer matching surrounding file style.

## This repo

- Frontend (`frontend/`): Biome + TypeScript + React / TanStack Start. No `as any` to silence types.
- Backend (`backend/`): idiomatic Axum/Rust; no narrative comments or needless clones.
- Respect [AGENTS.md](../../../AGENTS.md) role boundaries when a factory role is active (e.g. Agent B does not rewrite tests).
- Match existing patterns; do not impose a foreign style guide.

## Focus areas

- Extra comments that are unnecessary or inconsistent with local style (especially narrative “what this does” comments)
- Defensive checks or try/catch blocks that are abnormal for trusted code paths
- Casts to `any` / `as unknown as …` used only to bypass type issues
- Deeply nested code that should be simplified with early returns
- Nested ternaries that should be `if`/`else` or a small helper
- Fake completeness: hardcoded success paths, placeholders, TODO stubs shipped as done
- One-off abstractions with a single caller
- Other patterns inconsistent with the file and surrounding codebase

## Manual slop pass

After mechanical cleanup, review the diff for:

1. Incomplete wiring — exports, routes, call sites updated together
2. State handling — loading / empty / error paths coherent where UI or API surface changed
3. Dependency discipline — no new package for trivial logic
4. Test quality — if tests changed, they exercise a real edge or failure, not only the happy path

## Guardrails

- Keep behavior unchanged unless fixing a clear bug.
- Prefer minimal, focused edits over broad rewrites.
- Do not silence linters or add blanket `eslint-disable` / `#[allow(...)]` to hide slop.
- Keep the final summary concise (1–3 sentences).

## Additional resources

- Anti-pattern checklist: [reference.md](reference.md)
