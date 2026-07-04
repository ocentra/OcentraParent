# Workpack 04 - Child Linux Service Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `04-child-linux-service-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child Linux package, service-manager lifecycle, and package proof boundary.

Current status: `complete`.

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

- A real Linux proof pack exists under `output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/`.
- The scoped proof runner and contract/test surface are live:
  - `packages/schema-domain/src/child-linux-service-package-proof.ts`
  - `packages/schema-domain/tests/proof/child-linux-service-package-proof.test.ts`
  - `scripts/test/child-linux-service-package-proof.mjs`
- The current lane proved source-level Linux package truth from:
  - `scripts/release/linux/build-agent-package.sh`
  - `scripts/release/linux/ocentra-parent-agent.service`
  - `scripts/smoke/linux-deb-smoke.sh`
- `scripts/release/linux/build-agent-package.sh` now bootstraps Linux cargo into PATH for non-login bash before the version gate runs.
- `cmd /c npm run release:package:linux` now passes in this Ubuntu 22.04 / glibc 2.35 WSL lane and builds the real child `.deb`, latest alias, checksum sidecars, and baseline metadata.
- `bash scripts/smoke/linux-deb-smoke.sh target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb` now passes and proves checksum verification, extracted-binary health, `dpkg -i`, `dpkg -r`, and purge cleanup on the baseline host.

## Proved states

- Intended Linux distribution path is explicit as a direct unsigned `.deb` artifact with sha256 sidecars.
- Linux support is explicitly bounded to Ubuntu 22.04 amd64 with glibc 2.35 baseline metadata.
- Real package artifacts now exist in the workspace:
  - `target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb`
  - `target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb`
  - `target/release-packages/linux/linux-baseline.json`
- `systemd` service-manager wiring is explicit:
  - install path
  - enable path
  - restart path
  - stop path
  - disable path
  - daemon-reload cleanup path
- `dpkg` install, remove, and purge executed successfully on the Ubuntu 22.04 baseline host.
- Respawn claims are explicit only as `systemd` source proof and do not extend beyond that boundary.
- Uninstall and daemon cleanup evidence exists for the negative path through `prerm` stop/disable hooks, `postrm` daemon-reload, and smoke-script remove/purge guards.

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

WP04 is closed as a proof-boundary workpack. It is not a production release-readiness, signed-repository, generic-Linux, or installed-runtime-parity claim.
