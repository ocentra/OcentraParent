# WP204 App/Game Linux Active-Window Tool Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP204 App/Game Linux Active-Window Tool Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Probe Linux/WSL active-window tooling without claiming foreground capture.

This closes the tool-detection part of the Linux foreground gap by checking for
`xdotool` or `xprop` and reducing `_NET_ACTIVE_WINDOW` to an opaque
observed/not-observed state. Raw window titles, process names, and foreground
app identity remain out of custody.

## Implementation

- Added `packages/parent-domain/src/app-game-linux-active-window-tool-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-linux-active-window-tool-proof.test.ts`.
- Added `scripts/test/app-game-linux-active-window-tool-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-linux-active-window-tool-proof
cmd /c node scripts/test/app-game-linux-active-window-tool-proof.mjs
```

## Proof

- `test-results/app-game-linux-active-window-tool-proof/proof.json`
- `output/app-game-plan-proof/204-app-game-linux-active-window-tool-proof/proof.json`

## Boundaries

Proved:

- WSL active-window probe tooling can be detected without installing packages.
- Active-window refs are reduced to an opaque observed/not-observed state.
- Raw window title custody, raw process-name custody, foreground capture,
  adapter dispatch, platform enforcement, and child-device delivery remain
  unclaimed.

Not proved:

- Linux foreground app/window capture.
- Raw active-window title custody.
- Linux policy enforcement, rollback, audit, adapter dispatch, provider
  delivery, or child-device delivery.
