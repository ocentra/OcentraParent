# Browser URL And Tab Evidence Expectations

Browser evidence is the product bridge between low-level device observation and
useful parent safety decisions. Process/window capture may prove that a browser
is active. Network/domain capture may prove network destinations. Neither one
proves which browser tab is open, which URL is active, or what page title the
child is seeing.

The focused implementation architecture lives in
[Browser URL And Tab Evidence Capture Architecture](../architecture/browser-url-tab-evidence-capture.md).

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
  integration boundary. The preferred MVP path is an Ocentra-managed browser
  launch/profile plus a browser-supported local bridge; browser extensions are
  not the default product path.
- Browser evidence is journaled and ingested before the portal or local AI uses
  it.
- Browser evidence collection must not block the service event loop.
- Browser-like processes outside the managed Ocentra browser boundary are
  reported as unmanaged browser use and possible bypass.

## Data Scope

Browser evidence may record:

- Browser family and supported status.
- Browser process/running status.
- Browser profile id where available and safe.
- Managed/unmanaged browser status.
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
- The Ocentra-managed browser boundary owns exact URL/tab evidence. Native
  process/window and network/domain adapters may detect unmanaged browser use,
  but must not infer exact URLs from that evidence.
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
- Managed browser launcher/profile contract.
- Browser window/tab evidence contract.
- Active-tab evidence contract.
- URL/title/domain normalization contract.
- Browser integration status and degraded reason contract.
- Unmanaged browser detection event and possible-bypass status.
- Journal write and query-store ingest path.
- Portal recent browser activity view.
- Tests for schema validation, URL/domain normalization, stale evidence, and
  unsupported/degraded states.
- Manual local validation against at least one supported browser.

## MVP Managed Browser Procedure

The Windows Rust agent should follow this procedure for browser URL/tab evidence:

1. Inventory installed browsers.

   Detect Chrome, Edge, Brave, Firefox, Opera, and other browser-like executables
   where practical. Record browser family, version, executable path,
   signature/hash where available, and whether the browser is supported for
   managed URL capture. MVP URL/tab capture should start with Chromium-family
   browsers that expose a browser-supported local debugging bridge.

2. Create an Ocentra-managed browser profile.

   Store managed browser profile data under an Ocentra-owned path such as
   `C:\ProgramData\Ocentra\Parent\ManagedBrowsers\chrome-child-profile`. This
   profile is separate from the child's normal browser profile. Modern Chrome
   requires a non-default user data directory for remote debugging, so exact
   URL/tab capture must not depend on attaching to the default user profile.

3. Launch the browser through Ocentra.

   Start the approved browser from the agent or approved launcher with a managed
   user data directory and a localhost-only browser bridge, for example:

   ```powershell
   chrome.exe --user-data-dir="C:\ProgramData\Ocentra\Parent\ManagedBrowsers\chrome-child-profile" --remote-debugging-port=<reserved-local-port>
   ```

   The bridge must bind only to localhost. Prefer an agent-reserved random local
   port over a fixed public convention such as `9222`. Track the launched
   process id, executable path, profile path, bridge port, managed session id,
   and browser family.

4. Connect to the browser bridge.

   The Rust agent connects to the managed browser's local bridge endpoints:

   ```text
   http://127.0.0.1:<reserved-local-port>/json/version
   http://127.0.0.1:<reserved-local-port>/json/list
   ```

   `/json/version` identifies the browser and protocol endpoint. `/json/list`
   provides page/tab targets with ids, titles, URLs, target types, and WebSocket
   debugger URLs where the browser supports them.

5. Capture browser evidence.

   For each supported target, record browser family, managed session id, browser
   process id, profile id/path reference, window id where available, tab id,
   active/inactive state where available, exact URL, normalized domain/origin,
   page title, timestamp, evidence id, source id, adapter id, and capability
   status. Store that evidence through the encrypted journal and SQLite query
   store before the portal or local AI consumes it.

6. Detect unmanaged browsers.

   Process/window capture keeps watching for browser-like processes. If
   `chrome.exe`, `msedge.exe`, `brave.exe`, Firefox, Opera, a portable browser,
   or another browser-like process is running outside the Ocentra-managed
   session, record `unmanaged-browser-detected` with process id, process name,
   executable path, signature/hash where available, and possible-bypass reason.
   This is not successful URL/tab evidence.

7. Make the managed browser the normal child path.

   Parent/admin setup should create an Ocentra browser launcher and later may set
   the managed browser as the default browser. Existing normal browser tabs
   cannot be relied on for exact URL capture unless they are inside the managed
   browser boundary. Product behavior should be explicit: Ocentra-managed
   browser sessions provide exact URLs; unmanaged browser sessions are bypass
   evidence and, in enforcement mode, may be blocked or terminated.

## Acceptance

- The system can distinguish "Chrome is the foreground app" from "the active
  Chrome tab is https://example.com/".
- The active tab has an exact URL, title, normalized domain, timestamp, evidence
  id, source id, and adapter id.
- Unsupported browsers and missing permissions are typed states, not silent
  failures.
- A normal or alternate browser running outside the managed Ocentra browser
  boundary is reported as unmanaged browser use; it is not counted as successful
  URL/tab capture.
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
status, reports unmanaged browser use separately, and makes the evidence
addressable by local AI or policy contracts without claiming content inspection.
