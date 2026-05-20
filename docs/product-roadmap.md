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

The long-term product is an agentic family-safety system. Child devices run local agents, parent surfaces show visibility and controls, and AI helps classify, explain, and recommend actions. The foundation must be evidence-first: capture facts, store them safely, query them locally, then build policy, blocking, sync, notifications, and AI on top.

## Current Position

We are currently in the evidence storage track.

Completed foundation:

- Repository scaffold with TypeScript workspaces, Rust crates, platform package scaffolds, validation gates, security scans, pre-commit hooks, and CI.
- Local Rust agent service and Vite portal using fixed ports.
- Loopback and LAN development modes with origin checks.
- WebSocket command/event protocol between portal and agent.
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
capture -> encrypted NDJSON journal -> SQLite query store -> local API -> portal/policy/reports
```

Rules:

- The encrypted journal is the required source of truth on every platform.
- SQLite is the default query/index store on every platform.
- Query stores are rebuildable from the journal.
- Portal and policy code talk to typed service/query APIs, not directly to SQLite files.
- DuckDB may return later as an optional analytics/export adapter, but not as the default app database.
- Web does not run the child-device agent. The portal talks to local, LAN, or cloud-routed agents.

## Execution Standard

Each product slice should be small and real.

Slice workflow:

- Start from clean `main`.
- Create a `codex/<slice-slug>` branch.
- Build the smallest useful production slice.
- Use focused local gates while developing.
- Run full local validation and build before PR merge.
- Commit locally after the slice is validated.
- Push PR branch when the slice is ready for CI.
- Fix CI failures on the same branch.
- Squash merge green PRs to `main`.
- Pull clean `main` before starting the next slice.

Production releases stay manual. Do not push or merge to `production` unless explicitly requested.

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
- Portal command buttons for journal/query health.

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
- Capture health/status command.
- Journal write path from real observations.
- SQLite ingest path from real observations.

Acceptance:

- The agent records real process/window observations on Windows.
- Events use existing activity contracts or deliberate contract extensions.
- Capture does not block the WebSocket service.
- Tests cover adapter mapping and service command behavior.
- Manual local run shows current process/window evidence in the portal.

Out of scope:

- Blocking.
- Content inspection.
- AI classification.
- Stealth or anti-tamper behavior.

### V0.4 Windows Network And Domain Observation

Purpose:

Observe network/domain activity enough to answer what services and sites are being used.

Expectation links:

- [Capture feature expectations](expectations/capture.md)
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
- Portal recent network activity view.

Acceptance:

- The agent records real network observations.
- Event model remains intent-first, not raw packet-first.
- No decrypted HTTPS payload capture.
- Portal can show recent domains and processes.
- Local tests prove parser/contract behavior.
- Integration smoke proves the service remains responsive while observing.

### V0.5 Live Activity Portal

Purpose:

Turn dev command proof into a usable local parent visibility surface.

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

### V0.6 Policy Contracts

Purpose:

Define rules before enforcing anything.

Expectation links:

- [Policy feature expectations](expectations/policy.md)
- [Contract feature expectations](expectations/contracts.md)
- [Evidence storage expectations](expectations/evidence-storage.md)

Deliverables:

- Parent/family/device contracts.
- Child profile contracts.
- App/site/category policy contracts.
- Time window contracts.
- Permission request contracts.
- Policy decision event contracts.
- Rust protocol parity.

Acceptance:

- Policies are schema-versioned.
- Invalid policies are rejected at boundaries.
- Policy events can be stored in the journal.
- No blocking behavior yet.

### V0.7 Local Policy Evaluator

Purpose:

Evaluate activity against parent rules and explain decisions.

Expectation links:

- [Policy feature expectations](expectations/policy.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Portal feature expectations](expectations/portal.md)

Deliverables:

- Local policy evaluator crate/module.
- Allowed/limited/blocked decision model.
- Reason codes.
- Evidence references.
- Dry-run mode.
- Portal policy preview.

Acceptance:

- Evaluator decisions are deterministic.
- Tests cover allow, timeout, blocked, unknown, and conflicting-policy cases.
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
- Timeout mode.
- Manual override/permission command.
- Enforcement audit events.
- Safety rollback path.

Acceptance:

- Enforcement actions produce journal evidence.
- Parent can tell what was blocked and why.
- Blocks are scoped to configured policy.
- Service remains uninstallable and debuggable in dev builds.
- No hidden anti-tamper claims until explicitly designed.

### V0.9 LAN Pairing And Multi-Device Local Control

Purpose:

Let a parent device control or observe another child device on the same local network.

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
- LAN command routing.
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
- Basic policy dry-run.
- Local-only reports.
- Update scaffold ready for production releases.

Acceptance:

- A parent can install on a Windows child PC.
- The parent can open the local/LAN portal and see real activity.
- The app can survive restart and continue writing evidence.
- The database can be rebuilt from journal.
- CI is green on `main`.

## Post-MVP Roadmap

### V2 Cloud Relay And Remote Parent Access

Purpose:

Support the parent-away-from-home use case.

Expectation links:

- [Cloud feature expectations](expectations/cloud.md)
- [Sync and export expectations](expectations/sync-export.md)
- [LAN pairing expectations](expectations/lan-pairing.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)

Deliverables:

- Cloudflare control plane.
- Authenticated device registry.
- Remote command/event relay.
- Sync queue.
- Conflict handling.
- Device heartbeat.
- Remote portal at `family.ocentra.ca`.

Acceptance:

- Parent can see device health remotely.
- Local-first operation still works if cloud is unavailable.
- Device commands are authenticated and auditable.

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
- Provider failures are logged and retryable.
- Parents can tune noise.

### V4 AI Classification And Explanation

Purpose:

Use AI to classify activity and explain risk without replacing evidence.

Expectation links:

- [AI feature expectations](expectations/ai.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Policy feature expectations](expectations/policy.md)
- [Contract feature expectations](expectations/contracts.md)

Deliverables:

- Classification contract.
- Local classifier adapter where feasible.
- API model adapter.
- Prompt/version governance.
- Evidence-grounded explanation.
- Human override feedback loop.

Acceptance:

- AI output references stored evidence.
- Policy does not depend on untraceable AI verdicts.
- Classifier failures degrade to unknown, not unsafe allow.
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

Acceptance:

- Parent can configure common rules without editing files.
- Child activity and policy decisions are explainable.
- Settings sync safely across devices.

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
- Talks to local/LAN/cloud-routed services.

## Data And Privacy Principles

- Store facts before analysis.
- Keep raw evidence local by default.
- Encrypt the append-only journal.
- Make query stores rebuildable.
- Record evidence references for policy and AI decisions.
- Avoid decrypted content capture unless a future explicit product/legal decision approves a specific boundary.
- Make data export and deletion first-class before paid production launch.

## Release Strategy

Main branch:

- CI integration branch.
- Builds and validates package previews.
- Does not publish production releases.

Production branch:

- Manual promotion only.
- Publishes versioned releases when secrets and version tags are ready.

Local development:

- Focused gates while coding.
- Full validation and build before PR merge.
- Do not wait idly on long CI if an independent slice can progress.

## Current Next Actions

1. Finish the SQLite activity query store PR.
2. Merge it after CI is green.
3. Start Windows process/window activity capture.
4. Add portal activity visibility for real captured events.
5. Then move into Windows network/domain observation.
