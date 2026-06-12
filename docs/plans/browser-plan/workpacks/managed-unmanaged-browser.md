<!-- agent-capsule -->

> Agent Capsule
> Doc: Managed And Unmanaged Browser Capability Guide
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Managed And Unmanaged Browser Capability Guide

Status: product capability guide for future portal UI and parent guidance.

This document explains what Ocentra Parent can and cannot know or control across
managed and unmanaged browser states. It is meant to feed later Policy and
Browser UI work, especially parent-facing guide sections where a parent chooses
between observation, managed browser use, bypass handling, and stricter browser
control.

This is not a moral policy document. The product should expose real capability
boundaries and let the parent choose the household rule posture. The important
engineering rule is that the UI must not imply exact browser knowledge or browser
control that the child-device agent cannot prove through the selected platform
adapter.

## Core Terms

### Managed Browser

A managed browser is a browser session that Ocentra starts, configures, and
tracks through an approved integration boundary.

For the Windows-first product path, this usually means:

- Edge, Chrome, or Chrome for Testing.
- An Ocentra-owned user data directory or profile.
- A managed browser session id.
- A local browser bridge such as Chromium DevTools Protocol (CDP), a managed
  browser extension plus native messaging host, browser policy, or a combination
  of these.
- Evidence written through the child-device agent journal before portal, policy,
  or local AI use.

The key property is not that the browser brand is Chrome or Edge. The key
property is that Ocentra owns the browser boundary for that session and can prove
which profile, process, bridge, and evidence path produced the browser state.

### Unmanaged Browser

An unmanaged browser is any browser-like process outside the Ocentra-managed
browser boundary.

Examples:

- Normal personal Chrome profile.
- Normal personal Edge profile.
- Firefox, Brave, Opera, Vivaldi, or another browser without a proven adapter.
- Portable browser builds.
- Renamed or copied browser executables.
- Embedded browser shells and WebViews when they are not part of an approved
  managed session.
- Any supported browser launched without the Ocentra profile, bridge, or policy
  boundary.

Unmanaged browser use can still be detected as app/process/network behavior. It
must not be treated as exact active-tab URL evidence unless a separate approved
adapter proves that exact state.

### Browser-Like Process

A browser-like process is a process that appears capable of rendering web
content or acting as a browser, even if it is not one of the main supported
browsers. This category matters for bypass handling. A strict parent policy may
choose to monitor, warn, terminate, or block browser-like processes that are not
inside the managed browser boundary.

### Exact Web Evidence

Exact web evidence means the product can prove one or more of these fields:

- Active tab URL.
- Page title.
- Normalized domain and origin.
- Browser tab/window id.
- Browser profile/session id.
- Navigation event.
- Download URL or final URL.
- Browser-native request URL for a page or resource.

Exact web evidence should come from a managed browser bridge, managed extension,
managed browser policy, or another explicit browser integration. Process/window
capture and network/domain capture are useful, but they do not prove exact active
tab URL.

### Bypass Evidence

Bypass evidence means the product saw browser or browser-like behavior outside
the managed boundary. It can prove that bypass happened or may have happened. It
does not prove the exact URL unless another capability also exists.

Bypass evidence may include:

- Browser process name, path, hash, or signature reference.
- Process id and parent process.
- Foreground window state.
- Running duration.
- Network destinations.
- DNS/domain observations.
- Download file creation.
- Whether Ocentra could terminate, block, or relaunch.

## The Main Capability Truth

Exact URL, active tab, page title, and browser download source are reliable only
inside a managed browser boundary.

Without managed browser control, Ocentra can still observe and control at lower
layers:

- App/process layer: browser app is running, foreground, timed, terminated, or
  blocked.
- Network/domain layer: destinations, domains, IPs, ports, protocol summaries,
  and flow volume where the platform exposes them.
- File layer: a file appeared in Downloads, which process likely wrote it, size,
  hash, path, and timestamps.
- Policy/audit layer: whether unmanaged browser behavior violated a parent rule.

Those lower layers are useful, but they are not the same as exact browser state.
The UI should treat them as separate evidence types.

## Capability Matrix

| Capability                        | Managed browser                                                       | Unmanaged browser                                         | Required layer                                         | Important limit                                                               |
| --------------------------------- | --------------------------------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------ | ----------------------------------------------------------------------------- |
| Detect installed browser          | Yes                                                                   | Yes                                                       | OS inventory                                           | Version/signature varies by OS and install path.                              |
| Detect running browser            | Yes                                                                   | Yes                                                       | OS process adapter                                     | A renamed or embedded browser may need browser-like heuristics.               |
| Know profile/session id           | Yes                                                                   | Usually no                                                | Managed launcher/profile                               | Personal profile ids should not be scraped.                                   |
| Know active tab URL               | Yes, if bridge or extension proves active tab                         | No, not reliably                                          | CDP or extension                                       | Window title/network destination is not enough.                               |
| Know tab list URL/title           | Yes with CDP/extension                                                | No, not reliably                                          | CDP or extension                                       | Active tab can still be unknown if adapter only proves tab list.              |
| Know page title                   | Yes with managed tab evidence                                         | No, not reliably                                          | CDP or extension                                       | Window title may be stale or misleading.                                      |
| Know normalized domain/origin     | Yes from exact URL                                                    | Sometimes from DNS/network                                | Browser bridge or network adapter                      | Domain observation is weaker than exact URL.                                  |
| Know search terms                 | No by default                                                         | No                                                        | Explicit future browser/content feature                | Search terms may appear in URLs, but should not be assumed or over-collected. |
| Know page body/content            | No by default                                                         | No                                                        | Explicit future content feature                        | Current boundary excludes page body text and decrypted payloads.              |
| Know video/page context           | Possible if URL/title/domain are enough, stronger with future adapter | Usually no                                                | Managed browser evidence plus local AI/policy          | Network to a video platform does not prove exact video.                       |
| Monitor navigation events         | Yes with extension or CDP                                             | No, not reliably                                          | Browser extension/CDP                                  | Unmanaged navigation cannot be observed as browser navigation.                |
| Block or redirect URL before load | Yes                                                                   | Not reliably                                              | Browser policy, extension DNR, CDP Fetch, proxy        | OS/network can block domains but not exact full URL in normal HTTPS.          |
| Block domain                      | Yes                                                                   | Yes, with limits                                          | Browser policy, extension, DNS/proxy, WFP/firewall/VPN | Domain/CDN/app overlap can cause false positives or gaps.                     |
| Block browser app launch          | Yes                                                                   | Yes                                                       | OS app control                                         | Requires platform-specific enforcement proof.                                 |
| Terminate browser process         | Yes                                                                   | Yes                                                       | OS process control                                     | Must audit result and handle unsaved data/race conditions.                    |
| Relaunch into managed browser     | Yes                                                                   | Yes, if target URL is known or default home is acceptable | OS process watcher plus managed launcher               | If unmanaged URL is not visible, exact page cannot be recovered.              |
| Detect download start/source URL  | Yes with downloads API/CDP/network event                              | No, not reliably                                          | Managed extension/CDP                                  | OS file creation sees the file, not guaranteed source URL.                    |
| Block or cancel browser download  | Yes with browser integration/policy                                   | Not reliably                                              | Extension downloads API, browser policy, network/proxy | OS can remove/quarantine after download but not always before.                |
| Time-limit browser use            | Yes                                                                   | Yes                                                       | Policy timer plus process/session control              | Exact site-level time needs managed URL evidence.                             |
| Site/category rules               | Yes, if category maps to URL/domain evidence                          | Partial                                                   | Managed browser, network/domain, local AI              | Category from network-only data is weaker.                                    |
| Audit what happened               | Yes                                                                   | Yes                                                       | Journal/query store                                    | Audit must include evidence source and capability status.                     |

## Managed Browser: What Is Possible

Managed browser is the path for exact web rules and browser-native control.

### Observation

A managed browser session can support:

- Browser family, channel, version, and support status.
- Managed profile id and session id.
- Process id, target id, window id, and tab id where exposed.
- Tab URL, pending URL, title, favicon, and active/inactive state where the
  browser integration permits.
- Navigation events and navigation timing.
- Request URLs and resource types through CDP or browser extension APIs.
- Download item metadata such as source URL, final URL, filename, MIME type,
  total bytes, received bytes, state, interruption reason, and danger status
  through a managed browser extension.
- Browser policy state, extension install state, native host state, bridge state,
  permission state, and degraded reason.

### Control

A managed browser session can support:

- Launching the approved browser with an Ocentra-owned profile.
- Setting a managed browser path as the normal child path where the OS and setup
  flow permit.
- Blocking, allowing, redirecting, or upgrading matching browser requests through
  declarative browser rules.
- Intercepting requests through CDP Fetch in a controlled managed session.
- Applying browser URL blocklist/allowlist policy where the browser and platform
  support it.
- Force-installing or pinning a managed extension where browser policy supports
  it.
- Monitoring, pausing, canceling, or classifying downloads where the managed
  extension/browser API supports it.
- Turning strict site rules into actual browser decisions, not just after-the-
  fact reports.
- Recording intervention mechanism, outcome, evidence id, policy decision id,
  and audit state.

### Limits And Risks

Managed browser is strong, but it is not magic:

- CDP is powerful and must never attach to a child's default personal profile.
- CDP may prove tab list before it proves the active tab. The product must keep
  `tab-list-only` and `unknown-active` states distinct.
- Browser extensions require packaging, permissions, native host registration,
  update management, and service worker heartbeat handling.
- Browser policy behavior differs by Chrome, Edge, platform, domain join, MDM,
  and browser channel.
- A child can still try another browser unless unmanaged browser handling exists.
- Browser request blocking can break sites if rules are too broad.
- Browser APIs can expose sensitive data if overused, so the product should keep
  the default data scope narrow: URL, title, domain, tab/window/session metadata,
  download metadata, timestamps, and evidence ids.

## Unmanaged Browser: What Is Possible

Unmanaged browser is controllable mainly as app/process/network behavior.

### Observation

The child-device agent may be able to observe:

- A browser executable exists.
- A browser process started or stopped.
- The process path, signature/hash reference, parent process, process id, and
  user/session.
- Foreground window state and possibly window title.
- Approximate browser use duration.
- Network destinations and flow summaries correlated to process where the OS
  exposes that data.
- DNS/domain observations.
- Files created or modified in download locations.
- Whether the process is supported, unsupported, unmanaged, stale, degraded, or
  blocked by parent rule.

### Control

The product can support, subject to platform proof:

- Monitor only.
- Warn/report unmanaged browser use.
- Ask the parent or require approval before continuing.
- Terminate unmanaged browser processes.
- Terminate unmanaged browser and launch managed browser.
- Block browser-like process launch through OS app control.
- Block network for unmanaged browsers by process, domain, DNS, VPN, proxy, or
  firewall layer where the platform supports it.
- Treat portable and unknown browsers as browser-like bypass candidates.
- Keep a parent-configured allowlist for exceptions.

### Limits

Unmanaged browser cannot be treated as exact web evidence:

- Process name does not prove URL.
- Foreground window title does not prove URL.
- DNS/domain traffic does not prove active tab or exact page.
- HTTPS hides path/query from OS network observers.
- QUIC, DNS-over-HTTPS, encrypted client hello, VPNs, proxies, CDNs, and app
  embedded traffic can reduce attribution.
- A download file appearing on disk does not reliably prove source URL.
- Portable or renamed browsers can evade simple process-name checks.
- Embedded WebViews may not look like normal browser processes.
- If the child types or navigates inside an already-running unmanaged browser,
  Ocentra usually cannot recover the exact target URL.

## Relaunch And Strict Managed-Browser Flows

A strict browser policy can be built around replacing unmanaged browser use with
managed browser use. Technically, this is a combination of detection, termination
or block, and managed launch.

Possible flow:

1. Agent detects an unmanaged browser-like process.
2. Agent classifies the process by executable, signature/hash reference, path,
   parent process, command line where available, and support status.
3. Agent checks parent policy for unmanaged browser handling.
4. If the process command line or launch intent contains a URL, the agent can
   capture that URL as a launch target.
5. Agent terminates or blocks the unmanaged browser process where the adapter
   supports it.
6. Agent launches the managed browser with the captured target URL, or with a
   safe managed start page if no URL is known.
7. Agent records an audit row with detected process, action, result, reason,
   evidence reference, and whether the target URL was recovered.

Important details:

- If the URL is only inside the unmanaged browser's active tab, the agent should
  assume it cannot recover it.
- If the unmanaged browser already has unsaved form state, termination may lose
  that state. The product can choose warn-first, grace period, or hard block
  modes, but those are parent policy choices.
- Relaunch loops are possible if browser defaults or shortcuts still point to
  unmanaged browser paths.
- False positives are possible for apps that embed browser engines for legitimate
  work. The policy needs exceptions.
- Elevated or protected processes may fail to terminate. The audit state must
  record failure, unsupported, or permission-required outcomes.

## Downloads

### Managed Browser Downloads

With a managed browser extension or equivalent browser integration, Ocentra can
observe and potentially act on browser download state.

Possible metadata:

- Browser download id.
- Initial URL.
- Final URL after redirects.
- Referrer where exposed.
- Filename and local path.
- MIME type.
- File size and received bytes.
- Start and end time.
- State: in progress, interrupted, complete.
- Error or interruption reason.
- Browser danger classification where exposed by the browser.
- Whether the download was initiated by an extension.

Possible actions:

- Monitor/report download start and completion.
- Cancel or interrupt downloads through browser API where supported.
- Block request before download through declarative browser rules, CDP Fetch,
  browser policy, proxy, DNS, or network layer.
- Quarantine, remove, or flag a file after download through OS/file policy where
  supported.

### Unmanaged Browser Downloads

Without a managed browser boundary, download knowledge is weaker.

Possible evidence:

- A file appeared in a known download folder.
- A browser-like process likely wrote or moved the file.
- File path, size, hash, extension, timestamps, and maybe zone/referrer metadata
  where the platform exposes it.
- Network flow around the same time.

Not reliable:

- Exact source URL.
- Browser tab that initiated the download.
- Whether the download came from a specific page.
- Browser danger classification.
- Full redirect chain.
- Whether a blocked site caused the file.

Therefore, unmanaged download policy should be file/process/network policy, not
exact browser download policy.

## Network And Domain Controls

Network/domain controls are useful for both managed and unmanaged browsers, but
they are less precise than browser-native URL evidence.

They can help with:

- Blocking or allowing domains.
- Summarizing suspicious destinations.
- Detecting browser bypass traffic.
- Blocking unknown browser network use.
- Enforcing DNS/proxy/VPN policies.
- Supporting local AI and policy with destination evidence.

They cannot reliably prove:

- Exact active URL.
- Full URL path/query in HTTPS.
- Page title.
- Search terms.
- Which tab was active.
- Page body or chat content.
- Whether a CDN-backed request belongs to a specific product page.

The UI should keep domain/network rules separate from exact URL/browser rules.

## Platform Capability Notes

### Windows

Windows is the strongest first target for this feature family.

Likely capability layers:

- Process/window observation.
- Foreground app/window evidence.
- Managed Edge/Chrome/Chrome-for-Testing profile launch.
- Chromium CDP for managed tab and request evidence.
- Managed Chrome/Edge extension plus native messaging host.
- Browser policy for URL allow/block lists and extension force install.
- App/process control through owned process termination, AppLocker, WDAC, or
  similar OS policy where edition/setup permits.
- Network/domain control through Windows Filtering Platform, firewall, DNS,
  proxy, or VPN-style adapters.
- ETW or platform event streams for process/network diagnostics where used
  carefully.

Windows caveats:

- Some OS controls require admin rights, service installation, policy setup, or
  specific Windows editions.
- Browser policy behavior can differ between domain-managed, MDM-managed, and
  local policy setups.
- Blocking all browser-like processes is possible only after robust identity and
  exception handling exist.
- Product claims should follow real host proof, not just contract presence.

### macOS

macOS can support managed browser profiles and some browser policies, but parity
requires separate proof.

Possible layers:

- Managed Chrome/Edge profile.
- Browser extension/native host.
- Browser policy through configuration profiles or MDM where supported.
- Process observation and termination with the right permissions.
- Network Extension or content filter paths where entitled and approved.

Caveats:

- Permissions, TCC, System Extensions, Network Extensions, and MDM posture matter.
- Do not assume Windows process/network control maps directly to macOS.

### Linux

Linux can support managed browser profiles and process/network controls, but the
implementation depends heavily on distro, desktop environment, packaging, and
privilege model.

Possible layers:

- Managed Chromium/Chrome profile.
- Browser extension/native host.
- Process control.
- Firewall, DNS, proxy, or nftables/iptables style controls.

Caveats:

- Desktop foreground-window proof varies.
- Packaging and service integration vary by distro.

### Android

Android browser control depends heavily on whether Ocentra is device owner,
profile owner, has accessibility permissions, has VPN/DNS control, or is only a
normal app.

Possible layers with stronger management:

- DevicePolicyManager package hiding or suspension.
- Always-on VPN with lockdown for network mediation.
- Managed Chrome or managed app configuration where Android Enterprise setup
  exists.
- App restrictions and package access delegation where permitted.
- Usage stats or accessibility for foreground/app visibility, if explicitly
  approved and enabled.
- Owned browser or WebView if the product chooses that path.

Limits:

- Desktop Chrome extensions are not a normal Chrome for Android control path.
- Exact URL in arbitrary mobile browsers is not generally reliable without a
  browser-specific integration, accessibility-based observation, VPN/proxy
  inference, or an owned browser.
- VPN/DNS/proxy can classify domains and flows, but not always full URL or active
  tab.
- Device owner/profile owner status changes what is possible.

### iOS And iPadOS

iOS and iPadOS are the most constrained child-device platforms.

Possible Apple-approved layers:

- Screen Time frameworks: Family Controls, Managed Settings, Device Activity.
- App, category, and web domain selection/monitoring through Screen Time tokens.
- Shielding apps and web domains through Managed Settings.
- Web content filtering through MDM/device management profiles.
- Network Extension content filter paths with required entitlements and
  deployment constraints.
- Supervised-device content filtering for stronger managed-device scenarios.

Limits:

- Third-party apps do not get general exact URL telemetry from arbitrary browsers.
- Screen Time APIs are privacy-preserving and token-based; they are not raw
  browser history APIs.
- Web domain shielding is not the same as full active-tab URL capture.
- MDM, supervision, entitlements, App Store review, and Family Controls approval
  affect what is shippable.

## Policy Modes To Represent Later In UI

The later portal UI can expose capability modes rather than pretending every
browser rule is equal.

### Observe Browser Use

What it means:

- Detect browsers.
- Show running/foreground state.
- Show managed browser status if present.
- Show unmanaged browser as bypass evidence.
- Show domains/network flow where available.

Works without:

- Managed browser exact URL capture.
- Enforcement adapters.

Does not provide:

- Reliable exact URL rules.
- Browser-native download source policy.

### Managed Browser For Exact Web Rules

What it means:

- Parent rules that target exact URL, site path, active tab, page title, video
  page, or browser download source require a managed browser session.
- The system can enforce or preview based on managed evidence.
- Unmanaged browser use becomes a separate bypass condition.

Works best with:

- Managed profile.
- CDP or extension.
- Browser policy.
- Journaled evidence and local policy decisions.

### Warn Or Ask On Unmanaged Browser

What it means:

- Unmanaged browser launches are allowed temporarily, but they create a policy
  event.
- The child or parent can be warned or asked.
- The product does not claim to know exact unmanaged URL.

Works with:

- Process/window observation.
- Optional notification/approval path.

### Relaunch Managed Browser

What it means:

- If unmanaged browser is opened, close it where supported.
- Launch the managed browser instead.
- Preserve the target URL only when the URL is visible from launch command,
  shortcut, protocol activation, or another approved source.

Requires:

- Process watcher.
- Termination permission.
- Managed browser launcher.
- Audit trail.

Does not guarantee:

- Recovery of a URL typed inside an already-running unmanaged browser.

### Block Browser-Like Processes

What it means:

- Only approved managed browser paths are allowed.
- Unknown, portable, or unmanaged browser-like processes can be blocked or
  terminated.

Requires:

- App/process control.
- Browser executable classification.
- Parent exceptions.
- Rollback path.
- Strong validation on the target OS.

Risk:

- Can block legitimate embedded browser workflows if classification is too broad.

### Domain/Network Fallback

What it means:

- Use DNS, firewall, VPN, proxy, or OS network policy to control broad domain or
  network destinations.

Useful for:

- Unmanaged browsers.
- Non-browser apps.
- Known high-risk domains.
- Device-wide allow/block posture.

Not enough for:

- Exact page, active tab, search terms, or browser download source.

## Current Ocentra Parent Posture

Current repository direction already models this split:

- Browser evidence contracts include managed and unmanaged browser states.
- Exact browser URL/title/domain evidence belongs to the managed browser
  boundary.
- Unmanaged browser use is reported as bypass or non-compliance evidence, not
  successful URL capture.
- Managed Chromium/CDP is the preferred first exact-evidence boundary.
- Managed extension/native messaging is a later supplement when active-tab,
  download, or richer browser event proof needs it.
- Process/window capture and network/domain observation remain useful but cannot
  infer exact URLs.
- Enforcement actions such as terminate, block, relaunch, or OS app control must
  remain capability-gated and proof-gated.

Relevant local docs:

- [`docs/expectations/browser-evidence.md`](expectations/browser-evidence.md)
- [`docs/architecture/browser-url-tab-evidence-capture.md`](architecture/browser-url-tab-evidence-capture.md)
- [`docs/architecture/network-flow-evidence-capture.md`](architecture/network-flow-evidence-capture.md)
- [`docs/product-roadmap.md`](product-roadmap.md)
- [`docs/policy Ui fix.md`](policy%20Ui%20fix.md)

## Future UI Rules

The Browser UI should eventually make these distinctions visible:

- Show exact URL rules only when managed browser evidence is available or when
  the rule is clearly marked as requiring managed browser.
- Show unmanaged browser detections as bypass evidence.
- Show network/domain rules as domain evidence, not exact URL evidence.
- Show app/process controls separately from URL controls.
- Show downloads as browser-native download evidence only for managed browser
  integrations that can prove download metadata.
- Keep capability status close to each action: ready, unsupported,
  permission-required, bridge-missing, managed-profile-missing, adapter-error,
  disabled-by-parent, monitor-only, or manual-required.
- Every strict action should have an audit path: detected state, parent rule,
  mechanism, outcome, timestamp, and evidence reference.

The parent should be able to choose policy posture with informed tradeoffs:

- observe only;
- require managed browser for exact web rules;
- warn on unmanaged browser;
- ask parent on unmanaged browser;
- close and relaunch managed browser;
- block browser-like processes;
- combine browser rules with domain/network fallback.

## Source References

External capability references:

- [Chrome tabs API](https://developer.chrome.com/docs/extensions/reference/tabs)
- [Chrome webNavigation API](https://developer.chrome.com/docs/extensions/reference/api/webNavigation)
- [Chrome declarativeNetRequest API](https://developer.chrome.com/docs/extensions/reference/api/declarativeNetRequest)
- [Chrome downloads API](https://developer.chrome.com/docs/extensions/reference/api/downloads)
- [Chrome DevTools Protocol Fetch domain](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/)
- [Chrome DevTools Protocol Target domain](https://chromedevtools.github.io/devtools-protocol/tot/Target/)
- [Chrome Enterprise URL block and allow policies](https://support.google.com/chrome/a/answer/7532419?hl=en)
- [Chrome URL blocklist filter format](https://support.google.com/chrome/a/answer/9942583?hl=en-EN)
- [Microsoft Edge ExtensionInstallForcelist policy](https://learn.microsoft.com/en-us/deployedge/microsoft-edge-policies/extensioninstallforcelist)
- [Windows Event Tracing](https://learn.microsoft.com/en-us/windows/win32/etw/about-event-tracing)
- [Windows AppLocker](https://learn.microsoft.com/en-us/windows/configuration/lock-down-windows-10-applocker)
- [Android DevicePolicyManager](https://developer.android.com/reference/android/app/admin/DevicePolicyManager)
- [Apple Screen Time frameworks](https://developer.apple.com/documentation/ScreenTimeAPIDocumentation)
- [Apple Managed Settings web domain shields](https://developer.apple.com/documentation/managedsettings/shieldsettings/webdomains-swift.property)
- [Apple Device Activity events for apps, categories, and web domains](https://developer.apple.com/documentation/deviceactivity/deviceactivityevent/init%28applications%3Acategories%3Awebdomains%3Athreshold%3A%29)
- [Apple WebContentFilter payload](https://developer.apple.com/documentation/devicemanagement/webcontentfilter)
- [Apple content filtering deployment guide](https://support.apple.com/en-euro/guide/deployment/dep1129ff8d2/web)
- [Apple Network Extension filter provider](https://developer.apple.com/documentation/networkextension/nefilterprovider)
