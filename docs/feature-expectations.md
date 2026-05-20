# Ocentra Parent Feature Expectations

This is the entry point for feature expectations. It tells future AI agents and human contributors where to look before implementing a feature. The expectation files define what a feature must prove; they do not prescribe a single implementation path.

## Prime Directive

Every feature must move Ocentra Parent toward a trustworthy family-safety product:

- Parents get clear, useful visibility and control.
- Child-device evidence is real, typed, timestamped, and auditable.
- Data is protected locally before it is queried, synced, classified, or acted on.
- Product claims never exceed what the implementation can prove.
- Validation must make lazy or fake implementations fail.

If a change cannot explain what parent problem it solves, what evidence it creates or consumes, and how it is validated, it is not ready.

## Universal Expectations

- [Feature request expectations](expectations/feature-request.md)
- [Universal done definition](expectations/universal-done.md)
- [Code quality expectations](expectations/code-quality.md)
- [Static analysis and security expectations](expectations/static-analysis-security.md)
- [Documentation expectations](expectations/documentation.md)
- [AI agent handoff expectations](expectations/ai-agent-handoff.md)

## Feature Expectations

- [Contract feature expectations](expectations/contracts.md)
- [Evidence storage expectations](expectations/evidence-storage.md)
- [Capture feature expectations](expectations/capture.md)
- [Portal feature expectations](expectations/portal.md)
- [Policy feature expectations](expectations/policy.md)
- [Enforcement feature expectations](expectations/enforcement.md)
- [LAN pairing expectations](expectations/lan-pairing.md)
- [Cloud feature expectations](expectations/cloud.md)
- [Notification feature expectations](expectations/notifications.md)
- [AI feature expectations](expectations/ai.md)
- [Sync and export expectations](expectations/sync-export.md)
- [Release and installer expectations](expectations/release-installer.md)
- [Billing and subscription expectations](expectations/billing.md)
- [Platform expectations](expectations/platforms.md)

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
