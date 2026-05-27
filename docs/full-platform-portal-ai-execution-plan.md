# Ocentra Parent Full Platform, Portal, And AI Plan

This is the single current execution plan for the platform/portal/AI reset. It
replaces the older pre-AI proof-only plan. The goal is to stop treating V0.8,
V0.9, platform support, mobile support, and AI as separate hand-wave tracks, and
turn them into concrete branches with real contracts, runtime paths, validation,
PRs, CI, and merges.

## Product Truth

Ocentra Parent is not a web parent portal product.

The Vite React portal is a development and HMR surface. The production parent
portal is a packaged parent app. React/TypeScript owns the user interface. Rust
owns product runtime behavior, contracts, storage, policy, AI scheduling,
device routing, timers, enforcement adapters, and service state.

Tauri is the preferred desktop parent shell. Tauri mobile is the first mobile
parent proof path because it preserves the Rust-first direction and allows the
React portal UI to be reused. Capacitor is a fallback only if Tauri mobile proof
fails for this product; it is not the default architecture.

The child side is not the parent portal. The child side is an agent role running
on the child device. On desktop, that is Rust service/runtime first. On Android
and iOS, the agent needs platform-specific wrappers around shared contracts,
journal/query behavior, protocol, and whatever OS-approved capture/enforcement
capabilities each platform can honestly provide.

The product must track parent desktop, parent mobile, child desktop, child
Android, and child iOS separately. No platform claim is complete until the
actual platform path has proof. CI mechanical proof is useful, but privileged
OS behavior, mobile permissions, signing, store policy, device-owner policy, and
iOS entitlement behavior need real device or platform proof before product
claims.

## Roles, Not Separate Products

Devices should be modeled by roles:

- `parent-controller`: can issue parent-approved commands while holding the
  household controller lease.
- `parent-observer`: can view status, reports, notifications, and drafts, but
  cannot control child devices.
- `child-agent`: captures, stores, evaluates, times, and enforces on a child
  device.
- `ai-provider`: owns local model/runtime access for a physical device and can
  optionally serve authorized household AI jobs.

One physical device may run more than one role. A common case is a parent
desktop that also belongs to a child and therefore runs both parent portal and
child-agent behavior. Another common case is three child devices plus a separate
parent portal on desktop or mobile.

Subscription device limits should count physical enrolled devices, not how many
roles are running on that device.

Even when parent and child roles run on the same physical machine, the parent
portal must talk to the child-agent role through the same typed local service or
loopback protocol boundary. It must not bypass into files, UI-only state,
SQLite, or direct runtime internals.

The dual-role local flow is:

```text
Parent portal UI
  -> parent Rust controller
  -> typed local loopback request
  -> local child-agent role on the same device
  -> child journal / SQLite / local safety AI / policy / enforcement
```

The product should label this clearly in the parent portal:

- this device is also monitored
- this device can control family devices
- local AI is available here
- this device is observer-only when it does not hold the controller lease

## Parent Controller Singleton

Multiple parent portals may be installed or open at the same time. Only one
active household controller lease may write/control at a time.

The active controller lease should include:

- household id
- controller device id
- parent actor id
- granted timestamp
- expires timestamp
- renewal timestamp
- release state
- takeover request reason
- takeover result
- audit event ids

Other parent portals become observers unless they acquire or take over the
lease. Observers can view current state, reports, notifications, and draft rule
changes. They cannot issue enforcement, approval, pairing, revocation, or policy
write commands.

Child agents must reject commands when any of these are true:

- controller lease is missing
- controller lease is expired
- controller device is wrong
- parent actor is unauthorized
- command origin is wrong
- command is stale
- command is replayed
- route target is not paired
- child device has revoked that controller

This lease model prevents race conditions when parent desktop and parent mobile
are both present.

## AI Architecture

There are two AI paths. They must remain separate.

### Child Safety AI

Child Safety AI runs on the child-agent side. It consumes typed evidence,
parent rules, local runtime status, and evidence-backed memory or graph
references. It returns schema-valid safety output such as allow, warn, block,
time-limit, ask-parent, or unknown. Deterministic policy and enforcement consume
that typed result. AI output alone is not household authority.

Child Safety AI is local-first. Remote/API AI is not part of normal child
blocking, timing, ask-parent, or enforcement decisions.

### Parent Assistant / MIA

Parent Assistant AI is parent-facing. It can answer parent questions, explain
evidence, prepare rule suggestions, summarize reports, and draft actions. It
must cite allowed evidence references and must never directly enforce. A parent
assistant answer can propose an action; the controller lease and child-agent
policy path decide whether anything is actually written or enforced.

### Current Repo AI Truth

`main` already has local AI runtime plumbing that should be reused instead of
inventing a second parent-side model path:

- `agent.local-ai.runtime.status.get`
- `agent.local-ai.chat.generate`
- `crates/agent-service/src/local_ai_chat_generation*.rs`

The current Rust service can call `llama-cli` when local execution is explicitly
configured. The relevant configuration surface includes:

- `OCENTRA_PARENT_LOCAL_AI_RUNTIME_BINARY`
- `OCENTRA_PARENT_LOCAL_AI_MODEL_FILE`
- `OCENTRA_PARENT_LOCAL_AI_EXECUTION_ENABLED`
- model id
- timeout
- max tokens

The user-owned C branch has parent assistant UI/contracts/scaffold work, but it
is not a finished backend path yet. Known C-side scaffold pieces include:

- `vendor/ocentra-parent-core-ui/.../ParentPortalChatBubble.tsx`
- `packages/parent-domain/src/parent-assistant.ts`
- `crates/agent-service/src/parent_assistant_api.rs`

That backend is still scaffold-only and reports a backend-not-connected state.
After C lands, the next real implementation slice should connect MIA chat to a
Rust parent assistant runtime using the existing local AI generation path,
including unavailable/configured states and one evidence-cited prompt flow.

The intended parent assistant runtime path is:

```text
Parent Portal Chat UI
  -> parent assistant TypeScript contracts
  -> agent protocol command/event
  -> Rust parent assistant runtime
  -> provider router
       -> local model provider via llama.cpp/llama-cli
       -> optional API provider only with parent authorization
  -> cited answer/action preview back to UI
```

Ownership should stay clean:

- UI only: React/TypeScript parent portal.
- Assistant contracts: `packages/parent-domain` and
  `packages/agent-protocol-domain`.
- Rust protocol mirror: `crates/agent-protocol`.
- Assistant runtime: `crates/agent-service`, split into focused modules such as
  `parent_assistant_runtime`, `parent_assistant_provider`,
  `parent_assistant_threads`, and `parent_assistant_prompt_context` as needed.
- Local model execution: reuse and extend `crates/agent-service/src/local_ai_*`.
- API provider: explicit provider module only; never hidden in UI and never
  used for child safety decisions.

### Single AI Runtime Per Physical Device

One physical device should load one local model/runtime by default. We should
not accidentally load one model for parent assistant and another model for child
safety on the same machine, because that wastes VRAM/RAM and creates competing
runtime state.

Each physical device with AI capability should expose an `ai-provider` role:

- provider id
- model id or opaque model reference
- runtime backend, such as local llama.cpp/llama-cli first
- capability flags
- local/API/LAN availability
- resource class
- busy/queued/degraded state
- current job class
- unavailable reason
- last checked timestamp

An AI scheduler owns model access on that physical device. Child safety jobs
have priority. Parent assistant/report jobs queue, throttle, or degrade when
child safety work or device resources require it. Duplicate model loads are not
allowed unless a future explicit advanced mode says otherwise.

The concrete dual-role device rule is:

```text
Device Runtime
  - role: child agent
  - role: parent controller, optional
  - role: local AI provider, singleton

Parent assistant request -> AI scheduler -> one loaded model
Child safety request     -> AI scheduler -> same loaded model
```

The portal should expose enough runtime status for a parent to understand why
MIA is fast, queued, degraded, unavailable, or disabled: model configured,
execution enabled, provider busy, child-safety job active, CPU/RAM/VRAM class,
and local/API/LAN provider source.

### LAN AI Provider Pool

Parent mobile does not run local AI by default. It submits authorized MIA/report
jobs to the household LAN AI provider pool. Any household device with
`ai-provider` role may opt into that pool.

LAN AI jobs must use typed permitted context and evidence references. They must
not send raw child files, broad SQLite dumps, raw journals, screenshots, or
private blobs by default.

If no LAN provider exists, parent mobile shows degraded/unavailable state.
Optional API AI is allowed only after explicit parent authorization, custody
labels, retention/deletion behavior, and evidence-citation rules are present.
API AI cannot override local child safety, policy, timers, or enforcement.

## Platform Plan

### Parent Desktop

Parent desktop is React UI inside Tauri plus Rust service/runtime boundary.
The Tauri shell embeds the built portal UI and connects through typed local
service/WebSocket commands. Tauri itself is not the business-logic backend.

Required work:

- keep Vite as dev-only HMR surface
- make packaged parent desktop launch and connect to the real Rust service path
- show controller lease state
- show device role state
- show AI provider state
- show live/degraded/unavailable source labels
- never run child capture, child policy, child AI, timers, or enforcement in UI

### Parent Mobile

Parent mobile is Tauri mobile proof-first. It should reuse the portal UI and
contracts where practical, with mobile-specific shell and permission behavior.

Required work:

- build and launch Android/iOS parent shell proof
- connect to local/LAN/cloud-routed service through typed contracts
- support observer mode and controller takeover/request flow
- submit parent assistant jobs to a LAN AI provider or show unavailable state
- avoid local model execution by default
- keep mobile parent separate from mobile child-agent claims

### Child Desktop

Child desktop is Rust service/agent first. It owns capture, storage, policy,
local AI safety evaluation, timers, enforcement adapters, and audit state.

Windows remains the first production-grade target. macOS and Linux support must
be tracked honestly as separate adapter/package/service-manager work.

Required work:

- real Windows app/game time-limit adapter path
- enforcement capability/status
- rollback/unavailable behavior
- restart recovery
- audit events
- local AI scheduler integration before model-backed enforcement claims
- no fake blocking claims

### Child Android

Child Android requires a native Android wrapper plus shared contracts and Rust
core where practical. Android claims must be separated by capability:
foreground service, notification permission, accessibility, VPN/DNS, device
owner, managed profile, usage stats, local storage, and package lifecycle.

Required work:

- Android service/app proof beyond scaffold launch
- SQLite/journal compatibility
- typed protocol bridge
- honest permission/capability statuses
- emulator/device proof for each capability before claiming parity

### Child iOS

Child iOS is the most restricted platform. It must use Apple-approved
capabilities only. Family Controls, Screen Time APIs, Network Extension,
notifications, background execution, signing, and TestFlight all need separate
proof.

Required work:

- iOS parent shell proof separately from child-agent proof
- entitlement/capability map
- simulator/device proof for what exists
- explicit unavailable/degraded states where Apple APIs do not allow a feature
- no desktop parity claim without Apple-approved evidence

### Web

`family.ocentra.ca` is for download, account, subscription, documentation,
status, and optional stateless report compilation. It is not the default parent
portal for child activity and not the child-agent runtime.

## Activity Surface Fix

### Problem Statement

The parent portal Activity surface currently has UI and layout work ahead of the
runtime contract. The UI can render Reports, Screen, App Use, Browser, Games,
and Network sections, but the data seam returns UI-check data only. That blocks
the Activity surface from becoming a real parent product surface because it
cannot request typed report/read-model data from the Rust runtime yet.

The fix is not to make Vite the backend. Vite is only the HMR/dev shell. The
fix is to create a main-backed Activity data contract and portal-to-Rust
service/command boundary that the parent portal can call from Vite during dev
and from the packaged Tauri parent app later.

Avoid describing this as "Tauri owns Activity data." Tauri may expose shell
commands or launch/connect to the local runtime, but Activity product data must
come from typed Rust service/read-model paths.

### Where We Are

- C has a UI-only Activity surface in
  `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx`.
- The current UI intent seam is
  `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`.
- That seam is deliberately marked with `TODO(activity-surface-tauri)`.
- The seam currently returns UI-check data for layout inspection only.
- The UI intentionally hides raw TODO text from users while keeping source TODO
  markers for the wiring pass.
- C is user-owned; primary should not assign work in C.

The UI currently expects these functions:

- `getDailyReport(request)`
- `getWeeklyReport(request)`
- `getMonthlyReport(request)`
- `saveActivityReport(report)`
- `listHistoricalReports(request)`
- `getScreenActivity(request)`
- `getAppUseActivity(request)`
- `getBrowserActivity(request)`
- `getGamesActivity(request)`
- `getNetworkActivity(request)`

### Where We Want To Be

The Activity surface should call a typed adapter that works in the dev portal
and packaged parent app:

The correct path is:

```text
Activity UI
  -> typed activity-domain request/response contracts
  -> agent-protocol-domain WebSocket command/event contracts
  -> Rust agent-protocol parity
  -> Rust service/read-model adapter
  -> encrypted journal / SQLite query store / stored evidence summaries
  -> typed result back to UI
```

Tauri commands may wrap or launch the local runtime where needed, but Activity
product data should be service/read-model backed first. Direct Tauri command
wiring is not the product source of truth unless an explicit shell-only command
is being added.

Behavioral target:

- scope can be `family` or `device`
- family reports fan out to available child devices, aggregate reachable
  responses, and record unavailable/offline sources
- per-device reports request only the selected device
- report generation creates an unsaved draft
- saving persists the draft through the selected Data storage target; Data-page
  storage selection may return a typed unavailable/stubbed state until wired
- historical reports are stored as JSON first
- report list items include file name, date, range, summary, saved state, and a
  parsed report document
- the viewer renders parsed report sections, not raw JSON
- Screen, App Use, Browser, Games, and Network tabs return structured
  user-facing read models scoped to family or selected device
- unavailable, offline, stale, permission-required, and scaffold-only states
  are first-class typed states

### Current Gap

The missing main-backed slice is:

- shared Activity domain contracts
- portal/agent command names and response contracts
- Rust protocol parity
- Rust service/read-model adapter boundary
- real typed unavailable/local-read-model responses
- tests proving contract, protocol, and adapter behavior

The gap is not C's UI layout. C should not have to redesign Activity to connect
this. After the main-backed slice merges, C can pull latest `main` and replace
`activity-ui-intent.ts` with the real adapter.

### Who Fills The Gap

- Primary: writes the work order, assigns the first free worker lane, reviews
  branch diff/validation, creates PR, watches CI, merges when green, and tells C
  when `main` is ready to pull.
- Activity worker lane: implements the contract/protocol/service adapter slice
  from fresh `main`. This can be A or B after their current assigned branches
  are integrated or intentionally parked.
- C: keeps the UI surface user-owned and consumes the merged adapter later. C
  should not be asked to invent backend contracts inside the UI branch.

### Activity Slice Checklist

The assigned worker must complete every item before `DONE/PR_READY`:

- [ ] Pull or rebase latest `main`.
- [ ] Run `npm run hub:inbox`, acknowledge current hub mail, report `STARTED`,
      and lock intended paths.
- [ ] Add `packages/activity-domain` Effect Schema contracts for Activity target
      scope, report frequency, report request, report list item, report document,
      report sections, and tab view rows for Screen, App Use, Browser, Games, and
      Network.
- [ ] Add portal/agent command names and response contracts in the appropriate
      domain/protocol package. Avoid naked strings in app/runtime source.
- [ ] Add Rust protocol parity in `crates/agent-protocol`.
- [ ] Add Rust service/read-model adapter stubs in `crates/agent-service` that
      return real typed unavailable or local-read-model responses.
- [ ] Ensure Vite does not own or fake Activity product data.
- [ ] Keep Data storage selection as a typed unavailable/stubbed state if the
      Data surface is not wired yet.
- [ ] Add TypeScript contract tests for accepted and rejected report
      requests/responses.
- [ ] Add Rust protocol serialization/parity tests.
- [ ] Add command/service adapter boundary tests.
- [ ] Add focused portal smoke or Playwright coverage proving Reports plus
      Screen/App Use/Browser/Games/Network can call the adapter and render typed
      states.
- [ ] Run focused validation for touched packages/crates.
- [ ] Run `npm run validate` before PR-ready unless primary explicitly accepts
      an omission with reason.
- [ ] Commit locally only when instructed/allowed by hub mail, push the branch,
      and report `DONE/PR_READY` with detailed scope, touched files/packages,
      validation commands/results, known gaps/risks, and PR body outline.

Done means the Activity adapter foundation is merged to `main` with green CI.
It does not mean C's UI branch has already consumed the adapter.

## V0.8 / V0.9 Execution Meaning

V0.8 is not done until the enforcement path has a real adapter boundary,
capability/status reporting, timer/recovery behavior, audit events, parent
cancel/override behavior, rollback/unavailable behavior, restart recovery, and
proof harness coverage. It must not claim real blocking until the OS adapter and
product proof exist.

V0.9 is not done until paired-device routing, controller lease, trusted-device
registry, wrong-device rejection, wrong-origin rejection, stale/replayed command
rejection, revocation-before-control, offline/stale selected-device state, LAN
proof, and audit events are all implemented and validated. It must not claim
production LAN pairing from scaffold-only or single-service checks.

V8/V9 production hardening work remains later. It depends on platform proof,
signing, release lifecycle, privacy/export/delete, threat model, and store
policy evidence. We should not label product-hardening complete while platform
runtime claims are still scaffold-only.

## Immediate Coordinator State

Current state at the time this plan was written:

- `main` is clean and synced with `origin/main`.
- `codex-a` is `DONE/PR_READY` for V0.8 Windows app time-limit adapter MVP.
- `codex-b` is `BLOCKED/PARTIAL` for V0.9 paired-device routing/revocation MVP.
- `codex-c` is user-owned and must not be assigned by primary.

Immediate sequence:

1. Primary reviews A's branch diff and validation.
2. If acceptable, primary creates or updates A PR with detailed scope.
3. Primary watches CI for A.
4. If CI fails, A fixes its own branch.
5. When A PR is green and reviewed, primary merges to `main`.
6. Primary pulls latest `main`.
7. B fetches/rebases on latest `main` and resolves its own conflicts.
8. Primary reviews B once B reports `DONE/PR_READY`.
9. B PR goes through the same CI/merge flow.
10. After both land, A and B get the next large implementation slices below.

## Worker Plan

Primary remains coordinator/reviewer/merger in this chat. It should not do
feature coding here unless the user explicitly redirects. It can maintain
coordination docs, send hub messages, review diffs, create PRs, watch CI, merge
green PRs, and retarget lanes.

`codex-c` is reserved for the user.

### Worker Operating Rules

Every worker assignment should start with the same shape:

- problem statement
- where we are
- where we want to be
- current gap
- who fills the gap
- exact checklist
- validation expectation
- what `DONE` means

Workers must keep the hub useful for everyone:

- [ ] Check hub mail before starting work.
- [ ] Acknowledge the latest hub instruction before coding.
- [ ] Pull or rebase latest `main` before starting a major coding slice.
- [ ] Report `STARTED` before editing.
- [ ] Lock intended paths before editing.
- [ ] Report meaningful progress after each major sub-slice, especially after
      contracts, Rust parity, service wiring, portal wiring, proof harness, and
      validation milestones.
- [ ] Use `hub:heartbeat` for idle/liveness. Do not overwrite `STARTED`,
      `BLOCKED`, or `DONE` with routine idle text.
- [ ] Check hub mail again before starting another major coding slice, before
      committing, and after any rebase/conflict resolution.
- [ ] If blocked, report `BLOCKED` with exact blocker, attempted proof, touched
      files, and what primary or another worker must decide.
- [ ] If another lane may be affected, inspect that lane's reported scope and
      coordinate through hub mail instead of duplicating work.
- [ ] `DONE/PR_READY` must include scope, touched files/packages, validation
      commands/results, commit state, known gaps/risks, and a PR body outline.

Primary must also keep the hub useful:

- [ ] Check `hub:status`, `lanes:status`, primary git status, relevant worker
      git status, open PRs, and CI before assigning or integrating work.
- [ ] Do not send duplicate work if a worker already has unread hub mail or an
      active branch with a clear report.
- [ ] Review branch diff and validation before PR creation.
- [ ] Route CI failures back to the owning worker unless the fix is purely
      coordinator-owned.
- [ ] Merge only after CI is green and the diff is acceptable.
- [ ] Pull latest `main` after merge.
- [ ] Tell active workers to fetch/rebase latest `main`.
- [ ] Report post-merge scope, validation, PR/merge state, risks, and next
      roadmap action.

### Worker A Next Large Slice

After current A work is merged, A should own the local AI runtime/provider slice.

Scope:

- one `ai-provider` role per physical device
- provider status contract hardening if gaps remain
- scheduler contract and Rust service state
- one local model/runtime access lane per device
- child-safety job priority over parent-assistant jobs
- queued/degraded/unavailable states
- no duplicate local model load for same physical device
- parent assistant job submission to local provider when allowed
- proof that parent+child roles on one device share the provider instead of
  starting two model runtimes

Validation:

- TypeScript contract tests
- Rust protocol parity tests
- Rust service/provider scheduler tests
- real unavailable/degraded provider lifecycle tests
- focused proof harness
- `npm run validate` before PR-ready

Stop condition:

A does not stop at doc-only or partial contract work. A stops only when the
branch has implementation, tests, validation evidence, a local commit, a pushed
branch, a detailed PR-ready report, and later any CI fixes required for merge.

### Worker B Next Large Slice

After B rebases on A/main and finishes current V0.9 work, B should own the
controller lease, trusted-device routing, and LAN AI job-routing slice.

Scope:

- household controller lease contract
- active controller write authority
- observer/read-only parent portals
- takeover/release/renewal flow
- trusted-device registry restart behavior
- paired/unpaired route selection
- wrong-device/wrong-origin/stale/replayed command rejection
- revocation-before-control
- offline/stale selected-device read model
- LAN provider capability advertisement
- authorized LAN AI job submit/accept/reject/result/degraded path
- audit events for controller, route, revocation, and LAN AI job decisions

Validation:

- TypeScript contract tests
- Rust protocol parity tests
- Rust service tests
- two-service LAN proof harness
- wrong-origin/wrong-device/replay/revocation tests
- `npm run validate` before PR-ready

Stop condition:

B does not stop at harness-only or doc-only work. B stops only when the branch
has implementation, tests, validation evidence, a local commit, a pushed branch,
a detailed PR-ready report, and later any CI fixes required for merge.

### Primary/Main Follow-Up Slice

If the user wants the current primary worktree used as a third worker lane, it
should be a separate branch and should not consume this coordinator chat as
feature-coding time.

Best primary-owned implementation/documentation slice:

- update architecture docs to encode roles, controller lease, AI provider pool,
  Tauri desktop/mobile direction, Activity service path, and platform proof
  matrix expectations
- update roadmap current-position text after A/B merges
- update platform deliverables expectations for parent mobile vs child mobile
- add an implementation tracking doc for platform/AI/portal gaps

This slice should still have a branch, validation, PR or direct doc-only
integration decision, and a post-merge hub report.

## Validation Policy

The local commit hook should stay light. It should run lane/hub guards and fast
source validation. It should not run Playwright, full Rust/TypeScript suites,
real-service smoke, package previews, or full `npm run validate` on every local
commit.

Use heavier checks intentionally:

- `npm run test:local` for local focused confidence
- `npm run test:e2e` for portal/Playwright behavior when UI changes
- `npm run precommit:full` when a developer wants the heavier local gate
- `npm run validate` before PR-ready handoff or integration
- `npm run ci:local` when reproducing CI locally

PR-ready means:

- focused tests pass
- branch-specific proof harness passes
- `npm run validate` passes or any omission is explicit and accepted by primary
- branch is pushed
- PR body/report includes detailed scope, touched files/packages, validation,
  known gaps/risks, and roadmap slice

Done means merged to `main` with green CI and primary post-merge sync complete.

## Decisions To Confirm

Recommended defaults unless the user overrides:

- Parent desktop: Tauri + React UI + Rust service/runtime.
- Parent mobile: Tauri mobile proof-first; Capacitor fallback only after a
  failed proof.
- Child desktop: Rust agent/service first, Windows first.
- Child Android: native Android wrapper plus shared Rust/contracts where
  practical; claims split by real OS capability.
- Child iOS: Apple-approved capability path only; no parity claim without
  entitlement/device proof.
- Parent controller model: one active controller lease, many observer portals.
- AI model runtime: one local provider/runtime per physical device by default.
- Parent mobile AI: LAN provider pool first, optional API only with explicit
  parent authorization.
- C lane: user-owned, no primary assignment.

Choices still needing explicit product confirmation:

- Should Android child support require device-owner/managed-profile mode for
  stronger enforcement, or should MVP stay observe/assist-first?
- Is child iOS a required MVP target, or an honest limited-support target until
  Apple entitlements are secured?
- Should a parent mobile portal ever be allowed to hold the active controller
  lease outside the home LAN, or should remote control wait for the cloud relay
  milestone?
- Which desktop-class devices are allowed to opt into LAN AI provider duty by
  default: parent desktop only, any trusted household desktop, or explicit
  per-device opt-in?
- Should API AI be disabled entirely until after local/LAN AI is proven, or
  built as an unavailable/explicitly-authorized adapter now?

## Non-Negotiable No-Fake Rules

- No UI-only fake product truth.
- No Vite backend pretending to be the child runtime.
- No parent portal running child safety AI.
- No duplicate local model load on the same physical device by accident.
- No mobile parity claim from scaffolds.
- No LAN control without pairing, lease, origin, replay, stale, and revocation
  checks.
- No enforcement claim without adapter proof and audit/recovery behavior.
- No API AI in normal child blocking, timing, ask-parent, or enforcement.
- No doc-only worker parking when implementation scope is assigned.

## Current Definition Of Success

This plan is successful when:

- A current V0.8 branch is reviewed, PR'd, CI-green, and merged.
- B current V0.9 branch is rebased/fixed, PR'd, CI-green, and merged.
- A then lands the AI provider/scheduler slice with proof.
- B then lands the controller lease/LAN routing/LAN AI job slice with proof.
- Activity surface fake-data paths are replaced by typed service-backed paths.
- Parent desktop, parent mobile, child desktop, child Android, and child iOS are
  each tracked with honest implemented/scaffold/manual-required/unavailable
  states.
- Roadmap and architecture docs match actual product architecture instead of
  vague cross-platform promises.
