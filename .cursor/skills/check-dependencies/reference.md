# Dependency check — reference

## Per-service commands

Run from each directory (or via `scripts/check.sh`).

### frontend / docs (npm)

```bash
# Use Node 26 (matches .nvmrc)
nvm use   # or fnm/asdf equivalent

npm outdated
npm audit
npm audit --omit=dev   # production-only vulns
```

Install/refresh lockfile after intentional bumps:

```bash
npm install
```

### backend (Cargo)

```bash
cargo tree -d                 # duplicate crates
cargo outdated                # needs: cargo install cargo-outdated
cargo audit                   # needs: cargo install cargo-audit
```

After intentional bumps in `Cargo.toml`:

```bash
cargo update
cargo test
```

### GitHub Actions

Dependabot covers workflow action pins. Manual check: open `.github/workflows/*.yml` and compare `uses:` tags to current major versions on the Actions marketplace.

## Dependabot.yml map

| `package-ecosystem` | `directory` | What it updates |
|---------------------|-------------|-----------------|
| `npm` | `/frontend` | `frontend/package.json` + lockfile |
| `npm` | `/docs` | `docs/package.json` + lockfile |
| `cargo` | `/backend` | `backend/Cargo.toml` + lockfile |
| `github-actions` | `/` | Actions in `.github/workflows` |

Common knobs:

- `schedule.interval`: `daily` \| `weekly` \| `monthly`
- `open-pull-requests-limit`: cap concurrent version-update PRs (default 5)
- `groups`: batch related updates into fewer PRs
- `ignore`: skip a package or version range

Security update PRs are separate from version updates; enable them in the GitHub UI. They are not fully controlled by `dependabot.yml`.

## Enabling Dependabot on GitHub

1. Push `.github/dependabot.yml` to the default branch.
2. Repo **Settings → Code security and analysis**:
   - Dependabot alerts — on
   - Dependabot security updates — on
   - Dependabot version updates — on (follows the YAML)
3. Confirm jobs under **Insights → Dependency graph → Dependabot**.

## Noise control

Prefer grouped weekly updates (already in this repo’s config) over daily ungrouped PRs. If a package churns uselessly, add an `ignore` entry rather than disabling the whole ecosystem.
