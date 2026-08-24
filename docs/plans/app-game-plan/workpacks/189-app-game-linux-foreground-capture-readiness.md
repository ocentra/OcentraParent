# WP189 App/Game Linux Foreground Capture Readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP189 App/Game Linux Foreground Capture Readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Turn Linux display and X11/Wayland socket probing into a typed foreground
capture readiness boundary.

This source phase does not prove active foreground capture or a current host
state. It records only what an actual Linux probe can establish and keeps
WSLg/Docker presence by itself outside the evidence boundary.

## Implementation

- Rust production source now owns Linux display classification and X11/Wayland
  socket readiness in `crates/screen-capture-adapter/src/linux_display.rs`,
  `linux_display_paths.rs`, and `linux_display_readiness.rs`.
- The agent-service platform status path consumes only the typed preflight and
  adds detail refs after the live probe reports readiness; WSL/Docker presence
  alone is not evidence.
- Linux capture uses process-group-contained child tools, one aggregate
  deadline, owner-held temporary files, capped artifacts, and Rust PNG
  validation. Metadata remains identity-free.
- No workpack tests, proof artifacts, or deployment validation were added in
  this source-only phase.

## Validation

Source-only validation is limited to focused Cargo checks, formatting, source
shape/architecture, Enforcer coordination, graph validation, and diff guards.
Linux-target compilation remains dependent on an available Linux C toolchain.
No tests or proof commands were run.

## Proof

No proof artifact exists. The expected Linux capture/readiness test roots are
absent and this workpack is not DONE or proof-complete.

## Boundaries

Proved:

- The production source shape can produce typed, redacted display and socket
  readiness from an actual Linux probe.

Not proved:

- Active foreground capture or App/Game ownership.
- Raw active-window title custody.
- AppArmor, SELinux, package manager, Flatpak, Snap, rollback, audit, launch
  blocking, adapter dispatch, platform enforcement, provider delivery, or
  child-device delivery.
