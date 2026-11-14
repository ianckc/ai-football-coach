---
name: check-dependencies
description: >-
  Audits packages and dependencies across frontend (npm), docs (npm), and
  backend (Cargo), and maintains Dependabot. Use when checking dependencies,
  outdated packages, npm audit, cargo audit, security vulnerabilities, Dependabot
  config, or reviewing dependency update PRs.
---

# Check dependencies

Interactive dependency review for AI Football Coach. Pair with Dependabot for ongoing PRs.

## Services

| Path | Ecosystem | Manifest | Lockfile | Node |
|------|-----------|----------|----------|------|
| `frontend/` | npm | `package.json` | `package-lock.json` | `.nvmrc` (26) |
| `docs/` | npm | `package.json` | `package-lock.json` | `.nvmrc` (26) |
| `backend/` | Cargo | `Cargo.toml` | `Cargo.lock` | — |

Also track GitHub Actions under `.github/workflows/` via Dependabot (`github-actions`).

## Default behavior

1. **Report only.** Do not bump versions, edit lockfiles, or open PRs unless the user asks.
2. Cover **all three** app directories every run (never only the open folder).
3. Prefer the check script, then fill gaps manually if a tool is missing.

## Workflow

Copy and track:

```
Dependency check:
- [ ] Run scripts/check.sh (or per-service commands)
- [ ] Summarize outdated (major / minor-patch)
- [ ] Summarize security (npm audit / cargo audit)
- [ ] Note Dependabot status / open update PRs if relevant
- [ ] Recommend next actions (no changes unless asked)
```

### 1. Run the inventory

From repo root:

```bash
bash .cursor/skills/check-dependencies/scripts/check.sh
```

Needs network. Optional Cargo tools: `cargo install cargo-outdated cargo-audit` if those sections fail.

### 2. Classify findings

| Priority | Criteria |
|----------|----------|
| Security | Known vulns (`npm audit`, `cargo audit`) |
| Major | Breaking semver / edition bumps |
| Minor / patch | Non-breaking updates |
| Leave alone | Dev-only noise, intentional pins, or high churn with no benefit |

Respect intentional pins (exact versions in manifests). Node engines / `.nvmrc` stay on 26 unless the user changes that policy.

### 3. Report format

```markdown
# Dependency check — YYYY-MM-DD

## Summary
One or two sentences: overall risk and whether action is needed.

## frontend (npm)
| Package | Current | Latest | Kind | Notes |
|---------|---------|--------|------|-------|
| … | … | … | security / major / minor | … |

## docs (npm)
(same table)

## backend (Cargo)
(same table)

## Dependabot
- Config: `.github/dependabot.yml` present / missing
- Open update PRs: list or none
- Security updates: enabled / unknown (tell user to confirm in GitHub Settings → Code security)

## Recommended next steps
1. …
```

If a service has nothing notable, write `No actionable updates.` under that section.

## Dependabot

Config lives at [`.github/dependabot.yml`](../../../.github/dependabot.yml).

- **Version updates**: weekly PRs for `frontend` npm, `docs` npm, `backend` cargo, and root `github-actions`.
- **Security updates**: enable in GitHub → Settings → Code security and analysis → Dependabot security updates (free; not controlled only by the YAML).
- **Reviewing Dependabot PRs**: check CI (frontend/backend workflows), changelog/breaking notes for majors, then merge or request a manual bump via this skill.
- **Changing schedule/groups**: edit `dependabot.yml`; keep one entry per directory/ecosystem. Details: [reference.md](reference.md).

When the user asks to “set up” or “fix” Dependabot, verify or update `.github/dependabot.yml` and remind them to enable alerts + security updates in the GitHub UI if not already on.

## Upgrades (only when asked)

1. Change one service (or one related group) at a time.
2. Update lockfiles with the ecosystem’s normal install (`npm install` / `cargo update`).
3. Run that service’s tests / quality scripts before declaring done.
4. Do not silently widen scope across frontend + docs + backend in one pass unless requested.

## Additional resources

- Command cheat sheet and Dependabot options: [reference.md](reference.md)
