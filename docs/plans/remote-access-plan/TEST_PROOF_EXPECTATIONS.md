# Test and Proof Expectations

| Risk surface             | Expected proof                                                                                        |
| ------------------------ | ----------------------------------------------------------------------------------------------------- |
| Remote session authority | AuthN/authZ matrix, household/device binding, stale/revoked session rejection.                        |
| Live view                | Permission, protected-surface, retention, redaction, unavailable relay, and degraded-state proof.     |
| Remote control           | Parent confirmation, child disclosure where required, input scope, escape/stop path, and abuse proof. |
| Relay transport          | reconnect, timeout, partial outage, retry storm, rate limit, DoS, and backpressure proof.             |
| Privacy/custody          | no unintended raw retention, delete/export boundary, audit and redacted logs.                         |
| Observability            | session start/stop/error metrics, alerts, traces, and support diagnostics without sensitive leakage.  |
| PR gate                  | route sync, proof artifact paths, negative cases, skipped-risk notes.                                 |

## Where tests should live

- Keep remote-access tests in remote-control/runtime package test trees and proof-output folder referenced by the selected workpack.
- Real peer/relay tests are preferred over mocked transport tests for reconnect, retry, and permission failures.
- Pair protocol proofs with protocol/domain package tests for token and session contracts.

## Expected test/proof inventory

- `remote-access.session.authn-authz`: household/device authN/authZ matrix rejects cross-household and stale/revoked sessions.
- `remote-access.relay.reconnect-retry-storm`: reconnect and retry-storm handling includes backpressure and rate control proof.
- `remote-access.control.permission-escape`: input scope and stop/escape path remain constrained and non-escalating.
- `remote-access.privacy.raw-retention`: no unintended raw retention or unapproved telemetry; custody and deletion proofs exist.
- `remote-access.observability.relay-failure`: session lifecycle logs, alerts, and traces include start/stop/error with redaction.

## Failure conditions

- No remote access DONE/PR_READY if negative authorization, replay, and relay failure paths are not tested.
- No remote access DONE/PR_READY if parent confirmation/child-disclosure boundaries remain implicit.
- No remote access DONE/PR_READY without explicit cancellation/escape/rollback and privacy leakage proof.
