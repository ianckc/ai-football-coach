# Factory loop notes

Append a short note after each finished `run-ticket` run.

## Template

```
### YYYY-MM-DD — owner/repo#N
- Rounds: N
- Outcome:
- Machine fixes (prompts/gates/docs):
```

### 2026-11-15 — ianckc/ai-football-coach#33
- Rounds: 5
- Outcome: Added `POST /login` (email/password JSON → `200` `{id,email,token}` on success; `400` shape errors; identical `401` for unknown email / wrong password). Review: no critical/high. `.force-red` absent.
- Machine fixes (prompts/gates/docs): AGENTS.md product surface updated (login route); `backend-api.mdx` login docs still deferred from update-docs pause (medium finding noted).

### 2026-11-15 — ianckc/ai-football-coach#16
- Rounds: 4
- Outcome: Hardened `POST /accounts` password handling (min length 8 after trim; Argon2 hashing via `spawn_blocking`; existing empty/whitespace reject retained). Review: no critical/high. Docs updated for 8-char rule. `.force-red` absent.
- Machine fixes (prompts/gates/docs): none; AGENTS.md product surface unchanged (existing accounts route only).

### 2026-11-14 — ianckc/ai-football-coach#17
- Rounds: 2
- Outcome: `frontend/` TanStack Start scaffold; `npm run dev` on :3000; root `/` identifies product; README + `docs/docs/frontend.mdx`; vitest `test` script for `quality-tests`. Review: no critical/high. `.force-red` absent.
- Machine fixes (prompts/gates/docs): AGENTS.md product surface updated (frontend route + real tests); stub `.force-red` marked legacy now that both apps exist.

### 2026-11-14 — ianckc/ai-football-coach#15
- Rounds: 4
- Outcome: Hardened `POST /accounts` validation (email lowercase uniqueness, whitespace-only password reject, wrong JSON types → 400, email rules documented). Review: no critical/high. `.force-red` absent.
- Machine fixes (prompts/gates/docs): none; AGENTS.md product surface unchanged (existing accounts route only).
