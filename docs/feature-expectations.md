<!-- agent-capsule -->

> Agent Capsule
> Doc: Ocentra Parent Feature Expectations
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Ocentra Parent Feature Expectations

This is the entry point for feature expectations. It tells future AI agents and human contributors where to look before implementing a feature. The expectation files define what a feature must prove; they do not prescribe a single implementation path.

For product meaning and status language, read
[Product Constitution](product-constitution.md) first. For the current
feature-by-feature status, read
[Product Capability Checklist](product-capability-checklist.md). For
per-feature docs, read [Feature List](feature-list.md). For competitor parity
gaps, read [Competitor Capability Map](competitor-capability-map.md).

## Prime Directive

Every feature must move Ocentra Parent toward a trustworthy family-safety product:

- Parents get clear, useful visibility and control.
- Parents own household rules. Ocentra provides transparent options, typed
  controls, evidence, and audit trails instead of hard-coded value judgments.
- Child-device evidence is real, typed, timestamped, and auditable.
- Data is protected locally before it is queried, synced, classified, or acted on.
- Ocentra-hosted services do not become the default store for child activity,
  journals, screenshots, reports, or parent rules.
- Product claims never exceed what the implementation can prove.
- Validation must make lazy or fake implementations fail.

If a change cannot explain what parent problem it solves, what evidence it creates or consumes, and how it is validated, it is not ready.

## Universal Expectations

- [Product constitution](product-constitution.md)
- [Feature list](feature-list.md)
- [Product capability checklist](product-capability-checklist.md)
- [Competitor capability map](competitor-capability-map.md)
- [Feature request expectations](expectations/feature-request.md)
- [Universal done definition](expectations/universal-done.md)
- [Code quality expectations](expectations/code-quality.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)
- [Documentation expectations](expectations/documentation.md)
- [AI agent handoff expectations](expectations/ai-agent-handoff.md)

## Feature Expectations

- [Contract feature expectations](expectations/contracts.md)
- [Family setup expectations](expectations/family-setup.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Data custody and local-first expectations](expectations/data-custody.md)
- [Capture feature expectations](expectations/capture.md)
- [Network flow evidence expectations](expectations/network-flow-evidence.md)
- [Browser URL and tab evidence expectations](expectations/browser-evidence.md)
- [App and game evidence expectations](expectations/app-game-evidence.md)
- [Screen evidence analysis expectations](expectations/screen-evidence.md)
- [Portal feature expectations](expectations/portal.md)
- [Policy feature expectations](expectations/policy.md)
- [Enforcement feature expectations](expectations/enforcement.md)
- [Social and video control expectations](expectations/social-video-control.md)
- [Location and geofence expectations](expectations/location-geofence.md)
- [App install and purchase approval expectations](expectations/app-install-purchase-approval.md)
- [Tamper and uninstall protection expectations](expectations/tamper-uninstall-protection.md)
- [LAN pairing expectations](expectations/lan-pairing.md)
- [Cloud feature expectations](expectations/cloud.md)
- [Notification feature expectations](expectations/notifications.md)
- [AI feature expectations](expectations/ai.md)
- [Parent assistant chat expectations](expectations/parent-assistant-chat.md)
- [Sync and export expectations](expectations/sync-export.md)
- [Release and installer expectations](expectations/release-installer.md)
- [Billing and subscription expectations](expectations/billing.md)
- [Platform expectations](expectations/platforms.md)

## Feature Documentation Requirement

Every product capability must have a row in
[Product Capability Checklist](product-capability-checklist.md) and a
per-feature document linked from [Feature List](feature-list.md). A capability
that is user-visible, privacy-sensitive, platform-specific, AI-backed,
enforcement-backed, remote, or subscription-gated must also point to an
expectation file.

Each feature row or expectation file must answer:

- What parent-visible behavior should exist?
- Which child-device/runtime behavior is required?
- Which platforms are in scope?
- What data is captured, stored, synced, or exposed?
- What is local-only, parent-owned, Ocentra-hosted, unavailable, or degraded?
- What proof makes the product claim true?
- What remains a gap?

## How To Use These Files

When starting a feature:

1. Find the roadmap milestone in [product-roadmap.md](product-roadmap.md).
2. Open every expectation file linked by that milestone.
3. Identify the product claim being made.
4. Implement the smallest real slice that satisfies the relevant expectations.
5. Validate with focused gates during development and the full merge gate before PR merge.

When finishing a feature, report:

- What changed.
- What parent-visible behavior exists now.
- What product claim is now true.
- What remains intentionally out of scope.
- Exact validations run.
- Current git state.
