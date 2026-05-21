# Ocentra Parent Product Roadmap

This roadmap describes the product we are building, the order we intend to build it in, and the acceptance bar for each stage. It is intentionally product-facing and implementation-facing: every milestone should produce a real vertical slice, not a fake demo.

Per-feature acceptance expectations live in [`feature-expectations.md`](feature-expectations.md). Each milestone below links to the expectation files that define what the feature must prove without over-prescribing how it must be implemented.

## Product Goal

Ocentra Parent helps parents understand and guide what children do on connected devices. The product should answer concrete parent questions:

- Which apps, sites, games, chats, and video platforms are active?
- Is the child using the device for school work or drifting into social media, games, adult content, or other unsafe activity?
- What happened before an alert, timeout, permission request, or block?
- Can the system explain the evidence behind a decision?
- Can parents set boundaries without needing to become device administrators or network experts?

The long-term product is an agentic family-safety system. Child devices run local
agents with local AI safety evaluation, parent-owned surfaces show visibility and
controls, and remote services may later help with subscription, notification,
relay, or stateless report compilation. The parent surface is a rule-setting,
approval, and observability layer; it does not execute capture, AI, scripts,
policy evaluation, or enforcement. The child-device safety decision path stays
local: capture facts, store them safely, query them locally, then run local AI
and typed policy decisions before blocking, timing, asking the parent, syncing,
or notifying.

Ocentra does not decide the household rulebook. The product provides transparent
capabilities, local-first data handling, typed controls, and audit trails. Parents
decide which observation modes, schedules, categories, limits, and enforcement
actions are appropriate for their child and home.

Ocentra also does not own family data by default. Child activity evidence lives
on the child device, parent device, LAN, or a parent-configured storage provider.
Ocentra-hosted services are for downloads, account/subscription, entitlement,
update metadata, minimal notification routing, authenticated relay, and optional
stateless compile/report operations that do not retain child activity data.

## Current Position

We are currently in the evidence storage track.

Completed foundation:

- Repository scaffold with TypeScript workspaces, Rust crates, platform package scaffolds, validation gates, security scans, pre-commit hooks, and CI.
- Local Rust agent service and Vite portal using fixed ports.
- Loopback and LAN development modes with origin checks.
- WebSocket intent/event protocol between portal and agent.
- TypeScript domain contracts using Effect Schema.
- Rust protocol parity for shared contracts.
- Encrypted append-only activity journal.
- Journal rotation and replay validation.
- Windows MSI, updater scaffold, package-preview workflow, and production-branch release separation.

Current local slice:

- SQLite-backed activity query store replacing the earlier DuckDB direction.
- Encrypted journal remains the source of truth.
- SQLite is the default cross-platform local query/index store.
- DuckDB is not a core runtime dependency.

Next product slice:

- Windows process/window activity capture.

## Architecture Commitments

The storage architecture is:

```text
capture -> encrypted NDJSON journal -> SQLite query store -> local AI/policy/enforcement -> local API -> portal/reports
```

Rules:

- The encrypted journal is the required source of truth on every platform.
- SQLite is the default query/index store on every platform.
- Query stores are rebuildable from the journal.
- Portal code talks to typed service/query APIs, not directly to SQLite files.
- Portal code authors rules, approvals, and visibility requests; child-device agents validate and execute them.
- Portal code must not run OS commands, capture adapters, AI safety evaluation, policy evaluation, enforcement, timers, or scripts.
- Parent-authored rules decide household outcomes. Product defaults and category
  labels are evidence and control aids, not hidden Ocentra value judgments.
- Production parent portals are parent-owned local packaged apps or mobile apps
  first. Tauri is the preferred desktop-shell candidate; the current Vite portal
  is a dev scaffold until the packaged app exists.
- `family.ocentra.ca` is a public/download/account/subscription surface and may
  host authenticated status or stateless report compilation. It is not the
  default child-activity data store.
- Ocentra-hosted storage must not retain raw evidence, generated reports,
  screenshots, browser history, or parent rules by default.
- Child-device local AI is the required safety evaluator for page, video-link, app, domain, and activity context.
- Local AI receives typed evidence plus parent rules and returns typed decisions such as allow, warn, block, time-limit, or ask-parent.
- Enforcement adapters act only on schema-valid decisions and record audit events.
- Remote/API AI or hosted report compilation is optional and secondary: parent
  assistant, richer reports, unknown classification, or remote summaries only
  after explicit parent-controlled storage/privacy boundaries exist. It cannot
  replace the child-device local safety evaluator.
- DuckDB may return later as an optional analytics/export adapter, but not as the default app database.
- Web does not run the child-device agent or any child-device workflow. Parent
  surfaces talk to local, LAN, parent-owned storage, or cloud-routed services
  through typed contracts.

## Execution Standard

Each product slice should be small and real, but roadmap milestones such as V0.1, V0.2, and V0.3 are feature-branch units, not automatic product releases.

Slice workflow:

- Start from clean `main`.
- Create a `codex/<slice-slug>` branch.
- Build the smallest useful production slice inside that branch.
- Use focused local gates while developing.
- Commit locally as meaningful sub-slices are completed.
- Push the feature branch regularly when useful for backup, review, or remote CI signal.
- Do not treat every local sub-slice as a separate product release.
- Open or refresh the PR when the feature/milestone is ready to integrate.
- Run full local validation and build before PR merge.
- Use PR and `main` CI as integration gates, not production release gates.
- Fix PR CI failures on the same branch.
- Squash merge green PRs to `main`.
- Pull clean `main` before starting the next slice.

Production releases stay manual. Do not push or merge to `production` unless explicitly requested. It is valid to merge multiple milestones to `main` first, then create a product release later when requested for real install/download testing.

Validation expectations:

- TypeScript contracts use Effect Schema.
- Rust protocol structs mirror shared contract shape.
- No Zod.
- No naked domain strings.
- No app/runtime inline string literals.
- No test doubles, mocks, fakes, stubs, spies, MSW, Nock, Sinon, or fake green tests.
- Source files stay inside shape budgets.
- Portal UI changes get Playwright coverage.
- Runtime behavior gets real local service integration tests.

## Milestone Roadmap

### V0.1 Foundation And Evidence Contracts

Purpose:

Create the repo, contracts, and runtime boundaries that make future product code hard to fake.

Expectation links:

- [Feature request expectations](expectations/feature-request.md)
- [Universal done definition](expectations/universal-done.md)
- [Code quality expectations](expectations/code-quality.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)
- [Contract feature expectations](expectations/contracts.md)
- [Release and installer expectations](expectations/release-installer.md)

Deliverables:

- Workspace scaffold.
- Rust agent crates.
- Vite portal.
- Local and LAN dev scripts.
- Fixed ports.
- CI, hooks, security scan, dependency policy, SBOM.
- Installer and update scaffolding.
- Activity event contracts.
- Journal contracts.
- Query contracts.

Acceptance:

- Full validation passes.
- CI runs package previews.
- Contract tests prove TypeScript and Rust shape parity.
- README explains product intent and local dev loop.

Status:

- Mostly complete.

### V0.2 Trusted Local Evidence Store

Purpose:

Make the agent able to write and query trusted local facts before capture or blocking exists.

Expectation links:

- [Data custody and local-first expectations](expectations/data-custody.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Contract feature expectations](expectations/contracts.md)
- [Portal feature expectations](expectations/portal.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)

Deliverables:

- Encrypted append-only NDJSON journal.
- Journal rotation.
- Journal replay.
- SQLite query store.
- Duplicate event protection.
- Recent activity summary API.
- Ingest status API.
- Portal health/query controls for journal and ingest status.

Acceptance:

- Real journal entries are encrypted.
- Tampered ciphertext fails.
- Rotated segments replay in order.
- SQLite can rebuild from journal replay.
- Query summary uses real stored rows.
- Local WebSocket smoke proves the Rust service responds.
- Portal E2E proves the UI can reach the real agent.

Status:

- In progress. SQLite pivot is local.

### V0.3 Windows Process And Window Activity Capture

Purpose:

Start observing useful local activity without blocking, AI, or invasive content inspection.

Expectation links:

- [Capture feature expectations](expectations/capture.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Contract feature expectations](expectations/contracts.md)
- [Portal feature expectations](expectations/portal.md)
- [Platform expectations](expectations/platforms.md)

Deliverables:

- Windows process snapshot adapter.
- Active window observation adapter where available.
- Process lifecycle/activity events.
- Foreground app/window activity events.
- Activity source IDs for adapter, host, and observation mode.
- Capture health/status intent.
- Journal write path from real observations.
- SQLite ingest path from real observations.

Acceptance:

- The agent records real process/window observations on Windows.
- Events use existing activity contracts or deliberate contract extensions.
- Capture does not block the WebSocket service.
- Tests cover adapter mapping and service intent behavior.
- Manual local run shows current process/window evidence in the portal.

Out of scope:

- Blocking.
- Content inspection.
- Local AI decisioning, which begins at V0.6/V0.7 after capture evidence exists.
- Stealth or anti-tamper behavior.

### V0.4 Windows Network And Domain Observation

Purpose:

Observe network/domain activity enough to answer what services and sites are being used.

Expectation links:

- [Capture feature expectations](expectations/capture.md)
- [Network flow evidence expectations](expectations/network-flow-evidence.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Contract feature expectations](expectations/contracts.md)
- [Portal feature expectations](expectations/portal.md)
- [Platform expectations](expectations/platforms.md)

Deliverables:

- Windows network observation adapter.
- Domain/IP/port observation events.
- DNS/domain attribution where available.
- Process-to-network correlation where feasible.
- Unknown/suspicious destination categorization placeholder.
- Flow summaries for connection counts, first/last seen, bytes sent/received
  where available, top destinations, and unusual traffic digests.
- VPN/proxy/tunnel indicator status where available.
- Portal recent network activity view.

Acceptance:

- The agent records real network observations.
- Event model remains intent-first, not raw packet-first.
- No decrypted HTTPS payload capture.
- No raw packet dump as the normal evidence store.
- AI consumes only stored flow evidence references or agent-generated network
  digests; AI does not sniff packets or invent traffic.
- Portal can show recent domains and processes.
- Local tests prove parser/contract behavior.
- Integration smoke proves the service remains responsive while observing.

### V0.5 Live Activity Portal

Purpose:

Turn dev protocol proof into a usable local parent visibility surface.

Expectation links:

- [Portal feature expectations](expectations/portal.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Contract feature expectations](expectations/contracts.md)
- [Documentation expectations](expectations/documentation.md)

Deliverables:

- Activity timeline.
- Recent apps/processes.
- Recent domains.
- Device health.
- Journal/query-store status.
- Copy/export visible diagnostics for debugging.
- Dev log view backed by NDJSON logs.

Acceptance:

- One primary result panel updates instead of adding endless boxes.
- Parent can see what the agent is observing.
- Portal validates all service payloads through Effect Schema.
- Playwright covers health, recent activity, copy, and log visibility.

### V0.5.1 Browser URL And Tab Evidence Capture

Purpose:

Add the missing browser evidence layer that proves which supported browser tab is
open, which URL is active, and what page/domain is being observed. This is the
core product bridge before local AI can make useful page/site safety decisions.
The preferred boundary is an Ocentra-managed browser launch/profile with a
local browser-supported bridge. Browser extensions are not the default product
path for this milestone.

Expectation links:

- [Browser URL and tab evidence expectations](expectations/browser-evidence.md)
- [Browser URL and tab evidence capture architecture](architecture/browser-url-tab-evidence-capture.md)
- [Capture feature expectations](expectations/capture.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Contract feature expectations](expectations/contracts.md)
- [Portal feature expectations](expectations/portal.md)
- [Platform expectations](expectations/platforms.md)

Deliverables:

- Supported-browser detection contract.
- Running-browser detection contract.
- Ocentra-managed browser launch/profile contract.
- Browser window/tab evidence contract.
- Active-tab evidence contract.
- Exact URL, page title, normalized domain, timestamp, evidence id, source id,
  adapter id, and capability status.
- Browser integration status for unsupported, unmanaged browser, missing bridge,
  permission, stale evidence, and adapter-error states.
- Unmanaged-browser detection event for normal or alternate browsers that are
  running outside the managed Ocentra browser boundary.
- Journal/query-store ingest for browser evidence.
- Portal recent browser activity view.
- Local AI input references to browser evidence ids.

Acceptance:

- The system distinguishes "browser process is active" from "active tab URL is
  known."
- A supported browser can produce real active-tab URL/title/domain evidence in a
  local run through an Ocentra-managed browser session.
- Unsupported or permission-limited browser states are visible and typed.
- A supported or browser-like process running outside the managed browser
  boundary is reported as unmanaged browser use and possible bypass, not as a
  successful URL capture.
- Browser evidence is stored before portal or AI use.
- Local AI contracts can reference browser evidence without requiring page body
  content.
- No page body text, screenshots, keystrokes, form values, browser secrets, or
  decrypted HTTPS payloads are captured.

### V0.5.2 App And Game Evidence Sessions

Purpose:

Make native app and game usage queryable before AI or enforcement depends on it.
The Rust agent observes processes, foreground windows, installed app/game
metadata, launcher hints, and local session duration. AI may classify unknown or
ambiguous evidence from stored digests, but it does not scan the machine and it
does not invent duration.

Expectation links:

- [App and game evidence expectations](expectations/app-game-evidence.md)
- [App and game evidence sessions architecture](architecture/app-game-evidence-sessions.md)
- [Capture feature expectations](expectations/capture.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Policy feature expectations](expectations/policy.md)
- [Enforcement feature expectations](expectations/enforcement.md)
- [Portal feature expectations](expectations/portal.md)

Deliverables:

- Installed app/game inventory contract.
- Running process app/game observation contract.
- Foreground app/game session contract.
- App/game session summary contract with running time, foreground time, run
  count, first/last seen, and evidence references.
- Deterministic known-game catalog match before AI classification.
- AI digest/input contract that references stored app/game evidence.
- Policy target contract for app, process, launcher, game title, and category.
- Dry-run enforcement handoff for block, terminate, time-limit, and ask-parent.

Acceptance:

- The system can distinguish "Steam is running" from "a Steam game is running"
  when launcher/library/process evidence makes that distinction possible.
- Running time and foreground time come from journal/query evidence, not portal
  state or AI guesses.
- AI consumes only stored evidence references or agent-generated digests.
- Parent game/category time-limit rules can evaluate the session summary.
- Child-facing status can explain that a game/app was stopped by parent policy
  and ask for parent permission when enforcement is enabled.

### V0.5.3 Local Screen Evidence Analysis Queue

Purpose:

Add an optional local visual evidence layer for ambiguous activity. The Rust
agent captures screen/window images on a configurable cadence or trigger, stores
them only as encrypted temporary queue jobs, lets a local OCR/vision model
produce typed JSON summaries, then deletes the image. Policy consumes only
schema-valid summaries and evidence references.

Expectation links:

- [Screen evidence analysis expectations](expectations/screen-evidence.md)
- [Local screen evidence analysis queue architecture](architecture/local-screen-evidence-analysis-queue.md)
- [Capture feature expectations](expectations/capture.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [AI feature expectations](expectations/ai.md)
- [Policy feature expectations](expectations/policy.md)
- [Enforcement feature expectations](expectations/enforcement.md)
- [Portal feature expectations](expectations/portal.md)

Deliverables:

- Screen capture capability/status contract.
- Encrypted temporary screen-analysis queue contract with TTL and retry state.
- Local OCR/vision result contract with summary, categories, confidence, risk
  signals, source evidence refs, image digest, and deletion state.
- Parent opt-in settings for disabled-by-default enablement, cadence, triggers,
  strict mode, OCR snippets, redaction, TTL, deletion behavior, and policy-use
  state.
- Configurable cadence, such as five-minute default and one-minute strict mode.
- Triggered capture hooks for foreground app, managed URL, game/app foreground,
  and unusual network changes.
- Policy target support for visible activity categories and screen-derived risk
  signals.
- Parent-controlled enablement, cadence, trigger, retention/deletion settings,
  disclosure, and summary view.

Acceptance:

- Screen images do not leave the child PC for remote/API AI or cloud processing.
- Ocentra-hosted services do not store child screenshots, screen summaries,
  journals, SQLite evidence, generated reports, or parent rules by default.
- Temporary images are encrypted while queued and deleted after successful local
  analysis or TTL expiry.
- Rust validates AI JSON before journal/query ingest or policy use.
- Enforcement acts only from typed policy decisions, not raw AI text.
- Parent-facing UI/docs show whether screen analysis is enabled, who enabled it,
  the current cadence/triggers, retention/deletion status, and resulting policy
  decisions.

### V0.6 Local AI Safety Decision Contracts

Purpose:

Define the child-device local AI decision boundary before enforcing anything.

Expectation links:

- [AI feature expectations](expectations/ai.md)
- [Local AI and TabAgent reuse architecture](architecture/local-ai-and-tabagent-reuse.md)
- [Policy feature expectations](expectations/policy.md)
- [Contract feature expectations](expectations/contracts.md)
- [Evidence storage expectations](expectations/evidence-storage.md)

Deliverables:

- Parent/family/device contracts.
- Child profile contracts.
- App/site/category policy contracts.
- Time window contracts.
- Permission request contracts.
- Local AI input contract: evidence, URL/page/video/app context, parent rules, recent activity context.
- Local AI output contract: allow, warn, block, time-limit, ask-parent, unknown, confidence, reason, evidence references.
- Local screen-analysis summary reference contract for OCR/vision results when
  that capture slice exists.
- Local model/runtime status contracts inspired by TabAgentServer, but owned by Ocentra Parent.
- Memory and knowledge-graph reference contracts for future evidence-backed agent intelligence.
- Policy decision event contracts.
- Rust protocol parity.

Acceptance:

- Policies are schema-versioned.
- Local AI decision inputs and outputs are schema-versioned.
- Memory and graph references are schema-versioned and optional until implemented.
- Invalid policies are rejected at boundaries.
- Invalid AI decisions are rejected at boundaries.
- Policy events can be stored in the journal.
- Remote/API AI is not required for child-device decisions.
- No blocking behavior yet.

### V0.7 Local AI Policy Evaluator

Purpose:

Run local AI against captured activity and parent rules, then produce typed policy decisions.

Expectation links:

- [AI feature expectations](expectations/ai.md)
- [Local AI and TabAgent reuse architecture](architecture/local-ai-and-tabagent-reuse.md)
- [Policy feature expectations](expectations/policy.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Portal feature expectations](expectations/portal.md)

Deliverables:

- Local model/provider adapter boundary.
- TabAgentServer reuse review for model cache, execution providers, provider lifecycle, memory, and knowledge-graph ideas.
- Safety context builder from stored evidence, parent rules, recent activity, and local memory or graph references.
- Local AI policy evaluator crate/module.
- Allowed/limited/blocked decision model.
- Reason codes.
- Evidence references.
- Time-limit/timer decision model.
- Ask-parent decision model.
- Dry-run mode.
- Portal policy preview.

Acceptance:

- AI outputs are parsed into deterministic typed decisions.
- Memory and knowledge-graph-derived context points back to source evidence before it can influence a decision.
- Explicit parent rules override ambiguous AI output.
- Tests cover allow, timeout, block, ask-parent, unknown, and conflicting-policy cases.
- Portal can show why a decision happened.
- Enforcement is still disabled by default.

### V0.8 Enforcement Adapters

Purpose:

Start enforcing simple local decisions after policy evaluation is trusted.

Expectation links:

- [Enforcement feature expectations](expectations/enforcement.md)
- [Policy feature expectations](expectations/policy.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Platform expectations](expectations/platforms.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)

Deliverables:

- Windows enforcement adapter.
- Process block/terminate mode where appropriate.
- Network/domain block mode where appropriate.
- Managed-browser-only enforcement mode.
- Unmanaged-browser terminate/block mode for browser-like processes outside the
  Ocentra-managed browser boundary.
- Timeout mode.
- Timer-backed temporary block/unblock flow.
- Parent override/permission intent.
- Enforcement audit events.
- Safety rollback path.

Acceptance:

- Enforcement actions produce journal evidence.
- Parent can tell what was blocked and why.
- Blocks are scoped to configured policy.
- Unmanaged browser termination records the detected process, path/signature
  evidence where available, policy reason, result, and rollback/unavailable
  state.
- Service remains uninstallable and debuggable in dev builds.
- No hidden anti-tamper claims until explicitly designed.

### V0.9 LAN Pairing And Multi-Device Local Control

Purpose:

Let a parent device configure and observe another child device on the same local network while execution remains on the child-device agent.

Expectation links:

- [LAN pairing expectations](expectations/lan-pairing.md)
- [Contract feature expectations](expectations/contracts.md)
- [Portal feature expectations](expectations/portal.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)

Deliverables:

- Device discovery.
- Pairing flow.
- Pairing proof contract.
- Trusted-device registry.
- LAN rule/query/approval routing.
- Multi-device portal selector.

Acceptance:

- Pairing does not expose anonymous control.
- Origin and route checks remain enforced.
- Portal can switch between at least two local agents.
- Integration tests cover local routing contracts.

### V1.0 Local MVP

Purpose:

Ship a usable Windows-first local product for a parent to install and observe activity.

Expectation links:

- [Evidence storage expectations](expectations/evidence-storage.md)
- [Capture feature expectations](expectations/capture.md)
- [Portal feature expectations](expectations/portal.md)
- [Policy feature expectations](expectations/policy.md)
- [Release and installer expectations](expectations/release-installer.md)
- [Platform expectations](expectations/platforms.md)

Deliverables:

- Windows MSI install/uninstall.
- Headless service autostart.
- Local portal.
- Process/window capture.
- Network/domain observation.
- Encrypted journal.
- SQLite query store.
- Activity timeline.
- Basic local AI policy dry-run.
- Local-only reports.
- Update scaffold ready for production releases.

Acceptance:

- A parent can install on a Windows child PC.
- The parent can open the local/LAN portal and see real activity.
- The child-device agent can evaluate a narrow page, video-link, app, or domain observation against parent rules locally.
- The app can survive restart and continue writing evidence.
- The database can be rebuilt from journal.
- CI is green on `main`.

## Post-MVP Roadmap

### V2 Parent-Owned Remote Access And Cloud Relay

Purpose:

Support the parent-away-from-home use case without making Ocentra the family-data
store.

Expectation links:

- [Data custody and local-first expectations](expectations/data-custody.md)
- [Cloud feature expectations](expectations/cloud.md)
- [Sync and export expectations](expectations/sync-export.md)
- [LAN pairing expectations](expectations/lan-pairing.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)

Deliverables:

- Cloudflare control plane.
- Authenticated device registry.
- Remote rule/query/approval event relay.
- Parent-owned storage connector contracts.
- Sync queue for parent-owned storage or local cache.
- Stateless report compile contract where remote compilation exists.
- Conflict handling.
- Device heartbeat.
- `family.ocentra.ca` download/account/subscription/status surface.
- Packaged parent portal direction, with Tauri as preferred desktop candidate.

Acceptance:

- Parent can see device/account health remotely.
- Parent can view reports from local cache, reachable child agent, or
  parent-owned storage with source/custody clearly labeled.
- Local-first operation still works if cloud is unavailable.
- Device rule updates, approval decisions, and visibility requests are authenticated and auditable.
- Ocentra-hosted infrastructure does not retain child activity evidence or
  generated reports by default.

### V3 Notifications

Purpose:

Notify parents when attention is needed.

Expectation links:

- [Notification feature expectations](expectations/notifications.md)
- [Policy feature expectations](expectations/policy.md)
- [Evidence storage expectations](expectations/evidence-storage.md)

Deliverables:

- Notification domain contracts.
- Provider adapter boundary.
- Push/email/WhatsApp candidates.
- Alert rules.
- Alert audit trail.
- Quiet hours and escalation policy.

Acceptance:

- Alerts explain evidence and policy reason.
- Alert bodies minimize child details and link back to an authenticated parent
  surface for sensitive context.
- Provider failures are logged and retryable.
- Parents can tune noise.

### V4 Parent-Owned Reports And Optional Assistant

Purpose:

Use local or explicitly parent-authorized report/assistant flows for richer
parent explanations without replacing local child-device safety decisions or
creating default Ocentra custody of child data.

Expectation links:

- [Data custody and local-first expectations](expectations/data-custody.md)
- [AI feature expectations](expectations/ai.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Policy feature expectations](expectations/policy.md)
- [Contract feature expectations](expectations/contracts.md)

Deliverables:

- Parent assistant contract.
- Local assistant and optional API model adapter boundaries.
- Prompt/version governance.
- Evidence-grounded explanation.
- Cross-device and long-window summary support from parent-owned storage or
  local cache.
- Stateless report compile request/result contracts.
- Human override feedback loop.

Acceptance:

- Report/assistant output references stored evidence without copying raw evidence
  into Ocentra-hosted storage by default.
- Child-device blocking does not require remote/API AI availability.
- API classifier failures degrade to local-only evaluation, unknown, or ask-parent.
- Tests cover schema and decision boundaries.

### V5 Parent Policy Product

Purpose:

Make policy management usable for non-technical parents.

Expectation links:

- [Policy feature expectations](expectations/policy.md)
- [Portal feature expectations](expectations/portal.md)
- [Sync and export expectations](expectations/sync-export.md)
- [Billing and subscription expectations](expectations/billing.md)

Deliverables:

- Parent accounts.
- Family setup.
- Child profiles.
- App/site/category rules.
- Time budgets.
- Permission requests.
- Schedules.
- Reports.
- Audit history.
- Source/custody indicators for local, LAN, parent-owned storage, and
  Ocentra-hosted non-activity metadata.

Acceptance:

- Parent can configure common rules without editing files.
- Child activity and policy decisions are explainable.
- Settings sync safely across devices through local or parent-owned storage
  boundaries.

### V6 Mobile Agents

Purpose:

Extend child-device support beyond Windows.

Expectation links:

- [Platform expectations](expectations/platforms.md)
- [Capture feature expectations](expectations/capture.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Release and installer expectations](expectations/release-installer.md)

Deliverables:

- Android foreground/device-admin path where allowed.
- iOS capability investigation and approved APIs.
- Shared Rust/domain core where practical.
- Platform-specific capture/enforcement adapters.
- Mobile packaging and store-readiness docs.

Acceptance:

- Platform claims match real OS capabilities.
- Android and iOS do not pretend to have desktop-level control if the OS forbids it.
- Mobile agents share contracts and journal format.

### V7 Subscription And Monetization

Purpose:

Turn the product into a sellable service.

Expectation links:

- [Billing and subscription expectations](expectations/billing.md)
- [Portal feature expectations](expectations/portal.md)
- [Cloud feature expectations](expectations/cloud.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)

Deliverables:

- Plans and entitlements.
- Stripe billing.
- Trial flow.
- Device limits.
- Subscription status sync.
- Admin/support flows.

Acceptance:

- Paid features are entitlement-gated.
- Billing state is auditable.
- Local safety behavior degrades responsibly if billing checks are unavailable.

### V8 Production Hardening

Purpose:

Make the product reliable, secure, supportable, and maintainable.

Expectation links:

- [Release and installer expectations](expectations/release-installer.md)
- [Sync and export expectations](expectations/sync-export.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)
- [Documentation expectations](expectations/documentation.md)
- [Code quality expectations](expectations/code-quality.md)

Deliverables:

- Signing for Windows releases.
- macOS signing/notarization if macOS agent ships.
- Android signing/store release.
- iOS store/TestFlight path if iOS agent ships.
- Crash reporting.
- Update rollback.
- Backup/export.
- Privacy and retention controls.
- Legal/compliance review.
- Threat model review.
- Abuse-resistance design.

Acceptance:

- Install/update/uninstall paths are proven.
- Parents can export or delete family data.
- Security boundaries are documented and tested.
- Product claims match actual behavior.

## Platform Strategy

Windows:

- First real implementation target.
- MSI installer, service autostart, local capture, local policy, local enforcement.

macOS:

- Scaffolded early.
- Capture/enforcement depends on approved system APIs and permissions.
- Do not claim parity until proven.

Linux:

- Scaffolded early.
- Useful for CI and local testing.
- Desktop capture/enforcement adapters can be added after Windows.

Android:

- Scaffolded early.
- Likely foreground service plus OS-approved management APIs.
- SQLite should be the default local query store.
- Store/device-owner restrictions must be handled honestly.

iOS:

- Scaffolded early.
- Most restrictive child-device platform.
- Use Apple-approved capabilities only.
- Do not claim capture or enforcement beyond what entitlements allow.

Web:

- Parent portal/control surface.
- Does not run the child-device agent.
- Does not run child-device capture, AI, policy evaluation, enforcement, timers, or scripts.
- Talks to local/LAN/cloud-routed services through typed rule, approval, query, and event contracts.

## Data And Privacy Principles

- Store facts before analysis.
- Keep raw evidence local by default.
- Encrypt the append-only journal.
- Make query stores rebuildable.
- Record evidence references for policy and AI decisions.
- Run child-device safety decisions locally on the child device.
- Send evidence to remote/API AI only through explicit parent action,
  data-custody, privacy, retention, and deletion boundaries.
- Avoid decrypted content capture unless a future explicit product/legal decision approves a specific boundary.
- Make data export and deletion first-class before paid production launch.

## Release Strategy

Main branch:

- CI integration branch.
- Builds and validates package previews.
- Does not publish production releases.
- Can contain multiple completed milestones before any product release is made.

Feature branches:

- Own active milestone work such as V0.2 evidence storage or V0.3 Windows capture.
- May contain several local commits.
- May be pushed regularly for backup, review, or remote CI signal.
- Should open a final PR when the milestone is ready to merge to `main`.
- Should not publish product releases.

Production branch:

- Manual promotion only.
- Publishes versioned releases when secrets and version tags are ready.
- Used only when explicitly requested for a real product release or install/download test.

Product release cadence:

- Product releases are deliberate checkpoints, not every merge to `main`.
- A release may include one milestone or a batch of milestones, such as V0.1 through V0.5, depending on what is useful to install and test.
- When a release is requested, first confirm `main` is green, versioning is intentional, release notes match actual behavior, and production signing/update requirements are satisfied or explicitly scoped.

Local development:

- Focused gates while coding.
- Full validation and build before PR merge.
- Do not wait idly on long CI if an independent slice can progress.

## Current Next Actions

Current completed-on-main foundation:

- V0.1/V0.2 scaffold, contracts, encrypted journal, and SQLite query store.
- V0.3 Windows process/window capture.
- V0.4 Windows network/domain observation foundation.
- V0.5 live activity portal visibility.
- V0.5.1 browser URL/tab evidence research/spec.
- V0.5.1 browser URL/tab managed bridge implementation plan.
- V0.5.1 managed browser bridge runtime boundary with typed browser state
  contracts, local evidence storage, and portal read visibility.
- V0.5.2 app/game evidence sessions research/spec.
- V0.5.3 local screen evidence analysis queue research/spec.
- Network flow evidence research/spec.
- V0.6 local AI safety decision contract groundwork.
- V0.6 local AI evidence context-builder reconciliation plan.

Next coordinator slices:

1. Rebase active app/game and network evidence branches on latest `main` after
   the V0.5.1 browser bridge runtime merge.
2. Implement the app/game session read model next, keeping process/window
   capture as input evidence rather than proof by itself.
3. Implement network flow evidence runtime read models and portal visibility
   from stored flow evidence, without claiming exact browser URL/tab or
   decrypted content.
4. Implement the local screen evidence queue after the browser and app/game read
   paths have stable evidence references.
5. Start the local AI dry-run evaluator only after browser, app/game, network,
   and screen evidence references can all be read through typed local contracts.
