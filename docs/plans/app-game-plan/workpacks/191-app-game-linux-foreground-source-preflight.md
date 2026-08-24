# WP191 App/Game Linux Foreground Source Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP191 App/Game Linux Foreground Source Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Turn Linux display/socket readiness and bounded foreground-tool probes from
WP189 into a typed foreground source preflight.

This source phase proves only the preflight shape. It does not assert a current
Windows/WSL host state, compose App/Game ownership, or authorize capture.

## Implementation

- Rust source now exposes a typed Linux foreground-source preflight from
  `crates/screen-capture-adapter/src/linux_foreground_source.rs` with truthful
  WSLg/native display, X11/Wayland socket, xprop/xdotool, and opaque active
  window states.
- The asynchronous platform proof handler runs the live probe in bounded
  `spawn_blocking` work and fails closed on timeout or join failure.
- The preflight is source capability only: it does not compose App/Game
  ownership, enforcement authority, or raw window identity.
- No workpack tests, proof artifacts, or deployment validation were added in
  this source-only phase.

## Validation

Source-only validation is limited to focused Cargo checks, formatting, source
shape/architecture, Enforcer coordination, graph validation, and diff guards.
Linux-target compilation remains dependent on an available Linux C toolchain.
No tests or proof commands were run.

## Proof

No proof artifact exists. The expected Linux preflight test roots are absent and
this workpack is not DONE or proof-complete.

## Boundaries

Proved:

- Production source defines a typed, bounded foreground-source preflight with
  fail-closed probe outcomes.

Not proved:

- Active foreground capture or App/Game ownership.
- Raw active-window title custody.
- AppArmor, SELinux, package manager, Flatpak, Snap, rollback, audit, launch
  blocking, adapter dispatch, platform enforcement, provider delivery, or
  child-device delivery.
