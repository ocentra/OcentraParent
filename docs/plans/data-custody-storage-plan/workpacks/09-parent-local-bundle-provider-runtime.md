<!-- agent-capsule -->

> Plan: `data-custody-storage-plan`
> Workpack: `WP-data-custody-storage-plan-09-parent-local-bundle-provider-runtime`
> Status: routed source work only; implementation, tests, proof, and PR readiness are open.

# WP09 Parent Local Bundle Provider Runtime

## Intent

Close the missing parent-local/provider-neutral runtime boundary for encrypted
data bundles. This workpack owns real byte custody and restart-safe job state;
it does not invent a cloud provider or move data-class authority into the
storage layer.

## Scope and ownership

In scope:

- parent-local encrypted bundle byte persistence and retrieval;
- cryptographic byte-level hash/signature verification before acceptance;
- atomic write, replace, recovery, and corruption quarantine semantics;
- manual and scheduled backup job custody, retry, restart, and idempotency;
- a provider-neutral adapter boundary that returns opaque custody status and
  never exposes readable child payloads;
- explicit no-provider, no-fallback, and manual-required states.

Out of scope:

- cloud SDK/OAuth/provider implementation or provider credentials;
- Account household/device authority or Device Trust key ownership;
- data-class mutation, restore/apply/rollback orchestration, or receipt minting;
- portal/desktop UI and proof artifact publication.

## Required handoffs and dependencies

- Data WP02 supplies key-custody and decrypt-scope decisions.
- Data WP03 supplies parent-owned provider/sync state and encryption-before-upload
  boundaries.
- Data WP04 supplies retention/delete/tombstone ordering and no-resurrection
  constraints.
- Data WP05 supplies bundle manifest/preflight/integrity contracts.
- Account WP05 supplies the current household/member/device/session/capability/
  lease composer when a job requires authority.
- Device Trust and Eventing are consumed only for the exact signer-key/readiness
  and durable retry/event boundaries they own; neither is re-owned here.

## Acceptance criteria

- Bytes are encrypted before persistence or provider handoff and are never
  accepted solely because a manifest says they are valid.
- Hash/signature verification binds the exact bundle bytes, household, source,
  version, and key/signing context; mismatch quarantines without mutation.
- Writes use an atomic commit/recovery protocol and do not publish partial
  bundles after interruption.
- Manual and scheduled jobs persist retry/restart/idempotency state without
  duplicating a bundle or silently changing retention/tombstone state.
- Provider-neutral adapters expose only opaque references/status and preserve
  manual-required state when no supported provider exists.
- No runtime path accepts authority selectors, keys, or provider identities
  from request/JSON data.

## Expected tests and proof (deferred until source wave is complete)

- exact byte hash/signature mismatch and wrong-household/key negatives;
- atomic interruption/restart and partial-write recovery;
- retry/idempotency and duplicate-job suppression;
- encrypted-before-persistence/provider boundary;
- unsupported provider/manual-required and provider disconnect states;
- redaction of payloads, keys, provider identifiers, and local paths.

Expected proof root: `output/data-custody-storage-plan-proof/09-parent-local-bundle-provider-runtime/`.

## No-claim boundary

This route does not claim provider SDK execution, Account authority, Device
Trust readiness, data-class restore/apply, portal rendering, or production
custody completion until the owning source, expected tests, focused gates, and
proof are accepted.

