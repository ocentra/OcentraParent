# 32 Browser Structured Extraction Before Screenshot

## Target State

Managed-browser URL/title/meta/DOM/accessibility signals answer first when possible.

## MVP Boundary

This is part of capture MVP because it prevents unnecessary screenshots.

## Checklist

- [ ] Define managed browser structured extraction contract.
- [ ] Capture URL/title/meta safely.
- [ ] Capture DOM visible text with strict limits.
- [ ] Prefer platform/domain parser before image capture.
- [ ] Redact private content and raw DOM overflow.
- [ ] Produce `no_screen_needed` when structured evidence is enough.

## Proof

- Tests showing screenshot is skipped when URL/title/DOM answer policy question.
- Portal/read-model evidence refs show structured source.
