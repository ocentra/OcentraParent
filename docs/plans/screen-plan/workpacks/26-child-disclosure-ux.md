# 26 Child Disclosure UX

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `26 Child Disclosure UX`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Child-visible/local disclosure, parent-enabled status, calm wording, and no hidden capture are implemented.

## Current State

Rust production source now owns the disclosure projection in
`crates/agent-protocol/src/screen_child_disclosure.rs` and its calm copy table
in `screen_child_disclosure_copy.rs`. It consumes the real screen read-model
capability/deletion state and derives enabled, paused, disabled, unavailable,
manual-required, capture-active, protected-surface, and summary-ready states.
The parent runtime consumes that projection in
`crates/parent-runtime-core/src/parent_ui_bridge/screen_summary.rs` only as a
parent diagnostic/status projection. No shipped child surface currently
renders or delivers this projection, so child-visible runtime delivery and
rendering remain open. Hidden capture, raw screenshot display, remote viewer,
policy authority, and child-agent delivery remain explicitly unclaimed. Tests,
proof artifacts, and desktop/mobile screenshots are deferred to the
test/proof phase.

## Checklist

- [ ] Define child-visible status.
- [ ] Define local disclosure copy.
- [ ] Define paused/disabled states.
- [ ] Define capture-active state where platform permits.
- [ ] Avoid hidden capture.
- [ ] Add screenshots/proof.

## Proof

- Expected proof root: `output/screen-plan-proof/screen-child-disclosure/`.
- Expected artifacts remain deferred: `proof-summary.json`, desktop screenshot,
  mobile screenshot, and validation command log.
- Child-agent runtime deployment/delivery remains open.
- Product text is source-authored; review and runtime proof remain open.
