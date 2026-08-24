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

Keep Linux/WSL active-window tool probing unavailable without claiming
foreground capture until a real OS process-custody primitive is owned.

The tool-detection part of the Linux foreground gap remains open: xprop and
xdotool are disabled until an owner supplies an OS primitive that guarantees
bounded custody across setsid and pid-namespace escapes. `_NET_ACTIVE_WINDOW`
therefore remains an opaque observed/not-observed type with no runtime
observation. Raw window titles, process names, and foreground app identity
remain out of custody.

## Implementation

- Rust xprop/xdotool subprocess probing is removed fail-closed. No
  per-request worker or process-group path can claim custody across setsid or
  pid-namespace escapes, so fixed tool presence and display/socket readiness do
  not produce a tool result.
- `_NET_ACTIVE_WINDOW` remains typed observed/not-observed, but runtime state
  is always unavailable in this source phase; private window IDs/selectors
  never enter public metadata or proof refs.
- Service Linux detail/probe refs are empty unless a separately owned preflight
  is source-ready, and the production handler supplies unavailable. Static
  WSL/Docker presence never mints a proof ref.
- Linux xwd/convert capture is disabled fail-closed until a safe FD-backed
  handoff is established. Selected-window/title capture is unavailable because
  raw-title search violates the metadata boundary.
- No workpack tests, proof artifacts, or deployment validation were added in
  this source-only phase.

## Validation

Source-only validation is limited to focused Cargo checks, formatting, source
shape/architecture, Enforcer coordination, graph validation, and diff guards.
The adapter Linux library check passed under WSL; no tests or proof commands
were run.

## Proof

No proof artifact exists. The expected Linux active-window tool test roots are
absent and this workpack is not DONE or proof-complete.

## Boundaries (validation-open; not completion)

Source-only boundary:

- Production source keeps active-window tool probing unavailable instead of
  overclaiming custody; tests and proof are still required before any
  implementation or completion claim.
- Raw window title custody, raw process-name custody, foreground capture,
  adapter dispatch, platform enforcement, and child-device delivery remain
  unclaimed.

Not proved:

- Linux foreground app/window capture.
- A real OS process-custody primitive for xprop/xdotool descendants.
- Raw active-window title custody.
- Linux policy enforcement, rollback, audit, adapter dispatch, provider
  delivery, or child-device delivery.
