# Browser UI/UX Requirements Guide

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser UI/UX Requirements Guide`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This guide turns current browser evidence, browser policy, unmanaged fallback,
and intervention requirements into parent-facing UI/UX acceptance criteria.
Treat it as product guidance for service-backed UI, not as a claim that current
portal screens already implement every state.

## Main UI Rule

The UI must keep these states distinct:

- Managed exact evidence: URL/title/domain/tab evidence from an Ocentra-owned
  managed browser session.
- Managed target-list evidence: tab targets are known, but active tab is not
  proved.
- Unmanaged browser use: browser-like process outside the managed boundary;
  possible bypass; exact URL unavailable.
- Network/domain evidence: destination metadata; not page or active-tab proof.
- AI classification: evidence-backed content/category/risk/benefit analysis;
  not final household authority.
- Social account/feed gate: evidence-backed social route, account, approval, or
  feed state; not a generic browser/app block.
- Browser-game gate: evidence-backed game portal, cloud session, runtime signal,
  account, purchase, educational claim, or time-budget state; not a generic
  website block.
- Unsupported/manual-required: product cannot currently claim the action or
  evidence source.

Never show exact URL fields for unmanaged browser evidence. Never show an active
tab as current when the source state is stale or target-list-only. Never show AI
classification as the reason an action happened without the parent policy rule
and adapter capability beside it.

## Required Screens Or Modes

- Browser inventory dashboard.
- Managed browser setup/status.
- Managed browser sessions.
- Current tab evidence.
- Active-tab certainty details.
- Unmanaged browser bypass review.
- Browser policy authoring/preview.
- URL/video intelligence review.
- AI classification and degraded-state explanation.
- Social platforms overview.
- Approved social accounts.
- Pending account approval requests.
- New or secondary account attempts.
- Feed/Reels/Shorts rules.
- Messaging/contact risk rules.
- Browser-game dashboard.
- Cloud-gaming approval.
- Educational-game rules.
- Game account/purchase approval.
- Unblocked-game site states.
- Browser warning/block results.
- Evidence details drawer.
- Capability and degraded-state diagnostics.
- Manual platform proof/status matrix.

These may live in existing Devices, Activity, Browser, Policy, Reports, or
diagnostics routes. Do not create a second product concept if a current
service-backed surface already owns the parent workflow.

## Browser Inventory Requirements

Each inventory row should show:

- browser product name;
- browser family and channel;
- installed/running state;
- management tier;
- exact URL capability;
- active-tab capability;
- managed profile readiness;
- unmanaged fallback state;
- reason code for unsupported/degraded/manual-required state;
- next action.

Example states:

```text
Microsoft Edge
Installed
Managed support: Ready
Exact URL: Available in managed session
Active tab: Unknown until focus proof
Unmanaged fallback: App Control manual-required
```

```text
Firefox
Installed
Managed support: Later adapter required
Exact URL: Not claimed
State if opened: Unmanaged or unsupported browser use
```

## Managed Session Requirements

Session cards should show:

- managed session id reference;
- browser family/channel/version;
- managed profile id or redacted profile ref;
- bridge kind and redacted endpoint ref;
- bridge connected/disconnected state;
- capability status;
- degraded reason;
- custody label;
- last observed;
- freshness/staleness.

Raw DevTools websocket URLs and raw local profile paths must not appear in
normal copy/debug output.

## Tab Evidence Requirements

Tab rows should show:

- URL when allowed by evidence scope;
- title when available;
- normalized domain/origin;
- browser family;
- evidence id;
- source id;
- adapter id;
- observed time;
- fresh/stale state;
- active state: known-active, known-inactive, or unknown;
- custody/source label.

If active state is unknown, say that directly. Do not hide the row, and do not
promote it to current active tab.

## Unmanaged Browser Requirements

Unmanaged cards should show:

- process name;
- browser family guess;
- process id where safe;
- executable/path/signature/hash refs where available;
- reason code;
- confidence;
- exact URL unavailable;
- possible bypass label;
- available actions based on capability.

Available action states:

- report only;
- warn child;
- ask parent;
- terminate process;
- relaunch managed browser;
- OS block configured;
- OS block manual-required;
- unavailable/degraded.

The UI must not imply that Ocentra can recover the exact page from an unmanaged
browser process.

## Policy Authoring Requirements

Browser policy UI must render from the typed authoring manifest. If UI needs a
new browser-policy question, update the contract/manifest first.

Policy preview must show:

- target type;
- evidence requirement;
- current capability;
- proof fallback;
- action that would run;
- whether action is observe, dry-run, warn, block, terminate, ask, or
  manual-required;
- evidence refs and policy decision refs when available.

Dry-run must read as preview only and must not imply a block occurred.

## URL And Video Intelligence Requirements

Parent UI should keep these fields separate:

- observed URL/video/platform;
- URL shape result;
- metadata evidence;
- memory/cache state;
- AI provider and model/runtime status;
- classification, confidence, and reason codes;
- benefit signals and risk signals;
- parent rule matched;
- final policy decision;
- action taken or unavailable;
- whether the child saw the page before the decision;
- degraded/manual-required state.

Child UI should use calm states such as checking, allowed, needs parent
approval, limited, blocked by parent rule, or could not classify. Do not show
copy that shames the child or says the AI is judging them.

If analysis finishes after playback or browsing starts, parent UI must label it
as a post-analysis action rather than real-time prevention.

## Social Account And Feed Gate Requirements

Parent UI should support:

- social platforms overview;
- approved social accounts;
- pending account approval requests;
- new/fake/secondary account attempts;
- platform rule builder;
- feed, reels, and shorts rules;
- video platform rules;
- messaging/contact risk rules;
- evidence details;
- AI analysis details;
- action/audit timeline;
- manual-required and unsupported states.

Approval cards should separate platform, child, device, evidence, Ocentra
recommendation, parent action, and audit state. They should not accuse the child
of making a fake account from weak evidence.

Child UI should use calm copy such as:

```text
Your family rules require parent approval before creating a new social account.
```

```text
This looks like a new or different account. Ask your parent to continue.
```

```text
Short-video feeds are limited right now.
```

Do not use shame language, "you are being watched", or "AI caught you".

## Browser Game Requirements

Parent UI should support:

- browser-game dashboard;
- game evidence drawer with URL, runtime signals, metadata, AI, policy, and
  audit refs;
- educational-game allow rules;
- unknown-game ask-parent flow;
- browser-game time budget;
- cloud-gaming approval rule;
- game account/signup/purchase approval request;
- unblocked-game portal warning/block state;
- unmanaged browser game bypass state.

Child UI should use calm copy such as:

```text
Ocentra is checking whether this game matches your family rules.
```

```text
This game needs parent approval before you can play.
```

```text
Game time is limited right now.
```

Do not use copy that says the child is addicted, the game is evil, or AI caught
them.

## Warning And Block Requirements

Before product claims, warning/block UI must show:

- parent rule;
- requested URL/domain/target;
- evidence ref;
- policy decision ref;
- intervention mechanism;
- action outcome;
- child-facing delivery state;
- audit ref;
- rollback/failure state when relevant.

Managed block pages should be local Ocentra pages with clear source/custody
labels and no raw protocol details.

## Diagnostic Requirements

Activity/Browser diagnostics should expose:

- browser inventory scan state;
- managed session lifecycle;
- bridge connection/disconnection;
- tab evidence timing;
- stale/degraded transitions;
- unmanaged detection events;
- policy preview decisions;
- intervention attempts and outcomes;
- audit references;
- manual-required platform gaps.

Diagnostics should render typed read models and event history from the Rust
service. They should not become a fake local browser log console.

## Empty, Stale, And Failure States

Required states:

- no supported browser installed;
- supported browser installed but not configured;
- managed profile missing;
- bridge disconnected;
- bridge stale;
- adapter error;
- permission required;
- unsupported browser;
- unmanaged browser detected;
- manual proof required;
- platform unavailable.

Each state needs a reason and next action.

## Accessibility And Safety

- Long URLs and titles must wrap/truncate without breaking layout.
- Punycode and suspicious domains must not spoof labels.
- Titles and URLs must render as text, never HTML.
- 100 tabs and many unmanaged browsers must not freeze the page.
- Copy/debug output must redact bridge endpoint and profile path details.

## UI Done Signal

Browser UI is done for a workpack only when:

- it consumes service-backed read models;
- it labels source/custody/capability clearly;
- weak evidence is not upgraded into stronger claims;
- Playwright covers empty, normal, degraded, stale, unsupported, unmanaged, and
  malicious fixture states;
- screenshots or proof artifacts are captured when a real browser claim is
  made.
