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

- Microsoft Edge Stable.
- Google Chrome or Chrome for Testing.
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
- Labels unmanaged browser use as possible bypass.
- Shows missing bridge, unsupported browser, stale evidence, and adapter errors
  as first-class states.

Local AI reference provider:

- Supplies browser evidence ids and concise typed summaries to later local AI
  contracts.
- Does not pass page body text, cookies, storage, screenshots, or decrypted
  network payloads.

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

Unmanaged browser evidence should include:

- Evidence id and timestamp.
- Browser-like process id/name.
- Executable path reference and signature/hash reference where available.
- Window title metadata only when allowed by the capture milestone.
- Browser family guess with confidence, if known.
- Reason: unsupported browser, supported browser outside managed session,
  portable browser, bridge missing, policy bypass candidate, or adapter error.
- No exact URL field.

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
- copy/debug output with event ids, timestamps, source ids, capability status,
  and redacted bridge/profile references.

The portal must not:

- launch browsers directly;
- run capture code;
- query browser profiles;
- connect to DevTools endpoints;
- read journal or SQLite files;
- infer exact URL from window title or network destination.

## Acceptance Tests And Manual Validation

Contract tests:

- valid and invalid supported-browser snapshots;
- valid and invalid managed session payloads;
- URL/title/domain parsing and normalization;
- stale evidence state;
- unsupported/degraded/missing-bridge states;
- unmanaged browser detected without exact URL;
- local AI evidence reference shape.

Rust/adapter tests:

- managed launch rejects default profile configuration;
- bridge endpoint is loopback-only and redacted in diagnostics;
- Chromium target-list payload maps into tab evidence;
- active state remains unknown unless proven;
- adapter errors become typed degraded status.

Storage tests:

- browser evidence writes to the encrypted journal;
- SQLite ingest rebuilds browser read models from journal replay;
- duplicate browser evidence ids do not double-count;
- stale/degraded evidence survives replay accurately.

Portal tests:

- recent browser panel shows managed evidence from the real service path;
- unmanaged browser status appears as possible bypass;
- missing bridge and stale evidence are visible;
- copy/debug output redacts bridge/profile details and includes evidence ids.

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
