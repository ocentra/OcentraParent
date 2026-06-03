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

Use the standard checklist in [workpacks README](README.md).
