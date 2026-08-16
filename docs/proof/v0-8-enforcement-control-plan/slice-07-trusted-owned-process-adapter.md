# WP04 Receipt-Required Transition Fail-Closed Boundary

Workpack: `04-owned-process-time-limit`.

This note records a fail-closed transition boundary only. It does not provide a
retained WP04 proof artifact and does not claim receipt persistence, trusted
adapter authority, an untrusted service command can enforce, an OS process was
terminated, or broad app blocking is available.

## Scope

- `crates/policy-control-core`: receipt-required delivery states reject the
  receiptless transition path.
- `crates/child-policy-core`: the ordinary handoff keeps receipt-required
  states fail-closed until an authenticated trusted-dispatch ledger exists.

## Evidence

- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff -- --nocapture`
  - result: focused regression.
  - negative: a rollback with a real `Delivered` prior state, valid reason,
    and valid rollback reference still rejects without a trusted execution
    receipt.

## Artifact boundary

The workpack's deterministic `output/` and `test-results/` roots are ignored
and no retained WP04 artifact has been attached to this PR. This note is not a
substitute for the required `00-scope-summary.md`, `01-negative-case-proof.md`,
`02-no-claim-boundary.md`, and `16-validation-commands.log` artifacts.

## No-claim boundary

`agent-service` does not yet receive an authenticated, persisted trusted
dispatch record before it parses a command payload. Therefore this packet
neither authorizes direct service envelopes nor proves receipt persistence, an
owned-process side effect, rollback execution, parent visibility, or platform
enforcement. The required next owner is the parent-runtime-to-agent-service
trusted dispatch ledger: it must issue non-forgeable adapter authority, bind it
to delivery identity and receipt data, persist verified provenance, and expose
an authenticated reload boundary.
