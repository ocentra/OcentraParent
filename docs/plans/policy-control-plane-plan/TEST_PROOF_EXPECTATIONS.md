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

Failure: no policy DONE with happy-path-only authoring, no ack proof, or no negative authorization/schedule tests.
