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

- Rust production source owns Linux display classification and X11/Wayland
  socket readiness in `crates/screen-capture-adapter/src/linux_display.rs`,
  `linux_display_paths.rs`, `linux_display_readiness.rs`,
  `linux_socket_security.rs`, and `linux_socket_connect.rs`. Only fixed,
  canonical runtime roots with a trusted owner/mode chain are accepted;
  arbitrary absolute `WAYLAND_DISPLAY` values, remote/invalid `DISPLAY`,
  symlink sockets, unsafe owners/modes, and unbounded connects fail closed.
- The agent-service platform status path stays unavailable for a live Linux
  foreground probe. Its explicit read-model seam accepts a separately owned
  typed preflight, but it never mints refs from WSL/Docker presence or display
  readiness alone.
- Linux xwd/convert capture is intentionally unavailable. A compile-checked
  FD-backed handoff was not established in this source-only phase, so no
  replaceable temporary pathname is passed to an external capture tool.
  Trusted display/source observation remains separate from capture custody.
- xprop/xdotool subprocess probing is also intentionally removed. No OS
  primitive in this source phase guarantees custody across setsid or
  pid-namespace escapes, so X11 active-window results and their refs remain
  unavailable rather than being inferred from a display/socket probe.
- No workpack tests, proof artifacts, or deployment validation were added in
  this source-only phase.

## Validation

Source-only validation is limited to focused Cargo checks, formatting, source
shape/architecture, Enforcer coordination, graph validation, and diff guards.
The adapter Linux library check passed under WSL; no tests or proof commands
were run.

## Proof

No proof artifact exists. The expected Linux capture/readiness test roots are
absent and this workpack is not DONE or proof-complete.

## Boundaries (validation-open; not completion)

Source-only boundary:

- The production source retains a typed, redacted display and socket readiness
  boundary for later tests; this source-only edit does not prove the behavior.

Not proved:

- Active foreground capture or App/Game ownership.
- Linux xwd/convert capture custody; the exact missing owner is a safe
  FD-backed handoff that keeps both external tools attached to the validated
  producer-owned artifact.
- Selected-window/title capture, which remains unavailable because raw-title
  search is outside the metadata boundary.
- Live xprop/xdotool probing or active-window ownership; the missing owner is a
  real OS process-custody primitive for escaped descendants.
- Raw active-window title custody.
- AppArmor, SELinux, package manager, Flatpak, Snap, rollback, audit, launch
  blocking, adapter dispatch, platform enforcement, provider delivery, or
  child-device delivery.
