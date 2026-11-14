#!/usr/bin/env bash
# Inventory outdated packages and known vulns across frontend, docs, and backend.
# Report-only. Exit 0 even when outdated/vulns exist so the agent can summarize.
set -u

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
cd "$ROOT"

section() {
  printf '\n======== %s ========\n' "$1"
}

run_npm() {
  local dir="$1"
  section "$dir (npm)"
  if [[ ! -f "$dir/package.json" ]]; then
    echo "SKIP: missing $dir/package.json"
    return
  fi
  if [[ ! -d "$dir/node_modules" ]]; then
    echo "NOTE: $dir/node_modules missing — npm outdated/audit may be incomplete. Run npm ci in $dir first."
  fi
  (
    cd "$dir"
    echo "--- npm outdated ---"
    npm outdated || true
    echo "--- npm audit ---"
    npm audit --json 2>/dev/null | head -c 200000 || npm audit || true
  )
}

run_cargo() {
  local dir="$1"
  section "$dir (cargo)"
  if [[ ! -f "$dir/Cargo.toml" ]]; then
    echo "SKIP: missing $dir/Cargo.toml"
    return
  fi
  (
    cd "$dir"
    echo "--- cargo tree -d (duplicates) ---"
    cargo tree -d 2>/dev/null || echo "cargo tree unavailable"
    if command -v cargo-outdated >/dev/null 2>&1 || cargo outdated -h >/dev/null 2>&1; then
      echo "--- cargo outdated ---"
      cargo outdated || true
    else
      echo "SKIP cargo outdated: install with: cargo install cargo-outdated"
    fi
    if command -v cargo-audit >/dev/null 2>&1 || cargo audit -h >/dev/null 2>&1; then
      echo "--- cargo audit ---"
      cargo audit || true
    else
      echo "SKIP cargo audit: install with: cargo install cargo-audit"
    fi
  )
}

section "repo root: $ROOT"
echo "Dependabot config: $([[ -f .github/dependabot.yml ]] && echo present || echo MISSING)"

run_npm frontend
run_npm docs
run_cargo backend

section "done"
echo "Summarize with the check-dependencies skill report format. Do not upgrade unless asked."
