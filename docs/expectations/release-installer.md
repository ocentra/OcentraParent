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
  boundaries, support diagnostic redaction, CI artifacts, and manual proof
  requirements.

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
- Update paths reject unsigned or incorrectly signed manifests once signing is enabled.
- Package claims match real artifacts.
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
