<!-- agent-capsule -->

> Agent Capsule
> Doc: Device Location Tracking Capability Guide
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Device Location Tracking Capability Guide

Status: product capability guide for future portal UI and parent guidance.

This document explains what Ocentra Parent can and cannot know or control across
device tracking, geolocation, location history, geofences, and check-in flows. It
is meant to feed later Policy, Devices, Reports, Notifications, and mobile-agent
work where a parent chooses between local-only observation, live location, safe
arrival/departure alerts, manual check-ins, and stricter managed-device
tracking.

This is not a moral policy document. The product should expose real capability
boundaries and let the parent choose the household rule posture. The important
engineering rule is that the UI must not imply exact device location, continuous
tracking, or geofence enforcement that the child-device agent cannot prove
through the selected platform adapter.

## Core Terms

### Device Location Evidence

Device location evidence is a timestamped observation about where a device may
be, how fresh that observation is, which adapter produced it, which permission
state allowed it, and how precise it is.

Possible fields:

- Latitude, longitude, altitude, heading, speed, and bearing where exposed.
- Horizontal and vertical accuracy.
- Timestamp from the platform provider and ingest timestamp from Ocentra.
- Provider kind: GPS/GNSS, Wi-Fi, cellular, IP, Bluetooth/beacon, fused provider,
  user-entered default, or unknown.
- Permission state: denied, foreground-only, background, approximate, precise,
  reduced-accuracy, supervised/managed, or unknown.
- Device state: online, offline, low power, battery saver, airplane mode, no
  signal, service disabled, or adapter unavailable.
- Custody label: child-local, parent-device cache, LAN-live, parent-owned
  storage, Ocentra-hosted non-activity metadata, or unavailable.

The key property is not that the product has coordinates. The key property is
that Ocentra can prove the source, permission state, freshness, accuracy, and
custody of the coordinates before using them in reports, policy, notifications,
or local AI context.

### Live Tracking

Live tracking means the parent surface receives repeated fresh location updates
while the child device is online and the platform permits the child-agent app or
service to receive location updates in the required execution state.

Live tracking does not mean perfect continuous movement. Platforms throttle
background updates, users can revoke permissions, battery policy can pause work,
and desktop devices often have weak location providers.

### Location History

Location history is the durable record of prior device location evidence. In
Ocentra Parent, location history should be a child-device journal/query-store
feature by default, not a default Ocentra cloud store.

Location history can support:

- Parent reports.
- Arrival and departure audit.
- Missed check-in investigation.
- Device lost/stolen review.
- Safety explanation after an alert.
- Export/delete/retention flows.

Location history must not be silently uploaded to Ocentra-hosted storage by
default.

### Geofence

A geofence is a geographic area used for arrival, departure, or dwell detection.
Most OS APIs model it as a center point plus radius. Product language may call it
"home", "school", "practice", or "friend's house", but the runtime contract
should carry an explicit region shape, radius, schedule, proof requirement,
transition type, and fallback when the platform cannot monitor it.

### Check-In

A check-in is a child-initiated or parent-requested confirmation event. It can
include a current location sample when permission allows, but it is not the same
as continuous tracking.

Check-in is useful when:

- Background location is unavailable or not appropriate.
- The child device is on a constrained platform.
- The parent wants low-noise confirmation instead of a live map.
- The agent can send an ask/confirm prompt without tracking movement all day.

### Last Known Location

Last known location is the newest location evidence the agent can return without
forcing a fresh fix. It may be quick and battery-efficient, but it may be stale,
missing, approximate, or cleared by OS settings or service restart.

The UI must always show freshness and source. "Last seen 28 minutes ago from
Wi-Fi, approximate" is honest. "Current location" is not honest unless a fresh
sample was actually acquired.

### Location Capability State

Location capability state is the current runtime truth about whether the device
can produce the requested location evidence.

Examples:

- `ready-precise-background`
- `ready-foreground-only`
- `ready-approximate-only`
- `permission-required`
- `background-permission-required`
- `service-disabled`
- `platform-unsupported`
- `manual-required`
- `offline-last-known-only`
- `battery-throttled`
- `adapter-error`

Capability state should sit next to every strict location control in the UI.

## The Main Capability Truth

Mobile devices can often support useful live location, geofences, and check-ins,
but only with explicit OS permission, foreground/background execution support,
and honest battery/accuracy handling.

Desktop and laptop devices can often support one-time or intermittent location,
but they are frequently less precise and less reliable because they may lack GPS,
cellular radios, or always-on background execution. They often infer location
from Wi-Fi, IP, or manually configured defaults.

Across all platforms:

- Location permission is user-visible and revocable.
- Background location is a separate capability from foreground location.
- Approximate/reduced accuracy must be represented separately from precise
  location.
- Last known location is not proof of current location.
- Geofence alerts are not instant and may be throttled.
- Offline devices can only report last known location and last contact time.
- Battery saver, low-power modes, radio state, and network reachability can
  degrade tracking.
- Parent-owned storage and local/LAN operation are the normal custody boundary.
- Every strict policy or alert must carry evidence source, timestamp, accuracy,
  custody, and adapter state.

## Capability Matrix

| Capability                         | Mobile child agent                     | Desktop/laptop child agent          | Required layer                         | Important limit                                            |
| ---------------------------------- | -------------------------------------- | ----------------------------------- | -------------------------------------- | ---------------------------------------------------------- |
| One-time current location          | Yes, with permission                   | Sometimes, with permission          | OS location API                        | Fresh fix may fail indoors, offline, or with service off.  |
| Last known location                | Yes, if provider cache exists          | Sometimes                           | OS location API or local cache         | May be stale, null, approximate, or user-entered.          |
| Foreground live tracking           | Yes, while app/session active          | Sometimes, while app/service active | OS location API                        | Requires visible use or active session semantics.          |
| Background live tracking           | Platform-dependent                     | Limited                             | Background execution plus permission   | Throttled, battery-sensitive, and often entitlement-bound. |
| Geofence enter/exit                | Android/iOS with limits                | Weak or app-running-only on desktop | OS geofence/region monitoring          | Delayed events, radius limits, count limits, false exits.  |
| Dwell alerts                       | Android/iOS with limits                | Weak                                | Geofence plus dwell support            | Not immediate; large radius may be required.               |
| Check-in with current location     | Yes, with prompt and permission        | Yes, if current fix available       | Notification/prompt plus location API  | Child must respond unless automatic policy exists.         |
| Location history                   | Yes, if locally journaled              | Yes, if locally journaled           | Agent journal/query store              | Retention/custody/delete controls required.                |
| Family map                         | Yes from latest evidence               | Yes from latest evidence            | Query/read model plus map rendering    | Map must label freshness and accuracy.                     |
| Parent arrival/departure alerts    | Yes, with geofence/check-in evidence   | Limited                             | Geofence or sampled evidence           | Notifications must minimize sensitive detail.              |
| Exact continuous movement trail    | Sometimes, but expensive and sensitive | Usually no                          | Frequent location updates              | Battery, consent, and retention concerns.                  |
| Lost-device location               | OS/product-specific                    | OS/product-specific                 | OS lost mode/Find My/device management | Not a generic third-party API on every platform.           |
| Remote live location away from LAN | Future, via relay or parent storage    | Future                              | Authenticated relay/sync               | Ocentra must not become default location-history store.    |
| Enforce location-based policy      | Possible after proof                   | Limited                             | Local policy plus location evidence    | Missing proof must degrade to ask/warn/report.             |

## Live Tracking: What Is Possible

Live tracking should be modeled as a session with a clear reason, cadence,
custody, and end condition.

Possible session reasons:

- Parent opens live map.
- Child is travelling between known places.
- Child missed an expected arrival.
- Device is marked missing.
- Parent explicitly starts a temporary safety session.
- A policy rule asks for short-term verification after a geofence miss.

Possible session fields:

- Session id, child id, device id, requester, and reason code.
- Requested accuracy: approximate, precise, or best-available.
- Requested cadence: one-shot, on-change, interval, high-accuracy burst, or
  geofence-only.
- Maximum duration and auto-stop reason.
- Permission requirement and user-visible disclosure state.
- Delivery path: local, LAN, authenticated relay, parent cache, or parent-owned
  storage.
- Audit events for start, update, degrade, stop, and parent reveal.

Limits:

- OSes may throttle background updates.
- Apps may need foreground service notification, background mode, entitlement,
  device-owner state, MDM supervision, or explicit user permission.
- High accuracy increases battery use.
- Indoor GPS may fail or fall back to Wi-Fi/cell/IP estimates.
- A child can turn off device location services, revoke permission, uninstall
  the app where allowed, power off the device, or lose network.
- Live map updates should never imply that a stale point is still current.

## Location History: What Is Possible

Location history is useful when it is scoped and explainable.

Recommended evidence tiers:

- `location-point`: individual timestamped point with accuracy and provider.
- `location-summary`: derived day/trip/place summary with redacted detail.
- `geofence-transition`: arrival, departure, dwell, missed arrival, or stale
  state.
- `check-in`: child response, optional location point, note, and prompt reason.
- `device-contact`: last online, last sync, battery, and network state.
- `audit-only`: policy decision or parent action without raw coordinate.

Recommended retention posture:

- Live point stream: short retention.
- Place/geofence audit: medium retention.
- Policy/audit references: longer retention.
- Parent-exported report: parent-chosen retention.
- Raw precise trail: disabled by default unless explicitly enabled.

Location history must preserve:

- Evidence source and adapter version.
- Accuracy, freshness, and permission state.
- Custody and retention labels.
- Redaction/deletion state.
- Parent reveal/audit trail for sensitive exact coordinates.

## Device Location Permissions

Location controls should distinguish these permission states:

- No permission.
- Foreground/when-in-use permission.
- Background/always permission.
- Approximate or reduced-accuracy permission.
- Precise/full-accuracy permission.
- OS location service disabled.
- Device policy allowed, denied, or user-in-control.
- Supervised/device-owner/MDM-only state.
- Unknown or stale state.

Permission prompts are not just setup friction. They are part of the product's
truth model. If a platform requires user approval, a visible disclosure, or an
OS-managed background indicator, the UI must expose that requirement instead of
phrasing it as an Ocentra-only switch.

## Accuracy Sources And Limits

Location accuracy depends on available hardware, radios, environment, and OS
policy.

Possible provider signals:

- GPS/GNSS: best outdoors, weaker indoors, battery-sensitive.
- Wi-Fi: strong for urban/indoor approximate positioning when databases and
  nearby access points are available.
- Cellular: useful wide-area estimate on mobile devices.
- IP address: coarse and often wrong for household, VPN, carrier-grade NAT, or
  corporate networks.
- Bluetooth/beacon: local proximity, not global location.
- Manual/default location: a fallback, not current device proof.
- Fused provider: OS/provider chooses from several sources.

The product should represent:

- `accuracyMeters`
- `altitudeAccuracyMeters` when known
- `sourceKinds`
- `freshnessSeconds`
- `confidence`
- `isApproximate`
- `isPrecise`
- `isUserEnteredDefault`
- `isSimulatedOrDeveloperMode` if detectable

Do not convert accuracy into false certainty. A 3 km approximate point and a 20 m
fresh GPS fix should look different in maps, reports, notifications, and policy.

## Geofences

Geofences should be treated as coarse arrival/departure signals, not precise
boundaries.

Possible geofence rules:

- Arrived at school during a schedule.
- Left school before dismissal.
- Did not arrive at practice by a time.
- Stayed near home after bedtime.
- Notify if device leaves a travel corridor.

Runtime requirements:

- Region id, label token, latitude, longitude, radius, schedule, transition
  type, dwell duration, and expiration.
- Minimum radius and maximum count per platform.
- Proof requirement: platform geofence, sampled location, check-in, or manual.
- Fallback when geofence monitoring is unavailable.
- Debounce/noise control to avoid alert storms.

Limits:

- Geofence events can be delayed.
- Small radii can be unreliable.
- Wi-Fi, cell, and GPS availability affect transition quality.
- Some platforms wake apps for geofence events; others only work while the app is
  running.
- Geofence count limits require compile-time validation.
- Dwell events are useful to reduce alert noise but can delay notifications.

## Check-In And Safety Prompts

Check-in is the low-friction fallback when continuous tracking is unavailable,
not desired, or too sensitive.

Possible flows:

1. Parent requests check-in from the parent surface.
2. Child device receives a local notification or in-app prompt.
3. Child chooses a typed response such as safe, need help, delayed, arriving,
   leaving, or call me.
4. Agent optionally requests current location if permission allows.
5. Agent journals response, timestamp, optional coordinate, delivery state, and
   custody label.
6. Parent sees status and can escalate if unanswered.

Important details:

- A check-in response without a fresh coordinate is still useful.
- A fresh coordinate without child response should be labeled as location-only.
- Unanswered check-ins should produce a notification intent only through an
  explicit alert rule.
- Sensitive child notes should not appear in third-party notification previews.

## Device Online, Offline, And Battery State

Every map and report should separate location state from contact state.

Useful contact fields:

- Last agent heartbeat.
- Last location sample.
- Last successful parent sync.
- Battery percentage and charging state where available.
- Low-power/battery-saver state where available.
- Network type and reachability summary.
- Permission/service state.
- Pending upload count.

Fallback behavior:

- Online with permission: show fresh or actively updating state.
- Online without permission: show permission-required state and last known point.
- Offline: show last contact time and last known point.
- Low battery: reduce cadence and explain degraded state.
- Service disabled: show location-service-disabled, not "tracking failed".
- Adapter error: show degraded status with audit reference.

## Reports And Maps

Parent-facing maps should be evidence views, not surveillance theater.

Map/report rules:

- Show freshness on every point.
- Visualize accuracy radius when useful.
- Distinguish approximate from precise.
- Distinguish live, last-known, check-in, geofence, and manual/default points.
- Let parents reveal exact coordinates only when the data scope permits it.
- Keep summaries useful without requiring raw trail exposure.
- Record parent reveal, export, delete, and retention actions.

Possible summaries:

- Recent location status by child/device.
- Arrivals/departures by place.
- Missed arrival/departure alerts.
- Check-in timeline.
- Device offline and battery timeline.
- Location permission health.
- Exportable parent report with custody labels.

## Custody, Retention, And Audit

Location is sensitive child activity evidence. It must follow the local-first
custody model.

Default custody:

- Raw location evidence lives on the child device.
- Parent surfaces read local/LAN, parent cache, parent-owned storage, or cloud
  relay through typed contracts.
- Ocentra-hosted services may route minimal notification or relay metadata, but
  they must not retain raw location history by default.

Retention needs:

- Retention policy per data class.
- Delete expired raw points.
- Keep redacted summaries only if parent policy allows.
- Export/delete flows that name data classes and destinations.
- Audit for parent reveal, export, sync, delete, policy change, and strict
  alert action.

Audit fields:

- Policy decision or parent action.
- Evidence reference.
- Location source and adapter state.
- Accuracy/freshness.
- Custody label.
- Retention class.
- Notification intent reference where alerts were sent.
- Failure/degraded reason.

## Child-Facing Disclosure

Location features need visible child-device disclosure.

Recommended disclosure states:

- Location controls disabled.
- Check-in only.
- Arrival/departure alerts enabled.
- Live map temporarily active.
- Background location enabled by parent and OS permission.
- Last-known report only because device is offline.
- Location unavailable because permission/service is off.

The child-facing surface should avoid parent diagnostics, but it should not hide
that location tracking is enabled when the OS and product require disclosure.

## Missing-Proof Fallbacks

When requested proof is missing, Ocentra should degrade explicitly.

Examples:

- Current location unavailable -> show last known location with timestamp.
- Background permission missing -> offer foreground/check-in mode.
- Geofence unavailable -> use scheduled check-in or sampled location if allowed.
- Precise permission denied -> use approximate-only rules or mark precise rules
  unavailable.
- Device offline -> show last contact and queue parent request.
- Battery throttled -> reduce cadence and record degraded state.
- Platform unsupported -> show unavailable/manual-required, not a fake toggle.
- Parent relay unavailable -> continue local policy and send when reachable.

Policy should choose fallback decisions such as allow, observe, warn, ask,
manual-check-in, last-known-report-only, or unavailable. The runtime should not
guess a child's location from stale or weak evidence.

## Platform Capability Notes

### Windows

Windows can expose location through Windows location APIs and policy-controlled
app access, but it is not a mobile live-tracking platform by default.

Likely capability layers:

- `Windows.Devices.Geolocation.Geolocator` for one-time and continuous location
  where the app has permission.
- Windows location service using GPS, Wi-Fi, cell towers, and IP where available.
- Windows privacy settings and MDM/Policy CSP for whether Windows apps may
  access location.
- Wi-Fi BSSID access increasingly tied to precise-location consent.
- Child-agent service/contact state independent of location service state.

Windows caveats:

- Desktop/laptop hardware may have no GPS or cellular radio.
- Location can be approximate, IP-derived, stale, manually configured, or
  unavailable.
- A background Windows service cannot assume Store-app-style foreground consent
  semantics without implementation proof.
- Continuous tracking has battery impact and should be cadence-limited.
- Product claims need real Windows adapter proof, not only a contract.

### macOS

macOS can use Core Location, but parity requires separate proof.

Possible layers:

- Core Location authorization and location updates.
- Reduced/full accuracy state.
- Region monitoring only while the app is running and the system is awake,
  according to Apple geofence documentation.
- Find My Mac as an Apple account feature, not a generic third-party tracking
  API.
- MDM/device management posture for managed-device features where available.

Caveats:

- TCC permissions, background execution, launchd behavior, signing, and
  notarization matter.
- Do not assume Windows service behavior maps to macOS.
- Lost-device and Find My behavior should be described as OS/account feature
  context unless Ocentra has an approved API and proof.

### Linux

Linux location depends on distro, desktop environment, service availability, and
permission agent behavior.

Possible layers:

- GeoClue over D-Bus on desktops that ship/configure it.
- IP/Wi-Fi based approximate location if the service/provider is available.
- Browser geolocation with user permission for web surfaces, separate from the
  child agent.
- Manual/default location fallback.

Caveats:

- No universal Linux live-location stack exists across distros.
- Permission agents and desktop portals vary.
- Headless/service deployments may have no useful location provider.
- Product claims must name distro/service assumptions and real proof.

### Android

Android is the strongest likely mobile child-agent path, but it is permission
and policy constrained.

Possible layers:

- Foreground location with `ACCESS_COARSE_LOCATION` and/or
  `ACCESS_FINE_LOCATION`.
- Background location with `ACCESS_BACKGROUND_LOCATION` when core functionality
  and Play policy allow it.
- Approximate vs precise permission state.
- Fused Location Provider for last known, current, and periodic updates.
- Geofencing API with platform count, permission, delay, and radius limits.
- Foreground service and visible notification for long-running location use
  where required.
- Device owner/profile owner policy only after real Android Enterprise or device
  owner proof.

Limits:

- Background location is restricted and must be core to the app.
- Background updates are throttled.
- Geofencing responsiveness can be delayed.
- Approximate permission may be the only granted precision.
- Users can change permission, precision, and location service settings.
- Store policy matters for a child-agent product.

### iOS And iPadOS

iOS and iPadOS can support location features through Apple-approved APIs, but
they are restrictive and must be proof-gated.

Possible Apple-approved layers:

- Core Location When In Use and Always authorization.
- Reduced/full accuracy state.
- Standard, significant-change, visit, and region monitoring where permitted.
- Background location with required mode, authorization, and user disclosure.
- Family Sharing/Find My location sharing as an Apple user feature, not an
  Ocentra-owned raw telemetry API.
- Supervised MDM Lost Mode device location for managed/supervised devices only.

Limits:

- Third-party apps cannot silently access general Find My data.
- Always/background location has explicit prompts and disclosure requirements.
- Region monitoring has platform limits and is not instant.
- MDM device location is tied to Lost Mode and supervision requirements.
- Entitlements, App Store review, Family Controls, and device supervision affect
  what is shippable.

## Policy Modes To Represent Later In UI

The later portal UI can expose capability modes rather than pretending every
location rule is equal.

### Observe Last Known Location

What it means:

- Show newest location evidence when available.
- Show freshness, accuracy, source, permission, and custody.
- Do not run continuous tracking.

Works without:

- Background permission.
- Geofence setup.
- Live relay.

Does not provide:

- Real-time movement.
- Arrival/departure alerts unless separately enabled.

### Check-In Only

What it means:

- Parent can request a child response.
- Agent may include current location if permission allows.
- Unanswered check-ins can drive explicit notification rules.

Works with:

- Foreground prompt/notification.
- Optional location permission.

Does not guarantee:

- Child response.
- Current location when permission/service is unavailable.

### Arrival And Departure Alerts

What it means:

- Parent defines places and schedules.
- Agent records geofence or sampled evidence.
- Notifications are sent only through alert rules.

Requires:

- Geofence or sampled-location capability.
- Debounce/noise controls.
- Custody and retention settings.

Does not guarantee:

- Instant transitions.
- Small-radius precision.

### Temporary Live Map

What it means:

- Parent starts a time-limited live tracking session.
- Agent sends repeated updates while online and permitted.
- Session has a visible reason, duration, and audit trail.

Requires:

- Location permission.
- Runtime update path.
- Local/LAN/relay delivery path.
- Battery-aware cadence.

Risk:

- High sensitivity and battery impact.

### Missing Device Mode

What it means:

- Parent marks device missing.
- Agent tries to provide current or last known location.
- Device contact, battery, and network state become prominent.

Requires:

- Existing permission or platform-specific lost-device capability.
- Clear separation between Ocentra evidence and OS account features.

Does not guarantee:

- Tracking a powered-off or offline device.

### Location-Based Policy Decision

What it means:

- Location evidence can contribute to local policy, such as "ask if leaving
  school early" or "notify if not at practice by 18:00".

Requires:

- Typed policy target.
- Evidence freshness and accuracy thresholds.
- Explicit fallback when proof is stale or unavailable.
- Local child-agent evaluation.

Does not allow:

- Portal-side policy evaluation.
- Guessing current location from stale last-known evidence.

## Current Ocentra Parent Posture

Current repository direction already supports the right boundary model:

- Child-device agents own capture, journal, query, local AI, policy, and
  enforcement paths.
- Parent surfaces author rules and view reports; they do not execute child
  capture or policy.
- Raw child activity evidence is local-first by default.
- Remote relay and notifications must minimize child details.
- Android and iOS capability claims are currently scaffold/manual-required until
  real device proof exists.
- Platform claims must distinguish implemented, scaffold-only, unavailable,
  degraded, manual-required, and not-implemented states.

Device location should follow the same proof standard before any UI claims
"live", "precise", "background", "geofence", "arrival", or "history".

Relevant local docs:

- [`docs/product-roadmap.md`](product-roadmap.md)
- [`docs/expectations/data-custody.md`](expectations/data-custody.md)
- [`docs/expectations/sync-export.md`](expectations/sync-export.md)
- [`docs/expectations/platforms.md`](expectations/platforms.md)
- [`docs/expectations/policy.md`](expectations/policy.md)
- [`docs/expectations/notifications.md`](expectations/notifications.md)

## Future UI Rules

The Device Location UI should eventually make these distinctions visible:

- Show live tracking only when a fresh update path is available.
- Show last known location separately from current/live location.
- Show exact coordinate reveal separately from summary/place reporting.
- Show approximate/reduced accuracy as a first-class state.
- Show geofence alerts as delayed/coarse arrival/departure evidence.
- Show check-in as a separate workflow from tracking.
- Keep permission state close to every control.
- Keep custody, retention, and delete/export status visible for location history.
- Show child-facing disclosure mode when background/live tracking is enabled.
- Show degraded/offline/manual-required states instead of disabled-looking
  mystery failures.
- Every strict action should have an audit path: detected state, parent rule,
  mechanism, outcome, timestamp, accuracy, freshness, custody, and evidence
  reference.

The parent should be able to choose policy posture with informed tradeoffs:

- last-known only;
- check-in only;
- geofence arrival/departure;
- temporary live map;
- missing device mode;
- location-based policy alerts;
- parent-owned history/report export.

## Source References

External capability references:

- [Android request location permissions](https://developer.android.com/develop/sensors-and-location/location/permissions)
- [Android access location in the background](https://developer.android.com/develop/sensors-and-location/location/background)
- [Android get the last known location](https://developer.android.com/develop/sensors-and-location/location/retrieve-current)
- [Android request location updates](https://developer.android.com/develop/sensors-and-location/location/request-updates)
- [Android create and monitor geofences](https://developer.android.com/develop/sensors-and-location/location/geofencing)
- [Android background location and battery life](https://developer.android.com/develop/sensors-and-location/location/battery)
- [Android DevicePolicyManager](https://developer.android.com/reference/android/app/admin/DevicePolicyManager)
- [Apple Core Location](https://developer.apple.com/documentation/corelocation)
- [Apple requesting authorization to use location services](https://developer.apple.com/documentation/corelocation/requesting-authorization-to-use-location-services)
- [Apple handling location updates in the background](https://developer.apple.com/documentation/corelocation/handling-location-updates-in-the-background)
- [Apple monitoring geographic regions](https://developer.apple.com/documentation/corelocation/monitoring-the-user-s-proximity-to-geographic-regions)
- [Apple reduced accuracy location key](https://developer.apple.com/documentation/bundleresources/information-property-list/nslocationdefaultaccuracyreduced)
- [Apple Device Location MDM command](https://developer.apple.com/documentation/devicemanagement/device-location-command)
- [Apple Find My and location sharing](https://support.apple.com/guide/personal-safety/find-my-and-location-sharing-ips05ede4573/web)
- [Windows Geolocator class](https://learn.microsoft.com/en-us/uwp/api/windows.devices.geolocation.geolocator)
- [Windows location-aware app guidelines](https://learn.microsoft.com/en-us/windows/uwp/maps-and-location/guidelines-and-checklist-for-detecting-location)
- [Windows get the user's location](https://learn.microsoft.com/en-us/windows/uwp/maps-and-location/get-location)
- [Windows location service connected experience](https://learn.microsoft.com/en-us/windows/privacy/essential-services-and-connected-experiences)
- [Windows Wi-Fi access and location consent changes](https://learn.microsoft.com/en-us/windows/win32/nativewifi/wi-fi-access-location-changes)
- [Windows Privacy Policy CSP location access](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-privacy)
- [GeoClue reference manual](https://www.freedesktop.org/software/geoclue/docs/)
