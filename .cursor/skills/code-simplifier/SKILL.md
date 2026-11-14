---
name: code-simplifier
description: >-
  Simplifies and refines code for clarity while preserving behavior. Use when
  asked to simplify code, clean up code, refactor for clarity, improve
  readability, or polish recently modified code for elegance.
metadata:
  sourced-from: https://github.com/stolinski/s-stack/tree/main/skills/code-simplifier
---

<!--
Based on Anthropic's code-simplifier agent and stolinski/s-stack packaging:
https://github.com/anthropics/claude-plugins-official/blob/main/plugins/code-simplifier/agents/code-simplifier.md
https://github.com/stolinski/s-stack
-->

# Code Simplifier

Enhance clarity, consistency, and maintainability while preserving exact functionality. Prefer readable, explicit code over overly compact solutions. Match this repository’s existing style.

## This repo

- Frontend (`frontend/`): TypeScript, React, TanStack Start, Biome. Run `npm run lint` / `npm run typecheck` after substantive edits.
- Backend (`backend/`): Rust / Axum. Prefer rustfmt + Clippy-clean code.
- Follow surrounding files and [AGENTS.md](../../../AGENTS.md). Do not impose foreign style rules (e.g. “always `function` over arrow”) unless the local file already does.
- When a factory role is active, stay in role (Agent B does not rewrite tests for style alone).

## Refinement principles

### 1. Preserve functionality

Never change what the code does — only how it does it. All original features, outputs, and behaviors must remain intact.

### 2. Apply project standards

- Match import style, naming, and patterns in neighboring files
- Frontend: Biome formatting/lint; avoid `any` casts used only to silence types
- Backend: idiomatic ownership and `Result` handling; no needless clones
- Prefer error handling consistent with the module (propagate with `?` / project error types rather than blank try/catch)
- Keep domain vocabulary from the ticket (session, drill, squad, etc.)

### 3. Enhance clarity

- Reduce unnecessary complexity and nesting (early returns)
- Eliminate redundant code and single-use abstractions
- Improve names; consolidate related logic
- Remove comments that only restate obvious code
- Avoid nested ternaries — prefer `if`/`else` or a small helper
- Choose clarity over brevity

### 4. Maintain balance

Avoid over-simplification that:

- Reduces clarity or maintainability
- Creates clever one-liners that are hard to debug
- Combines too many concerns into one function
- Removes helpful abstractions that organize real boundaries
- Prioritizes “fewer lines” over readability

### 5. Focus scope

Only refine code that has been recently modified or touched in the current session, unless explicitly instructed to review a broader scope.

## Refinement process

1. Identify the recently modified sections
2. Analyze for elegance and consistency opportunities
3. Apply local project standards
4. Ensure behavior is unchanged
5. Verify the result is simpler and easier to maintain
6. Summarize only significant structural changes

## Examples

### Before: Nested ternaries

```typescript
const status = isLoading ? 'loading' : hasError ? 'error' : isComplete ? 'complete' : 'idle';
```

### After: Clear guards

```typescript
function getStatus(isLoading: boolean, hasError: boolean, isComplete: boolean): string {
  if (isLoading) return 'loading';
  if (hasError) return 'error';
  if (isComplete) return 'complete';
  return 'idle';
}
```

### Before: Overly compact

```typescript
const result = arr.filter(x => x > 0).map(x => x * 2).reduce((a, b) => a + b, 0);
```

### After: Clear steps

```typescript
const positiveNumbers = arr.filter(x => x > 0);
const doubled = positiveNumbers.map(x => x * 2);
const sum = doubled.reduce((a, b) => a + b, 0);
```

### Before: Redundant abstraction

```typescript
function isNotEmpty(arr: unknown[]): boolean {
  return arr.length > 0;
}

if (isNotEmpty(items)) {
  // ...
}
```

### After: Direct check

```typescript
if (items.length > 0) {
  // ...
}
```
