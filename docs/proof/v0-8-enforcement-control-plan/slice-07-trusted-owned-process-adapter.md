# WP04 Trusted Adapter Receipt Persistence Precondition

Workpack: `04-owned-process-time-limit`.

This receipt proves the policy-delivery persistence precondition only. It does
not claim that an untrusted service command can enforce, that an OS process was
terminated, or that broad app blocking is available.

## Scope

- `crates/policy-control-core`: an `Applied` delivery state persists only with
  the exact execution receipt validated against delivery identity, sequence,
  audit references, and state context.
- `crates/child-policy-core`: the trusted-adapter receipt handoff is separate
  from the receiptless path, which remains manual-required.

## Evidence

- `cargo test -p ocentra-policy-control-core --test unit policy_delivery -- --nocapture`
  - result: pass; 46 tests.
  - positive: a matching receipt advances `Delivered` to `Applied` and replay
    of the same receipt is idempotent.
  - negative: a receipt with mismatched audit references is rejected and the
    delivery remains `Delivered`.
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff -- --nocapture`
  - result: pass; 9 tests.
  - positive: the child handoff retains the exact applied receipt.
  - negative: receipt-required states on the ordinary handoff remain
    `ManualRequired`.

## No-claim boundary

`agent-service` does not yet receive an authenticated, persisted
`PolicyDeliveryRecord`/trusted dispatch event before it parses a command
payload. Therefore this packet neither authorizes direct service envelopes nor
proves an owned-process side effect, rollback, parent visibility, or platform
enforcement. The required next owner is the parent-runtime-to-agent-service
trusted delivery bridge and its receipt ledger.
