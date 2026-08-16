# Native Apps UI/UX Requirements Guide

This guide defines parent and child UI requirements for native app work. It is
not a claim that the portal already implements every state.

## UI Rule

The UI must keep evidence, policy, authority, and action result separate:

```text
Installed app inventory is weaker than running app evidence.
Running app evidence is weaker than foreground/activity evidence.
Policy would block is weaker than adapter blocked and journaled the result.
Manual-required is a real product state, not an error to hide.
```

## Parent Outcomes

Parents should be able to see:

- which apps are installed or detectable;
- which apps are running now;
- which app was foreground/active and when;
- running and foreground duration;
- unknown/new apps;
- risk app candidates;
- app categories and classification confidence;
- app policy and schedule outcomes;
- app control authority tier;
- which actions are available now;
- which actions need setup, enrollment, admin/root, entitlement, MDM,
  supervised-device, device-owner, or manual proof;
- evidence and audit references for every claim.

## Main Surfaces

App surfaces may be dedicated pages or sections, but they should eventually
cover:

- Apps overview;
- Installed apps;
- Running now;
- Foreground now;
- Recent app sessions;
- Daily rollups;
- New/unknown apps;
- Risk apps;
- Approval requests;
- App rules;
- Capability/platform status;
- Evidence details;
- Audit timeline.

Until dedicated pages exist, extend current service-backed portal surfaces:

- live activity panel/state;
- activity timeline;
- policy preview;
- capability guidance;
- device rule scope;
- portal layout surface/content panels;
- parent app navigation entry.

## App Card Requirements

Each visible app row should show only backed claims:

- display label;
- category and source/confidence;
- installed state;
- running state;
- foreground state;
- duration;
- policy state;
- capability/authority state;
- evidence source label;
- last observed;
- next action.

Example structure:

```text
Discord
Category: Messaging / Social
Installed: Yes
Running: Yes
Foreground: 12 min today
Policy: Ask parent during homework
Status: Warn-only on this platform
Evidence: process + inventory + foreground window
```

## Unknown App Requirements

Unknown app rows should avoid fear language:

```text
Unknown App
Process: abc123.exe
Publisher: Unknown
Path: redacted
Seen: Today 4:22 PM
Risk: Portable app candidate
Action: Ask parent
```

Never label an unknown app as dangerous without evidence. Use unknown, risk
candidate, manual review, or ask-parent.

## Capability Labels

Use clear capability labels:

- Can observe;
- Can warn;
- Can ask parent;
- Can count running time;
- Can count foreground time;
- Can time-limit;
- Can terminate running process;
- Can shield;
- Can hide/suspend;
- Can block launch;
- Block launch manual-required;
- Unsupported on this platform;
- Permission required;
- Device owner required;
- MDM required;
- Supervision required;
- System extension required;
- Admin/root service required;
- Entitlement/signing required.

## Platform Setup UI

Every stronger control should show setup cost:

- normal app mode;
- parent-approved permission mode;
- accessibility/screen-time mode;
- managed-device mode;
- admin/root/system-extension mode;
- kiosk/single-app mode;
- manual-required.

The UI should not imply corporate MDM behavior on a personal device unless the
device is actually managed that way.

## Policy Authoring UI

Policy UI should render from typed app-control manifest/contracts and show:

- target type;
- evidence requirement;
- category confidence requirement;
- unknown/new app behavior;
- duration mode;
- action;
- authority tier;
- platform availability;
- manual-required reason;
- preview decision;
- audit refs.

Do not invent app policy questions in portal code. Use parent-domain manifest
contracts and update the manifest when the acceptance contract changes.

## Child UX

Child-facing copy should be short and calm:

- "This app is limited by your family rules."
- "This new app needs parent approval before you can use it."
- "Your app time is almost finished."
- "This app is blocked right now. You can ask your parent for more time."
- "This app is not available on this device right now."

Avoid:

- "You were caught."
- "This app is dangerous."
- "AI blocked you."
- raw process names as the main child-facing reason when a friendlier label is
  available;
- parent diagnostics or private evidence details.

## Evidence Detail UI

Evidence details should show:

- source type;
- observed time;
- freshness;
- custody;
- identity fields used;
- confidence;
- reason codes;
- redacted path/hash/signature refs where allowed;
- policy decision refs;
- action/audit refs.

Evidence details must not show:

- raw command lines with secrets;
- app internal documents;
- chat/message content;
- keystrokes;
- screenshots unless screen-evidence scope explicitly owns them;
- launcher credentials or tokens;
- decrypted network payloads.

## States To Snapshot

Every UI workpack must capture snapshots for applicable states:

- normal inventory;
- running now;
- foreground now;
- ended session;
- unknown/new app;
- risk app;
- approval request;
- policy preview;
- warn/ask/time-limit;
- action result;
- stale;
- degraded;
- permission-required;
- manual-required;
- unsupported;
- malicious/long values;
- narrow viewport.

If no UI changed, the proof pack must include `ui-not-applicable.md`.
