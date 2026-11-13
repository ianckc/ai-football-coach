# AI Football Coach — factory helpers
# Requires: gh, jq, swamp (for run-ticket / quality recipes)
# Config: factory.env (see factory.env.example)

set dotenv-filename := "factory.env"
set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# List project tickets filtered by Status (default: Ready).
# Examples:
#   just tickets
#   just tickets "In Progress"
# STATUS=Done just tickets
[group('tickets')]
tickets status=env_var_or_default("STATUS", "Ready"):
    #!/usr/bin/env bash
    set -euo pipefail
    status={{ quote(status) }}
    OWNER="${FACTORY_PROJECT_OWNER:-}"
    NUMBER="${FACTORY_PROJECT_NUMBER:-}"
    if [ -z "$OWNER" ] || [ -z "$NUMBER" ]; then
      echo "Set FACTORY_PROJECT_OWNER and FACTORY_PROJECT_NUMBER (see factory.env.example)" >&2
      exit 1
    fi
    command -v gh >/dev/null || { echo "gh CLI is required" >&2; exit 1; }
    command -v jq >/dev/null || { echo "jq is required" >&2; exit 1; }
    echo "Tickets on project $OWNER/$NUMBER (Status=$status):"
    items="$(gh project item-list "$NUMBER" --owner "$OWNER" --format json --limit 100)"
    count="$(printf '%s' "$items" | jq --arg status "$status" '
      [.items[]
        | select(.content != null)
        | select(.content.type == "Issue" or (.content.url // "" | test("/issues/")))
        | select((.status // .["Status"] // "") == $status)
      ] | length')"
    if [ "$count" = "0" ]; then
      echo "  (none)"
      exit 0
    fi
    printf '%s' "$items" | jq -r --arg status "$status" '
      .items[]
      | select(.content != null)
      | select(.content.type == "Issue" or (.content.url // "" | test("/issues/")))
      | select((.status // .["Status"] // "") == $status)
      | "  - #\(.content.number // "?") \(.content.title // "(no title)")\n    \(.content.url // "")\n    itemId=\(.id)"'

alias list := tickets

# Create a GitHub Issue, add it to the Project, set Status (default: Ready).
# Examples:
#   just ticket-create "Add session planner"
#   just ticket-create "Add drills" "$(printf '%s\n' '## Acceptance criteria' '- [ ] Coach can list drills')"
# STATUS="In Progress" just ticket-create "Spike"
[group('tickets')]
ticket-create title body="" status=env_var_or_default("STATUS", "Ready"):
    #!/usr/bin/env bash
    set -euo pipefail
    title={{ quote(title) }}
    issue_body={{ quote(body) }}
    status={{ quote(status) }}
    OWNER="${FACTORY_PROJECT_OWNER:-}"
    NUMBER="${FACTORY_PROJECT_NUMBER:-}"
    REPO="${FACTORY_REPO:-}"
    if [ -z "$OWNER" ] || [ -z "$NUMBER" ]; then
      echo "Set FACTORY_PROJECT_OWNER and FACTORY_PROJECT_NUMBER (see factory.env.example)" >&2
      exit 1
    fi
    if [ -z "$REPO" ] || [ "$REPO" = "your-org/ai-football-coach" ]; then
      REPO="$(gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null || true)"
    fi
    if [ -z "$REPO" ]; then
      echo "Set FACTORY_REPO=owner/repo in factory.env" >&2
      exit 1
    fi
    command -v gh >/dev/null || { echo "gh CLI is required" >&2; exit 1; }
    command -v jq >/dev/null || { echo "jq is required" >&2; exit 1; }

    if [ -z "$issue_body" ]; then
      issue_body="$(printf '%s\n' \
        '## Acceptance criteria' \
        '' \
        '- [ ] …')"
    fi

    echo "Creating issue in $REPO…"
    url="$(gh issue create --repo "$REPO" --title "$title" --body "$issue_body")"
    echo "  $url"

    echo "Adding to project $OWNER/$NUMBER…"
    item_json="$(gh project item-add "$NUMBER" --owner "$OWNER" --url "$url" --format json)"
    item_id="$(printf '%s' "$item_json" | jq -r '.id')"
    echo "  itemId=$item_id"

    project_id="${FACTORY_PROJECT_ID:-}"
    field_id="${FACTORY_STATUS_FIELD_ID:-}"
    option_id=""
    case "$status" in
      Ready|"${FACTORY_STATUS_READY:-Ready}")
        option_id="${FACTORY_STATUS_READY_OPTION_ID:-}"
        ;;
      "In Progress"|"${FACTORY_STATUS_IN_PROGRESS:-In Progress}")
        option_id="${FACTORY_STATUS_IN_PROGRESS_OPTION_ID:-}"
        ;;
      Done|"${FACTORY_STATUS_DONE:-Done}")
        option_id="${FACTORY_STATUS_DONE_OPTION_ID:-}"
        ;;
    esac

    if [ -z "$project_id" ] || [ -z "$field_id" ] || [ -z "$option_id" ]; then
      echo "Resolving Status field/option for '$status'…"
      fields="$(gh project field-list "$NUMBER" --owner "$OWNER" --format json --limit 50)"
      project_id="$(gh project view "$NUMBER" --owner "$OWNER" --format json -q .id)"
      field_id="$(printf '%s' "$fields" | jq -r '
        .fields[]
        | select(.name == "Status")
        | .id' | head -n1)"
      option_id="$(printf '%s' "$fields" | jq -r --arg status "$status" '
        .fields[]
        | select(.name == "Status")
        | .options[]?
        | select(.name == $status)
        | .id' | head -n1)"
    fi

    if [ -n "$project_id" ] && [ -n "$field_id" ] && [ -n "$option_id" ]; then
      gh project item-edit \
        --id "$item_id" \
        --project-id "$project_id" \
        --field-id "$field_id" \
        --single-select-option-id "$option_id"
      echo "Status → $status"
    else
      echo "Could not set Status=$status — set it on the board, or configure FACTORY_PROJECT_ID / FACTORY_STATUS_*_OPTION_ID" >&2
    fi

    number="$(printf '%s' "$url" | grep -oE '[0-9]+$')"
    echo
    echo "Issue: $url"
    echo "Ref:   $REPO#$number"
    echo "Start: just ticket-start $number"

# Start work on a ticket: require a clean tree, sync main, create NNNN/slug branch,
# then set Project Status to In Progress.
# Examples:
#   just ticket-start 10
# just ticket-start 10   # → 0010/create-a-justfile-with-some-useful-commands
[group('tickets')]
ticket-start number:
    #!/usr/bin/env bash
    set -euo pipefail
    number={{ quote(number) }}
    if ! [[ "$number" =~ ^[0-9]+$ ]]; then
      echo "ticket-start expects a numeric issue number, got: $number" >&2
      exit 1
    fi

    if [ -n "$(git status --porcelain)" ]; then
      echo "Working tree is not clean — commit or stash changes before starting a ticket:" >&2
      git status --short >&2
      exit 1
    fi

    OWNER="${FACTORY_PROJECT_OWNER:-}"
    PROJECT_NUMBER="${FACTORY_PROJECT_NUMBER:-}"
    REPO="${FACTORY_REPO:-}"
    IN_PROGRESS="${FACTORY_STATUS_IN_PROGRESS:-In Progress}"

    if [ -z "$OWNER" ] || [ -z "$PROJECT_NUMBER" ]; then
      echo "Set FACTORY_PROJECT_OWNER and FACTORY_PROJECT_NUMBER (see factory.env.example)" >&2
      exit 1
    fi
    if [ -z "$REPO" ] || [ "$REPO" = "your-org/ai-football-coach" ]; then
      REPO="$(gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null || true)"
    fi
    if [ -z "$REPO" ]; then
      echo "Set FACTORY_REPO=owner/repo in factory.env" >&2
      exit 1
    fi
    command -v gh >/dev/null || { echo "gh CLI is required" >&2; exit 1; }
    command -v jq >/dev/null || { echo "jq is required" >&2; exit 1; }

    echo "Fetching issue #$number from $REPO…"
    issue_json="$(gh issue view "$number" --repo "$REPO" --json title,url,state)"
    title="$(printf '%s' "$issue_json" | jq -r .title)"
    url="$(printf '%s' "$issue_json" | jq -r .url)"
    state="$(printf '%s' "$issue_json" | jq -r .state)"
    if [ "$state" != "OPEN" ]; then
      echo "Issue #$number is $state — only open issues can be started" >&2
      exit 1
    fi

    padded="$(printf '%04d' "$number")"
    slug="$(
      printf '%s' "$title" \
        | tr '[:upper:]' '[:lower:]' \
        | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//; s/-+/-/g'
    )"
    if [ -z "$slug" ]; then
      echo "Could not slugify issue title: $title" >&2
      exit 1
    fi
    branch="${padded}/${slug}"

    if git show-ref --verify --quiet "refs/heads/$branch" \
      || git show-ref --verify --quiet "refs/remotes/origin/$branch"; then
      echo "Branch already exists: $branch" >&2
      exit 1
    fi

    echo "Checking out main and pulling origin/main…"
    git checkout main
    git pull origin main

    echo "Creating branch $branch…"
    git checkout -b "$branch"

    echo "Updating project Status → $IN_PROGRESS…"
    items="$(gh project item-list "$PROJECT_NUMBER" --owner "$OWNER" --format json --limit 100)"
    item_id="$(printf '%s' "$items" | jq -r --argjson num "$number" '
      .items[]
      | select(.content != null)
      | select((.content.number // 0) == $num)
      | .id' | head -n1)"
    if [ -z "$item_id" ] || [ "$item_id" = "null" ]; then
      echo "Issue #$number is not on project $OWNER/$PROJECT_NUMBER — add it to the board first" >&2
      exit 1
    fi

    project_id="${FACTORY_PROJECT_ID:-}"
    field_id="${FACTORY_STATUS_FIELD_ID:-}"
    option_id="${FACTORY_STATUS_IN_PROGRESS_OPTION_ID:-}"
    if [ -z "$project_id" ] || [ -z "$field_id" ] || [ -z "$option_id" ]; then
      fields="$(gh project field-list "$PROJECT_NUMBER" --owner "$OWNER" --format json --limit 50)"
      project_id="$(gh project view "$PROJECT_NUMBER" --owner "$OWNER" --format json -q .id)"
      field_id="$(printf '%s' "$fields" | jq -r '
        .fields[]
        | select(.name == "Status")
        | .id' | head -n1)"
      option_id="$(printf '%s' "$fields" | jq -r --arg status "$IN_PROGRESS" '
        .fields[]
        | select(.name == "Status")
        | .options[]?
        | select(.name == $status)
        | .id' | head -n1)"
    fi

    if [ -z "$project_id" ] || [ -z "$field_id" ] || [ -z "$option_id" ]; then
      echo "Could not resolve Status='$IN_PROGRESS' field/option IDs" >&2
      exit 1
    fi

    gh project item-edit \
      --id "$item_id" \
      --project-id "$project_id" \
      --field-id "$field_id" \
      --single-select-option-id "$option_id"

    echo
    echo "Issue:  $url"
    echo "Branch: $branch"
    echo "Status: $IN_PROGRESS"
    echo "itemId: $item_id"
    echo "Next:   just run-ticket '$REPO#$number'"

# Start the TDD factory for an issue (owner/repo#N, URL, or number with FACTORY_REPO).
[group('factory')]
run-ticket issue_ref:
    swamp workflow run run-ticket --input issue_ref={{ quote(issue_ref) }}

# Show workflow pauses waiting for approval.
[group('factory')]
approvals:
    swamp workflow approvals

# Print the agent contract (AGENTS.md via swamp).
[group('factory')]
contract:
    swamp behavior run show-agents-contract execute

# Run the tests quality gate.
[group('quality')]
tests:
    swamp behavior run quality-tests execute

# Run the static analysis quality gate.
[group('quality')]
static:
    swamp behavior run quality-static execute
