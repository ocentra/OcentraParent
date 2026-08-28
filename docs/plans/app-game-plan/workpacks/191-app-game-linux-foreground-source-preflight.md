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

Turn Linux display/socket readiness from WP189 into a typed foreground source
preflight while keeping foreground-tool execution unavailable until a real
process-custody primitive exists.

This source phase proves only the preflight shape. It does not assert a current
Windows/WSL host state, compose App/Game ownership, or authorize capture.

## Implementation

- Rust source exposes a typed Linux foreground-source preflight from
  `crates/screen-capture-adapter/src/linux_foreground_source.rs` with truthful
  WSLg/native display and trusted X11/Wayland socket states. WSLg requires a
  WSL signal plus a complete trusted `/mnt/wslg/runtime-dir` ancestor/socket
  chain; WSL/Docker presence alone is unavailable. Remote/invalid `DISPLAY`
  and pure Wayland cannot make the foreground source ready.
- The asynchronous platform proof handler does not spawn a per-request live
  probe. It returns the typed unavailable state until an owned single-flight
  worker with a real OS process-custody primitive is available; the explicit
  read-model seam remains non-authoritative input for later validation.
- xprop/xdotool subprocess probing is removed fail-closed. The source never
  exposes an active-window result or ref from display/socket readiness alone.
- The preflight is source capability only: it does not compose App/Game
  ownership, enforcement authority, or raw window identity.
- Linux xwd/convert capture stays fail-closed because a safe FD-backed
  producer-owned handoff is not yet established. Selected-window/title capture
  is unavailable because raw-title search is outside the metadata boundary.
- Canonical `04783a5b7` adds all three expected real test modules for the
  adapter preflight, service read-model boundary, and protocol serialization.
  They are checked-in source only and have not been executed in this phase.

## Validation

The prior adapter source packet passed its recorded focused source checks. The
2026-08-28 code-and-test source packet intentionally ran no tests, builds,
validation scans, proof, pre-commit, CI, or deployment commands.

## Proof

No proof artifact exists. The expected Linux preflight test roots are present
but unexecuted, so this workpack is not DONE or proof-complete.

## Boundaries (validation-open; not completion)

Source-only boundary:

- Production source retains a typed, bounded display/socket preflight with
  fail-closed foreground-source and tool outcomes for later tests; this
  source-only edit does not prove the behavior.

Not proved:

- Active foreground capture or App/Game ownership.
- Live xprop/xdotool probing; safe process custody across escaped descendants is
  not established.
- Raw active-window title custody.
- AppArmor, SELinux, package manager, Flatpak, Snap, rollback, audit, launch
  blocking, adapter dispatch, platform enforcement, provider delivery, or
  child-device delivery.

## Graph ownership correction — 2026-08-25

WP191 is the production owner for the Linux foreground-source/preflight
integration roots: `crates/agent-service/src/activity_api.rs`,
`app_game_adapter_host_capabilities.rs`,
`app_game_adapter_host_capabilities_linux.rs`,
`app_game_platform_proof_status_payload.rs`, and
`crates/screen-capture-adapter/src/linux_foreground_source.rs`. It consumes
WP189's display/socket/X11 capture-readiness foundation through the reviewed
`WP191 -> WP189` dependency. WP204 is test/contract-only and owns no
production root. Canonical `04783a5b7` contains the three expected real test
roots; their execution, proof, and runtime validation remain open.
