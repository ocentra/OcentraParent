# Approval Contract Proof

This proof covers contract-level unknown approval states only.

New contract surface:

- `AppGameControlApprovalCandidate` describes new inventory apps, unknown
  runtime processes, portable executables, installers/updaters,
  launcher-game candidates, and unknown game-like executables.
- Approval requests can carry candidate metadata, child status refs, child
  reason refs, and unanswered fallback state.
- Approval decisions can carry parent response scope, allow-once expiry,
  audit references, and persistence state for replayable/replayed approval
  decisions.
- Action-result proof includes `unknown-game-like` as a proof kind and keeps
  unsupported block paths manual-required.

Acceptance covered by tests:

- New inventory app candidates parse with evidence refs and child status refs.
- Unknown runtime process approval requests carry evidence refs and child
  status refs.
- Reason-ref-backed requests require child reason refs.
- Weak unknown game-like executable candidates cannot choose a hard deny
  fallback as if the game proof were known.
- Allow-once approvals require expiry.
- Replayed or replayable decisions require audit refs.
- Storage-unavailable decisions cannot claim audit-backed replay.
- Manual-required block outcomes do not dispatch unsupported adapters.

Boundary:

This proof does not create live candidate production, parent/child approval UI,
service read models, notification delivery, hard blocking, or platform adapter
authority.

