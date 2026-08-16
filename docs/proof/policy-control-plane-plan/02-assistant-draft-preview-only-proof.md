# WP02 assistant-draft preview-only proof

## Scope

- Plan: `policy-control-plane-plan`
- Workpack: `WP02 parent authoring/preview`
- Slice: `policy-authoring.assistant-draft-preview-only`
- Evidence date: `2026-08-06`

This proof records the live Rust contract and parent-surface projection for an
assistant-drafted policy request. It is a bounded preview/authority proof; it
does not close WP02 or claim that parent template/manual-rule authoring,
preview-to-save UX, mobile/accessibility, or child delivery is complete.

## Contract evidence

`crates/policy-control-core/tests/unit/policy_request.rs`
(`assistant_draft_stays_preview_only_until_parent_confirms`) proves that:

1. an `assistant-draft` request registers as `PreviewOnly` and
   `ParentConfirmationRequired`;
2. direct parent approval before confirmation fails with
   `assistant-preview-only`;
3. an active parent confirmation records `ParentConfirmed` and its audit
   reference; and
4. only after that confirmation can the approval resolve to `Approved`.

The adjacent role tests prove child and observer actors cannot perform that
confirmation or self-approval.

## Parent-surface evidence

The Rust-owned policy preview bridge keeps the same boundary visible to the
portal:

- `crates/parent-runtime-core/src/parent_ui_bridge/policy_preview/summary.rs`
  reports that an assistant draft remains preview-only until parent
  confirmation;
- `crates/parent-runtime-core/src/parent_ui_bridge/policy_preview/access_write.rs`
  reports that the preview route exposes no typed write command; and
- `crates/parent-runtime-core/tests/integration/parent_ui_bridge/runtime_and_activity_tests.rs`
  (`policy_preview_confirm_action_dispatches_rust_owned_command_and_reloads_snapshot`)
  proves the Rust-owned confirmation request and reload path, including the
  `ParentConfirmed` and audit-reference details.

This is visibility and authority evidence only. A confirmed preview remains
separate from delivery, enforcement, and child-device application.

## Validation

```text
cargo test -p ocentra-policy-control-core --test unit policy_request -- --nocapture
cargo test -p ocentra-parent-runtime-core --test integration policy_preview -- --nocapture
npx vitest run apps/portal/tests/policy/policy-preview-route-panel.test.ts
npm run lint:architecture -- --files crates/policy-control-core,crates/parent-runtime-core,apps/portal
git diff --check
```

## Explicit non-claims / remaining WP02 gaps

- no template or manual-rule authoring completion;
- no proof that preview-to-save UX is complete;
- no child delivery or enforcement proof;
- no mobile/accessibility closeout;
- no claim that assistant confirmation itself is a general policy write.

WP02 remains `Partial / open` in the manifest until those independent slices
have their own live proof.
