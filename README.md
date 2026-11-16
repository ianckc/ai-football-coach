# AI Football Coach

Use AI to help football coaches plan training sessions and improve their teams.

## Swamp TDD factory

This repo is a [Swamp](https://swamp-club.com) software factory. GitHub Issues on a Project board are the work queue. Swamp pauses at gates; you paste prompts into Cursor for:

1. **Agent A** — failing tests only  
2. **Agent B** — implement only enough to go green  
3. **Agent C** — code review only (after static analysis)

Quickstart:

```bash
cp factory.env.example factory.env   # set project owner/number + repo
swamp behavior run list-ready-issues execute
swamp workflow run run-ticket --input issue_ref='owner/repo#123'
swamp workflow approvals             # paste prompt into Cursor, then approve/resume
```

Full guide (docs site): [`docs/docs/software-factory.mdx`](docs/docs/software-factory.mdx) — run `cd docs && npm start` and open **Software factory**. Agent contract: [`AGENTS.md`](AGENTS.md).

Product apps: `backend/` (Rust/Axum) and `frontend/` (TanStack Start). API reference: [`docs/docs/backend-api.mdx`](docs/docs/backend-api.mdx) (also **Backend API** on the docs site). Frontend setup: [`docs/docs/frontend.mdx`](docs/docs/frontend.mdx) (also **Frontend** on the docs site).

## Backend

The Axum API lives in [`backend/`](backend/). It persists accounts in Postgres via `DATABASE_URL`.

### Requirements

- Rust **1.100** (pinned by [`rust-toolchain.toml`](rust-toolchain.toml); MSRV in [`backend/Cargo.toml`](backend/Cargo.toml))

Dependency resolution uses a 3-day crates.io cooldown via [`backend/.cargo/config.toml`](backend/.cargo/config.toml) (`registry.global-min-publish-age`). To force a newer version for an urgent update:

```bash
cd backend
CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE=allow cargo update <crate> --precise <version>
```

### Local Postgres

Compose runs two local Postgres services: `postgres` on port `5432` for manual `cargo run` (named volume), and `postgres-test` on `5433` for `cargo test` / factory gates (disposable). See [`docs/docs/backend-api.mdx`](docs/docs/backend-api.mdx).

```bash
docker compose up -d
cd backend
cp .env.example .env   # sets DATABASE_URL for manual-dev postgres :5432
cargo run
```

Migrations under `backend/migrations/` run automatically on startup. Production uses the same `DATABASE_URL` pattern with a hosted provider (e.g. Neon).

## Frontend

The TanStack Start app lives in [`frontend/`](frontend/).

### Requirements

- [Node.js](https://nodejs.org/) 26 (see [`frontend/.nvmrc`](frontend/.nvmrc))

### Installation

```bash
cd frontend
npm install
```

### Local development

```bash
cd frontend
npm run dev
```

Starts the frontend at [http://localhost:3000](http://localhost:3000).

To exercise `/health` or `/accounts/create` against a local backend, run `backend/` (default [http://127.0.0.1:8000](http://127.0.0.1:8000)) and open [http://localhost:3000/health](http://localhost:3000/health) or [http://localhost:3000/accounts/create](http://localhost:3000/accounts/create). Optional: `cp frontend/.env.example frontend/.env` to set `VITE_BACKEND_BASE_URL` (defaults to `http://127.0.0.1:8000`). See [`docs/docs/frontend.mdx`](docs/docs/frontend.mdx).
### Tests

TypeScript Vitest acceptance checks live under `frontend/tests/`.

```bash
cd frontend
npm test -- --run
```

### Quality gates

```bash
cd frontend
npm run format:check
npm run lint
npm run typecheck
```

Biome provides lint and format. GitHub Actions runs these plus tests and production build on frontend PRs. Details: [`docs/docs/frontend.mdx`](docs/docs/frontend.mdx).
## Docs site

The [Docusaurus](https://docusaurus.io/) docs site lives in [`docs/`](docs/).

### Requirements

- [Node.js](https://nodejs.org/) 26 (see [`docs/.nvmrc`](docs/.nvmrc))

### Installation

```bash
cd docs
npm install
```

### Local development

```bash
cd docs
npm start
```

Starts the docs site at [http://localhost:3001](http://localhost:3001).

### Build

```bash
cd docs
npm run build
cd docs && npm run serve
```
