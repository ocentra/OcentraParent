# Browser URL And Tab Evidence Capture Architecture

Status: V0.5.1 research/spec. This document defines the browser evidence
architecture before runtime implementation. It does not add feature code.

## Product Claim

Ocentra Parent may claim exact browser URL/tab visibility only inside a managed
browser integration boundary. Process/window capture can prove that a browser
window is foreground. Network/domain capture can prove traffic or DNS/domain
metadata. Neither path proves the exact active tab URL, page title, tab id, or
browser profile.

The first product path is:

```text
Ocentra-managed browser launch/profile
  -> localhost-only browser-supported bridge
  -> typed browser evidence
  -> encrypted journal
  -> SQLite query store
  -> portal read model and local AI evidence references
```

Unmanaged browser use is still useful evidence, but it is bypass evidence, not
URL evidence.

## Source Facts

Official browser and OS docs establish the implementation boundary:

| Fact                                                                                                                                                                                                              | Product impact                                                                                                                   | Source                                                                                                                                                                                                                                |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Chrome 136 and newer ignore remote debugging switches against the default Chrome data directory unless a non-standard `--user-data-dir` is supplied.                                                              | The managed browser must use an Ocentra-owned non-default profile and must not attach to the child's real default profile.       | [Chrome remote debugging security update](https://developer.chrome.com/blog/remote-debugging-port)                                                                                                                                    |
| Chrome DevTools Protocol exposes `/json/version`, `/json/list`, target ids, target titles, target URLs, and WebSocket debugger URLs when Chrome starts with a remote debugging port.                              | A managed Chromium-family browser can produce tab target URL/title evidence through a local bridge.                              | [Chrome DevTools Protocol HTTP endpoints](https://chromedevtools.github.io/devtools-protocol/)                                                                                                                                        |
| Microsoft Edge DevTools Protocol matches Chrome DevTools Protocol and supports `--remote-debugging-port`, `--user-data-dir`, `/json/version`, and `/json/list`.                                                   | Edge is a strong Windows MVP candidate for the same managed-browser bridge pattern.                                              | [Microsoft Edge DevTools Protocol](https://learn.microsoft.com/en-us/microsoft-edge/devtools/protocol/)                                                                                                                               |
| Chrome extension `tabs` permission grants access to sensitive `tabs.Tab` properties such as `url`, `pendingUrl`, `title`, and `favIconUrl`; `activeTab` grants temporary host permission after a user invocation. | Extension capture can provide strong active-tab semantics, but it adds permission and distribution requirements.                 | [Chrome tabs API](https://developer.chrome.com/docs/extensions/reference/api/tabs)                                                                                                                                                    |
| Chrome and Edge native messaging use a registered native host over stdin/stdout with length-prefixed JSON and allowed extension origins.                                                                          | An extension-to-native bridge is viable, but it becomes a separate trust and lifecycle boundary from the managed browser bridge. | [Chrome native messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging), [Edge native messaging](https://learn.microsoft.com/en-us/microsoft-edge/extensions/developer-guide/native-messaging)      |
| Firefox WebExtensions expose a tabs API with URL/title permission requirements, and Firefox WebDriver BiDi can be enabled with `--remote-debugging-port` on `127.0.0.1`.                                          | Firefox should be a later adapter with separate proof, not assumed equivalent to Chromium CDP.                                   | [MDN tabs API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs), [MDN WebDriver BiDi connection](https://developer.mozilla.org/en-US/docs/Web/WebDriver/How_to/Create_BiDi_connection)                |
| Win32 `GetForegroundWindow` returns the foreground window handle. Windows Filtering Platform is a network traffic processing platform.                                                                            | OS foreground and network adapters cannot prove exact browser tab URL by themselves.                                             | [GetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow), [Windows Filtering Platform](https://learn.microsoft.com/en-us/windows/win32/fwp/about-windows-filtering-platform) |

## MVP Browser Support

Phase 1 should be Windows plus Chromium-family managed sessions:

- Microsoft Edge Stable: primary Windows MVP candidate.
- Google Chrome or Chrome for Testing: supported after managed non-default
  profile launch, loopback bridge behavior, and version detection are proven.
- Brave only after executable identity, launch flags, bridge behavior, and
  managed-profile storage are proven.

Other browsers are explicit states:

- Firefox: candidate later through WebDriver BiDi or WebExtensions plus native
  messaging after a separate adapter proof.
- Opera, Arc, portable browsers, embedded WebView, and unknown Chromium forks:
  unsupported or unmanaged until executable identity, profile isolation, bridge
  behavior, and validation are proven.
- Any supported browser running outside the managed Ocentra session:
  unmanaged browser use and possible bypass.

The support matrix must be represented as data, not prose-only assumptions:

| Browser family            | V0.5.1 state                     | URL/title support path                            | Active-tab support path                                        |
| ------------------------- | -------------------------------- | ------------------------------------------------- | -------------------------------------------------------------- |
| Microsoft Edge            | Supported MVP target             | Managed Chromium DevTools Protocol profile        | Proven CDP focus/target signal, or marked unknown until proven |
| Chrome/Chrome for Testing | Supported MVP target after proof | Managed Chromium DevTools Protocol profile        | Proven CDP focus/target signal, or marked unknown until proven |
| Brave                     | Candidate after proof            | Managed Chromium bridge if launch/profile matches | Same as Chromium only after executable and adapter proof       |
| Firefox                   | Later adapter                    | WebDriver BiDi or managed extension/native host   | Separate proof required                                        |
| Opera/Arc/forks/WebView   | Unsupported or unmanaged         | None in MVP                                       | None in MVP                                                    |
| Portable/unknown          | Unmanaged possible bypass        | None                                              | None                                                           |

Browser family, channel, version, executable identity, managed support state,
and reason codes should be queryable by the portal so parents can distinguish
"installed but unsupported" from "supported but not managed" and "managed but
degraded".

## Components

Browser inventory adapter:

- Detects installed and running browser-like executables.
- Records browser family, channel, version where available, executable path
  reference, signature/hash reference where available, process id, and support
  status.
- Does not collect URL evidence.

Managed browser launcher:

- Creates an Ocentra-owned profile under an agent-controlled data root.
- Launches an approved browser executable with the managed profile.
- Reserves a random loopback bridge port or uses browser-supported port
  discovery when available.
- Tracks managed browser session id, profile id, process id, bridge endpoint,
  browser family, and launch/status timestamps.

Browser bridge adapter:

- Connects only to the agent-launched local bridge.
- Reads browser/version metadata and tab/page targets.
- Emits raw adapter observations into typed mapping code.
- Must never connect to a default personal profile or an unmanaged browser
  bridge.

Managed extension and native host adapter, if needed:

- Is optional for V0.5.1 and must be explicitly designed before runtime work.
- May be installed only into an Ocentra-managed profile or policy-managed
  browser scope.
- Uses browser extension APIs for active tab/window events and URL/title access
  only after the required permissions are present.
- Uses a registered native messaging host to deliver length-prefixed JSON to the
  local agent boundary.
- Records extension id, native host id, install state, permission state,
  browser family, profile id, managed session id, and last heartbeat.
- Must reject messages from unregistered extension origins, unmanaged profiles,
  unknown native hosts, stale sessions, or schema-invalid payloads.
- Must not expand data scope beyond URL/title/domain/tab/window metadata.

Evidence mapper:

- Normalizes tab target data into schema-valid browser evidence.
- Derives URL origin/domain through shared URL/domain parsing contracts.
- Marks active-tab state as known only when the integration can prove it.
- Marks missing, stale, unsupported, degraded, permission-limited, and
  adapter-error states explicitly.

Journal and query-store path:

- Writes browser evidence to the encrypted journal before portal or local AI
  use.
- Replays into SQLite read models for recent browser activity, current managed
  session status, active-tab state, and unmanaged-browser status.

Portal read model:

- Displays only typed service data.
- Shows exact URLs only for managed browser evidence.
- Labels unmanaged browser use as bypass or non-compliance evidence.
- Shows missing bridge, unsupported browser, stale evidence, and adapter errors
  as first-class states.

Local AI reference provider:

- Supplies browser evidence ids and concise typed summaries to later local AI
  contracts.
- Does not pass page body text, cookies, storage, screenshots, or decrypted
  network payloads.

## State Model

Browser integration state should be explicit enough that the portal, policy, and
AI layers never need to infer capability from missing rows.

Capability status values:

- `available`: current managed evidence is fresh and schema-valid.
- `tab-list-only`: managed tab targets are known, but active tab is not proven.
- `unsupported-browser`: browser family or channel has no approved adapter.
- `unmanaged-browser`: browser-like process is outside the managed boundary.
- `managed-profile-missing`: profile setup has not completed.
- `bridge-missing`: the managed bridge is not reachable.
- `permission-limited`: a required browser or extension permission is absent.
- `stale`: last known evidence is past its freshness window.
- `adapter-error`: the adapter failed and reported a typed reason.
- `disabled-by-parent`: parent-controlled setting disabled browser capture.

Managed install and bridge state values:

- `not-installed`.
- `installed-unsupported`.
- `installed-supported`.
- `managed-profile-ready`.
- `launch-pending`.
- `running-managed`.
- `bridge-connected`.
- `bridge-disconnected`.
- `extension-missing`, if the extension adapter is used.
- `native-host-missing`, if the extension adapter is used.
- `permission-required`.
- `stopped`.
- `error`.

Evidence freshness rules:

- Every observation has `observedAt`.
- Every active-tab claim has `freshUntil` or an equivalent expiry window.
- Consumers must treat evidence as stale after expiry even if the last known URL
  is still in SQLite.
- A bridge disconnect immediately creates a degraded status and must not leave
  the last active URL displayed as current.
- Journal replay must recreate stale/degraded status from timestamps and status
  events instead of relying on portal memory.

## Managed Browser Launch Contract

A managed browser session should include:

- `managedBrowserSessionId`.
- `browserFamily`.
- `browserChannel`.
- `browserVersion`.
- `executablePathRef`, plus signature/hash reference where available.
- `profileId`.
- `profilePathRef`, redacted in debug exports unless needed for local support.
- `processId`.
- `bridgeKind`, such as `chromium-devtools-protocol`.
- `bridgeEndpointRef`, redacted in logs and portal copy output by default.
- `startedAt`.
- `stoppedAt` when closed.
- `status`.
- `degradedReason` when applicable.
- `parentSettingRef` or setup action reference when launch is enabled by a
  parent-controlled setting.
- `custodyLabel`, such as `child-device-local`.
- `permissionState`, when an extension/native host is in use.

Launch rules:

- Use a non-default Ocentra-owned profile.
- Prefer an agent-reserved random local port over a fixed convention such as
  `9222`.
- Bind and connect over loopback only.
- Do not use wildcard remote origins.
- Do not attach to an existing default user profile.
- Do not persist raw WebSocket debugger URLs in logs or portal copy output.
- Shut down the bridge with the managed session when possible.

## Evidence Contracts

The contract set should be added before runtime code depends on browser
evidence:

- Supported browser capability snapshot.
- Running browser state.
- Managed install and permission state.
- Managed browser session started/stopped/degraded.
- Browser tab target observation.
- Active browser tab evidence.
- Browser integration status.
- Unmanaged browser detected.
- Browser evidence query/read model.
- Browser evidence reference for later local AI and policy inputs.

Browser tab evidence should include:

- Evidence id.
- Schema version.
- Observed at timestamp.
- Fresh-until timestamp or stale-after duration.
- Source id and adapter id.
- Device/host reference.
- Browser family, channel, and managed session id.
- Profile id where available and safe.
- Process id.
- Window id where available.
- Tab id where available.
- Target id where available.
- Active tab state: `known-active`, `known-inactive`, or `unknown`.
- Exact URL.
- Normalized origin and domain.
- Page title.
- Capability status.
- Degraded reason when status is not fully available.
- Staleness/expiry timestamp.
- Custody label, normally `child-device-local`.
- Query visibility label for local/LAN, parent cache, parent-owned export, or
  unavailable state.

Unmanaged browser evidence should include:

- Evidence id and timestamp.
- Browser-like process id/name.
- Executable path reference and signature/hash reference where available.
- Window title metadata only when allowed by the capture milestone.
- Browser family guess with confidence, if known.
- Reason: unsupported browser, supported browser outside managed session,
  portable browser, bridge missing, policy bypass candidate, or adapter error.
- No exact URL field.

Identifier rules:

- Evidence ids are generated by the child-device agent before journal write and
  are stable through SQLite replay.
- Source ids identify the browser evidence source, such as managed Chromium CDP,
  managed extension/native host, or unmanaged-process detector.
- Adapter ids identify the concrete platform adapter implementation and version.
- Managed browser session ids are scoped to one launched managed browser
  session.
- Profile ids are stable references to Ocentra-owned profiles and must not be
  raw private profile paths in portal/debug output.
- Window, tab, and target ids are browser-provided when available and should be
  treated as adapter-scoped, not globally stable household identifiers.

## Active Tab Truth Ladder

The implementation must distinguish levels of certainty:

| Level                     | Meaning                                                                                      | Parent-visible claim                                               |
| ------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Browser process running   | Process/window adapter sees a browser executable.                                            | Browser appears active or running. Exact URL unknown.              |
| Foreground browser window | OS foreground/window evidence points to a browser window.                                    | Browser window foreground. Exact tab URL unknown.                  |
| Managed tab list known    | Browser bridge returns page/tab targets with URL/title.                                      | Managed browser tabs observed. Active tab may still be unknown.    |
| Active target proven      | Browser bridge or extension bridge proves which tab is active in the focused browser window. | Exact active tab URL/title/domain known.                           |
| Evidence stale/degraded   | Last proven evidence is too old or bridge status changed.                                    | Browser evidence stale or degraded. Exact current URL not claimed. |

`/json/list` target output is enough for managed tab-list evidence. It is not by
itself enough to mark an active browser tab unless the adapter has separate
proof for active/focused target state.

If Chromium CDP cannot provide reliable active-tab proof for the managed browser
without unsafe permissions, the implementation should either:

- record tab-list evidence and mark active state as unknown, or
- introduce a scoped managed-profile extension plus native messaging bridge as a
  deliberate second-phase adapter.

It must not silently promote process foreground or target-list evidence into
active-tab evidence.

## Bridge Options

| Option                                  | What it can prove                                                                                     | Advantages                                                                                             | Risks and limits                                                                                                                                       | V0.5.1 posture                                                                   |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------- |
| Managed Chromium DevTools Protocol      | Browser metadata and tab targets with URL/title; active state only if proven by adapter behavior.     | No store extension required; works with an Ocentra-owned profile; strong Chrome/Edge official support. | Remote debugging is powerful and must never attach to the child's real profile; active-tab proof needs validation.                                     | Preferred first implementation boundary.                                         |
| Managed extension plus native messaging | Active tab, tab URL/title, tab/window events, and native agent delivery when permissions are granted. | Browser API has explicit active-tab and tab event semantics.                                           | Requires extension packaging/distribution, permission UX, native host registration, service worker lifecycle handling, and stricter origin validation. | Compare and keep as supplement/fallback if CDP active-tab proof is insufficient. |
| OS process/window capture               | Browser process, executable, foreground window handle/title where allowed.                            | Useful for unmanaged/bypass detection and correlation.                                                 | Cannot prove tab URL or page title reliably.                                                                                                           | Use only as correlation and bypass evidence.                                     |
| Network/domain observation              | Remote domains/IPs/ports/process correlation where available.                                         | Useful for domain/flow visibility and suspicious network summaries.                                    | Cannot prove exact URL path, active tab, page title, or browser profile.                                                                               | Keep separate from URL/tab evidence.                                             |

## Privacy And Security Rules

Do not capture:

- Page body text.
- Chat message content.
- Screenshots.
- Keystrokes.
- Form values.
- Cookies, tokens, browser local storage, session storage, or secrets.
- Decrypted HTTPS payloads.
- Raw DevTools protocol dumps.

The browser bridge is sensitive because it can expose powerful browser
inspection features. The first implementation must:

- use an Ocentra-owned non-default profile;
- keep bridge access loopback-only;
- choose a per-session random local port where practical;
- store bridge endpoint details as redacted references;
- reject default-profile and unmanaged bridge attachment;
- close the bridge when the managed browser session ends;
- report bridge unavailable or degraded rather than falling back to unsafe
  attachment;
- treat any raw protocol error payload as diagnostic data that requires
  redaction before portal copy/export.

If a managed extension/native host is added, the implementation must also:

- declare the minimum browser permissions needed for URL/title/tab state;
- record extension install, enabled, disabled, permission-required, and
  native-host-missing states;
- validate the extension origin against the registered native messaging host;
- use schema validation before journal write;
- report service-worker sleep or missed heartbeat as stale/degraded evidence;
- provide parent/admin setup status without showing extension secrets or raw
  browser internals.

## Journal And Query Flow

Browser evidence follows the same evidence custody path as other activity:

```text
browser adapter observation
  -> browser evidence mapper
  -> ActivityEventEnvelope or browser evidence envelope
  -> encrypted journal write
  -> SQLite ingest
  -> browser activity read model
  -> portal and local AI evidence references
```

SQLite read models should be rebuildable from the journal and should expose:

- latest managed browser integration status;
- recent managed tab observations;
- latest active tab evidence when known and fresh;
- stale/degraded bridge status;
- unmanaged browser detections;
- evidence ids for local AI/policy references.
- custody/source label for each read-model row.
- install, permission, and bridge state history needed for support/debug views.

Minimum local journal flow:

1. Adapter emits a raw local observation with adapter id and source id.
2. Mapper validates URL/title/domain, ids, timestamps, and capability status.
3. Agent assigns or validates the evidence id.
4. Agent writes a browser evidence envelope to the encrypted NDJSON journal.
5. SQLite ingest consumes the journal event and updates read models.
6. Portal, policy, and AI consumers query typed service/read-model APIs.

Portal, policy, and AI consumers must be able to reference `browserEvidenceId`
without receiving raw protocol payloads or direct browser/profile access.

Portal and local AI paths must not read browser profile files, DevTools state,
journal files, or SQLite files directly.

## Portal Behavior

The portal should show:

- supported browsers and support status;
- managed browser session state;
- bridge status and degraded reason;
- recent managed tabs with URL/title/domain when available;
- active tab only when active state is proven and fresh;
- unmanaged browser use as possible bypass;
- stale evidence as stale, not current;
- managed install state and permission state;
- data source/custody label, normally live local/LAN child agent for the MVP;
- copy/debug output with event ids, timestamps, source ids, capability status,
  and redacted bridge/profile references.

The portal must not:

- launch browsers directly;
- run capture code;
- query browser profiles;
- connect to DevTools endpoints;
- read journal or SQLite files;
- infer exact URL from window title or network destination.

## Portal, Policy, And AI Handoff

Portal handoff:

- The portal queries browser read models from the agent service.
- It shows exact URL/title/domain only for managed, journaled browser evidence.
- It marks unsupported, unmanaged, stale, missing bridge, permission-limited,
  disabled, and adapter-error states distinctly.
- It labels local/LAN, parent device cache, parent-owned storage/export, and
  unavailable sources so hosted surfaces do not appear to store child activity.

Policy handoff:

- Policy evaluation receives browser evidence references and normalized
  URL/origin/domain fields from the local agent path.
- Policy rules may target browser family, managed/unmanaged state, normalized
  domain/origin, URL pattern, capability status, and stale/degraded state.
- Enforcement cannot act on guessed URLs. It may act on unmanaged-browser
  bypass status only after a later enforcement milestone defines that policy.

AI handoff:

- Local AI inputs may reference browser evidence ids, URL/title/domain metadata,
  active-state certainty, timestamps, and recent local context.
- Local AI must treat `tab-list-only`, `unknown-active`, stale, degraded, and
  unmanaged browser states differently from proven active-tab evidence.
- Remote/API AI and hosted report compilation are optional later flows and must
  use explicit parent-controlled custody boundaries. They are not required for
  child-device browser safety decisions.
- AI does not receive page body text, screenshots, cookies, storage, browser
  secrets, raw DevTools protocol dumps, or decrypted network payloads from this
  feature.

## Acceptance Tests And Manual Validation

Contract tests:

- valid and invalid supported-browser snapshots;
- valid and invalid managed session payloads;
- managed install and permission state transitions;
- URL/title/domain parsing and normalization;
- stale evidence state;
- unsupported/degraded/missing-bridge states;
- unmanaged browser detected without exact URL;
- local AI evidence reference shape.
- custody/source labels for local, LAN, parent cache, parent-owned export, and
  unavailable states.

Rust/adapter tests:

- managed launch rejects default profile configuration;
- managed launch records profile id, session id, process id, source id, and
  adapter id;
- bridge endpoint is loopback-only and redacted in diagnostics;
- Chromium target-list payload maps into tab evidence;
- active state remains unknown unless proven;
- adapter errors become typed degraded status.
- unmanaged bridge endpoints and stale managed session ids are rejected.
- extension/native host payloads, if added, reject unknown origins, missing
  permissions, and schema-invalid messages.

Storage tests:

- browser evidence writes to the encrypted journal;
- SQLite ingest rebuilds browser read models from journal replay;
- duplicate browser evidence ids do not double-count;
- stale/degraded evidence survives replay accurately.
- portal/policy/AI read APIs see only journaled evidence references and typed
  summaries.

Portal tests:

- recent browser panel shows managed evidence from the real service path;
- unmanaged browser status appears as bypass or non-compliance evidence;
- missing bridge and stale evidence are visible;
- copy/debug output redacts bridge/profile details and includes evidence ids.
- install, permission, unsupported, stale, degraded, and custody/source labels
  are visible and do not look like successful URL capture.

Manual Windows validation:

1. Launch the agent with browser evidence enabled in dev mode.
2. Start an Ocentra-managed Edge or Chrome profile through the agent.
3. Navigate the managed browser to two safe test URLs.
4. Confirm the journal and SQLite read model include tab target evidence.
5. Confirm active tab is known only if the adapter proof exists.
6. Start a normal Chrome/Edge window outside the managed session.
7. Confirm the service records unmanaged browser use with no exact URL.
8. Open the portal on the lane-specific port and verify visible status, evidence
   rows, stale/degraded states, and redacted copy/debug output.

Managed profile matrix validation:

```powershell
cmd /c npm run test:managed-browser-matrix
```

The matrix harness opens real installed Chrome, Edge, and Firefox executables
when available, creates Ocentra-owned profiles `managed-browser-profile-a`,
`managed-browser-profile-b`, and `managed-browser-profile-c`, launches each
profile with a loopback bridge, opens the configured test URLs in separate tabs,
and writes a JSON evidence artifact under
`test-results/managed-browser-profile-matrix`. Profiles run sequentially for
stability; the goal is evidence quality, not load testing browser startup.

Chrome and Edge use Chromium DevTools Protocol. The harness does not stop at
`/json/list`: it attaches to each page target, enables `Page`/`Runtime`, captures
current URL/title, activates one target with `Page.bringToFront`, verifies
runtime focus/visibility with `document.hasFocus()` and
`document.visibilityState`, then navigates each tab to a probe URL while
journaling `Page.frameNavigated`, same-document navigation, and post-navigation
snapshots. It waits for `document.readyState === "complete"` around navigation
probes and saves per-tab protocol screenshots beside the JSON artifact.

Firefox uses WebDriver BiDi. The harness launches Firefox with
`--remote-debugging-port`, connects to `ws://127.0.0.1:<port>/session`, creates
tabs with `browsingContext.create`, navigates them with
`browsingContext.navigate`, activates one context with `browsingContext.activate`,
captures the tab tree with `browsingContext.getTree`, evaluates title/focus
state with `script.evaluate`, and journals `browsingContext` navigation/load
events.

The default URLs are:

- `https://example.com/`
- `https://www.wikipedia.org/`
- `https://www.youtube.com/`

Override them for an offline or site-specific run with:

```powershell
$env:OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_URLS = 'https://example.com/,https://www.youtube.com/'
cmd /c npm run test:managed-browser-matrix
```

This proves managed-profile connection, visible page targets, URL visibility,
page title visibility where available, active tab evidence for a
protocol-activated tab, and visited URL journaling across a navigation sequence.
External site failures remain evidence, not success claims: the JSON artifact
records Firefox BiDi navigation errors separately when a managed Firefox profile
captures a requested URL but the browser cannot complete the network load.
Longer-term production history should persist these protocol events in the
encrypted journal instead of relying on a one-shot current-tab snapshot.

## Implementation Phases

Phase 0, this spec:

- Add architecture and acceptance plan.
- Do not implement runtime feature code.

Phase 1, contracts:

- Add TypeScript Effect Schema contracts in the owning domain package.
- Add Rust protocol structs only after TypeScript contracts and tests exist.
- Include browser capability, managed session, tab evidence, active state,
  degraded status, unmanaged browser, and read-model contracts.

Phase 2, managed Chromium launch:

- Add Windows inventory and managed launcher behind a platform adapter.
- Use Ocentra-owned profile roots.
- Reserve loopback bridge port and record managed session state.
- Prove Chrome/Edge launch and shutdown behavior locally.

Phase 3, target evidence:

- Read `/json/version` and `/json/list` from the managed bridge.
- Map tab target URL/title/domain into journaled evidence.
- Keep active state unknown unless independently proven.
- Add SQLite ingest and query read model.

Phase 4, portal visibility:

- Add recent browser evidence panel backed by service read models.
- Add unmanaged browser and degraded status states.
- Add copy/debug coverage with redaction.

Phase 5, active-tab hardening:

- Prove active-tab state through safe managed-browser protocol behavior, or
  explicitly design a scoped managed extension plus native messaging bridge.
- Do not move to AI/policy decisions until active-state certainty is encoded.

Phase 6, broader browser support:

- Add Firefox or other browser adapters only after separate official-doc review,
  contract extensions, and manual validation prove the boundary.

## Done Signal

V0.5.1 is done when a local run through an Ocentra-managed browser session
records real browser URL/title/domain evidence into the encrypted journal and
SQLite query store, distinguishes open-tab evidence from active-tab evidence,
shows honest status in the portal, reports unmanaged browser use separately,
and exposes evidence ids for later local AI/policy contracts without collecting
page content or browser secrets.
