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

## Validation

- Runtime status tests for every state.
- Local generation request/result tests.
- Invalid output parser tests.
- Timeout/cancel tests.
- Dry-run policy integration tests.
- Journal/replay proof.
- Portal degraded-state screenshot if UI changes.
