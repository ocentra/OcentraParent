# WP02 Parent Attention Visibility Proof

Correlation: `policy-control-plane-plan / WP02 / parent-authoring-preview / attention-card`

## Implemented vertical slice

`crates/parent-runtime-core/src/parent_ui_bridge/policy_preview/cards.rs` now puts a Rust-owned `Parent attention` card ahead of ordinary preview cards when the current preview is:

- blocked by a conflict;
- manual-review required; or
- unsupported for this policy path; or
- offline or stale target state, even when the activity-store read model has no save state.

The card carries the Rust-projected attention type, source evidence/state, and save state. The portal continues to render the generated card snapshot; it does not infer policy state or create a browser-side policy model.

When multiple state fields are present, Rust projects attention in this order: explicit schedule conflict finding, unsupported target, offline/stale target, manual-required, then generic blocked state. Real schedule conflict finding kinds include `overlapping-schedule`, `timezone-boundary`, `ambiguous-local-time`, `nonexistent-local-time`, and `clock-skew`; those stay conflict attention even when manual review is also required. Offline/stale target state remains parent attention before generic manual review and does not require a save-state field from the activity-store producer.

## Focused evidence

- `cargo test -p ocentra-parent-runtime-core --test integration policy_preview_ -- --nocapture`
  - 4 passed: existing typed assistant-confirm/reload path, real schedule-conflict projection, unsupported-over-manual projection, and offline/stale target projection without save state.
- `npx vitest run tests/policy/policy-preview-route-panel.test.ts` from `apps/portal`
  - 1 file and 6 tests passed, including generated conflict, target/manual, and generic blocked `Parent attention` cards rendering before ordinary preview cards.
- `cargo fmt --check`
  - passed.
- `npm run lint:architecture -- --files apps/portal/tests/policy/policy-preview-route-panel.test.ts crates/parent-runtime-core/src/parent_ui_bridge/policy_preview/cards.rs crates/parent-runtime-core/src/parent_ui_bridge/policy_preview/helpers.rs`
  - passed.

## Failure-path coverage

The Rust integration route uses the real local agent-service response path and verifies generated card fields for blocked previews with real schedule conflict reasons plus manual review, blocked manual review plus unsupported target, and activity-store-shaped offline/stale targets with manual review but no save state. A regression that demotes a schedule conflict behind manual review, lets manual review hide unsupported/offline/stale state, hides no-save-state attention, or turns a blocked view into a ready-to-save view fails the focused integration assertions.

## Honest scope boundary

This is a partial WP02 visibility slice, not workpack closure. WP02 remains open for parent template/manual-rule authoring, preview-to-save confirmation UX, and the Rust-owned opaque confirmed-request relay required to dispatch the existing typed confirmation command safely. It also does not claim delivery, enforcement, provider execution, or child-device application.
