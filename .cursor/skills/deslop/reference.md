# Deslop anti-pattern checklist

Use as a scan list; fix only what appears in the current diff unless asked for a wider pass.

## Comments

| Avoid | Prefer |
|-------|--------|
| Step-by-step narration of obvious code | No comment, or a one-line *why* |
| Restating the function name in a block comment | Delete |
| `#region` / decorative banners | Delete |

## Types (TypeScript)

| Avoid | Prefer |
|-------|--------|
| `as any`, `as unknown as T` to silence errors | Fix the type or narrow properly |
| Optional chaining + default chains that hide bugs | Explicit null checks at boundaries |
| Over-wide `Record<string, any>` | Concrete types or generics |

## Errors

| Avoid | Prefer |
|-------|--------|
| Empty `catch` / swallowed errors | Log or propagate in project style |
| try/catch around infallible local logic | Remove |
| `unwrap()` / `expect()` in Rust production paths | `?` + proper error types |

## Structure

| Avoid | Prefer |
|-------|--------|
| Deep nesting | Early return / guard clauses |
| Nested ternaries | `if` / `else` or small named helper |
| Generic wrapper used once | Inline |
| Duplicate near-identical helpers | One shared helper matching local style |

## Completeness

| Avoid | Prefer |
|-------|--------|
| Placeholder returns (`return []`, fake IDs) marked done | Real implementation or leave failing/unwired intentionally |
| Canned success without calling the real path | Wire through |
| New dep for a 5-line helper | Local function |
