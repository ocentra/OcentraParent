# Security Negative Proof

Negative contract cases covered by focused tests:

- Unknown native app candidates must cite evidence refs.
- Unknown native app approval requests must cite child status refs when a
  candidate is present.
- Reason-ref-backed child request states must cite child reason references.
- Allow-once decisions must expire.
- Replayable and replayed approvals must cite audit references.
- Storage-unavailable decisions cannot claim audit-backed replay.
- Manual-required block outcomes cannot include an enforcement result.

No-claim boundary:

This proof does not upgrade native app blocking. Platform hard controls remain
manual-required until live adapter authority, rollback, audit, service, and UI
proof exist.

