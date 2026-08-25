# 33 Managed Browser CDP Screenshot Capture Path

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `33 Managed Browser CDP Screenshot Capture Path`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

CDP screenshot capture is scoped to managed browser page/window/crop and never becomes desktop capture.

## MVP Boundary

This is capture MVP for browser pages, browser games, video, social web, and web signup/payment screens.

## Checklist

- [ ] Verify current Chrome DevTools Protocol `Page.captureScreenshot` docs.
- [ ] Define page, viewport, and crop capture modes.
- [ ] Keep capture tied to managed browser target ID.
- [ ] Add capture size and crop limits.
- [ ] Queue image through encrypted temporary image queue.
- [ ] Record URL/title/target evidence refs.
- [ ] Exclude live screencast from MVP default.

## Source packet status

The source-first packet now owns the Rust contract, private managed-launch
capability, browser-authority target binding, bounded CDP transport, separate
screen adapter, and encrypted queue handoff at:

- `crates/schema/src/managed_browser_cdp_capture.rs`
- `crates/schema/src/managed_browser_cdp_capture/validation.rs`
- `crates/agent-core/src/browser_managed_session.rs`
- `crates/agent-core/src/browser_managed_session/accessors.rs`
- `crates/agent-core/src/browser_managed_session/launch.rs`
- `crates/agent-core/src/browser_managed_session/capability.rs`
- `crates/agent-core/src/browser_bridge_capture.rs`
- `crates/agent-core/src/browser_bridge_capture/authority.rs`
- `crates/agent-core/src/browser_bridge_capture/binding.rs`
- `crates/agent-core/src/browser_bridge_capture/identity.rs`
- `crates/agent-core/src/browser_bridge_capture/identity_match.rs`
- `crates/agent-core/src/browser_bridge_capture/port_owner.rs`
- `crates/agent-core/src/browser_bridge_capture/process.rs`
- `crates/agent-core/src/browser_bridge_capture/target.rs`
- `crates/agent-core/src/browser_bridge_capture/transport.rs`
- `crates/screen-capture-adapter/src/managed_browser_cdp.rs`
- `crates/screen-capture-adapter/src/managed_browser_cdp/decoder.rs`
- `crates/screen-capture-adapter/src/managed_browser_cdp/structure.rs`
- `crates/screen-capture-adapter/src/managed_browser_cdp/chunks.rs`
- `crates/agent-service/src/browser_runtime_status.rs`
- `crates/agent-service/src/screen_managed_browser_cdp_runtime.rs`

The seven checklist rows remain unchecked. Tests and proof remain deferred. The
browser plan still owns the URL/target trigger producer; this packet accepts
only a browser-owned managed launch record, verifies the loopback endpoint's
OS-reported process owner and executable before launch authority/capture, and
fails closed with manual-required when the platform cannot prove that binding.
Queue timestamps are service-generated from trusted current time and bounded
by the queue TTL; caller-supplied timestamps are not accepted. No live browser trigger/runtime,
AI/OCR/VLM, retention, remote, policy, enforcement, or product completion
claim is made by this source packet.

## Deferred validation

The real-browser proof and focused contract/security/runtime tests are a later
validation wave. Their expected proof/script and test roots are not production
source and are intentionally not created or mapped by this source-only packet.

## Non-Claims

- This does not claim managed-browser production URL-trigger ownership.
- This does not claim OCR/VLM quality, policy action, enforcement, live view, or
  screenshot retention mode.
- Retained proof artifacts do not store raw screenshots; they keep hashes,
  dimensions, target evidence refs, queue custody, and deletion proof.
