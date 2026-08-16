# Workpack 04 - Child Linux Service Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `04-child-linux-service-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child Linux package, service-manager lifecycle, and package proof boundary.

Current status: `production code drafted / test-deferred`.

Proof root: `output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/`

## Owns

- Linux child package shape
- service-manager install and restart truth
- package signing and distribution state
- uninstall and daemon cleanup behavior on Linux

## Must prove

- the package installs through the intended Linux distribution path
- the service manager start/stop/restart state is honest
- platform-specific package manager gaps are explicit
- respawn or recovery claims match Linux service-manager proof
- no generic "Linux support" claim hides distro limits

## Failure conditions

- respawn is claimed without service-manager proof
- package proof is used to claim macOS or Windows readiness
- distro-specific gaps are hidden
- manual-required states are omitted

## Execution truth

- Production code is drafted in this pass: the Linux package builder, systemd unit, package name, install paths, and maintainer-service names now target the child composition executable and child package identity. Network transport remains an explicit downstream gap.
- Tests, validation, and proof are deferred; proof-root references do not establish completion in this phase.
- Existing proof-runner and contract/test references remain deferred in this pass:
  - `packages/schema-domain/src/child-linux-service-package-proof.ts`
  - `packages/schema-domain/tests/proof/child-linux-service-package-proof.test.ts`
  - `scripts/test/child-linux-service-package-proof.mjs`
- The source files targeted by this pass are:
  - `scripts/release/linux/build-agent-package.sh`
  - `scripts/release/linux/ocentra-parent-agent.service`
  - `scripts/smoke/linux-deb-smoke.sh`
- Package construction, installation, and runtime behavior are not validated in this pass.

## Intended source states (unvalidated)

- Intended Linux distribution path is explicit as a direct unsigned `.deb` artifact with sha256 sidecars.
- Linux support is explicitly bounded to Ubuntu 22.04 amd64 with glibc 2.35 baseline metadata.
- `systemd` service-manager wiring is explicit:
  - install path
  - enable path
  - restart path
  - stop path
  - disable path
  - daemon-reload cleanup path
- `dpkg` lifecycle, service health, respawn, and cleanup remain unvalidated.

## Manual-required states

- Installed service steady-state health after package-managed launch remains manual-required because the maintainer scripts tolerate `systemctl` failures and the smoke path does not assert an active service state before removal.
- Crash recovery and restart proof beyond the scripted `Restart=always` boundary remain manual-required.
- Signed package distribution, repository publication, and package-feed promotion remain manual-required.
- Non-`systemd` distro support remains manual-required.

## Exact validations

- `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
- `cmd /c npm run test:child-linux-service-package-proof`
- `node --test tests/release/linux/linux-package-baseline.test.mjs`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/child-linux-service-package-proof.ts packages/schema-domain/tests/proof/child-linux-service-package-proof.test.ts scripts/test/child-linux-service-package-proof.mjs scripts/release/linux/build-agent-package.sh tests/release/linux/linux-package-baseline.test.mjs`
- `cmd /c npm run release:package:linux`
- `bash scripts/smoke/linux-deb-smoke.sh target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb`

## No-claim boundary

- This workpack does not claim generic Linux distribution support.
- This workpack does not claim signed package distribution, apt repository readiness, or production release readiness.
- This workpack does not claim steady-state installed service health or host-proved crash recovery.
- This workpack does not upgrade `Restart=always` into real Linux-host respawn proof.
- This workpack does not close Windows, macOS, Android, iOS, parent-client, setup-device-trust, or LAN rows.

## Closure truth

WP04 remains production-code drafted. Tests, validation, package artifacts, and retained proof are deferred; this pass does not close the distribution or installed-runtime boundary.
