# Release And Installer Expectations

Release features are product features because parents need install/update paths that work.

## Expected Deliverables

- Version policy.
- Installer artifact.
- Install smoke.
- Uninstall smoke.
- Update manifest where applicable.
- Signature verification where applicable.
- Clear production/manual release boundary.
- Release-support proof covering update/rollback posture, signing/store claim
  boundaries, package-runtime evidence, support diagnostic redaction, CI
  artifacts, support incident handoff metadata, parent consent, safe
  support-bundle manifest disclosure, and manual proof requirements.
- V8 release/support readiness gate that summarizes package-preview artifacts,
  support-safe diagnostic/runbook readiness, updater rollback execution state,
  signing/store proof state, production publishing state, and manual platform
  gaps without promoting preview artifacts to production release evidence.

## Acceptance

- `main` builds previews and does not publish production releases.
- `production` publishes only by explicit promotion.
- Feature branches may be pushed regularly but do not publish product releases.
- Final PRs into `main` are CI integration events, not release events.
- Product releases can intentionally batch multiple completed milestones.
- Installer paths are documented.
- Support diagnostics keep only support-safe fields and redact tokens, child
  activity, raw URLs, screenshots, journals, SQLite snapshots, private paths,
  command lines, keystrokes, clipboard data, and message contents.
- Support incident handoff requires explicit parent consent and visible
  support-bundle data-class disclosure before export.
- Support bundle manifests may include release version, commit, platform,
  package/runtime, service, route, capability, degraded state, redaction
  summary, manual proof reference, and incident status metadata only.
- Support incident handoff must exclude tokens, child activity, raw URLs,
  screenshots, journals, SQLite snapshots, private paths, commands, keystrokes,
  clipboard data, and message contents.
- Support diagnostic references must point to support-safe proof JSON,
  workflow, redaction summary, manual runbook, or status-row references; they
  must not point to private user paths, command logs, raw URLs, screenshots,
  journals, SQLite stores, clipboard data, or message contents.
- Current `support-bundle-redaction-proof` records logging-domain incident rows
  for parent consent, ready/manual review, backend-upload manual-required,
  billing-escalation manual-required, and account-lookup manual-required states.
  It limits support bundles to release/package/runtime, service, route,
  capability, degraded-state, redaction, manual-proof, incident, billing-status,
  and account-status references and rejects tokens, child activity, raw URLs,
  screenshots, journals, SQLite snapshots, private paths, command lines,
  keystrokes, clipboard data, message contents, provider secrets, executed
  backend upload, billing provider contact, account lookup, remote support, and
  production SLA claims.
- Production support backend upload, account lookup, billing escalation, remote
  support, and production SLA remain explicit manual-required or
  not-implemented states until real support workflows exist.
- Update paths reject unsigned or incorrectly signed manifests once signing is enabled.
- Package claims match real artifacts.
- Parent desktop package-runtime proof names built portal dist, the Rust service
  boundary, package service-manager ownership, fixed loopback process ownership,
  connect-or-degrade behavior, blank-window guard state, and signed-channel
  update posture without treating Vite or preview CI artifacts as production
  release evidence.
- Parent desktop release/support readiness proof records the Windows, Linux,
  macOS, Android, and iOS package-preview artifact names but keeps production
  publishing, signing, store upload, and updater rollback execution
  manual-required or promotion-required until real credentials, promotion, and
  manual platform proof exist.
- Parent-facing install flow is understandable for non-technical users.
- `mobile-child-agent-capability-proof` package/runtime hooks may reference
  Android debug APK/checksum and iOS simulator/Xcode artifacts, but Play
  signing, TestFlight, App Store, physical-device install, and mobile child
  parity stay manual-required or planned until those release artifacts exist.

## Non-Goals

- Do not claim store distribution, notarization, or full signing before credentials and workflows exist.
- Do not publish production releases from `main`.
- Do not treat every milestone merge as a product release.
- Do not bypass update signature checks in production code.

## Done Signal

The target platform has a real install/update/uninstall path, CI or local packaging proves the artifact, and docs describe the current branch, PR, and production release boundary honestly.
