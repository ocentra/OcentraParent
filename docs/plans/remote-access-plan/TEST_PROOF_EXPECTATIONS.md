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

Failure: remote access DONE without security, privacy, permission, relay failure, and negative authorization proof.
