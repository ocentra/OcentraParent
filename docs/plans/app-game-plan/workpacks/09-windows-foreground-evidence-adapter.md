# 09 Windows Foreground App/Game Evidence Adapter

## Target State

Foreground-window evidence proves active focus for an app/game/launcher without
claiming content.

## Scope

- Foreground process/app identity.
- Observed timestamp.
- Optional permitted title/ref.
- Permission-limited and adapter-error states.
- Foreground transitions and gaps.

## Tests And Proof

- Foreground app updates foreground evidence.
- Background process does not gain foreground time.
- Foreground switch closes previous interval.
- Title can be omitted.
- Foreground evidence does not become content knowledge.

## Done Signal

Foreground evidence is separable from runtime and content, and sessionization can
derive active duration from stored rows.

## Completion Note

WP09 proof on branch `codex/app-game-windows-foreground-evidence` adds
contract/protocol/parser proof only. Live Windows foreground polling,
journal/SQLite ingest, service events, portal foreground rows, content
knowledge, policy execution, and broad blocking remain out of scope until later
workpacks add proof.

Use the standard checklist in [workpacks README](README.md).
