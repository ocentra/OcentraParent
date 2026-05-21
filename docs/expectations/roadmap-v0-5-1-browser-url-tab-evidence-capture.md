# V0.5.1 Browser URL And Tab Evidence Capture Expectations

This is the milestone-specific expectation file for V0.5.1 in `docs/product-roadmap.md`.

Supporting expectation files: [browser evidence](browser-evidence.md), [capture](capture.md), [evidence storage](evidence-storage.md), [contracts](contracts.md), [portal](portal.md), and [platforms](platforms.md).

## Outcome

- Supported managed-browser sessions can produce typed active URL/tab evidence with URL, title, normalized domain, active state, evidence id, source id, adapter id, and capability status.
- Browser URL/tab evidence is distinct from process/window and network/domain evidence.
- Unmanaged browser use is reported as unmanaged or possible bypass, not as successful URL capture.

## Acceptance

- Supported, unsupported, unmanaged, missing bridge, missing permission, stale evidence, and adapter-error states are represented.
- Browser evidence is journaled and ingested before portal or local AI use.
- The milestone does not capture page body text, screenshots, keystrokes, form values, browser secrets, or decrypted HTTPS payloads.

## Validation

- Run `npm run validate`.
- Include managed-browser contract tests, Rust protocol parity, real service read-model tests, and portal visibility checks.
