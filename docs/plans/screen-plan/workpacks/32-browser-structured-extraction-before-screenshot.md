# 32 Browser Structured Extraction Before Screenshot

## Target State

Managed-browser URL/title/meta/DOM/accessibility signals answer first when possible.

## MVP Boundary

This is part of capture MVP because it prevents unnecessary screenshots.

## Checklist

- [x] Define managed browser structured extraction contract.
- [x] Capture URL/title/meta safely.
- [x] Capture DOM visible text with strict limits.
- [x] Prefer platform/domain parser before image capture.
- [x] Redact private content and raw DOM overflow.
- [x] Produce `no_screen_needed` when structured evidence is enough.

## Proof

- Tests showing screenshot is skipped when URL/title/DOM answer policy question.
- Portal/read-model evidence refs show structured source.

Current proof: `output/screen-plan-proof/31-32-screen-router-structured-extraction/proof-summary.json`.

Non-claims: this is bounded structured-evidence contract proof only. It does not claim real managed-browser DOM or
accessibility producer runtime, portal read-model rendering, live screenshots, policy execution, enforcement, or
product-complete pipeline closure.
