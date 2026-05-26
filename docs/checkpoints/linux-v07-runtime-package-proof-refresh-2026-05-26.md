# Linux V0.7 Runtime Package Proof Refresh - 2026-05-26

## Scope

This record covers worker B's post-merge Linux runtime/package proof refresh on
branch `codex/linux-v07-runtime-package-proof-refresh`.

| Field                 | Value                                                                                                                                |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Base commit           | `c351dc19b9cc1a90a7b650cb2e8329bcb9618d3e`                                                                                           |
| Base subject          | `Add Windows package lifecycle proof harness`                                                                                        |
| Current-main CI run   | <https://github.com/ocentra/OcentraParent/actions/runs/26456009160>                                                                  |
| Current-main CI state | `completed` / `success`; updated `2026-05-26T15:08:30Z`                                                                              |
| Linux package job     | `77892317510`, `package-preview / Linux DEB Preview`, `completed` / `success`                                                        |
| Linux artifact        | `7218306018`, `ocentra-parent-linux-amd64-preview`, digest `sha256:bb27768da499f755db570d14b72718e0c98870c9fa9b6d5830e0ad5724c15016` |
| Package version       | `0.1.1`                                                                                                                              |
| Worktree              | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`                                                               |

This branch is a proof refresh over the merged Linux baseline implementation.
No portal, protocol, service, package manifest, signing, store, or product
behavior paths were changed.

## Baseline Decision

The supported Linux DEB preview baseline is Ubuntu 22.04 on `amd64`, with glibc
`2.35` as the minimum runtime baseline. Current main no longer depends on
`ubuntu-latest` for Linux DEB preview output, so the earlier
Ubuntu-24.04/glibc-2.39 artifact blocker is closed for the package preview path.

This does not mean every Linux distribution is supported. Distributions with
glibc below `2.35` remain blocked. Ubuntu 24.04+ may run the artifact, but this
record does not certify forward compatibility beyond the declared Ubuntu 22.04
baseline and package smoke evidence.

## Current-Main CI Proof

`gh run watch 26456009160 --repo ocentra/OcentraParent --interval 15 --exit-status`
completed successfully.

Key job ledger from
`gh run view 26456009160 --repo ocentra/OcentraParent --json databaseId,headSha,headBranch,status,conclusion,createdAt,updatedAt,url,jobs`:

| Job                                                    |   Database id | Result    |
| ------------------------------------------------------ | ------------: | --------- |
| `fail-fast / Format, Lint, Types, Rust Check`          | `77890061158` | `success` |
| `secret-scan / Secrets and Sensitive Files`            | `77890777392` | `success` |
| `dependency-policy / Dependency Audit, Licenses, SBOM` | `77890834467` | `success` |
| `validate / Pre-AI Proof Matrix`                       | `77890834524` | `success` |
| `build / Production Build`                             | `77890834557` | `success` |
| `validate / Real Portal To Rust E2E (ubuntu-latest)`   | `77891080213` | `success` |
| `validate / Real Portal To Rust E2E (windows-latest)`  | `77891080220` | `success` |
| `validate / Real Portal To Rust E2E (macos-latest)`    | `77891080503` | `success` |
| `validate / Full Validation Gate`                      | `77891080659` | `success` |
| `package-preview / Linux DEB Preview`                  | `77892317510` | `success` |
| `package-preview / Windows MSI Preview`                | `77892317520` | `success` |
| `package-preview / macOS PKG Preview`                  | `77892317600` | `success` |
| `package-preview / Android APK Preview`                | `77892317457` | `success` |
| `package-preview / iOS Simulator App Preview`          | `77892317509` | `success` |

Artifact ledger from
`gh api repos/ocentra/OcentraParent/actions/runs/26456009160/artifacts`:

| Artifact name                          |           Id | Digest                                                                    |
| -------------------------------------- | -----------: | ------------------------------------------------------------------------- |
| `ocentra-parent-linux-amd64-preview`   | `7218306018` | `sha256:bb27768da499f755db570d14b72718e0c98870c9fa9b6d5830e0ad5724c15016` |
| `ocentra-parent-security-sbom`         | `7218169866` | `sha256:a67fbb123bf25b5cd9ca74c476b1753dffc16cf5eaa17e0b6638b3224964cdaa` |
| `ocentra-parent-windows-x64-preview`   | `7218348399` | `sha256:8a782c9c45a735d9097499375f764358068bcd7b0c04ed38ce883512ea06b2d5` |
| `ocentra-parent-macos-preview`         | `7218290850` | `sha256:6aa78c2da20624e6595891de71e4bf675c29f43701d5e394f4a6bd591378392f` |
| `ocentra-parent-android-preview`       | `7218318456` | `sha256:dafab93895a6c1d379834d94f01476bb42343fb06058ec15aaaf07e6a08c9bc3` |
| `ocentra-parent-ios-simulator-preview` | `7218323956` | `sha256:1e2fdc33f74d39b08644fc9c7f953a2c20d4fa17dd6b80900a7fde3a03515e54` |

## CI Linux Artifact Inspection

The Linux artifact was downloaded to
`%TEMP%\ocentra-parent-linux-proof-26456009160` with:

```powershell
gh run download 26456009160 --repo ocentra/OcentraParent --name ocentra-parent-linux-amd64-preview --dir $env:TEMP\ocentra-parent-linux-proof-26456009160
```

Downloaded artifact contents:

```text
target/release-packages/linux/linux-baseline.json
target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb
target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb.sha256
target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb
target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb.sha256
test-results/linux-package-smoke/linux-deb-smoke-20260526T150648Z.log
test-results/linux-package-smoke/ocentra-parent-agent-linux-amd64-latest.deb.contents.txt
test-results/linux-package-smoke/ocentra-parent-agent-linux-amd64-latest.deb.health.json
```

`linux-baseline.json` from the CI artifact:

```json
{
  "package": "ocentra-parent-agent-linux-amd64-v0.1.1.deb",
  "latest": "ocentra-parent-agent-linux-amd64-latest.deb",
  "baseline": "ubuntu-22.04",
  "minimumGlibc": "2.35",
  "buildHost": {
    "prettyName": "Ubuntu 22.04.5 LTS",
    "id": "ubuntu",
    "versionId": "22.04",
    "glibc": "2.35"
  },
  "nonBaselineOverride": "false"
}
```

CI artifact sidecar and package field inspection through WSL:

```text
ocentra-parent-agent-linux-amd64-latest.deb: OK
ocentra-parent-agent-linux-amd64-v0.1.1.deb: OK
Package: ocentra-parent-agent
Version: 0.1.1
Architecture: amd64
Depends: libc6 (>= 2.35)
X-Ocentra-Linux-Baseline: ubuntu-22.04
X-Ocentra-Min-GLIBC: 2.35
X-Ocentra-Build-GLIBC: 2.35
```

CI smoke log excerpt:

```text
ocentra-parent-agent-linux-amd64-latest.deb: OK
Selecting previously unselected package ocentra-parent-agent.
Unpacking ocentra-parent-agent (0.1.1) ...
Setting up ocentra-parent-agent (0.1.1) ...
Removing ocentra-parent-agent (0.1.1) ...
linux-deb-smoke-ok:.../ocentra-parent-agent-linux-amd64-latest.deb baseline=ubuntu-22.04 glibc=2.35 host_glibc=2.35 install=ran log=.../test-results/linux-package-smoke/linux-deb-smoke-20260526T150648Z.log
```

CI health payload:

```json
{
  "schemaVersion": 1,
  "agent": {
    "deviceId": "local-dev-agent",
    "hostname": "unknown-host",
    "platform": "linux",
    "serviceVersion": "0.1.1"
  },
  "entries": [
    {
      "id": "dev-localhost-api-ready",
      "level": "info",
      "source": "agent-service",
      "message": "Agent service localhost API is reachable.",
      "fields": {
        "captureEnabled": false,
        "mode": "dev",
        "policyEngineEnabled": false,
        "remoteSync": null
      }
    }
  ]
}
```

## Local WSL Proof

Tooling metadata:

```text
wsl.exe --list --verbose
Ubuntu-22.04 stopped, WSL version 2
docker-desktop stopped, WSL version 2

docker version
docker: command not found on the Windows host

wsl.exe -d Ubuntu-22.04 -- bash -lc "grep -E '^(PRETTY_NAME|VERSION_ID|ID)=' /etc/os-release; getconf GNU_LIBC_VERSION; uname -a; rustc --version; cargo --version; node --version; npm --version"
PRETTY_NAME="Ubuntu 22.04.5 LTS"
VERSION_ID="22.04"
ID=ubuntu
glibc 2.35
Linux GameDev 5.15.167.4-microsoft-standard-WSL2 #1 SMP Tue Nov 5 00:21:55 UTC 2024 x86_64 x86_64 x86_64 GNU/Linux
rustc 1.90.0 (1159e78c4 2025-09-14)
cargo 1.90.0 (840b83a10 2025-07-30)
v22.22.0
10.9.4
```

Local build proof:

```text
wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent; bash scripts/release/linux/build-agent-package.sh"
dpkg-deb: building package 'ocentra-parent-agent' in '.../target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb'.
Built .../target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb
Built .../target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb
Baseline metadata .../target/release-packages/linux/linux-baseline.json
Finished `release` profile [optimized] target(s) in 32.39s
```

Local smoke proof:

```text
wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent; OCENTRA_PARENT_LINUX_SMOKE_PORT=4587 bash scripts/smoke/linux-deb-smoke.sh target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb"
ocentra-parent-agent-linux-amd64-latest.deb: OK
Selecting previously unselected package ocentra-parent-agent.
Unpacking ocentra-parent-agent (0.1.1) ...
Setting up ocentra-parent-agent (0.1.1) ...
Removing ocentra-parent-agent (0.1.1) ...
linux-deb-smoke-ok:.../ocentra-parent-agent-linux-amd64-latest.deb baseline=ubuntu-22.04 glibc=2.35 host_glibc=2.35 install=ran log=.../test-results/linux-package-smoke/linux-deb-smoke-20260526T150349Z.log
```

Local sidecar and package field proof:

```text
ocentra-parent-agent-linux-amd64-latest.deb: OK
ocentra-parent-agent-linux-amd64-v0.1.1.deb: OK
Package: ocentra-parent-agent
Version: 0.1.1
Architecture: amd64
Depends: libc6 (>= 2.35)
X-Ocentra-Linux-Baseline: ubuntu-22.04
X-Ocentra-Min-GLIBC: 2.35
X-Ocentra-Build-GLIBC: 2.35
```

Local smoke outputs were generated under ignored paths:

- `target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb`
- `target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb`
- `target/release-packages/linux/*.sha256`
- `target/release-packages/linux/linux-baseline.json`
- `test-results/linux-package-smoke/linux-deb-smoke-20260526T150349Z.log`
- `test-results/linux-package-smoke/ocentra-parent-agent-linux-amd64-latest.deb.contents.txt`
- `test-results/linux-package-smoke/ocentra-parent-agent-linux-amd64-latest.deb.health.json`

## What This Proves

- Current main at `c351dc19b9cc1a90a7b650cb2e8329bcb9618d3e` has a green CI
  Gate run with Linux package preview included.
- The current Linux preview artifact is built on Ubuntu 22.04.5 with glibc
  `2.35`, records that baseline in `linux-baseline.json`, and carries DEB
  metadata requiring `libc6 (>= 2.35)`.
- CI and local WSL smoke both check sidecar hashes, package fields, payload
  contents, extracted service launch, `/health`, and DEB install/remove.
- The earlier Ubuntu 22.04 local blocker from a glibc `2.39` artifact is closed
  for package preview artifacts built from current main.

## What Remains Manual-Required Or Not Yet Proven

- Docker proof is unavailable on this Windows host because the Docker CLI is not
  installed, even though a `docker-desktop` WSL distribution exists.
- WSL package smoke is not a full Linux device lifecycle proof. Real Ubuntu
  22.04 VM/device proof is still required for systemd boot/autostart, reboot
  survival, update behavior, purge/uninstall cleanup after reboot, journal
  retention, and service-manager behavior.
- Linux desktop capture, network attribution, enforcement adapters, privilege
  prompts, managed-device controls, and production hardening remain
  `not-yet-proven`.
- CI package preview does not prove production signing, store distribution,
  compliance approval, threat model completion, or release readiness.
- macOS host, iOS/TestFlight/entitlements, Android physical-device/device-owner,
  Windows signing, and household two-device LAN proof stay under their existing
  platform/manual-required labels.

## Owner-Ready Next Proof

Real Ubuntu 22.04 VM/device package lifecycle proof:

```bash
cat /etc/os-release
getconf GNU_LIBC_VERSION
uname -a
sha256sum --check ocentra-parent-agent-linux-amd64-latest.deb.sha256
dpkg-deb --field ocentra-parent-agent-linux-amd64-latest.deb \
  Package Version Architecture Depends X-Ocentra-Linux-Baseline X-Ocentra-Min-GLIBC X-Ocentra-Build-GLIBC
sudo dpkg -i ocentra-parent-agent-linux-amd64-latest.deb
systemctl status ocentra-parent-agent.service --no-pager
journalctl -u ocentra-parent-agent.service --no-pager -n 80
curl -i --max-time 10 http://127.0.0.1:4477/health
sudo reboot
systemctl status ocentra-parent-agent.service --no-pager
journalctl -u ocentra-parent-agent.service --no-pager -n 120
curl -i --max-time 10 http://127.0.0.1:4477/health
sudo dpkg -r ocentra-parent-agent
sudo dpkg -P ocentra-parent-agent
systemctl status ocentra-parent-agent.service --no-pager || true
```

Docker proof, if Docker becomes available:

```bash
docker run --rm -v "$PWD:/work" -w /work ubuntu:22.04 bash -lc '
  apt-get update &&
  apt-get install -y ca-certificates curl dpkg &&
  getconf GNU_LIBC_VERSION &&
  dpkg-deb --field target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb \
    Package Version Architecture Depends X-Ocentra-Linux-Baseline X-Ocentra-Min-GLIBC X-Ocentra-Build-GLIBC &&
  bash scripts/smoke/linux-deb-smoke.sh target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb
'
```

For any future matrix update, record exact host metadata, package field output,
sidecar output, service logs, health payload, install/remove/reboot behavior,
and explicit unsupported Linux capability labels. Do not upgrade desktop
capture, network attribution, enforcement, signing, stores, or device-management
claims from package smoke alone.

## Branch Validation

Completed on 2026-05-26 from
`C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`:

| Command                                                                                                                                                                                                                                                                                       | Result                                                                                                                                                                                                                             |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `gh run watch 26456009160 --repo ocentra/OcentraParent --interval 15 --exit-status`                                                                                                                                                                                                           | Passed; current-main CI Gate run `26456009160` completed `success`.                                                                                                                                                                |
| `gh run download 26456009160 --repo ocentra/OcentraParent --name ocentra-parent-linux-amd64-preview --dir %TEMP%\ocentra-parent-linux-proof-26456009160`                                                                                                                                      | Passed; downloaded artifact `7218306018` for inspection.                                                                                                                                                                           |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/AppData/Local/Temp/ocentra-parent-linux-proof-26456009160/target/release-packages/linux; sha256sum --check ...; dpkg-deb --field ..."`                                                                                            | Passed; both CI artifact sidecars returned `OK`, and DEB fields showed Ubuntu 22.04/glibc `2.35` baseline metadata.                                                                                                                |
| `wsl.exe --list --verbose` / `docker version` / `wsl.exe -d Ubuntu-22.04 -- bash -lc "grep -E '^(PRETTY_NAME\|VERSION_ID\|ID)=' /etc/os-release; getconf GNU_LIBC_VERSION; uname -a; rustc --version; cargo --version; node --version; npm --version"`                                        | Passed; WSL Ubuntu 22.04.5/glibc `2.35` available, Docker CLI unavailable on the Windows host.                                                                                                                                     |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent; bash scripts/release/linux/build-agent-package.sh"`                                                                                                                        | Passed; built versioned and latest DEBs plus `linux-baseline.json`.                                                                                                                                                                |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent; OCENTRA_PARENT_LINUX_SMOKE_PORT=4587 bash scripts/smoke/linux-deb-smoke.sh target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb"`                     | Passed; sidecar, package metadata, payload listing, extracted launch, `/health`, and install/remove smoke passed; log `test-results/linux-package-smoke/linux-deb-smoke-20260526T150349Z.log`.                                     |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent/target/release-packages/linux; sha256sum --check ocentra-parent-agent-linux-amd64-latest.deb.sha256; sha256sum --check ocentra-parent-agent-linux-amd64-v0.1.1.deb.sha256"` | Passed; both local sidecars returned `OK`.                                                                                                                                                                                         |
| `cmd /c npm run format:check`                                                                                                                                                                                                                                                                 | Passed; all matched files use Prettier style.                                                                                                                                                                                      |
| `cmd /c npm run test:pre-ai-proof`                                                                                                                                                                                                                                                            | Passed; `pre-ai-proof-ok: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                                                                                                                                   |
| `node --test scripts\release\linux\linux-package-baseline.test.mjs scripts\test\platform-packaging.test.mjs scripts\test\workflow-ci-trigger.test.mjs`                                                                                                                                        | Passed; 12 tests passed.                                                                                                                                                                                                           |
| `cmd /c npm run test:integration`                                                                                                                                                                                                                                                             | Passed on rerun; `websocket-local-smoke-ok` and `websocket-lan-smoke-ok`. This isolated an earlier transient local `npm run validate` attempt that failed because `websocket-local-smoke.mjs` did not see the dev log before exit. |
| `cmd /c npm run validate`                                                                                                                                                                                                                                                                     | Passed on rerun. Existing source-shape warnings, Vite chunk-size warning, and Chrome GCM `DEPRECATED_ENDPOINT` stderr remain warnings/noise, not failures.                                                                         |
