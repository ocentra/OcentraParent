# App Control Capability Guide

Status: product capability guide for future portal UI and parent guidance.

This document explains what Ocentra Parent can and cannot know or control for
native and installed application activity across desktop and mobile platforms.
It is meant to feed later Policy and App UI work, especially parent-facing guide
sections where a parent chooses between observation, time limits, ask-parent
flows, app blocking, and managed-device setup.

This is not a moral policy document. The product should expose real capability
boundaries and let the parent choose the household rule posture. The important
engineering rule is that the UI must not imply exact app knowledge or app control
that the child-device agent cannot prove through the selected platform adapter.

Native app implementation planning lives in
[Native Apps Plan](plans/app-plan/README.md). That folder turns this capability
guide into workpacks, proof gates, UI requirements, and platform authority-tier
checklists without moving this guide out of its source location.

Browser-specific URL, tab, download, and web-domain controls are covered by the
browser capability guide. Game-specific title, launcher, and game-session
details are covered by app/game evidence sessions and should later be refined in
game-control docs. This document focuses on native and non-game applications.

## Core Terms

### Native App

A native app is an installed or launchable application that runs as an operating
system app rather than as a browser tab. Examples:

- Windows Win32 desktop app.
- Windows packaged app or Microsoft Store app.
- macOS app bundle.
- Linux desktop app, package app, Flatpak, Snap, AppImage, or command-backed
  desktop entry.
- Android package.
- iOS or iPadOS application selected through Apple-approved controls.

The key property is not the packaging format. The key property is that Ocentra
can reason about the app through OS, package, process, window, launcher, usage,
or managed-device evidence rather than browser tab evidence.

### Managed App

A managed app is an app whose install, launch, use, or policy state is inside an
approved Ocentra or platform management boundary.

Examples:

- App installed by a supervised or managed device flow.
- App allowlisted or denylisted by an OS application-control policy.
- App represented by an opaque mobile platform token selected by a guardian.
- App launched through an Ocentra-controlled shortcut, launcher, or policy
  adapter.
- App process started by Ocentra and tracked with an owned process/session id.

The key property is proof. A managed app is not merely "known by name." It must
have adapter evidence showing which package/process/window/policy state produced
the claim.

### Unmanaged App

An unmanaged app is any app outside the approved management boundary.

Examples:

- A normal user-installed desktop app without Ocentra policy.
- A portable executable.
- A copied or renamed executable.
- A helper process launched by a known app but not mapped to a supported app
  identity.
- A mobile app that the platform does not expose through the approved parental,
  enterprise, or device-owner APIs.
- An app running on an unsupported platform adapter.

Unmanaged app use can still be observed as process, window, package, or usage
behavior where the platform permits. It must not be treated as exact category,
content, or enforceable app identity unless the evidence proves that identity.

### Unknown App

An unknown app is an observed process, package, shortcut, app bundle, or mobile
token that cannot be confidently mapped to a known app record.

Unknown is a valid state. It should not be silently promoted to a known app, a
game, a risky category, or a blocked target. Parent policy can choose what to do
with unknown apps, but the evidence label must stay honest.

### App Identity

App identity is the stable reference used for matching policy to app activity.
Depending on platform and capability, it may include:

- Package id, bundle id, AppUserModelID, package family name, desktop entry id,
  or application token.
- Executable path, file hash, publisher/signature, product name, or version.
- Process id and parent process id for a running observation.
- Window id and foreground state for active-use evidence.
- Installer/source reference and install state for inventory evidence.

No single field is enough on every platform. Policy should match through a typed
identity strategy that records which fields were available and how confident the
match was.

### App Session Evidence

App session evidence is a derived local read model backed by raw process,
window, package, usage, or platform activity evidence. It can prove:

- App was observed.
- App was running.
- App was foreground-active where foreground proof exists.
- App had a running or foreground duration inside a time window.
- App identity, category, or unknown state was derived from specific evidence
  ids.

It does not prove screen contents, app-internal content, keystrokes, chat text,
or exact in-app activity unless a later explicit feature creates a separate
approved evidence boundary.

### App Control Action

An app control action is a child-agent execution result that changes or attempts
to change app behavior. Examples:

- Launch.
- Warn.
- Ask parent.
- Terminate owned or target process.
- Suspend, hide, shield, or block app where the platform supports it.
- Start, extend, expire, or roll back a time limit.
- Install or uninstall a managed app where the platform and custody model allow
  it.

Every strict action needs an audit event with policy, evidence, adapter, outcome,
and rollback or unavailable state.

## The Main Capability Truth

Installed app inventory, running process state, foreground use, and app-control
actions are separate capabilities.

An adapter that can list apps does not necessarily prove foreground use. An
adapter that can observe foreground use does not necessarily block launch. An
adapter that can terminate a process does not necessarily install, uninstall,
suspend, or prevent relaunch. A mobile token that can shield an app does not
necessarily reveal the raw bundle id or app list.

Ocentra should model app control at these layers:

- Inventory layer: app appears installed, launchable, removable, managed, or
  unknown.
- Runtime layer: process/package/app session is running, foreground, background,
  stale, or unavailable.
- Duration layer: running and foreground time are derived from stored evidence,
  not portal refresh.
- Policy layer: parent rule matches app identity, category, unknown state,
  schedule, budget, or approval state.
- Enforcement layer: child-device adapter executes terminate, block, shield,
  suspend, hide, launch, install, uninstall, or time-limit actions.
- Audit layer: every parent-visible claim carries evidence source, custody,
  policy decision, adapter result, and capability status.

The UI must keep these layers visible. "This app is installed" is weaker than
"this app was foreground for 42 minutes today." "This policy would block" is
weaker than "this adapter blocked launch and journaled the result."

## Capability Matrix

| Capability               | Windows                                  | macOS                               | Linux                                 | Android                                 | iOS/iPadOS                                 | Required proof                             | Important limit                                           |
| ------------------------ | ---------------------------------------- | ----------------------------------- | ------------------------------------- | --------------------------------------- | ------------------------------------------ | ------------------------------------------ | --------------------------------------------------------- |
| Installed app inventory  | Yes, partial by source                   | Yes, partial by source              | Yes, partial by distro/desktop        | Yes, visibility-limited                 | Limited, token/MDM-managed paths           | Inventory adapter and source ids           | Inventory is not proof of current use.                    |
| Package/process identity | Strong for observed processes/packages   | Strong for bundles/processes        | Varies by package and desktop entry   | Strong package id when visible          | Opaque tokens or managed app metadata      | Identity fields plus confidence            | Renames, helpers, and wrappers reduce confidence.         |
| Running app observation  | Yes                                      | Yes                                 | Yes                                   | Limited; usage/accessibility/DO paths   | Limited through Screen Time/MDM signals    | Runtime observation evidence               | Background services may not equal user-facing app use.    |
| Foreground app evidence  | Yes                                      | Permission-dependent                | Desktop-environment-dependent         | Usage stats/accessibility-dependent     | Device Activity thresholds, not raw focus  | Fresh foreground or activity evidence      | Foreground does not prove in-app content.                 |
| Running duration         | Yes                                      | Yes                                 | Yes                                   | Usage-stat/session-dependent            | Device Activity threshold-based            | Ordered observations/session model         | Gaps and restarts need stale handling.                    |
| Foreground duration      | Yes                                      | Permission-dependent                | Desktop-environment-dependent         | Usage-stat/accessibility-dependent      | Threshold/event-based                      | Foreground observations or platform events | Portal polling must not count time.                       |
| App categories           | Derived from catalog/package/source      | Derived from catalog/package/source | Derived from desktop/package metadata | Package/category where exposed          | Opaque category tokens through Screen Time | Category source and confidence             | Category labels are policy inputs, not hidden blocks.     |
| Launch app               | Yes                                      | Yes                                 | Yes                                   | Yes, with package intents where allowed | Limited; open intents/managed flows        | Launch adapter result                      | Launch does not imply ongoing control.                    |
| Terminate app            | Yes, where permission permits            | Yes, where permission permits       | Yes, where permission permits         | Limited; device-owner/admin paths       | No general third-party terminate           | Target identity and adapter result         | Unsaved data and race conditions need UX/audit.           |
| Suspend/hide/shield app  | App control policy dependent             | MDM/profile dependent               | Desktop/policy dependent              | Device owner/profile owner capable      | Screen Time/Managed Settings capable       | Platform management proof                  | Mobile support depends on entitlements/setup.             |
| Block launch             | AppLocker/WDAC or similar proof required | MDM/system policy proof required    | Policy/permission proof required      | Device owner/profile owner capable      | Screen Time shield or MDM restriction      | Pre-launch enforcement proof               | Current repo must not claim broad blocking without proof. |
| Time-limit app use       | Yes for app sessions and owned terminate | Possible with platform proof        | Possible with platform proof          | Usage/DevicePolicy/Accessibility proof  | Device Activity threshold/shield path      | Timer plus action/result audit             | Needs fallback when action cannot enforce.                |
| Install app              | Installer/package manager path           | Installer/MDM/package path          | Package manager path                  | Package installer/device owner/MDM      | MDM/App Store managed distribution         | Install adapter/custody proof              | User consent, store policy, and signing matter.           |
| Uninstall app            | Installer/package manager path           | Installer/MDM/package path          | Package manager path                  | Device owner/MDM/package path           | MDM-managed app removal only               | Removal adapter/custody proof              | Personal app removal is often not available.              |
| Child-facing message     | Yes                                      | Yes                                 | Yes                                   | Yes                                     | Shield UI where supported                  | Local UI/notification/shield result        | Do not show parent diagnostics to child.                  |
| Parent report            | Yes                                      | Yes                                 | Yes                                   | Yes, if evidence exists                 | Yes, token/capability-limited              | Stored evidence and custody labels         | Reports must distinguish raw vs redacted fields.          |
| Audit/retention          | Yes                                      | Yes                                 | Yes                                   | Yes                                     | Yes                                        | Journal/query retention policy             | Local-first custody remains default.                      |

## App Evidence: What Is Possible

Native app evidence is strongest when the child-device agent combines inventory,
process, foreground-window, and session evidence.

### Installed App Inventory

Inventory can support:

- App display name where safe.
- Package id, bundle id, package family name, desktop entry id, AppUserModelID,
  or app token.
- Install source: installer, store package, app bundle, desktop entry, package
  manager, managed app distribution, or unknown.
- Version, publisher, signature, hash, install path, and executable path where
  available and policy permits.
- Category metadata from platform, catalog, desktop entry, app store metadata,
  or parent-maintained catalog.
- Install, update, uninstall, hidden, suspended, shielded, managed, unmanaged,
  unsupported, or permission-limited state where the platform exposes it.

Inventory limits:

- It is partial on every platform.
- It can miss portable apps, per-user installs, wrapped apps, web apps, and apps
  hidden by platform privacy.
- It can report apps that are installed but never used.
- Mobile app lists can be package-visibility-limited, tokenized, supervised-only,
  or MDM-only.
- Inventory should never be used as proof of activity without runtime evidence.

### Process And Window Evidence

Process/window observation can support:

- Process id, parent process id, executable path, process name, command-line
  handling status, user/session reference, and launch time where available.
- Publisher/signature/hash metadata where safe.
- Window id, title, active/foreground state, minimized/background state, and
  last foreground timestamp where available.
- Sessionization into running and foreground durations.
- Unknown, permission-limited, stale, unsupported, and adapter-error states.

Process/window limits:

- Process names can be renamed.
- Helper processes may not represent user-facing apps.
- Foreground window title may contain sensitive text and may be stale or
  misleading.
- Foreground proof does not reveal what happened inside the app.
- Background process duration is not the same as child attention.
- Elevated, protected, sandboxed, or cross-user processes can be unreadable or
  uncontrollable.

### Foreground Use And Duration

Foreground use can support parent questions such as:

- Which app is active now?
- How long was this app in foreground today?
- Did the time budget run out?
- Which evidence ids prove the count?

Duration rules:

1. Count running time from process/package/app-session observations.
2. Count foreground time only from foreground, usage, or platform activity proof.
3. Treat observation gaps and agent restarts as gaps unless replay or platform
   events prove continuity.
4. Store timer state in the child-device journal/query store.
5. Do not count portal-rendered time, parent polling, or UI refresh cadence as
   child activity.

### App Categories

App categories are useful for parent authoring and reporting:

- Education.
- Productivity.
- Communication.
- Entertainment.
- Social.
- Browser.
- Game.
- Creative.
- System.
- Unknown.

Category evidence can come from platform metadata, app catalogs, parent labels,
launcher metadata, local classifier digests, or OS-managed category tokens.

Limits:

- Category is not content.
- Category confidence must be recorded.
- Parent rules decide actions. Category labels alone should not block.
- Some platforms expose categories as opaque tokens rather than raw identifiers.
- Unknown or ambiguous categories should degrade to observe, ask, or parent
  review according to explicit policy.

## App Control: What Is Possible

App control is strongest when action is tied to a typed policy decision and a
capability-proven adapter.

### Launch

Launch control can support:

- Open an approved app.
- Relaunch a blocked/closed app later after a time budget resets.
- Open an app as part of an ask-parent approval.
- Prefer a managed app path or managed browser path for certain tasks.

Limits:

- Launching an app does not guarantee it stays foreground.
- Launching an unmanaged app can move outside Ocentra control.
- Mobile launch behavior depends on platform foreground and intent rules.

### Terminate

Terminate control can support:

- Stop an app after a block or time-limit decision.
- Stop an owned child process.
- Stop a target process when identity still matches the policy target.
- Record already-exited, target-changed, permission-limited, failed, or
  succeeded results.

Limits:

- Termination can lose unsaved work.
- Target processes can exit and relaunch between detection and action.
- Parent/child UX should support grace periods, warnings, and ask-parent flows.
- Some platforms do not allow third-party apps to kill other apps.
- Protected, elevated, system, or different-user processes may be unavailable.

### Suspend, Hide, Shield, Or Block

These are stronger than terminate because they try to prevent or interrupt app
access through platform policy.

Possible mechanisms:

- Windows application control policy, AppLocker, WDAC/App Control for Business,
  or a narrower service adapter where proven.
- macOS MDM profile, system extension, endpoint/security tooling, or managed
  app restriction where entitled and deployed.
- Linux policy, desktop/session integration, package-manager restriction, or
  service-level control where proven.
- Android DevicePolicyManager package hide/suspend, device owner/profile owner,
  managed configuration, or accessibility/VPN-adjacent UX where approved.
- iOS/iPadOS Screen Time Family Controls, Managed Settings shields, Device
  Activity thresholds, or MDM restrictions where entitled/supervised.

Limits:

- Broad app blocking is a privileged OS capability, not a normal UI toggle.
- Policy setup can require admin rights, device-owner enrollment, MDM,
  supervision, entitlements, app review, signing, or store distribution.
- Some systems support shielding/visibility restrictions rather than process
  termination.
- Platform APIs may expose opaque identifiers for privacy.
- Rollback and uninstall paths must be documented before strict policies ship.

### Time Limits

Time limits are an app-session policy plus an enforcement action.

The system needs:

- App/session identity.
- Running or foreground duration proof.
- Schedule and budget state.
- Warning threshold and grace state.
- Parent approval or extension state.
- Enforcement fallback for unsupported action.
- Audit event for warning, timeout, action, failure, extension, and rollback.

Time-limit limits:

- A timer without action is report-only.
- Foreground time and running time should be separate settings.
- Cross-device time budgets need sync/custody rules.
- Mobile time-limit enforcement depends on platform-specific APIs.

### Install And Uninstall

Install/uninstall control is a package lifecycle capability, not a general app
evidence feature.

Possible managed flows:

- Windows MSI/MSIX/package manager or managed installer.
- macOS installer/package/MDM managed app or declarative package management.
- Linux package manager, Flatpak, Snap, AppImage-managed wrapper, or desktop
  entry.
- Android package installer, device owner/profile owner, managed Play, or MDM.
- iOS/iPadOS MDM managed app distribution and managed app removal.

Limits:

- Ocentra must not remove personal apps unless a platform-approved managed path
  and parent/child custody model explicitly allow it.
- Store policies, signing, entitlements, user consent, device enrollment, and
  uninstall rights vary sharply.
- Install/uninstall actions must be audited separately from normal app
  observation.

## Managed, Unmanaged, And Unknown Apps

### Managed Apps

Managed apps can support stronger claims:

- Known identity from package/bundle/app token.
- Known policy source.
- Install or update provenance.
- Known allowed/blocked/shielded state.
- Stronger app lifecycle action where the platform supports it.

The UI can show managed apps as controllable only when the capability registry
reports ready or degraded-but-actionable state for the specific device.

### Unmanaged Apps

Unmanaged apps can still support:

- Running and foreground observation.
- Session duration.
- Category candidate.
- Ask-parent or warning events.
- Terminate where allowed.
- Report-only unknown or bypass state.

The UI should not promise pre-launch blocking, app-store restriction, or removal
unless the platform adapter proves it.

### Unknown Apps

Unknown apps need explicit parent policy:

- Observe only.
- Ask parent on first run.
- Warn child.
- Count time under unknown-app budget.
- Block or terminate only when the parent selected that posture and the platform
  adapter can prove the action.

Unknown-app controls need exception handling because OS components, helper apps,
updaters, launchers, accessibility tools, school tools, and security software can
appear unknown at first.

## Child-Facing Actions

Child-facing UX should be tied to policy and capability state:

- Warn before time limit.
- Show time remaining.
- Show that parent approval is needed.
- Show whether an app is paused, shielded, blocked, or closed by parent policy.
- Offer ask-parent, request more time, or use allowed alternative where policy
  supports it.
- Hide parent diagnostics, evidence ids, adapter errors, hashes, and internal
  policy fields from the child surface.

Strict actions should prefer a grace path where product policy allows:

1. Warn.
2. Count down.
3. Ask parent or request extension.
4. Execute the action.
5. Show the result.
6. Record audit.

Hard immediate block remains a parent posture, but it must still produce an
honest audit result and rollback path.

## Reports, Custody, Retention, And Audit

Parent reports can show:

- Installed/detectable app inventory.
- Running now.
- Foreground now.
- Recent sessions.
- Daily app/category rollups.
- Time budgets and remaining time.
- Unknown and permission-limited apps.
- Policy decisions.
- Enforcement actions and failures.
- Approval requests and parent responses.

Every row should carry:

- Evidence ids.
- Source adapter.
- Capability state.
- Custody label.
- Collection scope.
- Retention policy.
- Redaction status.
- Policy version and decision id when policy contributed.
- Adapter result id when enforcement contributed.

Retention guidance:

- Raw process/window evidence should be retained for the shortest useful local
  audit window.
- Daily rollups can be retained longer than raw observations if they are
  redacted.
- Exact executable paths and window titles may be sensitive and should have
  narrower retention and reveal controls.
- Ocentra-hosted storage is not the default child-activity store.
- Parent export and deletion must preserve audit integrity while respecting
  retention settings.

## Platform Capability Notes

### Windows

Windows is the strongest first target for desktop app evidence and early app
time-limit enforcement.

Likely capability layers:

- Installed-app inventory from uninstall records, Start Menu shortcuts,
  Microsoft Store packages, known install paths, package query APIs, and
  executable metadata.
- Process enumeration and process metadata.
- Foreground-window observation.
- Running and foreground sessionization.
- Owned-process launch and termination.
- Narrow target process termination after typed policy decisions.
- Broad app control through AppLocker, WDAC/App Control for Business, managed
  installer policy, or equivalent only after explicit host proof.
- Package lifecycle actions through installer/package mechanisms where product
  setup owns the package.

Windows caveats:

- AppLocker/WDAC behavior depends on Windows edition, policy deployment, signing,
  administrator rights, audit/enforce mode, and reboot or refresh behavior.
- Microsoft Store package identity and Win32 executable identity are different
  evidence families.
- Blocking by path alone can be bypassed by copy/rename unless hash, signer, or
  managed installer proof is used.
- The current roadmap distinguishes owned-process terminate and app time-limit
  proof from broad app blocking. Do not claim broad blocking until the adapter is
  proven.

### macOS

macOS can support app inventory, process/session observation, and managed app
policy, but parity requires separate proof.

Possible layers:

- Application bundle inventory.
- Running process and window/frontmost app observation with the required
  permissions.
- Launch Services, bundle identifiers, code signing, and app metadata.
- MDM managed app distribution and restrictions where enrolled.
- System Extensions, Endpoint Security, or Network Extension paths where
  entitled and deployed.
- Managed browser/app controls through configuration profiles where supported.

Caveats:

- Accessibility, Screen Recording, Full Disk Access, Endpoint Security, Network
  Extension, and MDM posture change what is possible.
- Some controls require supervised or managed devices.
- Do not assume Windows process control maps directly to macOS.

### Linux

Linux can support process/session observation and app inventory, but
implementation depends heavily on distro, desktop environment, display server,
package format, and privilege model.

Possible layers:

- Desktop entries and menu categories.
- Package manager inventory.
- Flatpak, Snap, AppImage, or custom install metadata.
- Process observation through OS process tables.
- Foreground-window observation through X11, Wayland compositor protocols, or
  desktop-specific APIs where available.
- Process termination where permission permits.
- Policy controls through service, user session, package, desktop, firewall, or
  container mechanisms where proven.

Caveats:

- Wayland commonly restricts global window inspection compared with X11.
- Desktop entry category metadata is useful but not a complete app ontology.
- Package managers differ by distro.
- Broad app blocking should be treated as manual-required until a concrete
  adapter is proven on the target distro and desktop.

### Android

Android app control depends heavily on whether Ocentra is a normal app, device
owner, profile owner, accessibility service, VPN/DNS app, or MDM-managed agent.

Possible layers:

- Package inventory through PackageManager subject to package visibility rules.
- Usage events/statistics when the user grants Usage Access or the app has the
  required privileged context.
- Foreground visibility through UsageStatsManager or Accessibility where
  approved and enabled.
- DevicePolicyManager package hiding, suspension, permission policy, managed
  configuration, and package lifecycle control for device owner/profile owner
  contexts.
- Managed Play or MDM package installation/removal where deployed.
- Always-on VPN with lockdown for network mediation, separate from app
  foreground proof.

Limits:

- A normal Android app cannot generally control all other apps.
- Package visibility rules can hide installed apps from inventory queries.
- Usage access is permission-gated and can be revoked.
- Accessibility is sensitive and must not be used as a stealth content capture
  path.
- Device owner/profile owner changes the capability class and setup burden.
- The roadmap currently treats Android package lifecycle proof as
  manual-required until real device artifacts exist.

### iOS And iPadOS

iOS and iPadOS are the most constrained child-device platforms.

Possible Apple-approved layers:

- Screen Time frameworks: Family Controls, Managed Settings, Device Activity.
- Opaque selections for applications, categories, and web domains.
- App/category shielding through Managed Settings.
- Threshold/event monitoring through Device Activity.
- MDM managed app install/removal and restrictions for enrolled devices.
- Supervised-device restrictions where applicable.

Limits:

- Third-party apps do not get a general raw list of every installed app for
  parental control.
- Screen Time APIs are privacy-preserving and token-based; tokens can be voided
  if authorization is revoked.
- App shielding is not the same as process termination or raw app telemetry.
- MDM and supervision determine app install/removal and restriction scope.
- Family Controls entitlements, App Store review, TestFlight, and runtime
  authorization are separate proof requirements.
- The roadmap separates iOS signing/entitlements from TestFlight and runtime API
  entitlements; do not claim iOS control until each is proven.

## Policy Modes To Represent Later In UI

The later portal UI can expose capability modes rather than pretending every app
rule is equal.

### Observe App Use

What it means:

- Detect installed or launchable apps where available.
- Show running and foreground state.
- Build running and foreground session durations.
- Show unknown, unsupported, stale, and permission-limited states.
- Produce reports without changing device behavior.

Works without:

- App blocking.
- Mobile device-owner/supervised setup.
- Broad application control.

Does not provide:

- Guaranteed launch blocking.
- Install/uninstall control.
- App-internal content knowledge.

### Warn Or Ask On App Use

What it means:

- Matching app activity remains allowed temporarily.
- The child sees a warning or ask-parent state.
- The parent sees the app/session evidence and can approve, deny, or extend.

Requires:

- App/session evidence.
- Child-facing local UI or notification/shield path.
- Parent approval contract.
- Expiry and audit.

### Time-Limit Apps

What it means:

- Running or foreground time is counted against a parent budget.
- Warning and grace rules apply.
- When the budget expires, the configured action runs if the adapter supports
  it.

Requires:

- Sessionization from stored evidence.
- Timer recovery after restart.
- Policy decision and enforcement audit.
- Fallback when enforcement is unavailable.

### Block Or Shield Apps

What it means:

- Parent rules prevent or interrupt app access.
- On desktop this may mean launch block or process termination.
- On mobile this may mean shield/hide/suspend through approved platform APIs.

Requires:

- Platform-specific control proof.
- Parent exceptions.
- Rollback path.
- Audit for every strict action.

Does not guarantee:

- App deletion.
- Exact in-app activity knowledge.
- Parity across platforms.

### Managed App Lifecycle

What it means:

- Install, update, uninstall, hide, or remove managed apps through a platform
  management boundary.

Requires:

- Managed-device or package-management setup.
- Signing/store/MDM or installer proof.
- Custody model for personal vs managed apps.
- Explicit parent-visible state and audit.

Risk:

- Personal device and family device expectations differ. The product should not
  imply corporate MDM behavior unless the device is actually managed that way.

## Current Ocentra Parent Posture

Current repository direction already models this split:

- App/game evidence contracts use stored local evidence and query/read models.
- Process/window evidence can support native app sessions, running time, and
  foreground time.
- Local AI can consume evidence references or structured digests; it does not
  scan the OS.
- Policy decisions must reference evidence and remain deterministic.
- V0.8 enforcement work has typed contracts, capability status, timer/recovery,
  and audit scaffolding.
- Windows has proof direction for owned-process terminate and app time-limit
  behavior, but broad app blocking remains manual-required until a real adapter
  proves it.
- Android package lifecycle and iOS Screen Time/entitlement behavior are
  manual-required until real device/platform proof exists.
- Parent portal is an authoring and visibility surface. It does not run app
  inventory, timers, policy evaluation, or enforcement.

Relevant local docs:

- [`docs/architecture/app-game-evidence-sessions.md`](architecture/app-game-evidence-sessions.md)
- [`docs/expectations/app-game-evidence.md`](expectations/app-game-evidence.md)
- [`docs/expectations/policy.md`](expectations/policy.md)
- [`docs/expectations/enforcement.md`](expectations/enforcement.md)
- [`docs/product-roadmap.md`](product-roadmap.md)
- [`docs/managed-unmanaged-browser.md`](managed-unmanaged-browser.md)

## Future UI Rules

The App UI should eventually make these distinctions visible:

- Show inventory, running, foreground, time-limit, install, uninstall, shield,
  suspend, block, and terminate as separate capability rows.
- Show the evidence source next to every app claim.
- Show exact package/process identity only when proof exists and retention
  allows it.
- Show unknown apps as unknown, not as risky by default.
- Show app category as a label with source/confidence, not as an automatic
  decision.
- Show app time limits only when duration evidence exists or when the rule is
  clearly marked as pending proof.
- Show strict actions as ready, unsupported, permission-required,
  manual-required, degraded, dry-run-only, adapter-error, or blocked-by-setup.
- Keep managed-device setup state close to mobile actions.
- Keep child-facing messages separate from parent diagnostics.
- Every strict action should have an audit path: evidence, parent rule, compiled
  policy, adapter mechanism, outcome, timestamp, and rollback/unavailable state.

The parent should be able to choose policy posture with informed tradeoffs:

- observe only;
- warn on app use;
- ask parent on app use;
- set app/category/unknown-app time budgets;
- close or terminate selected apps after a timer;
- shield, suspend, or block apps where the platform supports it;
- manage app installs/uninstalls only inside a platform-approved custody model.

## Source References

External capability references:

- [Windows foreground window API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
- [Windows process snapshot APIs](https://learn.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes)
- [Windows package query APIs](https://learn.microsoft.com/en-us/windows/win32/appxpkg/functions)
- [Get-AppxPackage](https://learn.microsoft.com/en-us/powershell/module/appx/get-appxpackage)
- [Windows AppLocker](https://learn.microsoft.com/en-us/windows/security/threat-protection/windows-defender-application-control/applocker/how-applocker-works-techref)
- [Windows App Control for Business and AppLocker overview](https://learn.microsoft.com/en-us/powershell/scripting/security/app-control/application-control)
- [Apple System Extensions](https://developer.apple.com/system-extensions/)
- [Apple Network Extension provider deployment](https://developer.apple.com/documentation/technotes/tn3134-network-extension-provider-deployment)
- [Apple Device Management](https://developer.apple.com/documentation/DeviceManagement)
- [Apple MDM commands and queries](https://developer.apple.com/documentation/devicemanagement/commands-and-queries)
- [Apple FamilyActivitySelection](https://developer.apple.com/documentation/FamilyControls/FamilyActivitySelection)
- [Apple Managed Settings ShieldSettings](https://developer.apple.com/documentation/managedsettings/shieldsettings)
- [Apple Screen Time frameworks](https://developer.apple.com/documentation/ScreenTimeAPIDocumentation)
- [Apple device management restrictions for iPhone and iPad](https://support.apple.com/en-us/guide/deployment/restrictions-for-iphone-and-ipad-dep0f7dd3d8/web)
- [Android DevicePolicyManager](https://developer.android.com/reference/android/app/admin/DevicePolicyManager)
- [Android UsageStatsManager](https://developer.android.com/reference/android/app/usage/UsageStatsManager)
- [Android PackageManager](https://developer.android.com/reference/android/content/pm/PackageManager)
- [Freedesktop Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry-spec/latest-single/)
- [Freedesktop Desktop Menu Specification](https://specifications.freedesktop.org/menu-spec/latest-single/)
