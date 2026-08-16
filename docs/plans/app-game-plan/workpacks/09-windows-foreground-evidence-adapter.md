# 09 Windows Foreground App/Game Evidence Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `09 Windows Foreground App/Game Evidence Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
