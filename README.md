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

Full operator guide: [`docs/factory-howto.md`](docs/factory-howto.md). Agent contract: [`AGENTS.md`](AGENTS.md).

Product apps (`backend/` Rust, `frontend/` TanStack) come next; until then red/green uses the `work-orders/.force-red` stub described in `AGENTS.md`.

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

Starts the docs site at [http://localhost:3000](http://localhost:3000).

### Build

```bash
cd docs
npm run build
cd docs && npm run serve
```
