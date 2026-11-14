---
name: frontend-best-practices
description: >-
  TypeScript, React 19, Biome, Vitest, and TanStack Start/Router best practices
  for frontend/. Use when writing or reviewing frontend code, routes, loaders,
  server functions, components, or running lint/typecheck/tests. Load versioned
  TanStack skills from node_modules — never invent Next.js or Remix APIs.
---

# Frontend best practices

Apply when working under `frontend/`. Match existing files; prefer domain words from the ticket (session, drill, squad).

## Non-negotiables

- **Not Next.js / Remix.** This app uses TanStack Start + TanStack Router. Do not invent `getServerSideProps`, App Router layouts, or Remix loaders.
- **Types are inferred.** Prefer full inference; avoid `any` and unnecessary casts.
- **Isomorphic by default.** Route loaders run on server and client. Use `createServerFn` (and related Start APIs) for server-only work — after reading the TanStack skills below.
- **Style from neighbors.** Follow patterns in `frontend/src/`; do not introduce a parallel folder layout.

## Tooling (from `frontend/`)

```bash
npm run lint          # biome lint
npm run typecheck     # tsc --noEmit
npm test -- --run     # vitest
npm run format:check  # biome format check
```

Run lint + typecheck after substantive edits. Use Vitest patterns already in the repo for new tests.

## TypeScript / React

- Strict typing at API and form boundaries; no `as any` to silence errors
- Prefer small components and route modules; avoid one-off abstraction layers
- Keep client-only APIs (`window`, `localStorage`) out of loader top-level — gate with Start SSR options or `useEffect` as documented in TanStack skills
- Prefer Biome-clean code; do not fight the formatter

## TanStack skills (version-synced)

**Always read the installed skill files under `frontend/node_modules/`** so guidance matches `package-lock.json`. Load only the skills for the boundary you are changing — do not dump every manual into context.

Paths are relative to `frontend/node_modules/`.

| Task | Load first |
|------|------------|
| React Start setup, `createStart`, `useServerFn` | `@tanstack/react-start/skills/react-start/SKILL.md` |
| Server functions, middleware, execution model, server routes | `@tanstack/start-client-core/skills/start-core/SKILL.md` then the matching sub-skill (`server-functions`, `middleware`, `execution-model`, `server-routes`, …) |
| Router core concepts | `@tanstack/router-core/skills/router-core/SKILL.md` |
| Data loading / loaders | `@tanstack/router-core/skills/router-core/data-loading/SKILL.md` |
| Search params | `@tanstack/router-core/skills/router-core/search-params/SKILL.md` |
| Path params | `@tanstack/router-core/skills/router-core/path-params/SKILL.md` |
| Navigation | `@tanstack/router-core/skills/router-core/navigation/SKILL.md` |
| Auth / guards | `@tanstack/router-core/skills/router-core/auth-and-guards/SKILL.md` |
| SSR | `@tanstack/router-core/skills/router-core/ssr/SKILL.md` |
| Type safety | `@tanstack/router-core/skills/router-core/type-safety/SKILL.md` |
| Not-found / errors | `@tanstack/router-core/skills/router-core/not-found-and-errors/SKILL.md` |
| Code splitting | `@tanstack/router-core/skills/router-core/code-splitting/SKILL.md` |
| React Router bindings | `@tanstack/react-router/skills/react-router/SKILL.md` |
| Router + Query composition | `@tanstack/react-router/skills/compositions/router-query/SKILL.md` (when Query is in use) |
| Bundler plugin | `@tanstack/router-plugin/skills/router-plugin/SKILL.md` |
| Virtual file routes | `@tanstack/virtual-file-routes/skills/virtual-file-routes/SKILL.md` |
| Start server runtime | `@tanstack/start-server-core/skills/start-server-core/SKILL.md` |
| Migrating from Next.js | `@tanstack/react-start/skills/lifecycle/migrate-from-nextjs/SKILL.md` |

If `node_modules` is missing, run `npm install` in `frontend/` before coding against TanStack APIs.

## Related skills

- Slop / AI noise cleanup: `deslop`
- Behavior-preserving clarity pass: `code-simplifier`
- Dependency / audit CVEs: `check-dependencies`
- Exploit-focused security review: `security-review`
