# Capture Feature Expectations

Capture features create real observations from a child device.

## Expected Deliverables

- Platform-specific adapter behind a platform-neutral boundary.
- Capability/status command.
- Observation event mapping.
- Source id and observer id.
- Failure reason when the OS capability is unavailable.
- Journal write path from real observations.
- Query-store ingest path from real observations.
- Local AI evaluation input when an observation is intended for safety decisioning.
- Dev portal visibility for captured evidence.

## Acceptance

- Tests prove mapping from adapter observation to activity event.
- Service remains responsive while capture is active.
- Capture can be disabled in dev.
- Capture failures do not crash the service.
- Platform claims are scoped to real tested behavior.
- Captured page, video-link, app, or domain context is explicit about what was observed and what was not observed.

## Windows Process And Window Capture

Expected:

- Observe process identity.
- Observe foreground window/app when available.
- Record timestamps and source ids.
- Avoid blocking the WebSocket command loop.
- Do not claim browser URL visibility from process/window capture alone.

## Windows Network And Domain Observation

Expected:

- Observe domain/IP/port/process correlation where available.
- Prefer normalized intent events over raw packet dumps.
- Do not decrypt HTTPS payloads.
- Do not claim content inspection.
- Record unknown attribution clearly instead of guessing.

## Non-Goals

- Do not add blocking.
- Do not add untyped AI classification.
- Do not add stealth or anti-tamper behavior.
- Do not claim unsupported OS capabilities.

## Done Signal

A local run records real OS observations into the journal and query store, and the portal can show those observations through the real service path.
