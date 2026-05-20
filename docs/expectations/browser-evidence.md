# Browser URL And Tab Evidence Expectations

Browser evidence is the product bridge between low-level device observation and
useful parent safety decisions. Process/window capture may prove that a browser
is active. Network/domain capture may prove network destinations. Neither one
proves which browser tab is open, which URL is active, or what page title the
child is seeing.

## Outcome Bar

Parent outcome:

- A parent can see which supported browsers are installed or detectable.
- A parent can see which supported browsers are running.
- A parent can see open browser windows and tabs where the browser integration
  permits it.
- A parent can see the active browser tab, exact URL, page title, normalized
  domain, timestamp, evidence id, and source integration id.
- A parent can tell when browser URL/tab evidence is unavailable, permission
  limited, unsupported, stale, or degraded.

Child-device outcome:

- The child-device agent collects browser evidence through a deliberate browser
  integration boundary, such as a browser extension, native messaging bridge, or
  browser-supported local bridge.
- Browser evidence is journaled and ingested before the portal or local AI uses
  it.
- Browser evidence collection must not block the service event loop.

## Data Scope

Browser evidence may record:

- Browser family and supported status.
- Browser process/running status.
- Browser profile id where available and safe.
- Window id and tab id where available.
- Active/inactive state for windows and tabs.
- Exact tab URL.
- Normalized domain and origin.
- Page title.
- Observation timestamp.
- Evidence id, source id, adapter id, and capability status.

Browser evidence must not record unless a later milestone explicitly approves it:

- Page body text.
- Chat message content.
- Screenshots.
- Keystrokes.
- Form values.
- Cookies, tokens, local storage, or browser secrets.
- Decrypted HTTPS payloads.

## Trust Boundary

- Browser integrations observe browser state and emit typed browser evidence.
- Native process/window and network/domain adapters must not guess browser tab
  URLs.
- Mapping code normalizes browser evidence into shared activity/evidence
  contracts.
- Local AI and policy evaluators consume only schema-valid browser evidence with
  evidence references.
- Portal views display browser evidence but do not run browser capture.

## Expected Deliverables

- Supported-browser capability contract.
- Browser running-state contract.
- Browser window/tab evidence contract.
- Active-tab evidence contract.
- URL/title/domain normalization contract.
- Browser integration status and degraded reason contract.
- Journal write and query-store ingest path.
- Portal recent browser activity view.
- Tests for schema validation, URL/domain normalization, stale evidence, and
  unsupported/degraded states.
- Manual local validation against at least one supported browser.

## Acceptance

- The system can distinguish "Chrome is the foreground app" from "the active
  Chrome tab is https://example.com/".
- The active tab has an exact URL, title, normalized domain, timestamp, evidence
  id, source id, and adapter id.
- Unsupported browsers and missing permissions are typed states, not silent
  failures.
- Browser evidence survives journal/query-store round trip before portal or AI
  use.
- Local AI input contracts can reference browser evidence by id.
- Tests prove invalid URLs, missing required ids, and out-of-range/stale states
  are rejected or marked degraded.
- No page body, screenshots, keystrokes, browser secrets, or decrypted traffic
  are captured.

## Done Signal

A local run records real browser URL/tab evidence from a supported browser into
the journal and query store, shows it in the portal with honest capability
status, and makes the evidence addressable by local AI or policy contracts
without claiming content inspection.
