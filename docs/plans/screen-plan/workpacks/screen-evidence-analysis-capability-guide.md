<!-- agent-capsule -->

> Agent Capsule
> Doc: Screen Evidence Analysis Capability Guide
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Screen Evidence Analysis Capability Guide

Status: product capability guide for future portal UI, policy authoring, local AI, and parent guidance.

This document explains what Ocentra Parent can and cannot know or control from screenshots, screen
recording, OCR, and local screen analysis. It is meant to feed later Policy, AI, Screen Evidence, and
Reports UI work, especially parent-facing guide sections where a parent chooses between no screen
analysis, local screenshot analysis, event-triggered analysis, stronger disclosure, and policy use.

This is not a moral policy document. The product should expose real capability boundaries and let the
parent choose the household rule posture. The important engineering rule is that the UI must not imply
screen visibility, content understanding, or capture retention that the child-device agent cannot prove
through the selected platform adapter, local analysis queue, and validated evidence summary.

## Core Terms

### Screen Evidence

Screen evidence is any local observation derived from visible pixels on a child device. It may come from a
single screenshot, a short capture frame, a bounded recording segment, a managed browser/window capture,
or a local OCR/vision result.

Screen evidence is high-sensitivity. The normal Ocentra path is not "save screenshots." The normal path
is:

- parent-enabled setting;
- platform capability and permission check;
- local capture of the approved scope;
- encrypted temporary queue storage;
- local OCR/vision analysis;
- schema-valid summary, category candidates, risk signals, confidence, evidence refs, digest, and deletion
  state;
- deletion of the raw image or frame data.

### Screenshot

A screenshot is a still image captured from an approved scope at one point in time.

Possible scopes:

- full screen or display;
- active window;
- managed browser window;
- selected app window;
- protected or unsupported scope represented as unavailable, not captured.

A screenshot can support visible category or risk classification after local analysis. It does not prove
duration, intent, exact browser URL, network destination, or full content history by itself.

### Screen Recording

Screen recording is a stream or sequence of frames captured over time. It can provide stronger timing and
transition evidence than a screenshot, but it is more sensitive and more expensive.

Ocentra should not treat screen recording as the default product path. If recording is ever enabled, it
needs a stricter parent opt-in, child-facing disclosure, short retention, strong custody labels, explicit
reason codes, and a separate proof record. For the current screen-evidence direction, still screenshots or
single-frame captures feeding an encrypted temporary local queue are the safer default.

### Managed Window Capture

Managed window capture means the child-device agent can capture only a window that belongs to an approved
managed boundary, such as an Ocentra-managed browser session or selected application window.

Managed window capture can reduce data scope compared with full-screen capture. It still cannot claim
exact browser URL or tab state unless the managed browser evidence layer proves those fields separately.

### OCR

OCR is local text extraction from a captured image or frame. OCR may support:

- visible text snippets;
- text category hints;
- credential-like text redaction;
- unsafe phrase or bypass-tool signals;
- policy explanation references.

OCR must be bounded. Snippets should have a maximum count and length, and disabling OCR snippets must still
allow image-category-only analysis. OCR text is not a transcript of everything the child did.

### Image Classification

Image classification is local vision analysis over a captured image or frame. It may produce visible
activity category candidates, risk signals, confidence values, and uncertainty reasons.

Image classification is evidence, not authority. It must not enforce by itself. Policy can act only after
validated screen summaries, evidence refs, parent rules, and deterministic policy decisions agree that the
result is eligible.

### Evidence Reference

An evidence reference is a stable pointer to stored local evidence or lifecycle state. Screen analysis
should cite:

- queue job lifecycle;
- image digest;
- deletion state;
- foreground app/window evidence;
- managed browser URL/tab evidence where available;
- app/game session evidence;
- network digest evidence;
- local model/runtime status;
- parent setting version;
- policy or AI decision that consumed the summary.

Screen summaries should strengthen other evidence; they should not replace browser, app/game, network, or
policy contracts.

## The Main Capability Truth

Screen evidence can help classify what was visibly on the device, but it is a high-sensitivity local
analysis layer, disabled by default, with no raw capture retention by default.

The product can reasonably claim screen-derived understanding only when all of these are true:

- the parent explicitly enabled screen analysis for the child/device/schedule;
- the current platform adapter supports the requested scope;
- required OS permission or management state is present;
- protected surfaces are skipped or represented as unavailable;
- the raw image or frame is stored only in an encrypted temporary local queue;
- local OCR/vision returns schema-valid output;
- confidence, category, risk signal, redaction, custody, and deletion states validate;
- the stored long-lived evidence is a summary plus refs, not raw pixels;
- policy and enforcement consume only typed summaries and evidence refs.

The product must not claim:

- hidden capture;
- cloud/API AI screenshot processing by default;
- permanent screenshot history by default;
- exact page, URL, chat, password, or intent from pixels alone;
- platform parity before real OS/device proof;
- enforcement from raw model text or unvalidated image classification.

## Capability Matrix

| Capability                 | Full screen/display                                 | Active window                                         | Managed browser/window                        | Local OCR/vision                         | Important limit                                                                   |
| -------------------------- | --------------------------------------------------- | ----------------------------------------------------- | --------------------------------------------- | ---------------------------------------- | --------------------------------------------------------------------------------- |
| Capture a still image      | Possible on desktop platforms with permission/proof | Possible where OS exposes window capture              | Possible if the managed boundary is active    | Input only after capture                 | Must skip protected/locked/permission-required states.                            |
| Capture a recording stream | Possible but high-sensitivity                       | Possible where OS supports selected window/app stream | Possible if managed scope is selected         | Usually sampled into frames or summaries | Not default; needs stronger opt-in and proof.                                     |
| Classify visible activity  | Broad but sensitive                                 | Narrower and usually more relevant                    | Narrowest for web/app context                 | Yes, with confidence                     | Category is not policy authority.                                                 |
| Extract OCR snippets       | Possible                                            | Possible                                              | Possible                                      | Yes, if enabled                          | Snippets must be bounded and redacted.                                            |
| Prove exact URL            | No                                                  | No                                                    | Only if browser evidence proves it separately | No                                       | Pixels can show text that looks like a URL, but that is not managed tab proof.    |
| Prove app/window context   | Correlate with foreground evidence                  | Stronger when captured source is a window             | Strong if managed session/window id is linked | No by itself                             | Capture source ids must be recorded.                                              |
| Prove duration             | No, single point in time                            | No, single point in time                              | No, single point in time                      | No                                       | Duration belongs to app/game/window/session evidence or recording-specific proof. |
| Detect protected surfaces  | Platform dependent                                  | Platform dependent                                    | Platform dependent                            | Not after the fact reliably              | Protected/secure/credential states must fail closed.                              |
| Feed local AI              | Summary/ref only by default                         | Summary/ref only by default                           | Summary/ref only by default                   | Yes after schema validation              | Raw image is not normal AI context-builder input.                                 |
| Feed policy                | Only via summary/ref                                | Only via summary/ref                                  | Only via summary/ref                          | Yes, after validation                    | Requires parent rule and confidence threshold.                                    |
| Feed enforcement           | Not directly                                        | Not directly                                          | Not directly                                  | Not directly                             | Enforcement requires typed policy decision and audit.                             |
| Show parent report         | Summary, confidence, refs, deletion state           | Same                                                  | Same                                          | Same                                     | Raw screenshot hidden by default.                                                 |
| Retain raw capture         | No by default                                       | No by default                                         | No by default                                 | No by default                            | Future retention needs separate custody/legal/privacy design.                     |

## Screenshot Possibilities And Limits

### What Is Possible

A screenshot-based feature can support:

- visible activity categories such as school, video, chat, game, shopping, productivity, adult content,
  violence, bypass tool, unknown, or low confidence;
- OCR snippets when parent settings allow them and local redaction permits storage;
- risk signals such as possible credential prompt, explicit content signal, bypass tool, unsafe visible
  content, self-harm signal, suspicious login, or unknown;
- correlation with foreground app, active window title, managed browser state, app/game session, and
  network digest refs;
- one-time manual parent test capture during setup;
- cadence capture with conservative intervals;
- event-triggered capture after foreground app change, managed URL change, app/game foreground start,
  unusual network digest, policy ambiguity, or local AI uncertainty;
- queue lifecycle and deletion proof.

### What Is Not Reliable

A screenshot cannot reliably prove:

- exact active browser URL unless managed browser evidence proves it;
- what the child typed before or after the frame;
- duration of use;
- whether visible text came from a webpage, chat, image, ad, overlay, or stale window;
- hidden background tabs or background apps;
- decrypted network content;
- app identity without OS process/window correlation;
- parent policy outcome without a typed parent rule.

## Screen Recording Possibilities And Limits

Screen recording can produce stronger temporal evidence than screenshots, but it changes the privacy and
custody profile. It should be treated as a separate, stricter mode.

Possible recording uses:

- short parent test session during setup;
- short rolling local analysis buffer that never becomes a retained video archive;
- event-triggered frame sampling for transitions;
- accessibility-like visible flow analysis where the platform permits it.

Limits and risks:

- recording creates more raw sensitive data than screenshots;
- storage, deletion, and failure handling are harder to prove;
- child-facing disclosure must be clearer;
- bandwidth, CPU/GPU, battery, and model runtime load are higher;
- protected media and secure surfaces may appear black, unavailable, omitted, or blocked depending on OS;
- platform consent prompts and indicators are common and must not be bypassed;
- iOS and Android have especially strong user-consent and OS-policy limits.

For current Ocentra planning, screen recording should stay authoring-only or manual-required until a
specific product slice proves local-only recording, short TTL, deletion, disclosure, and policy boundaries.

## Managed Browser Or Window Capture

Managed browser/window capture can reduce data scope compared with full-screen capture:

- capture only the Ocentra-managed browser window;
- capture only an approved app window;
- exclude the Ocentra app window where the platform supports exclusion filters;
- correlate capture with managed session id, window id, process id, and evidence refs.

Important limits:

- a managed browser screenshot is still not the source of exact URL truth;
- exact URL and title require browser evidence from CDP, extension, browser policy, or another approved
  browser integration;
- window capture can miss popups, overlays, system prompts, or secondary windows;
- window title and visible pixels can be stale or misleading;
- active window capture can break on virtual desktops, minimized windows, DRM/protected content, or
  permission changes.

## OCR And Image Classification

OCR and image classification should run locally on the child device for normal safety decisions.

Expected OCR constraints:

- disabled unless parent enables snippet storage or local analysis needs transient text;
- bounded snippet count and character length;
- local redaction before journal/report storage;
- sensitive tokens, passwords, credential-like text, payment data, and secrets redacted or skipped;
- OCR-disabled state represented explicitly;
- unsupported language, low resolution, or low confidence represented as unknown/degraded.

Expected image classification constraints:

- enum-backed categories and risk signals, not open-ended model prose;
- confidence in `0..1`;
- model/runtime ref and prompt/template version;
- uncertainty reason;
- source evidence refs;
- policy eligibility flag;
- invalid output rejection before storage or policy use.

## Local AI Queue And Storage

The local screen-analysis queue is the custody boundary for raw images.

Required behavior:

1. Capture scheduler creates a bounded job only when parent settings and capability state allow it.
2. Queue writer encrypts the image before durable storage.
3. Queue metadata records TTL, retry budget, scope, reason, evidence refs, digest, and deletion-required
   state.
4. Local OCR/vision worker reads the queue job and returns structured JSON only.
5. Result mapper validates categories, confidence, snippets, redaction, evidence refs, and deletion state.
6. Journal and SQLite store the validated summary and lifecycle state, not raw pixels.
7. Temporary images delete after success, invalid output, failure expiry, or TTL expiry.
8. Startup cleanup deletes expired or delete-pending queue jobs before capture resumes.

Queue jobs are not long-term evidence. The long-term evidence is the typed summary plus evidence refs,
digest, model/runtime ref, custody, and deletion/audit state.

## Triggers And Scheduling

Parent settings should decide whether cadence and trigger capture are allowed.

Potential cadence settings:

- disabled by default;
- conservative interval such as several minutes;
- stricter shorter interval only when explicitly enabled;
- schedule-aware capture windows;
- pause during sleep, lock, protected surface, permission-required state, or battery/resource pressure.

Potential triggers:

- foreground app change;
- active window change;
- managed browser URL change;
- app/game foreground start;
- unusual network digest;
- policy ambiguity;
- local AI uncertainty;
- child ask-parent flow;
- manual parent setup/test capture.

Trigger rules must debounce. Foreground, browser, app/game, and network changes must not flood the queue or
create a silent screenshot archive.

## Redaction And Minimization

Redaction must happen before parent-visible summaries, copy/debug output, reports, exports, or optional
assistant use.

Expected redaction behavior:

- no raw screenshot shown by default;
- no raw local file paths in portal copy/debug output;
- no encrypted image refs outside the child agent;
- OCR snippets bounded and redacted;
- credential-like text, passwords, tokens, payment fields, private keys, recovery codes, and session values
  redacted or omitted;
- protected regions skipped where the platform or local detector can identify them;
- uncertain redaction state degrades policy eligibility.

If redaction fails, the result should be invalid, unavailable, or summary-only, not parent-visible raw
content.

## Retention And Custody

Default retention posture:

- raw image/frame: encrypted temporary queue only, deleted after success or TTL expiry;
- stored summary: local journal and SQLite query store;
- parent report: summary, refs, confidence, custody label, and deletion state;
- parent cache/export: explicit parent-owned destination only;
- Ocentra-hosted storage: no child screen activity by default.

Every screen-related object should carry a custody label:

- `child-device-temp-queue`;
- `child-device-journal`;
- `child-device-query-store`;
- `live-local-child-agent`;
- `live-lan-child-agent`;
- `parent-device-cache`;
- `parent-owned-export`;
- `ocentra-hosted-non-activity`;
- `unavailable`.

`ocentra-hosted-non-activity` can describe account, entitlement, release, route, or notification metadata.
It must not be accepted as child screen-activity evidence.

## Permission-Required And Unavailable States

Screen analysis needs explicit capability state before capture attempts:

- disabled by parent;
- unsupported platform;
- unsupported scope;
- permission required;
- permission denied;
- permission limited;
- protected surface;
- screen locked;
- session unavailable;
- model unavailable;
- queue unavailable;
- redaction unavailable;
- degraded;
- adapter error;
- ready.

Capability status is not a successful screen observation. The UI should show these states close to the
setting or action they affect.

## Platform Capability Notes

### Windows

Windows is the first realistic desktop target for Ocentra screen analysis.

Likely capability layers:

- Windows Graphics Capture for display or application-window capture with system UI consent;
- screenshot or frame capture from an approved display/window scope;
- foreground process/window evidence from the Rust agent;
- local OCR through Windows OCR APIs or another local model boundary where packaged/available;
- local vision classification through an Ocentra-owned local model/provider boundary;
- encrypted temporary queue and journal/SQLite read model;
- managed browser/window correlation through managed browser evidence and process/window refs.

Windows caveats:

- capture support must be checked at runtime;
- consent, notification border, packaged app identity, service/session boundaries, and user desktop state
  affect what can be captured;
- secure desktop, lock screen, UAC prompts, credential surfaces, protected media, or DRM-protected content
  must be skipped or represented as protected/unavailable;
- service capture from a non-interactive session is not the same as user desktop capture;
- product claims should follow real host proof, not contract presence.

### macOS

macOS can support desktop, app, and window capture through approved screen-capture APIs, but user permission
is central.

Possible layers:

- ScreenCaptureKit display/app/window streams;
- macOS Screen Recording permission;
- local Vision framework OCR/classification or Ocentra local model boundary;
- process/window correlation where permissions and APIs allow;
- encrypted local queue and summary storage.

Caveats:

- Screen Recording permission, app restart after first grant, sandboxing, app bundle identity, TCC state,
  and signing/notarization affect real behavior;
- protected windows or windows that opt out of sharing may be unavailable or omitted;
- macOS parity requires real host proof, not package scaffold proof.

### Linux

Linux capture depends heavily on display server, desktop environment, portal backend, PipeWire, packaging,
and permissions.

Possible layers:

- XDG Desktop Portal ScreenCast for monitors, windows, or virtual sources where a portal backend supports
  them;
- PipeWire stream capture on Wayland-backed desktops;
- X11 screenshot paths where still supported;
- local OCR/vision through Ocentra-owned local model/provider boundary;
- process/window correlation depending on desktop environment.

Caveats:

- Wayland commonly requires a portal-mediated user selection flow;
- available source types vary by backend;
- restore/persistent permission behavior differs across desktop portals;
- foreground-window proof varies by compositor and desktop environment;
- Linux support needs distro/backend-specific proof.

### Android

Android screen capture depends on OS user consent and device management posture.

Possible layers:

- MediaProjection for screen or, on modern Android, selected app-window sharing;
- foreground service requirements for active capture;
- UsageStats, accessibility, VPN/DNS, device owner, or managed profile only where explicitly approved and
  enabled;
- on-device ML Kit or Ocentra local model boundary for OCR/image labeling where allowed;
- package lifecycle and policy state from DevicePolicyManager where device-owner/profile-owner setup
  exists.

Limits:

- MediaProjection requires user consent and can be revoked;
- Android 14 app-window sharing can restrict capture to a selected app and exclude system UI;
- normal apps cannot silently monitor arbitrary screen content in the background as a parental-control
  agent;
- device-owner/profile-owner state changes what is possible;
- screen capture may be disabled by policy or protected by app/window flags;
- Play policy and user disclosure constraints matter.

### iOS And iPadOS

iOS and iPadOS are the most constrained child-device targets for screen capture.

Possible Apple-approved layers:

- Screen Time frameworks: Family Controls, Managed Settings, Device Activity;
- ReplayKit for user-initiated app/screen recording or broadcasting flows;
- Managed Settings shields and Device Activity schedules/events;
- web-domain and app/category usage controls through Screen Time tokens.

Limits:

- third-party parental-control apps should not claim arbitrary hidden screenshot or screen-recording access;
- ReplayKit is user-consent oriented and not a stealth child-monitoring API;
- Screen Time APIs are privacy-preserving and entitlement/review-gated;
- iOS child-device support should rely on approved Screen Time/Device Activity/Managed Settings paths rather
  than desktop-style pixel capture unless a specific Apple-approved capability is proven.

## App And Window Correlation

Screen evidence is stronger when it is linked to other stored facts:

- foreground process/window evidence;
- managed browser session and tab evidence;
- app/game session summary;
- network flow digest;
- local model/runtime status;
- parent rule and setting version.

Correlation must be explicit. If the source window, app, or managed browser session cannot be proven, the
result should carry `source-unknown`, `window-unknown`, or `managed-session-unlinked` states.

Do not infer:

- exact URL from window title or OCR alone;
- app duration from repeated screenshots alone;
- network destination from visible content alone;
- child intent from category alone.

## Child-Facing Disclosure

Screen analysis must be visible as a product capability when enabled.

UI and device behavior should support:

- parent setting state visible in the parent portal;
- child-facing disclosure that screen analysis may run locally;
- clear difference between observe-only, dry-run, and enforcement-eligible modes;
- current permission-required or disabled state;
- reason text for warnings, asks, blocks, or time limits;
- no hidden background capture claims.

Disclosure copy should be accurate: local screen analysis may classify visible activity, but raw screenshots
are not retained by default and child activity is not uploaded to Ocentra-hosted services by default.

## Parent Reports

Parent reports should show:

- setting state and who changed it;
- capture reason and scope;
- category candidates and confidence;
- risk signals and confidence;
- bounded OCR snippets only when enabled and redacted;
- source evidence refs;
- local model/runtime status;
- custody/source label;
- deletion state and image digest;
- policy/AI decisions that consumed the screen summary;
- unavailable, protected, permission-required, low-confidence, stale, expired, invalid, delete-pending, or
  delete-failed states.

Parent reports should not show raw screenshots by default. A future raw-image reveal or retention mode would
need a separate design with explicit parent action, child disclosure posture, custody, retention, deletion,
and audit.

## Custody And Audit

Every strict screen-related action should have an audit path:

- parent setting version and actor ref;
- capability state at capture time;
- capture reason and scope;
- queue job id and image digest;
- encryption and deletion lifecycle;
- local model/runtime ref;
- validation result;
- summary/result id;
- policy decision id;
- enforcement result if any;
- custody label;
- retention/deletion state;
- adapter errors or permission changes.

Audit must include failed, skipped, protected, unavailable, expired, invalid, and delete-failed states. A
missing screen summary can be important evidence when a parent expects the feature to run.

## Proof Requirements

Before Ocentra calls screen analysis working on a platform, the proof should show the real product path:

- parent setting enabled through typed contracts;
- child-device agent or service detects capability and permission state;
- capture occurs only inside approved scope;
- image/frame enters encrypted temp queue;
- local OCR/vision analyzes it;
- schema validation accepts/rejects output correctly;
- raw image deletes after success or TTL expiry;
- journal and SQLite expose summary/read model;
- portal renders settings, status, summary, refs, custody, and deletion state;
- no Ocentra-hosted upload happens by default;
- protected/permission-required cases are visible as unavailable, not fake success.

Hosted CI can prove contracts, queue mechanics, validation, journal replay, and unavailable states.
Privileged OS/device capture needs real Windows/macOS/Linux/Android/iOS proof before product claims call it
fully working.

## Future UI Rules

The Screen Evidence UI should eventually make these distinctions visible:

- show screen analysis disabled by default;
- show capture scope as full screen, active display, active window, managed browser/window, app window, or
  unavailable;
- show whether OCR snippet storage is enabled;
- show redaction mode and redaction failures;
- show cadence and triggers separately;
- show queue health and deletion health;
- show local model/runtime status;
- show raw capture retention as off by default;
- show capability state close to each action: ready, unsupported, permission-required, permission-limited,
  protected-surface, model-unavailable, queue-unavailable, adapter-error, degraded, disabled-by-parent, or
  manual-required;
- show policy use as observe-only, dry-run, enforcement-eligible, or disabled;
- show exact proof requirement before screen-derived rules can enforce;
- keep parent reports evidence-cited and custody-labeled.

The parent should be able to choose policy posture with informed tradeoffs:

- no screen analysis;
- local observe-only summaries;
- local policy dry-run;
- local enforcement-eligible summaries with confidence thresholds;
- manual parent test capture;
- trigger-only capture;
- cadence plus trigger capture;
- OCR snippets off or bounded/on;
- strict deletion and no raw image retention by default.

## Source References

Local planning references:

- [Local Screen Evidence Analysis Queue Architecture](architecture/local-screen-evidence-analysis-queue.md)
- [Screen Evidence Analysis Expectations](expectations/screen-evidence.md)
- [AI Feature Expectations](expectations/ai.md)
- [Policy Feature Expectations](expectations/policy.md)
- [Enforcement Feature Expectations](expectations/enforcement.md)
- [Data Custody And Local-First Expectations](expectations/data-custody.md)
- [Real Evidence Proof Expectations](expectations/real-evidence-proof.md)
- [Platform Expectations](expectations/platforms.md)
- [Ocentra Parent Product Roadmap](product-roadmap.md)

External capability references:

- [Windows screen capture](https://learn.microsoft.com/en-us/windows/apps/develop/media-authoring-processing/screen-capture)
- [Windows GraphicsCaptureAccess](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscaptureaccess)
- [Windows.Media.Ocr](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr)
- [Apple ScreenCaptureKit](https://developer.apple.com/documentation/ScreenCaptureKit)
- [Apple capturing screen content in macOS](https://developer.apple.com/documentation/screencapturekit/capturing-screen-content-in-macos)
- [Apple Vision framework](https://developer.apple.com/documentation/vision)
- [XDG Desktop Portal ScreenCast](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html)
- [Android Media Projection](https://developer.android.com/media/grow/media-projection)
- [Android capture video and audio playback](https://developer.android.com/media/platform/av-capture)
- [Android MediaProjectionManager](https://developer.android.com/reference/android/media/projection/MediaProjectionManager)
- [Google ML Kit guides](https://developers.google.com/ml-kit/guides)
- [Google ML Kit image labeling](https://developers.google.com/ml-kit/vision/image-labeling)
- [Apple ReplayKit](https://developer.apple.com/documentation/replaykit)
- [Apple ReplayKit security](https://support.apple.com/guide/security/replaykit-security-seca5fc039dd/web)
- [Apple Screen Time technology frameworks](https://developer.apple.com/documentation/ScreenTimeAPIDocumentation)
