# Validation Budget Ladder

Use the smallest check that proves the assigned slice.

| Level | Default | Purpose |
| --- | --- | --- |
| V0 | yes | Read-only inspection and docs inventory. |
| V1 | yes | Exact touched-file static checks. |
| V2 | yes after source change | Targeted owner test for one package or crate. |
| V3 | dispatch note required | Full validation for the touched package or crate. |
| V4 | manager approval required | Multi-surface, UI, platform, or proof-runner validation. |
| V5 | manager approval required | Repo-wide or long-running validation. |

## Rules

- Do not escalate validation by habit.
- Do not run repo-wide checks for docs-only or inventory-only work.
- Do not run platform proof for a slice that only changes docs or ownership maps.
- Do not replace focused proof with a broad gate.
- Report the exact level, command, reason, result, and skipped higher levels.

## Default lane-manager wording

Use V0-V2 unless the dispatch packet explicitly grants V3 or higher.
