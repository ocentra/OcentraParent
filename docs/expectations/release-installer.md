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

## Acceptance

- `main` builds previews and does not publish production releases.
- `production` publishes only by explicit promotion.
- Installer paths are documented.
- Update paths reject unsigned or incorrectly signed manifests once signing is enabled.
- Package claims match real artifacts.
- Parent-facing install flow is understandable for non-technical users.

## Non-Goals

- Do not claim store distribution, notarization, or full signing before credentials and workflows exist.
- Do not publish production releases from `main`.
- Do not bypass update signature checks in production code.

## Done Signal

The target platform has a real install/update/uninstall path, CI or local packaging proves the artifact, and docs describe the current production boundary honestly.
