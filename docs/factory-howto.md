# How to run the Swamp TDD factory

Swamp runs the line (gates + pauses). Cursor does the coding when a gate is waiting.

## Prerequisites

1. `swamp auth login` (Swamp Club account)
2. `gh auth login` (or `gh auth refresh`) with access to the repo and Project
3. Copy `factory.env.example` → `factory.env` and set `FACTORY_PROJECT_OWNER`, `FACTORY_PROJECT_NUMBER`, `FACTORY_REPO`
4. Project board Status options: **Ready**, **In Progress**, **Done** (labels configurable in `factory.env`)

## Mental model

```
run-ticket (parent)
  ├─ check ready → fetch work order → mark In Progress
  ├─ tdd-round × up to 5 (child)
  │    Agent A → red gate → Agent B → green gate → continue?
  ├─ quality-static
  ├─ Agent C code review
  └─ bookkeeping → mark Done
```

Coding pauses and the continue signal use the **child** `tdd-round` run id.  
Review / bookkeeping use the **parent** `run-ticket` run id.

## Operator steps

### 0. List ready work

```bash
swamp behavior run list-ready-issues execute
```

### 1. Start a ticket

```bash
swamp workflow run run-ticket --input issue_ref='owner/repo#123'
# optional: --input factory_item_id='PVTI_…'
```

Save the **parent** run id.

### 2. See what is waiting

```bash
swamp workflow approvals
```

Save the **child** run id and copy the approval **prompt** into Cursor.

### 3. Agent A (failing tests)

1. Paste the Agent A prompt into Cursor; leave the suite red.
2. Unlock:

```bash
swamp workflow approve tdd-round write-failing-tests --run <child-run-id>
swamp workflow resume tdd-round --run <child-run-id>
```

### 4. Agent B (implement)

1. Paste the Agent B prompt; make the suite green.
2. Unlock:

```bash
swamp workflow approve tdd-round implement-to-green --run <child-run-id>
swamp workflow resume tdd-round --run <child-run-id>
```

### 5. Continue or stop?

```bash
swamp workflow signal --workflow tdd-round --key tdd-more-rounds --payload '{"continue":true}'
# or: '{"continue":false}'
swamp workflow resume tdd-round --run <child-run-id>
swamp workflow resume run-ticket --run <parent-run-id>
```

Use `continue:true` only if the next unchecked criterion would fail tests today.

### 6. Static analysis + Agent C

After rounds stop, static analysis runs automatically. Then approvals show `code-review` on the parent.

Paste the Agent C prompt; write findings under `work-orders/`. Approve only when critical/high are clear.

```bash
swamp workflow approve run-ticket code-review --run <parent-run-id>
swamp workflow resume run-ticket --run <parent-run-id>
```

### 7. Bookkeeping → Done

```bash
swamp workflow approve run-ticket bookkeeping-reminder --run <parent-run-id>
swamp workflow resume run-ticket --run <parent-run-id>
```

## Pre-product stub

Until `backend/` and `frontend/` exist, Agent A creates `work-orders/.force-red` and Agent B removes it so red/green gates still exercise the line.

## Control panel

See `AGENTS.md` or run `swamp behavior run show-agents-contract execute`.
