# Ocentra Parent Feature Expectations

This document tells future AI agents and human contributors what a feature is expected to deliver. It is not a step-by-step implementation recipe. The point is to make expectations precise enough that an agent can choose the implementation path while still being held to a hard product, safety, test, and code-quality bar.

## Prime Directive

Every feature must move Ocentra Parent toward a trustworthy family-safety product:

- Parents get clear, useful visibility and control.
- Child-device evidence is real, typed, timestamped, and auditable.
- Data is protected locally before it is queried, synced, classified, or acted on.
- Product claims never exceed what the implementation can prove.
- Validation must make lazy or fake implementations fail.

If a change cannot explain what parent problem it solves, what evidence it creates or consumes, and how it is validated, it is not ready.

## Feature Request Expectations

Every feature request should make these expectations explicit before implementation starts.

Required:

- Parent outcome: what parent-visible problem is solved.
- Child-device outcome: what changes on the child device, if anything.
- Platform scope: Windows, macOS, Linux, Android, iOS, web portal, Cloudflare, or shared domain only.
- Data scope: what data is observed, stored, queried, synced, or displayed.
- Trust boundary: local-only, LAN, cloud-routed, authenticated parent, unauthenticated dev, or installer/update path.
- Contract boundary: TypeScript domain, Rust protocol, service command/event, portal UI, release asset, or external provider.
- Success evidence: exact behavior that proves the feature works.
- Failure behavior: what happens when permissions, OS APIs, network, storage, cloud, provider, or model calls fail.
- Non-goals: what the feature must not claim or implement yet.
- Validation gate: focused tests during development and final gate before merge.

Preferred:

- Example event or payload shape.
- Example parent-facing copy or UI state, owned by text/domain packages.
- Security/privacy notes.
- Performance or concurrency expectations.
- Rollback/update considerations if release code changes.

## Universal Done Definition

A feature is done only when all relevant expectations are true:

- Contracts exist before runtime consumers depend on them.
- TypeScript runtime validation uses Effect Schema.
- Rust protocol structs mirror shared contract shapes when crossing the Rust boundary.
- Source of truth is clear.
- Runtime apps do not own protocol strings, route ids, field names, event names, policy ids, or display text literals.
- No naked domain strings are introduced.
- No test doubles are introduced.
- Tests prove behavior with real parsers, real storage, real service boundaries, or real UI automation.
- Failure paths are specified and tested where feasible.
- Dev logs or status surfaces expose enough information to debug without guessing.
- Docs are updated when a feature changes product behavior, architecture, release behavior, or platform claims.
- Local validation appropriate to the change passes.

## Code Quality Expectations

Code quality is a product feature in this repo. Future agents should not rely on taste, memory, or convention alone. The repo should force quality through contracts, lint rules, tests, source-shape checks, and CI.

Required:

- Keep modules small and owned by one reason to change.
- Split files before they become catch-all modules.
- Prefer shared domain packages over app-local constants.
- Prefer Rust protocol constants over inline service/core strings.
- Prefer explicit typed adapters over generic bags of data.
- Keep async request handlers responsive.
- Move blocking OS, filesystem, database, or provider calls behind deliberate blocking boundaries.
- Avoid global mutable state unless the ownership and lock scope are clear.
- Prefer deterministic state transitions over clever parallelism.
- Avoid broad refactors unrelated to the feature.

Forbidden:

- God files.
- God classes.
- Fake green tests.
- Mocked service behavior.
- New Zod usage.
- Manual string brands.
- Raw domain-bearing strings.
- UI-only demos that bypass the real agent or real contracts.
- Product claims that are not backed by implementation.

## Static Analysis And Security Expectations

The repo should treat static security findings as blockers unless proven irrelevant.

Current expectations:

- Secret scan must pass.
- Dependency policy must pass.
- SBOM generation must pass when dependencies change.
- Rust clippy must pass with warnings denied.
- TypeScript lint and type-check must pass.
- Source-shape guard must pass.
- Test-double guard must pass.
- Rust string-boundary guard must pass.
- App string-literal guard must pass.

Future CodeQL expectation:

- If CodeQL is added, new CodeQL alerts are merge blockers.
- A CodeQL alert may be dismissed only with a documented reason and a narrow code reference.
- Do not suppress CodeQL findings globally.
- Do not hide security findings behind generated code or broad ignore paths unless the generated boundary is documented and separately validated.

## Contract Feature Expectations

Contract features define meaning shared across runtimes.

Expected deliverables:

- Effect Schema contract in the owning domain package.
- Branded primitives for domain-bearing text.
- Parser helpers from the repo's schema-domain helpers.
- Exact valid and invalid TypeScript tests.
- Rust protocol struct when Rust sends or receives the shape.
- Rust serialization/parity test with exact field names and values.
- Protocol constants for commands, events, fields, ids, and stable strings.

Acceptance:

- Invalid payloads fail at the boundary.
- Valid payloads parse into branded/domain types.
- Rust and TypeScript agree on schema version, field names, and enum values.
- The contract does not leak implementation-specific storage details unless that is the contract's purpose.

Non-goals:

- Do not add runtime behavior just because a contract exists.
- Do not add broad future fields without a concrete expected use.

## Evidence Storage Expectations

Evidence storage features must protect facts before analysis.

Expected deliverables:

- Encrypted append-only journal write path.
- Replayable event format.
- Rotation policy.
- Tamper rejection.
- Query-store ingest.
- Query-store status.
- Rebuild path from journal to query store.
- Health/status payload exposed through the agent service.

Acceptance:

- Plain activity payloads do not appear in the journal file.
- Tampered journal lines fail to decrypt or parse.
- Rotated segments replay in write order.
- Duplicate events are not double-counted in the query store.
- SQLite queries return exact expected summaries.
- Query-store loss is recoverable by replaying the journal.

Non-goals:

- SQLite is not the evidence source of truth.
- Do not make policy or AI decisions directly from unreplayed raw files.

## Capture Feature Expectations

Capture features create real observations from a child device.

Expected deliverables:

- Platform-specific adapter behind a platform-neutral boundary.
- Capability/status command.
- Observation event mapping.
- Source id and observer id.
- Failure reason when the OS capability is unavailable.
- Journal write path from real observations.
- Query-store ingest path from real observations.
- Dev portal visibility for captured evidence.

Acceptance:

- Tests prove mapping from adapter observation to activity event.
- Service remains responsive while capture is active.
- Capture can be disabled in dev.
- Capture failures do not crash the service.
- Platform claims are scoped to real tested behavior.

Windows process/window capture expectations:

- Observe process identity.
- Observe foreground window/app when available.
- Record timestamps and source ids.
- Avoid blocking the WebSocket command loop.
- Do not claim browser URL visibility from process/window capture alone.

Windows network/domain observation expectations:

- Observe domain/IP/port/process correlation where available.
- Prefer normalized intent events over raw packet dumps.
- Do not decrypt HTTPS payloads.
- Do not claim content inspection.
- Record unknown attribution clearly instead of guessing.

## Portal Feature Expectations

Portal features must exercise the real agent path.

Expected deliverables:

- UI reads typed domain/protocol contracts.
- UI validates agent events through Effect Schema.
- UI uses text/domain packages for display text.
- One clear result area for command output where appropriate.
- Copy/debug affordance for sharing current result.
- Playwright coverage when UI behavior changes.

Acceptance:

- Portal connects to the real local service in tests.
- Playwright proves the visible behavior.
- Command clicks update existing panels instead of appending endless boxes unless the feature is explicitly a log view.
- Logs/history use a table or timeline pattern.
- UI remains usable on common desktop and mobile widths.

Non-goals:

- Do not bypass the Rust service with hardcoded browser state.
- Do not create a polished marketing dashboard before the underlying data path exists.

## Policy Feature Expectations

Policy features define what parents want the system to allow, limit, warn about, or block.

Expected deliverables:

- Parent/family/child/device contracts where needed.
- Policy rule contracts.
- Schedule/time-window contracts.
- Category/app/site/domain target contracts.
- Permission request contracts.
- Decision reason codes.
- Dry-run evaluator before enforcement.

Acceptance:

- Invalid rules fail schema validation.
- Conflicting rules have deterministic resolution.
- Policy decisions reference evidence.
- Decision events are journaled.
- Parent-facing explanation is stable and testable.

Non-goals:

- Do not enforce policy until the evaluator is trusted.
- Do not make AI the only source of a policy decision.

## Enforcement Feature Expectations

Enforcement features change device behavior and therefore need a higher bar.

Expected deliverables:

- Adapter boundary per platform.
- Explicit policy decision input.
- Enforcement action event.
- Reason code.
- Evidence reference.
- Manual override or safe rollback path.
- Clear status when enforcement capability is unavailable.

Acceptance:

- Enforcement acts only after a typed policy decision.
- Every action is journaled.
- Parent can see what happened and why.
- Failure to enforce is reported.
- Enforcement tests cover allowed, blocked, timeout, unavailable, and rollback paths where feasible.

Non-goals:

- Do not add stealth, anti-tamper, privilege escalation, or persistence-hardening claims without explicit product/security design.

## LAN Pairing Expectations

LAN features expose the child-device agent beyond loopback and must be treated as trust-boundary work.

Expected deliverables:

- Explicit LAN enablement.
- Origin allowlist.
- Pairing proof contract.
- Trusted device registry.
- Device identity display.
- Multi-device command routing.

Acceptance:

- Anonymous LAN control is rejected.
- Loopback remains the default.
- Pairing state is auditable.
- Portal can distinguish devices.
- Tests cover rejected and accepted routes.

Non-goals:

- Do not treat LAN as production auth.
- Do not expose broad unauthenticated control APIs.

## Cloud Feature Expectations

Cloud features support parent-away-from-home use cases.

Expected deliverables:

- Cloudflare control-plane boundary.
- Authenticated parent identity.
- Authenticated device identity.
- Device heartbeat.
- Command/event relay.
- Sync queue.
- Retry/backoff behavior.
- Conflict handling.
- Local-first fallback.

Acceptance:

- Local operation works when cloud is unavailable.
- Remote commands are authenticated and auditable.
- Device state cannot be overwritten silently by stale cloud state.
- Cloud logs do not leak sensitive child activity beyond intended product data.

Non-goals:

- Do not replace local evidence storage with cloud-only storage.
- Do not add paid provider requirements to local development.

## Notification Feature Expectations

Notification features should reduce parent anxiety, not create noise.

Expected deliverables:

- Notification contract.
- Alert reason codes.
- Provider adapter boundary.
- Delivery status.
- Retry/failure handling.
- Quiet hours.
- Parent preference controls.

Acceptance:

- Notifications reference evidence and policy reason.
- Provider failure is visible and retryable.
- Parents can tune frequency.
- Sensitive details are minimized in push/WhatsApp/email bodies.

Non-goals:

- Do not send alerts from raw unclassified noise.
- Do not hardcode one provider into core policy logic.

## AI Feature Expectations

AI features assist classification, explanation, and recommendations. They do not replace evidence.

Expected deliverables:

- AI input contract.
- AI output contract.
- Model/provider adapter boundary.
- Prompt/version ownership.
- Evidence references.
- Confidence/unknown state.
- Failure/degraded behavior.
- Human override feedback path where relevant.

Acceptance:

- AI output is schema-validated.
- AI output points to stored evidence.
- Unknown or failed classification is safe and explicit.
- Policy can explain when AI contributed and when it did not.
- Tests cover parser behavior and decision integration without mocking provider truth.

Non-goals:

- Do not claim AI can see content that was not captured.
- Do not let untraceable AI output directly enforce blocking.
- Do not hide model/provider calls inside unrelated modules.

## Sync And Export Expectations

Sync/export features move family data across boundaries and need privacy discipline.

Expected deliverables:

- Export contract.
- Encryption boundary.
- Retention policy.
- Import/replay behavior.
- Sync status.
- Conflict model.
- Parent-visible export/delete controls before paid production.

Acceptance:

- Exported data is encrypted or intentionally human-readable with explicit parent action.
- Import validates schema versions.
- Sync failures do not corrupt local evidence.
- Parent can understand what data moved where.

Non-goals:

- Do not silently upload raw evidence before cloud privacy decisions are made.

## Release And Installer Expectations

Release features are product features because parents need install/update paths that work.

Expected deliverables:

- Version policy.
- Installer artifact.
- Install smoke.
- Uninstall smoke.
- Update manifest where applicable.
- Signature verification where applicable.
- Clear production/manual release boundary.

Acceptance:

- `main` builds previews and does not publish production releases.
- `production` publishes only by explicit promotion.
- Installer paths are documented.
- Update paths reject unsigned or incorrectly signed manifests once signing is enabled.
- Package claims match real artifacts.

Non-goals:

- Do not claim store distribution, notarization, or full signing before credentials and workflows exist.

## Billing And Subscription Expectations

Billing features should gate paid product value without breaking local child safety irresponsibly.

Expected deliverables:

- Plan contract.
- Entitlement contract.
- Stripe boundary.
- Billing status sync.
- Trial state.
- Device limit policy.
- Grace/failure behavior.

Acceptance:

- Paid features check entitlements through typed contracts.
- Billing failures are visible.
- Local safety behavior degrades deliberately when billing cannot be checked.
- No billing secret is committed or exposed to the portal.

Non-goals:

- Do not put Stripe logic inside capture, journal, or enforcement modules.

## Platform Expectations

Windows:

- First production-grade agent target.
- Service, MSI, process/window capture, network observation, local policy, and enforcement are expected here first.

macOS:

- Scaffold and package preview are useful early.
- Capture/enforcement claims require real permission/API proof.

Linux:

- Useful for CI, package proof, and future desktop support.
- Do not assume Windows capture adapters apply.

Android:

- SQLite is the expected local query store.
- Use platform-approved foreground/device-management capabilities.
- Do not claim desktop-level control unless device-owner policy or equivalent is actually implemented.

iOS:

- Most restrictive target.
- Use Apple-approved capabilities and entitlements only.
- Do not claim background monitoring or enforcement beyond proven APIs.

Web:

- Parent portal and control surface only.
- Does not run the child-device agent.

## Documentation Expectations

Docs must change when the product claim changes.

Update docs when changing:

- Roadmap status.
- Architecture boundaries.
- Public commands or contracts.
- Storage model.
- Platform support claims.
- Installer/update behavior.
- Security/privacy posture.
- Validation gates.

Docs should not:

- Claim future features as implemented.
- Hide uncertainty.
- Use marketing wording where a technical limitation matters.

## AI Agent Handoff Expectations

When an AI agent starts a feature, it should identify:

- Current branch and worktree cleanliness.
- Relevant roadmap milestone.
- Feature expectation sections that apply.
- Existing contracts and tests.
- Exact focused gates for iteration.
- Final gate before merge.
- Product claims being added or changed.

When an AI agent finishes a feature, it should report:

- What changed.
- What parent-visible behavior exists now.
- What product claim is now true.
- What remains intentionally out of scope.
- Exact validations run.
- Current git state.
