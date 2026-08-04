# WP04 Typed Policy Delivery Receipt and Rollback Proof

Workpack: `04-owned-process-time-limit`.
Branch: `codex/lane-policy-enforcement`.

## Scope

This slice proves the typed policy-control handoff up to the adapter receipt
boundary:

```text
confirmed policy source -> compiled domain artifact -> queued delivery
-> delivered -> adapter execution receipt -> applied -> rollback receipt
-> rolledBack
```

The ordinary child handoff remains fail-closed for receipt-required states.
Only the receipt-bearing handoff can advance `Applied` or `RolledBack`.

## Evidence

- `cargo test -p ocentra-policy-control-core --test unit typed_adapter_receipt_advances_applied_delivery_and_rolls_back_idempotently`
  proves the compiled-policy fixture reaches delivery, stores the applied
  receipt, stores the rollback receipt with `Applied` as the restored state,
  and treats duplicate rollback replay as a no-op.
- `cargo test -p ocentra-policy-control-core --test unit forged_adapter_receipt_cannot_advance_applied_delivery`
  proves mismatched adapter evidence cannot advance the delivered record.
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff trusted_adapter_handoff_persists_applied_and_rolled_back_receipts`
  proves the child-policy handoff carries both receipt-bearing transitions.
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff delivery_rollback_with_valid_context_still_requires_execution_receipt`
  proves a valid rollback reason and prior-state reference are insufficient
  without adapter receipt evidence.

## Enforcer evidence

- Proof run `policy-enforcement.typed-policy-delivery-receipt-rollback-20260804014143-8aedffcf` passed with zero diagnostics for the policy-core unit and version-skew suites.
- Proof run `policy-enforcement.child-policy-delivery-handoff-receipt-rollback-20260804014153-6da4e184` passed with zero diagnostics for the child-policy replay suite.
- `npm run lint:architecture -- --files crates/policy-control-core crates/child-policy-core` passed.
- `cargo fmt --all -- --check` passed.
- `cargo clippy -p ocentra-policy-control-core -p ocentra-child-policy-core --all-targets -- -D warnings` passed.
- Enforcer checks for reexports, validation-bypass, placeholder-implementation,
  weak-assertions, required-tests, and no-test-doubles passed on the claimed
  files.
- Enforcer architecture check remains blocked only by the pre-existing
  `RR-6.1` raw-string findings at lines 33, 37, 61, and 65 of
  `crates/policy-control-core/src/policy_delivery.rs`; no waiver was added.

## Validation result

Validation is recorded in the handoff report and must include the focused Rust
tests, `git diff --check`, and the scoped Enforcer checks. Existing repository
architecture debt is reported separately; no waiver or bypass is added.

## No-claim boundary

This proof is limited to typed policy compilation, delivery transitions,
receipt validation, and rollback linkage. It does not claim a real OS process
termination, authenticated receipt persistence or reload, non-forgeable
cross-process adapter authority, parent-visible rendering, mobile parity, or
broad installed-app blocking. The next dependency is the enforcement runtime
trusted-dispatch ledger and platform-owned adapter execution boundary.
