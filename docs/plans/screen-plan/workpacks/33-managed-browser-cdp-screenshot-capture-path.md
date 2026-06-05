# 33 Managed Browser CDP Screenshot Capture Path

## Target State

CDP screenshot capture is scoped to managed browser page/window/crop and never becomes desktop capture.

## MVP Boundary

This is capture MVP for browser pages, browser games, video, social web, and web signup/payment screens.

## Checklist

- [x] Verify current Chrome DevTools Protocol `Page.captureScreenshot` docs.
- [x] Define page, viewport, and crop capture modes.
- [x] Keep capture tied to managed browser target ID.
- [x] Add capture size and crop limits.
- [x] Queue image through encrypted temporary image queue.
- [x] Record URL/title/target evidence refs.
- [x] Exclude live screencast from MVP default.

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
