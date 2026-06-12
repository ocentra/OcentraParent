# Test and Proof Expectations

| Risk surface        | Expected proof                                                                             |
| ------------------- | ------------------------------------------------------------------------------------------ |
| Source of truth     | schema/contract negative tests, version skew, migration, duplicate truth rejection.        |
| Parent authoring    | preview, validation, conflict, empty/error/stale/degraded UI states.                       |
| Schedule/time       | timezone, DST, clock skew, expiry boundary, school night/weekend exceptions.               |
| Delivery            | idempotency, replay, stale policy, out-of-order update, offline child, retry, ack.         |
| Authorization       | parent/co-parent/child/support roles, privilege escalation, assistant action confirmation. |
| Enforcement handoff | compiled policy result, rollback, manual-required state, audit event proof.                |
| Observability       | redacted logs, metrics sanity, trace completeness, alert firing.                           |
| PR gate             | route sync, proof artifacts, skipped-risk notes, remaining gaps.                           |

## Where tests should live

- Place policy control-plane tests in policy-domain, plan-owned, or owning enforcement workpack test trees before implementation completion.
- Prefer end-to-end cross-plan proof tests in `TEST_PROOF_EXPECTATIONS`-linked proof-output paths over mock-only tests.
- Keep fixtures in explicit policy/version boundary files with schema version tags and schedule fixtures.

## Expected test/proof inventory

- `policy-control.source-truth.version-skew`: version and schema mismatch tests reject stale policy inputs.
- `policy-control.authoring.conflict-resolution`: conflicting edits/approvals resolve with deterministic precedence or explicit manual-required output.
- `policy-control.schedule.clock-dst-boundary`: timezone/DST/expiry tests for schedule exceptions, bonus-time windows, and revocation edges.
- `policy-control.delivery.replay-idempotency`: offline replay, stale update, duplicate ack, and retry ordering are safe.
- `policy-control.enforcement.handoff-boundary`: policy outputs do not invoke enforcement without handoff proof.

## Failure conditions

- No policy DONE/PR_READY if policy authoring proof remains happy-path only.
- No policy DONE/PR_READY if source-of-truth merge/conflict behavior, delivery ack, or rollback is unproven.
- No policy DONE/PR_READY if authorization/schedule negative tests and observability redaction are missing.
