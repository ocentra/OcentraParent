# LAN Discovery UI/UX Requirements Guide

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Discovery UI/UX Requirements Guide`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This guide incorporates the pasted ChatGPT UI/UX requirements, but applies them
to Ocentra Parent's current project reality. Treat it as product guidance and
acceptance criteria for service-backed UI. It does not claim the current portal
already implements every screen. For this LAN plan, codex-b owns the full
service-to-UI path; other lanes are coordination boundaries only.

## Where We Are

Ocentra Parent has a service-backed development portal and a V0.9 LAN read-model
foundation, not a finished consumer LAN setup flow.

Today the product portal loads LAN pairing state through the host bridge,
requests LAN pairing status on startup, and can send a LAN browser-discovery
scan command from the Devices / LAN Pairing surface. Product updates flow back
through the host bridge / Tauri event path rather than a UI-owned WebSocket.
The service-backed overview rows show Local
agent, LAN discovery, and Device pairing trends from LAN pairing status,
browser-discovery, and add-device events.

When a valid add-device read model is present, the parent portal renders a LAN
device grid and selected-device detail tabs. Current screenshots show:

- Devices / Local Area Network: LAN Devices and Parent Portal modes, scan
  button state, connected/available/offline/unsupported legend, local child
  agent row, passive LAN rows, infrastructure row, and detail tabs for Info,
  Update, and Capability.
- Activity: Family and Per Device selector, Reports/Browser/App Use/Games/
  Screen/Network/Tracking/Remote Screen tabs, and network evidence summary
  fields such as destination, process, read-model state, connections, bytes, and
  summary.
- Overview and adjacent policy routes: Local agent, LAN discovery, and Device
  pairing readiness cards; Network policy controls for family/per-device,
  domain/IP/protocol/process, proof quality, weak-proof behavior, observe,
  warn, ask parent, limit, and block.

The current UI can show canonical household devices, local child-agent hardware
inventory, passive LAN neighbors, router/infrastructure rows, scan summary
counts, trusted registry input, pairing request input, selected-device
readiness, and stale/offline/revoked/manual-required/unavailable states. It
intentionally keeps passive neighbors and routers out of controllable target
lists and shows unreported hardware fields instead of inventing CPU/GPU/memory.

This is still a development surface. First-run household setup, child profile
creation, assign/rename/ignore, trust management actions, parent-started and
child-started pairing UX, evidence drawer, duplicate review, interface picker,
platform permission/error copy, alerting, product-grade offline/agent-offline/
confidence labels, and policy target persistence from the canonical device
registry are not complete.

## Where We Want To Be

The LAN discovery experience should become the parent's add-device, household
inventory, and diagnostic evidence workflow, not just a status grid. A
nontechnical parent should be able to open Devices, understand what LAN
discovery can and cannot know, choose the right network interface, see scan
progress, review grouped devices, pair a child agent, assign or rename devices,
trust household infrastructure, ignore noise without deleting evidence, and
recover from stale/offline/revoked states.

Every device card must answer:

- What is this device?
- How sure are we?
- How did we find it?
- Is it confirmed or just guessed?
- Who assigned it?
- When was it last seen?
- What can the parent do next?

The service contract should drive the UI: scan summary and device cards from
the LAN add-device read model, pairing/trust/revocation from Rust-backed
commands and events, confidence/evidence/history from typed evidence refs, route
availability from selected-device readiness, and diagnostic logs from
Activity/Network read models.

The product rule is:

```text
Discover automatically. Explain honestly. Confirm with child agent.
Let parent decide ownership. Never pretend guesses are facts.
```

## Main UI Rule

The UI must keep these states distinct:

- Discovered: Ocentra found a LAN device, but does not know the owner.
- Assigned: a parent manually linked the device to a child/profile.
- Confirmed: an Ocentra child agent cryptographically confirmed identity.

Never show a guessed owner. `Likely iPhone/iPad`, `Unassigned`, and `Seen on
LAN` are acceptable. A child name is acceptable only after parent assignment or
child-agent confirmation.

## Required Screens Or Modes

- LAN discovery dashboard with Confirmed Child Devices, Assigned Devices,
  Unknown Devices, Trusted Devices, Ignored Devices, and Offline Devices.
- First-time LAN setup explaining what LAN discovery can and cannot know.
- Progressive scan state: reading local network, scanning nearby devices,
  identifying names, looking for child agents, done.
- Pair child device flow for parent-started and child-started pairing.
- Unknown device review flow.
- Device assignment flow.
- Trust and ignore flows.
- Evidence details drawer.
- Network interface picker.
- Permissions, alerts, empty states, and error states.
- Activity / Network diagnostic evidence view for LAN, network-flow, pairing,
  child-agent heartbeat, route rejection, and permission/error logs.

These may be separate screens, panels, drawers, route modes, or sections inside
the existing Devices/household flow. Do not create a second product concept if
the current portal already has a better service-backed surface.

## Device Card Requirements

Each card should receive enough typed state to show:

- display name;
- device type guess;
- IP address;
- MAC/vendor where available;
- online, recently seen, stale, offline, or agent-offline state;
- confidence label;
- evidence source badges;
- assignment status;
- agent status when present;
- last seen;
- primary action.

Unknown device copy should stay calm: `New device found`, `Unrecognized device`,
or `Needs review`. Do not use threat language unless specific risk evidence
exists.

## Pairing UX Requirements

Parent-started pairing:

- Parent clicks Add Child Device.
- UI lets parent choose Windows, macOS, Linux, Android, or iPhone/iPad.
- UI shows install instructions, same-network requirement, pairing code, expiry,
  QR code, copy action, and cancel action.
- When the child connects, UI shows device name, platform, IP, agent version,
  child assignment selector, confirm and assign, confirm without assigning, and
  reject.

Child-started pairing:

- Child app looks for Ocentra Parent on the network.
- If found, it asks for the pairing code shown on the parent app.
- If not found, it offers manual pairing code, manual parent IP, or later
  cloud/account pairing.

Failure states must cover expired code, wrong code, already paired, different
family/account, invalid signature, parent not found, outdated child app,
firewall blocked, and not on same Wi-Fi.

## Assignment, Trust, And Ignore

Manual assignment must show the device evidence, child/profile selector,
optional friendly name, and confirmation action. After assignment, the card
must say `Assigned by parent` and must not imply `Confirmed by Ocentra Agent`.

Trusted means a known household device that is not a child device, such as a
router, printer, TV, NAS, or parent laptop. Ignored means hidden from the normal
dashboard. Ignore must not immediately delete evidence.

## Evidence Details

Every visible device needs a details panel with:

- Summary;
- Identity;
- Network;
- Evidence;
- History;
- Agent status;
- Actions.

The evidence panel should explain confidence with reasons, not magic. It should
show sources such as ARP sweep, mDNS, passive ARP, SSDP, manual assignment, and
Ocentra agent hello/heartbeat when present.

## Activity And Network Diagnostics

Activity / Network is the parent-visible diagnostic surface for this plan. It
should make service behavior inspectable so manual review can catch issues that
tests miss.

Required diagnostic coverage:

- LAN scan sessions: selected interface, trigger, start, phased progress,
  finish, counts, skipped interfaces, and failure reason.
- Device evidence: source, timestamp, device id, IP, MAC/vendor when available,
  hostname/service when available, confidence reason, merge decision, and
  canonical target.
- Pairing and trust: pairing code lifecycle, hello validation, heartbeat,
  assignment, rename, trust, ignore, revocation, wrong-origin rejection,
  wrong-device rejection, stale, offline, and safe unpaired fallback.
- Network evidence: destination, domain/IP/protocol/port, process when
  available, byte counts, connection count, proof quality, weak/missing proof
  behavior, custody/source label, and read-model state.
- Policy connection: Network Policy and other per-device policy pages must bind
  to the canonical household device target before enforcement or observation
  claims become product-visible.

The Activity / Network view should not become a pretend log console. It should
render typed read models and event history from the Rust service so the user can
visually inspect what happened and report UI/product gaps.

## Confidence And Merge UX

Main UI labels:

- Confirmed;
- High confidence;
- Medium confidence;
- Low confidence;
- Unknown.

Exact percentages should live in details, not the card headline. Possible
duplicate flows must show both candidate records, merge reasons, and actions:
Merge, Keep Separate, Ignore Suggestion. Automatic merge is allowed only for
strong evidence.

## Offline, Stale, And Agent State

Devices should not disappear just because they are offline. Required labels:

- Online now;
- Recently seen;
- Offline;
- Agent offline;
- Device seen on LAN;
- Ocentra Agent connected.

A child device can be LAN online with agent offline, agent online with changed
LAN IP, or fully offline. The UI must distinguish those cases.

## Network Interface And Permission UX

If multiple interfaces exist, show the recommended home interface and hide
Docker, Hyper-V, WSL, VPN, and loopback by default behind advanced controls.

Permission copy must stay platform-specific:

- Windows: firewall/private network permission.
- macOS: Local Network permission.
- Linux: network capability or service install requirements.
- Android/iOS child app: Local Network access for pairing.

## Alerts, Empty States, And Errors

Alert types:

- new unknown device joined;
- known child device came online;
- known child device went offline;
- child agent disconnected;
- pairing request received;
- possible duplicate found;
- possible device identity changed.

Empty states must cover no devices found and no child devices paired. Error
states must cover scan failed, permission denied, no LAN interface, firewall
blocked, mDNS unavailable, SSDP unavailable, pairing timeout, invalid child
signature, database error, and network changed during scan.

## Required UI Components

- `LanDiscoveryPage`
- `ScanProgressHeader`
- `NetworkInterfacePicker`
- `DeviceGroupSection`
- `DeviceCard`
- `DeviceStatusBadge`
- `ConfidenceBadge`
- `EvidenceSourceBadges`
- `DeviceDetailsDrawer`
- `AssignDeviceModal`
- `PairChildDeviceModal`
- `PairingCodePanel`
- `PairingRequestModal`
- `TrustDeviceModal`
- `IgnoreDeviceModal`
- `DuplicateMergeSuggestion`
- `OfflineDeviceBanner`
- `DiscoveryErrorPanel`

Names may adapt to existing portal conventions, but the capability surface must
remain covered.

## Required UI States

```ts
type DeviceUiState =
  | 'unknown'
  | 'known_unassigned'
  | 'assigned_manual'
  | 'confirmed_agent'
  | 'trusted_household'
  | 'ignored'
  | 'offline'
  | 'stale'
  | 'possible_duplicate'
  | 'identity_changed';
```

Required badges:

- Unknown;
- Assigned;
- Confirmed;
- Trusted;
- Offline;
- Agent Offline;
- Private MAC;
- Low Confidence;
- New.

## Playwright Coverage

UI coverage must include:

- first-time setup;
- scan progress;
- empty dashboard;
- device card rendering;
- unknown device assignment;
- trusted device flow;
- ignored device flow;
- pairing code generation;
- pairing success;
- pairing timeout;
- agent confirmed badge;
- agent offline badge;
- evidence drawer;
- duplicate suggestion;
- network interface picker;
- permission denied state;
- malicious hostname escaping;
- long hostname truncation;
- offline/online transition;
- manual assignment persists after rescan.
- Activity / Network diagnostic view renders LAN scan, network evidence,
  pairing, route rejection, and policy-target state from service-backed data.
