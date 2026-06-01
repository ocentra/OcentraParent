# Production Distribution And Support

## Parent Outcome

Parents can install, update, uninstall, subscribe, get support, understand
privacy/data handling, and trust that platform-specific releases are real, not
scaffold artifacts.

## Ocentra Requirement

Production distribution is part of the product. A CI package preview is not a
shipping claim. Signed installers, app stores, update channels, support docs,
privacy/legal docs, billing, and release proof must be explicit.

## Roadmap And Expectations

- Roadmap: V7 subscription and monetization, V8 production hardening.
- Expectations: [release installer](../expectations/release-installer.md),
  [billing](../expectations/billing.md),
  [platform deliverables](../expectations/platform-deliverables.md),
  [static analysis/security](../expectations/static-analysis-security.md),
  [documentation](../expectations/documentation.md).
- Modules: `crates/agent-updater`, `platforms/android`, `platforms/ios`,
  release scripts, root README, public website/account surface.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
production distribution, remote parent access, billing/subscription, and mobile
coverage.

All mature competitors ship through app stores, installers, support surfaces,
billing, privacy documents, and update channels. Ocentra must not look like a
dev-only repo when it claims consumer readiness.

## Current Ocentra State

- Windows MSI/updater scaffolding exists.
- Cross-platform package previews exist as CI mechanics.
- Billing/support/public website/store distribution are planned or incomplete.

## Current Gap

Need production signing, release channels, app store paths, Play/TestFlight,
support docs, privacy/legal docs, billing entitlement flows, update rollback,
support bundle redaction, and public download/account/status surfaces.

## Checklist

- [ ] Windows signing and installer release proof.
- [ ] Updater channel, rollback, and failure status.
- [ ] macOS signing/notarization if shipped.
- [ ] Android Play signing/store proof if shipped.
- [ ] iOS TestFlight/App Store proof if shipped.
- [ ] Billing/subscription/account flow.
- [ ] Privacy, retention, export/delete, and support docs.
- [ ] Support bundle redaction and incident process.

## Next AI Instructions

Do not promote scaffold package previews to product support. Every release claim
must name platform, signing/store state, smoke proof, support path, and known
limitations.
