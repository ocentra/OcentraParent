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

The source-first packet now owns the Rust contract, browser-authority target
binding, bounded CDP transport, separate screen adapter, and encrypted queue
handoff at:

- `crates/schema/src/managed_browser_cdp_capture.rs`
- `crates/agent-core/src/browser_bridge_capture.rs`
- `crates/screen-capture-adapter/src/managed_browser_cdp.rs`
- `crates/agent-service/src/screen_managed_browser_cdp_runtime.rs`

Tests and proof remain deferred. The browser plan still owns the URL/target
trigger producer; this packet accepts only a browser-owned target authority and
fails closed when custody or target binding is absent. No live browser runtime,
AI/OCR/VLM, retention, remote, policy, enforcement, or product completion claim
is made by this source packet.

## Proof

- `scripts/test/screen-managed-browser-cdp-capture-proof.mjs` opens a real
  public managed-browser page in Chromium, uses Chrome DevTools Protocol
  `Page.captureScreenshot` for bounded page, viewport, and crop modes, validates
  the `@ocentra-parent/activity-domain` request/artifact contracts, encrypts the
  temporary screenshot bytes through the screen queue handoff, deletes raw and
  encrypted temp material, and writes redacted proof artifacts to
  `output/screen-plan-proof/33-managed-browser-cdp-screenshot-capture-path/`.
- `packages/activity-domain/tests/screen-managed-browser-cdp-capture.test.ts`
  proves the browser screenshot path rejects desktop capture, full-screen
  display capture, live screencast, raw screenshot retention, remote upload,
  unbounded crop requests, and non-deleted/non-temp-queue artifacts.

## Non-Claims

- This does not claim managed-browser production URL-trigger ownership.
- This does not claim OCR/VLM quality, policy action, enforcement, live view, or
  screenshot retention mode.
- Retained proof artifacts do not store raw screenshots; they keep hashes,
  dimensions, target evidence refs, queue custody, and deletion proof.
