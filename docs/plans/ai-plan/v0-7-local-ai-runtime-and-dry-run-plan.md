# V0.7 Local AI Runtime And Dry-Run Plan

## Goal

Run local AI in a dry-run safety path where model availability, model output,
policy integration, and parent explanation are visible, but enforcement remains
disabled until V0.8 proof.

## Runtime Boundary

The runtime owns:

- local model artifact lookup;
- runtime status;
- provider capability;
- model load/unload status;
- generation request lifecycle;
- timeout and cancellation;
- invalid-output handling;
- resource/backpressure status;
- model cache status;
- dry-run result journal.

The runtime may execute AI locally or delegate execution to a trusted household
provider through the Household Mesh Bridge. Delegation changes execution
location only. It does not move evidence ownership, policy authority,
enforcement authority, or audit ownership away from the child agent.

The runtime does not own:

- capture;
- parent policy;
- enforcement;
- portal product data;
- remote/API child safety decisions;
- raw screenshot retention.

## First Dry-Run Case

Use a narrow stored-evidence case:

```text
stored browser/app/game/network/screen/tracking evidence
  -> local context builder
  -> local text model dry-run adapter
  -> schema-valid AI result
  -> deterministic policy dry-run evaluator
  -> AI/policy journal entry
  -> parent portal explanation
```

## Required States

- configured;
- unconfigured;
- unavailable;
- loading;
- loaded;
- generating;
- cancelling;
- timed out;
- invalid output;
- low confidence;
- degraded;
- failed;
- disabled by parent;
- disabled by platform.
- provider-discovered;
- provider-stale;
- provider-offline;
- provider-revoked;
- claim-requested;
- claim-granted;
- claim-rejected;
- lease-active;
- lease-expired;
- result-received;
- result-accepted;
- result-rejected;
- mobile-dormant;
- mobile-fallback-eligible.

## Validation

- Runtime status tests for every state.
- Local generation request/result tests.
- Invalid output parser tests.
- Timeout/cancel tests.
- Dry-run policy integration tests.
- Journal/replay proof.
- Same-device dry-run proof.
- LAN provider route dry-run proof.
- Claim/lease proof.
- Expired lease rejection proof.
- Wrong-provider result rejection proof.
- Child-agent policy authority proof.
- Portal degraded-state screenshot if UI changes.
