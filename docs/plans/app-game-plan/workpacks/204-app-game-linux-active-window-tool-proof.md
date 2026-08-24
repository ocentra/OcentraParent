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

Define bounded Linux/WSL active-window tool probes without claiming foreground
capture.

This closes the tool-detection part of the Linux foreground gap by checking for
`xdotool` or `xprop` and reducing `_NET_ACTIVE_WINDOW` to an opaque
observed/not-observed state. Raw window titles, process names, and foreground
app identity remain out of custody.

## Implementation

- Rust tool probing now runs bounded xprop and xdotool argv calls under
  process-group containment in
  `crates/screen-capture-adapter/src/linux_tools.rs` and
  `crates/screen-capture-adapter/src/linux_process.rs`.
- `_NET_ACTIVE_WINDOW` is reduced to typed observed/not-observed; private
  selectors never enter public metadata or proof refs.
- Service adds xprop/xdotool refs only when the corresponding live probe
  succeeds. Static WSL/Docker presence does not mint a proof ref.
- No workpack tests, proof artifacts, or deployment validation were added in
  this source-only phase.

## Validation

Source-only validation is limited to focused Cargo checks, formatting, source
shape/architecture, Enforcer coordination, graph validation, and diff guards.
Linux-target compilation remains dependent on an available Linux C toolchain.
No tests or proof commands were run.

## Proof

No proof artifact exists. The expected Linux active-window tool test roots are
absent and this workpack is not DONE or proof-complete.

## Boundaries

Proved:

- Production source bounds active-window tool probing and reduces the result to
  an opaque observed/not-observed state.
- Raw window title custody, raw process-name custody, foreground capture,
  adapter dispatch, platform enforcement, and child-device delivery remain
  unclaimed.

Not proved:

- Linux foreground app/window capture.
- Raw active-window title custody.
- Linux policy enforcement, rollback, audit, adapter dispatch, provider
  delivery, or child-device delivery.
