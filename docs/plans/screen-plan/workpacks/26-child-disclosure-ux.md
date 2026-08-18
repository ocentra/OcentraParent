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

Rust production source owns the disclosure projection in
`crates/agent-protocol/src/screen_child_disclosure.rs` and its calm copy table
in `screen_child_disclosure_copy.rs`. Its owner factory consumes the typed
`ScreenAnalysisParentSetting` and `ActivityCaptureCapabilityStatus` authority:
`available` maps to enabled, a disabled parent setting maps to disabled, and
all other typed capability outcomes fail closed to unavailable. It does not
derive current disclosure from historical `ScreenAnalysisResult` rows or image
deletion states. Paused, manual-required, capture-active, protected-surface,
and summary-ready remain unreachable until an authoritative current owner
supplies those lifecycle states.

The parent runtime uses an unavailable projection only for a diagnostic,
proposed, not-delivered status label; it does not consume the historical row
as current disclosure authority. No shipped child surface renders or delivers
this projection. Hidden capture, raw screenshot display, remote viewer, policy
authority, and child-agent delivery remain explicitly unclaimed. Tests, proof
artifacts, screenshots, child delivery, and checklist rows remain open.

## Checklist

- [ ] Define child-visible status.
- [ ] Define local disclosure copy.
- [ ] Define paused/disabled states.
- [ ] Define capture-active state where platform permits.
- [ ] Avoid hidden capture.
- [ ] Add screenshots/proof.

## Proof

- Expected proof root: `output/screen-plan-proof/26-child-disclosure-ux/`.
- Expected artifacts remain deferred: `proof-summary.json`, desktop screenshot,
  mobile screenshot, and validation command log.
- Child-agent runtime deployment/delivery remains open.
- Product text is source-authored; review and runtime proof remain open.
