# 33 Managed Browser CDP Screenshot Capture Path

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

## Proof

- CDP screenshot proof for managed page/window/crop.
- Tests proving browser screenshot path cannot capture desktop.
