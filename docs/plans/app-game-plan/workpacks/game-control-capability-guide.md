<!-- agent-capsule -->

> Agent Capsule
> Doc: Game Control Capability Guide
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Game Control Capability Guide

Status: product capability guide for future portal UI and parent guidance.

This document explains what Ocentra Parent can and cannot know or control across
native games, launcher-managed games, browser games, cloud games, mobile games,
and console ecosystems. It is meant to feed later Policy, Enforcement, App/Game
Evidence, and Parent Portal work where a parent chooses between observation,
time budgets, parent approval, launch controls, and stricter game enforcement.

This is not a moral policy document. The product should expose real capability
boundaries and let the parent choose the household rule posture. The important
engineering rule is that the UI must not imply exact game knowledge or game
control that the child-device agent cannot prove through the selected platform
adapter.

## Core Terms

### Native Game

A native game is a locally installed executable or package that runs on the
child device outside a browser. On Windows, that can mean a Win32 executable, a
Microsoft Store package, a launcher-owned child process, or a game engine helper
process that can be tied to a game session through deterministic local evidence.

Native game evidence can come from process/window capture, installed app
inventory, package metadata, executable signature/hash, launcher manifests, and
foreground-window state. It should not depend on screenshots, keystrokes, chat
capture, decrypted network payloads, or AI scanning the machine.

### Launcher Or Store

A launcher or store is an app that installs, updates, starts, or manages games.

Examples:

- Steam.
- Epic Games Launcher.
- Xbox app or Microsoft Store.
- Riot Client.
- Battle.net.
- EA app.
- Ubisoft Connect.
- GOG Galaxy.
- Roblox.
- Minecraft launchers.

Launcher evidence is useful, but it is not automatically game evidence. "Steam
is running" is not the same as "a Steam game is running." A launcher can be
logged in, downloading, updating, showing a store page, running chat, or
starting a helper process without proving that a child is actively playing a
particular game.

### Browser Game

A browser game runs inside a browser tab, browser shell, WebView, or web app.
Browser games are governed primarily by the browser evidence and browser control
boundary.

Ocentra can treat a browser game as game activity only when the evidence can
prove it through:

- Managed browser URL/tab evidence.
- Domain or site-category evidence, with the weaker proof level clearly shown.
- Local AI/category output over stored browser evidence or a safe digest.
- Parent-authored rules that intentionally count specified browser sites as
  game time.

Without managed browser proof, process/window evidence can prove browser use,
not exact game title or exact web game page.

### Cloud Game

A cloud game is streamed from a remote service into a browser, native client, TV
app, mobile app, or console.

Examples:

- Xbox Cloud Gaming.
- GeForce NOW.
- PlayStation cloud streaming.
- Luna or similar game-streaming services.
- Cloud games inside a browser.

Cloud-game evidence depends on the surface:

- Native cloud client: app/process/session evidence can prove the client and
  foreground time.
- Managed browser: exact URL/domain/title can help classify the game service.
- Console: only the console platform's own family/parent controls may know play
  time or title.
- Network-only evidence: can suggest a service or domain, but usually cannot
  prove exact title or active play.

### Foreground Game Session

A foreground game session is a derived read-model row backed by stored evidence
that proves a known, possible, or unknown game-like process was running and had
foreground focus for a measured interval.

Foreground time must come from process/window observations and sessionization in
the child-device agent. It must not come from portal refresh state.

### Duration Proof

Duration proof is the evidence chain that supports a time-budget or report
claim. It should carry:

- Session id.
- Process/package identity.
- First observed time.
- Last observed time.
- Running duration.
- Foreground duration.
- Observation gaps.
- Stale or permission-limited status.
- Evidence ids behind the rollup.

Duration proof is especially important because game rules are commonly phrased
as "one hour of games" or "no games during school hours." The system must be
honest about whether it counted all running process time, foreground time,
launcher time, cloud-client time, or platform-provided screen/game time.

### Process Identity

Process identity is the local identity that lets the agent decide whether two
observations refer to the same process or executable lineage.

Possible fields:

- Process id.
- Parent process id.
- Process name.
- Executable path.
- Executable hash.
- Publisher and signature status.
- Package family name or app package id.
- Launcher app id or manifest id, where safely readable.
- Command line only where policy permits and the adapter redacts sensitive
  values.

Process identity can be spoofed by renamed files, launch wrappers, injectors,
overlays, or helper processes. It should be treated as evidence with confidence,
not as unqualified truth.

### Protected Or Anti-Cheat Process

Some games and anti-cheat systems use drivers, protected processes, elevated
services, kernel callbacks, tamper protection, or anti-debugging behavior.
Ocentra must not try to bypass those controls.

The product can still represent:

- Game process observed.
- Foreground window observed.
- Duration counted.
- Termination attempted.
- Termination denied.
- Protected or permission-limited status.
- Manual-required setup.

It must not claim it can inspect protected memory, defeat anti-cheat, hide from
anti-cheat, or guarantee termination of protected processes.

## The Main Capability Truth

Game controls are reliable only at the evidence layer that the current adapter
can prove.

Ocentra can usually reason about game use from these layers:

- App/process layer: executable or package exists, starts, runs, foregrounds,
  exits, or can be terminated.
- Launcher/store layer: local library hints, launcher process, manifest-backed
  title/app id, install path, and launch relationship where readable.
- Browser layer: managed URL/tab/domain/title evidence for browser games.
- Network/domain layer: service destinations and flow summaries where the
  platform exposes them.
- Platform family-control layer: Android, iOS, Xbox, PlayStation, or Nintendo
  controls when Ocentra is integrating with an approved platform path rather
  than the local desktop agent.
- Policy/audit layer: whether parent-authored game rules produced allow, warn,
  ask, limit, block, terminate, or report-only decisions.

Those layers are useful, but they are not interchangeable. A process can prove a
native game is running. A launcher manifest can help map the process to a title.
A foreground window can prove active focus. A browser URL can prove a browser
game only inside the managed browser boundary. Network traffic can suggest a
game service, but it should not be treated as exact title or chat/content
evidence.

## Capability Matrix

| Capability                              | Native desktop game                     | Launcher/store game                          | Browser/cloud game                         | Required layer                                  | Important limit                                                              |
| --------------------------------------- | --------------------------------------- | -------------------------------------------- | ------------------------------------------ | ----------------------------------------------- | ---------------------------------------------------------------------------- |
| Detect installed game inventory         | Yes, partial                            | Yes, if manifests/packages are readable      | Browser games no; cloud clients yes        | OS inventory, package manager, launcher adapter | Inventory is partial and may miss portable, copied, or streamed games.       |
| Detect running game process             | Yes                                     | Yes, if game child process is visible        | Browser process or cloud client only       | OS process adapter                              | A launcher running is not enough to prove game play.                         |
| Detect foreground game session          | Yes, where window focus is available    | Yes, if child process/window is attributed   | Managed browser/cloud client foreground    | OS foreground-window or platform activity       | Foreground title can be stale or misleading.                                 |
| Count running duration                  | Yes                                     | Yes, with sessionization                     | Yes for client/browser process             | Process observations                            | Running time is not the same as active play time.                            |
| Count foreground duration               | Yes                                     | Yes, if foreground evidence links to session | Yes, at browser/client level               | Foreground window or platform activity          | Full-screen/exclusive modes and overlays can degrade focus proof.            |
| Know exact game title                   | Sometimes                               | Stronger with manifest/package match         | Only if browser/platform/title evidence    | Inventory, launcher, package, catalog           | Unknown and possible-game states must remain visible.                        |
| Know in-game mode, level, match, or map | No by default                           | No by default                                | No by default                              | Explicit future game integration                | Do not inspect game memory, telemetry, or screen content by default.         |
| Know multiplayer/chat content           | No                                      | No                                           | No                                         | Explicit future approved integration            | Voice/text chat capture is out of current scope.                             |
| Know game rating/category               | Sometimes                               | Sometimes from store/catalog                 | Sometimes from platform/category source    | Catalog/rating provider, store metadata         | Rating systems vary by region and may not cover every title.                 |
| Block game launch                       | Possible with OS/app-control proof      | Possible for target executable/package       | Browser controls for browser games         | OS app control, package policy, browser policy  | Broad blocking is platform- and privilege-dependent.                         |
| Terminate running game                  | Possible for owned/accessible processes | Possible for child process                   | Possible for client/browser process        | OS process control                              | Protected/elevated/anti-cheat processes may deny termination.                |
| Time-limit game use                     | Yes, if duration proof exists           | Yes, if session attribution exists           | Yes at browser/client/service level        | Policy timer plus session evidence              | Title-level limits need title-level proof.                                   |
| Ask parent for approval                 | Yes                                     | Yes                                          | Yes                                        | Policy and approval protocol                    | Parent response must be validated by the child-device agent.                 |
| Report game usage                       | Yes                                     | Yes, with source labels                      | Yes, with surface-specific labels          | Journal/query store                             | Reports must include proof level and custody.                                |
| Control console play                    | Not from desktop agent                  | Not from desktop agent                       | Not from desktop agent                     | Console family platform                         | Console control requires platform account/device integration, if available.  |
| Control mobile game package             | Platform-dependent                      | Store-specific metadata maybe                | Browser games through browser/mobile layer | Android DPM, iOS Screen Time APIs, store policy | Normal apps do not get desktop-like control on mobile.                       |
| Preserve save state before termination  | No guarantee                            | No guarantee                                 | No guarantee                               | Game/platform-specific support                  | Hard termination can lose unsaved progress or disconnect online sessions.    |
| Prove anti-cheat compatibility          | No by default                           | No by default                                | Usually not relevant                       | Vendor-specific testing                         | Ocentra should avoid invasive inspection and record degraded status instead. |

## Native Games: What Is Possible

Native games are the strongest local-desktop target when the OS exposes process,
window, package, and executable metadata.

### Observation

The child-device agent may be able to observe:

- Installed app/game inventory from local install records, shortcuts, packages,
  and known folders.
- Microsoft Store or UWP package inventory where the platform API exposes it.
- Process start, running state, parent process, process id, and exit.
- Executable path, name, publisher, signature status, and hash where permitted.
- Foreground window state and window title where permitted.
- Launcher child-process relationships when process ancestry and launcher hints
  support them.
- Current session, running duration, foreground duration, observation gaps, and
  stale state.
- Deterministic known-game match, possible-game state, unknown-process state, or
  permission-limited state.

### Control

Subject to platform proof, native game controls can include:

- Observe only.
- Warn child.
- Ask parent.
- Start or stop a time budget.
- Terminate an accessible process after a schema-valid policy decision.
- Block launch through OS app-control policy where the platform/setup supports
  it.
- Block package availability through platform package policy where available.
- Apply a temporary block with rollback/expiry.
- Keep exceptions for school, accessibility, therapy, esports, clubs, or parent
  overrides.

### Limits And Risks

Native game control is not magic:

- Running process time may overcount launchers, background updaters, or game
  menus.
- Foreground time may undercount or overcount in exclusive full-screen,
  multi-monitor, remote-play, overlay, or streaming scenarios.
- Renamed executables and portable installs can evade simple path/name rules.
- Games can spawn helper processes or anti-cheat services that are not the game
  title.
- Terminating games can lose unsaved data, disconnect online sessions, or create
  account penalties in some multiplayer games.
- Anti-cheat and protected-process systems can make inspection or termination
  unsupported.
- Blocking launch broadly can affect school software, accessibility tools,
  creative apps, launchers, and update services unless exceptions exist.

## Launcher And Store Games

Launchers make game attribution better, but only when their local evidence is
safe and readable.

### Observation

Launcher adapters can provide:

- Launcher kind and install status.
- Local library/install hints.
- Launcher app id or package id.
- Manifest-backed title.
- Install path hints.
- Executable path hints.
- Last manifest observation time.
- Capability status: ready, unavailable, permission-required, unsupported,
  stale, or adapter-error.

Launcher evidence should avoid:

- Private account tokens.
- Credentials.
- Purchase history unless explicitly approved later.
- Cloud saves.
- Chat messages.
- Launcher social graph.
- Raw launcher network traffic.

### Control

Launcher-aware control can support:

- Counting launcher-owned child game processes as game time.
- Treating launcher-only time separately from game-play time.
- Blocking a known game executable while leaving the launcher available.
- Asking parent when an unknown launcher child process appears.
- Reporting launcher install/library status as capability evidence.

### Limits

Launcher integrations differ widely:

- Some launchers expose local manifests; others use databases, encrypted state,
  cloud state, or per-user folders.
- Launchers and stores update their local formats.
- A game can be installed outside the launcher or moved after install.
- A launcher can start external anti-cheat, overlay, crash handler, or updater
  processes.
- Store metadata can lag installed state.

Therefore the policy must keep launcher confidence and process proof separate.

## Browser Games And Cloud Games

Browser games and cloud games should not be forced into native-game assumptions.

### Browser Games

Browser games are governed by browser evidence:

- Exact URL/title/domain rules require managed browser proof.
- Network/domain-only browser game classification is weaker.
- Site-category or local AI classification can support reports and policy only
  when it references stored browser evidence.
- Time budgets can count browser game time only according to the configured
  proof level: managed active tab, managed tab list, browser foreground, domain
  flow, or report-only.

If the browser is unmanaged, Ocentra should report browser/game bypass evidence
instead of claiming exact game title.

### Cloud Games

Cloud game controls depend on the client:

- Native cloud client: process/window/session proof can count client usage.
- Managed browser cloud game: managed URL/title/domain can support service or
  title rules when exposed.
- Console cloud game: only console platform controls can enforce.
- Network-only cloud traffic: useful as a destination/service indicator, not
  exact play proof.

Cloud services may expose a game catalog, but Ocentra should not infer the exact
played title from a service domain unless the adapter proves it.

## Game Categories And Age Ratings

Game rules often need categories, maturity ratings, or content descriptors.
These should be represented as evidence and parent-rule targets, not hidden
product decisions.

Possible category sources:

- Deterministic local catalog.
- Store or launcher metadata.
- ESRB, PEGI, IARC, CERO, USK, ACB, or other regional rating authority data.
- Parent-authored household category.
- Local classifier output over stored evidence digest for unknown games.

Important limits:

- Rating systems differ by country or store.
- Ratings may not exist for mods, private builds, web games, prototypes, or
  sideloaded games.
- Store category is not the same as real-time in-game behavior.
- Multiplayer, chat, user-generated content, loot boxes, spending, and online
  interaction may be represented as separate descriptors where the source
  provides them.
- Parent choices should be explicit. A category label alone should not enforce
  without a matching parent-authored rule.

## Multiplayer, Chat, And Social Limits

Ocentra should distinguish game usage from communication content.

Can represent:

- Game process or package is active.
- Known service/launcher is active.
- Parent rule says multiplayer-capable games need approval.
- Platform/store metadata says a title has interactive elements where a trusted
  rating source provides that descriptor.
- Console or platform family settings may restrict communication inside that
  platform.

Cannot represent by default:

- Text chat content.
- Voice chat content.
- Friends list.
- Lobby membership.
- Match participants.
- Moderation state.
- Private messages.
- In-game purchases or inventory.

Those would require explicit future product/legal/security approval and a
platform-specific integration. The current game-control posture should use
category/rating/interactive-descriptor evidence and parent approvals rather than
message surveillance.

## Enforcement Actions

### Launch

The product can launch a managed or approved game only if a future product path
chooses that behavior. For now, game controls should focus on observation,
approval, time budgets, and enforcement handoff. Any launch action needs:

- Parent-authored allow/approval.
- Known target identity.
- Platform launch mechanism.
- Audit record.
- No secret launcher/account handling in app source.

### Block Launch

Launch blocking can mean different things:

- OS app-control rule for an executable or publisher.
- Package suspension or hide policy.
- Launcher-specific block, if a supported launcher integration exists.
- Browser rule for browser games.
- Console platform family setting.

The UI must show which mechanism is active. "Blocked" should never hide that
the current platform only supports observe or manual-required state.

### Terminate

Terminate means the child-device agent attempts to stop the running process
after a typed policy decision says to enforce.

Termination must record:

- Target process id and identity at decision time.
- Recheck that the target has not changed.
- Adapter mechanism.
- Result: terminated, already-exited, permission-limited, protected,
  target-changed, failed, unavailable, or observe-only.
- Timer/approval/override state.
- Evidence references.

Hard termination can have user impact. Parent UI should support warn-first,
grace period, ask-parent, and time-extension options where policy allows.

### Time Limit

Time limits can count:

- Foreground game time.
- Running game process time.
- Launcher child-game time.
- Known-game category time.
- Possibly-game time.
- Browser game time with managed proof.
- Cloud client time.
- Platform-reported screen/game time where the platform provides it.

The policy should name the counting mode. If title proof is unavailable, the
timer should degrade to category, possible-game, unknown-game, or report-only
according to parent settings.

### Schedule And Budget

Schedules and budgets should support:

- Daily minutes.
- Weekly minutes.
- School hours.
- Bedtime.
- Weekend differences.
- Grace periods.
- Warning thresholds.
- One-time extension.
- Session extension.
- Parent approval expiry.
- Reset behavior.

All timer decisions need journaled recovery so the child-device agent can
survive restart without silently losing active limits.

## Parent Approval

Parent approval is a typed policy path, not a portal-only popup.

Approval requests should include:

- Child profile.
- Device.
- Game or game candidate.
- Known/possible/unknown state.
- Requested action: launch, continue, extend, unblock, allow once, allow
  session, allow until time, or deny.
- Evidence refs.
- Timer state.
- Expiry.
- Local/LAN/cloud route and custody label.

If the parent is unreachable, the child-device agent must follow a deterministic
parent-authored fallback such as deny, allow temporarily, continue observe-only,
or keep waiting.

## Reports, Custody, Retention, And Audit

Game reports must carry proof and custody labels.

Parent-visible report fields can include:

- Installed/detectable games.
- Running now.
- Foreground now.
- Recent sessions.
- Daily/weekly rollups.
- Game category or rating source.
- Unknown and possible-game candidates.
- Launcher status.
- Policy decisions.
- Time-budget state.
- Approval requests and responses.
- Enforcement results.
- Capability status and unavailable reasons.

Retention should separate:

- Raw process/window evidence.
- Inventory snapshots.
- Launcher hints.
- Session summaries.
- Policy audit.
- Enforcement audit.
- Parent approval history.
- Redacted reports.

Custody labels should distinguish child-local, LAN-live, parent-cache,
parent-export, parent-report, parent-owned storage, hosted non-activity
metadata, unavailable, and stale. Ocentra-hosted services must not become the
default store for raw game activity evidence.

## Platform Capability Notes

### Windows

Windows is the strongest first target for local desktop game controls.

Likely capability layers:

- Process and window observation.
- Foreground app/window evidence.
- Installed app inventory through uninstall records, shortcuts, known folders,
  package inventory, and launcher manifests.
- Executable path, hash, publisher, and signature status where available.
- Microsoft Store package inventory where the platform API exposes it.
- Owned-process termination where the target is accessible and the adapter
  proves the result.
- App control through AppLocker, App Control for Business, WDAC, or similar
  mechanisms where edition/setup permits.
- Firewall, DNS, proxy, or WFP-style network controls for service/domain
  fallback where a separate network policy exists.

Windows caveats:

- Some controls require admin rights, service installation, policy setup, or
  specific Windows editions.
- Protected processes and anti-cheat processes may reject access or
  termination.
- Broad blocking is manual-required until proven on the host OS.
- Product claims should follow real host proof, not just contract presence.

### macOS

macOS can support app/process observation and some management paths, but parity
requires separate proof.

Possible layers:

- Process/app observation with appropriate permissions.
- App inventory and bundle metadata.
- Managed app restrictions through MDM or configuration profiles where deployed.
- Network Extension or content filter paths where entitled and approved.

Caveats:

- TCC, System Extensions, Endpoint Security, Network Extensions, MDM posture,
  signing, notarization, and entitlements matter.
- Do not assume Windows process control maps directly to macOS.

### Linux

Linux can support process and package observation, but implementation varies by
distro, desktop environment, package manager, privilege model, and compositor.

Possible layers:

- Process observation.
- Package manager inventory.
- Desktop window/focus observation where the environment exposes it.
- Launcher manifests where readable.
- Firewall, DNS, proxy, nftables, or iptables style controls.

Caveats:

- Foreground-window proof varies across X11, Wayland, desktop environment, and
  sandbox model.
- Package and launcher paths vary.
- Blocking requires distro-specific proof and rollback.

### Android

Android game controls depend heavily on whether Ocentra is device owner,
profile owner, has package-access delegation, has usage-stats/accessibility
permissions, or is only a normal app.

Possible stronger-management layers:

- DevicePolicyManager package suspension or hiding for installed packages.
- Uninstall blocking for managed packages where allowed.
- Usage stats or accessibility-based foreground visibility, when explicitly
  approved and enabled.
- Always-on VPN with lockdown for network mediation.
- Managed Google Play or Android Enterprise configuration for managed devices.

Limits:

- A normal Android app cannot broadly terminate or block arbitrary games like a
  desktop service.
- Package suspension and hiding require device/profile owner or delegated
  authority.
- Exact in-game state, chat, or title inside cloud/browser surfaces is not
  generally available.
- Mobile child-agent claims must stay manual-required until real device proof
  exists.

### iOS And iPadOS

iOS and iPadOS are constrained and privacy-preserving.

Possible Apple-approved layers:

- Family Controls selection for applications, categories, and web domains.
- Device Activity schedules, thresholds, and reports for applications,
  categories, and web domains.
- Managed Settings shields for applications, application categories, web
  domains, and web-domain categories.
- App and category tokens rather than raw unrestricted app inventory in normal
  flows.
- MDM/supervised-device restrictions where a separate managed-device deployment
  exists.

Limits:

- Third-party apps do not get desktop-like process control.
- Screen Time APIs are token-based and privacy-preserving.
- Entitlements, App Store review, Family Controls authorization, and device
  family setup affect what is shippable.
- Ocentra should not claim iOS game control until entitlement and device proof
  exist.

### Consoles

Consoles should be treated as external platform family-control systems unless
Ocentra later builds an approved integration.

Xbox:

- Microsoft Family Safety supports app/game limits across connected Windows,
  Xbox, and Android devices.
- Xbox Family Settings can apply screen time and game-related restrictions for
  accounts in a family group.
- Ocentra should not claim direct Xbox console control from the local desktop
  agent.

PlayStation:

- PlayStation family controls include playtime controls, age levels for games,
  communication/user-generated content restrictions, spending limits, and
  allowed-games exceptions.
- Ocentra should treat PlayStation state as platform-account/platform-console
  evidence only if an approved integration exists.

Nintendo Switch:

- Nintendo Switch Parental Controls can set play-time limits, bedtime, suspend
  software when time is up, age-rating restrictions, and some communication
  restrictions.
- Some limits are console-level rather than per-user.
- Ocentra should not claim direct Switch control from the local desktop agent.

## Policy Modes To Represent Later In UI

### Observe Game Use

What it means:

- Detect installed/detectable games where inventory exists.
- Show running and foreground state.
- Show launcher status.
- Show known, possible, unknown, stale, unsupported, and permission-limited
  states.
- Show reports and evidence refs.

Does not provide:

- Guaranteed blocking.
- Exact title for every process.
- In-game content, chat, or multiplayer proof.

### Time-Limit Games

What it means:

- Count a configured duration mode.
- Warn before limit.
- Ask or block when limit is reached.
- Journal timer state and recovery.

Requires:

- Session evidence.
- Time-budget policy.
- Child-agent timer ownership.
- Deterministic fallback when evidence is stale.

### Ask Parent For New Or Unknown Games

What it means:

- Known blocked games, unknown game-like processes, or new launcher game
  candidates can create approval requests.
- Parent can approve once, approve session, approve until time, deny, or extend
  time.

Requires:

- Approval protocol.
- Evidence refs.
- Expiry/fallback behavior.

### Block Or Terminate Games

What it means:

- A schema-valid policy decision can trigger a platform adapter.
- The adapter attempts a scoped action and writes a result.

Requires:

- Platform capability proof.
- Target recheck.
- Audit.
- Rollback/expiry for temporary blocks.

Risks:

- Unsaved progress loss.
- Online disconnection.
- Permission/protected-process failure.
- False positives for game-like creative, school, or accessibility apps.

### Platform Family Controls

What it means:

- Use Android, iOS, Xbox, PlayStation, Nintendo, or store/platform family-control
  paths when those platforms expose the right controls and Ocentra has a
  legitimate integration.

Not enough for:

- Claiming the Windows child-device agent controls consoles.
- Claiming desktop-level process control on mobile.
- Claiming platform-account data without approved APIs and custody.

## Current Ocentra Parent Posture

Current repository direction already models this split:

- App/game evidence contracts represent installed/detectable inventory,
  running state, foreground sessions, duration rollups, launcher hints, known
  games, possible games, unknown processes, stale states, and evidence refs.
- Browser games belong to the managed/unmanaged browser boundary unless the
  game-control policy explicitly counts browser evidence as game time.
- Policy consumes typed app/game session summaries and evidence refs. It does
  not scan the OS.
- AI may classify unknown or ambiguous app/game evidence only from stored
  evidence or agent-generated digests.
- V0.8 has proof-backed enforcement spine work and a narrow owned-process app
  time-limit direction, but broad app/domain/browser blocking is still
  manual-required until real adapters prove it.
- Enforcement actions such as terminate, block launch, time-limit, or
  ask-parent must remain capability-gated and audit-gated.

Relevant local docs:

- [`docs/architecture/app-game-evidence-sessions.md`](architecture/app-game-evidence-sessions.md)
- [`docs/expectations/app-game-evidence.md`](expectations/app-game-evidence.md)
- [`docs/expectations/policy.md`](expectations/policy.md)
- [`docs/expectations/enforcement.md`](expectations/enforcement.md)
- [`docs/product-roadmap.md`](product-roadmap.md)
- [`docs/managed-unmanaged-browser.md`](../../../plans/browser-plan/workpacks/managed-unmanaged-browser.md)

## Future UI Rules

The Game UI should eventually make these distinctions visible:

- Show exact title rules only when deterministic game/package/launcher evidence
  exists.
- Show possible-game and unknown-game rules as lower-confidence policy targets.
- Show browser games with browser proof level, not native-game proof.
- Show cloud games with client/browser/platform proof level.
- Show launcher status separately from game-play status.
- Show game ratings and categories with source and region.
- Show multiplayer/chat restrictions as platform or metadata limitations, not
  chat-content capture.
- Keep capability status close to each action: ready, unsupported,
  permission-required, protected-process, anti-cheat-limited, launcher-missing,
  manifest-unreadable, stale, adapter-error, disabled-by-parent, monitor-only,
  manual-required, or unavailable.
- Every strict action should have an audit path: detected state, parent rule,
  mechanism, outcome, timestamp, evidence ref, and custody label.

The parent should be able to choose policy posture with informed tradeoffs:

- observe only;
- time-limit known games;
- count possible games separately;
- ask parent for new or unknown games;
- warn before bedtime or school-hours play;
- terminate after a grace period;
- block launch where the platform proves it;
- apply platform family controls where a platform integration exists;
- combine game controls with browser and network fallback.

## Source References

External capability references:

- [Windows process snapshot API](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
- [Windows foreground window API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
- [Windows process security and protected process access limits](https://learn.microsoft.com/en-us/windows/win32/procthread/process-security-and-access-rights)
- [Windows AppLocker](https://learn.microsoft.com/en-us/windows/configuration/lock-down-windows-10-applocker)
- [Windows PackageManager.FindPackagesForUser](https://learn.microsoft.com/en-us/uwp/api/windows.management.deployment.packagemanager.findpackagesforuser)
- [Microsoft Family Safety app and game limits](https://support.microsoft.com/en-us/account-billing/set-app-and-game-limits-a45e2d2d-4b55-a320-c8e5-daf610447a05)
- [Microsoft Family Safety screen time limits](https://support.microsoft.com/en-us/account-billing/set-screen-time-limits-across-devices-a593d725-fc4c-044c-284d-32eab0305ffd)
- [Xbox Family Settings app](https://www.xbox.com/en-US/apps/family-settings-app)
- [Android DevicePolicyManager](https://developer.android.com/reference/android/app/admin/DevicePolicyManager)
- [Apple Family Controls](https://developer.apple.com/documentation/familycontrols)
- [Apple FamilyActivitySelection](https://developer.apple.com/documentation/FamilyControls/FamilyActivitySelection)
- [Apple DeviceActivityEvent applications/categories/webDomains threshold](https://developer.apple.com/documentation/deviceactivity/deviceactivityevent/init%28applications%3Acategories%3Awebdomains%3Athreshold%3A%29)
- [Apple Managed Settings ShieldSettings](https://developer.apple.com/documentation/managedsettings/shieldsettings)
- [Steamworks builds and manifests](https://partner.steamgames.com/doc/store/application/builds)
- [Steamworks install scripts](https://partner.steamgames.com/doc/sdk/installscripts?l=english)
- [ESRB ratings guide](https://www.esrb.org/ratings-guide/)
- [PlayStation parental controls](https://www.playstation.com/en-us/support/account/ps5-parental-controls-spending-limits/)
- [PlayStation playtime controls](https://www.playstation.com/en-ie/support/account/play-time-controls-playstation/)
- [Nintendo Switch Parental Controls app](https://www.nintendo.com/mobile-apps/parental-controls/)
- [Nintendo Switch parental controls setup](https://en-americas-support.nintendo.com/app/answers/detail/a_id/22447/p/989)
