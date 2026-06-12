<!-- agent-capsule -->

> Agent Capsule
> Doc: Linux Package Baseline And Package Proof - 2026-05-25
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Linux Package Baseline And Package Proof - 2026-05-25

## Scope

This record covers worker B's Linux package baseline implementation on branch
`codex/linux-package-baseline-and-package-proof`.

| Field                 | Value                                                                                                      |
| --------------------- | ---------------------------------------------------------------------------------------------------------- |
| Base commit           | `0ebfb9e4ffa5352e0afd759725b226d3c6624e12`                                                                 |
| Base subject          | `Merge remote-tracking branch 'origin/codex/v0.7-windows-controlled-evidence-and-package-lifecycle-proof'` |
| Current-main CI run   | <https://github.com/ocentra/OcentraParent/actions/runs/26423129817>                                        |
| Current-main CI state | `completed` / `success`; Linux package-preview job `77782269220` passed before this branch's baseline fix  |
| Package version       | `0.1.1`                                                                                                    |
| Worktree              | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`                                     |

This branch changes Linux package scripts, package-preview wiring, focused
Linux release tests, and proof docs only. It does not touch C-owned portal,
protocol, service, or package manifest locks.

## Baseline Decision

The Linux DEB preview target is now explicitly `ubuntu-22.04` on `amd64`, with
glibc `2.35` as the minimum supported runtime baseline. Ubuntu 22.04 is
supported by the preview artifact when the package is built through the updated
script or package-preview job. Older Linux distributions with glibc below
`2.35` remain unsupported. Ubuntu 24.04+ may run the artifact, but the branch
does not require Ubuntu 24.04+ and no longer builds the Linux package on
`ubuntu-latest`.

## Implementation

| File                                                    | Change                                                                                                                                                                                                                                                                                                                                                         | Proof intent                                                                 |
| ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `.github/workflows/package-preview.yml`                 | Changed `linux-deb` from `ubuntu-latest` to `ubuntu-22.04`, records `/etc/os-release` and `getconf GNU_LIBC_VERSION`, uploads `linux-baseline.json` and smoke logs.                                                                                                                                                                                            | Prevent new glibc 2.39 artifacts from package preview.                       |
| `scripts/release/linux/build-agent-package.sh`          | Refuses non-baseline release builds unless `OCENTRA_PARENT_LINUX_ALLOW_NON_BASELINE=true`; records `X-Ocentra-Linux-Baseline`, `X-Ocentra-Min-GLIBC`, `X-Ocentra-Build-GLIBC`, `Depends: libc6 (>= 2.35)`, and `linux-baseline.json`; writes relocatable SHA-256 sidecars; stages under `/tmp` by default so WSL `/mnt/c` permissions do not break `dpkg-deb`. | Encode the baseline in both build behavior and package metadata.             |
| `scripts/release/linux/ocentra-parent-agent.service`    | Uses `OCENTRA_PARENT_AGENT_ADDR=127.0.0.1:4477`, matching the Rust service constant.                                                                                                                                                                                                                                                                           | Make installed Linux service configuration match the actual runtime env var. |
| `scripts/smoke/linux-deb-smoke.sh`                      | Checks SHA-256 sidecar, DEB fields, glibc preflight, payload contents, extracted service unit, extracted binary launch, `/health`, and install/remove when passwordless `sudo` is available; writes logs under `test-results/linux-package-smoke`.                                                                                                             | Make package smoke prove runtime launch, not just install/remove.            |
| `scripts/release/linux/linux-package-baseline.test.mjs` | Focused Node test for workflow baseline, build guard metadata, systemd env var, and hardened smoke coverage.                                                                                                                                                                                                                                                   | Gives this branch a direct test path while A owns `scripts/test`.            |

## Current-Main CI Reference

`gh run view 26423129817 --repo ocentra/OcentraParent --json headSha,conclusion,status,createdAt,updatedAt,jobs`
reported head SHA `0ebfb9e4ffa5352e0afd759725b226d3c6624e12`, status
`completed`, conclusion `success`, created `2026-05-25T23:00:29Z`, updated
`2026-05-25T23:15:06Z`.

Relevant job ids:

| Job                                                   |   Database id | Status    |
| ----------------------------------------------------- | ------------: | --------- |
| `fail-fast / Format, Lint, Types, Rust Check`         | `77781552611` | `success` |
| `validate / Full Validation Gate`                     | `77781852957` | `success` |
| `validate / Real Portal To Rust E2E (ubuntu-latest)`  | `77781852989` | `success` |
| `validate / Real Portal To Rust E2E (windows-latest)` | `77781852979` | `success` |
| `validate / Real Portal To Rust E2E (macos-latest)`   | `77781853002` | `success` |
| `package-preview / Linux DEB Preview`                 | `77782269220` | `success` |

The current-main Linux artifact id was `7206048049` with GitHub artifact digest
`sha256:cf5184e7de82cc721534f6c61c286504826eb4b25c50e263de0843eff20f2654`.
That run proves the pre-change package-preview mechanics only. It does not
prove the Ubuntu 22.04 baseline added by this branch.

## Local WSL Proof

Tooling metadata:

```text
wsl.exe --list --verbose
Ubuntu-22.04 stopped, WSL version 2
docker-desktop stopped, WSL version 2

wsl.exe -d Ubuntu-22.04 -- bash -lc "grep -E '^(PRETTY_NAME|VERSION_ID|ID)=' /etc/os-release && getconf GNU_LIBC_VERSION && rustc --version && cargo --version && node --version && npm --version"
PRETTY_NAME="Ubuntu 22.04.5 LTS"
VERSION_ID="22.04"
ID=ubuntu
glibc 2.35
rustc 1.90.0 (1159e78c4 2025-09-14)
cargo 1.90.0 (840b83a10 2025-07-30)
v22.22.0
10.9.4
```

The first WSL package attempt found a real staging issue:
`dpkg-deb: error: control directory has bad permissions 777`. The fix moved
package staging to a Linux temp directory while keeping artifacts in
`target/release-packages/linux`.

Baseline guard negative proof:

```text
wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent && if OCENTRA_PARENT_LINUX_BASELINE_VERSION=24.04 bash scripts/release/linux/build-agent-package.sh >/tmp/ocentra-linux-baseline-negative.log 2>&1; then exit 1; else cat /tmp/ocentra-linux-baseline-negative.log; fi"
Linux package builds must run on ubuntu-24.04 with glibc 2.35.
Observed host: Ubuntu 22.04.5 LTS; ID=ubuntu; VERSION_ID=22.04; glibc=2.35.
Use the package-preview linux-deb job or a matching baseline builder for release proof.
Set OCENTRA_PARENT_LINUX_ALLOW_NON_BASELINE=true only for local unsupported experiments.
```

Build proof:

```text
wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent && bash scripts/release/linux/build-agent-package.sh"
dpkg-deb: building package 'ocentra-parent-agent' in '.../target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb'.
Built .../target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb
Built .../target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb
Baseline metadata .../target/release-packages/linux/linux-baseline.json
```

Smoke proof:

```text
wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent && OCENTRA_PARENT_LINUX_SMOKE_PORT=4587 bash scripts/smoke/linux-deb-smoke.sh target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb"
ocentra-parent-agent-linux-amd64-latest.deb: OK
Selecting previously unselected package ocentra-parent-agent.
Preparing to unpack .../ocentra-parent-agent-linux-amd64-latest.deb ...
Unpacking ocentra-parent-agent (0.1.1) ...
Setting up ocentra-parent-agent (0.1.1) ...
Removing ocentra-parent-agent (0.1.1) ...
linux-deb-smoke-ok:.../ocentra-parent-agent-linux-amd64-latest.deb baseline=ubuntu-22.04 glibc=2.35 host_glibc=2.35 install=ran log=.../test-results/linux-package-smoke/linux-deb-smoke-20260525T231911Z.log
```

Package metadata proof:

```text
Package: ocentra-parent-agent
Version: 0.1.1
Architecture: amd64
Depends: libc6 (>= 2.35)
X-Ocentra-Linux-Baseline: ubuntu-22.04
X-Ocentra-Min-GLIBC: 2.35
X-Ocentra-Build-GLIBC: 2.35
```

Baseline sidecar and health proof:

```text
target/release-packages/linux/linux-baseline.json
baseline=ubuntu-22.04, minimumGlibc=2.35, buildHost.prettyName=Ubuntu 22.04.5 LTS, buildHost.glibc=2.35

sha256sum --check ocentra-parent-agent-linux-amd64-latest.deb.sha256
ocentra-parent-agent-linux-amd64-latest.deb: OK

sha256sum --check ocentra-parent-agent-linux-amd64-v0.1.1.deb.sha256
ocentra-parent-agent-linux-amd64-v0.1.1.deb: OK

test-results/linux-package-smoke/ocentra-parent-agent-linux-amd64-latest.deb.health.json
schemaVersion=1, platform=linux, serviceVersion=0.1.1, event=dev-localhost-api-ready
```

Generated artifacts and logs are intentionally ignored:

- `target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb`
- `target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb`
- `target/release-packages/linux/*.sha256`
- `target/release-packages/linux/linux-baseline.json`
- `test-results/linux-package-smoke/*`

## What This Proves

- The branch selects Ubuntu 22.04/glibc 2.35 as the Linux DEB build/runtime
  baseline instead of making the artifact Ubuntu 24.04+ only.
- The package builder refuses accidental release builds on a newer distro/glibc
  by default.
- A package built on WSL Ubuntu 22.04 launches its extracted Rust service binary
  and responds on `/health`.
- The DEB can be installed and removed on the available Ubuntu 22.04 WSL host
  with passwordless `sudo`.
- The systemd unit payload now uses the env var read by the Rust service.

## What Remains Manual-Required Or Not Yet Proven

- Docker proof is unavailable on this Windows host because the Docker CLI is not
  installed, even though a `docker-desktop` WSL distribution exists.
- Real Linux systemd boot/autostart, reboot survival, update behavior, and
  uninstall cleanup after reboot remain manual-required on a full Linux VM or
  device. WSL install/remove smoke is not reboot proof.
- Linux desktop capture, network attribution, enforcement adapters, privilege
  prompts, and managed-device behavior remain not-yet-proven.
- macOS, iOS/TestFlight/entitlements, Android physical-device behavior, Windows
  signing, stores, and household two-device LAN proof are outside this Linux
  package baseline branch and remain governed by their existing gap labels.

## Branch Validation

Completed on 2026-05-25 from
`C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`:

| Command                                                                                                                                                                                                                                                                                                                                    | Result                                                                                              |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------- |
| `cmd /c npm run format:check`                                                                                                                                                                                                                                                                                                              | Passed; all matched files use Prettier style.                                                       |
| `cmd /c npm run test:pre-ai-proof`                                                                                                                                                                                                                                                                                                         | Passed; `pre-ai-proof-ok: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`    |
| `node --test scripts\release\linux\linux-package-baseline.test.mjs scripts\test\platform-packaging.test.mjs scripts\test\workflow-ci-trigger.test.mjs`                                                                                                                                                                                     | Passed; 12 tests passed.                                                                            |
| `git diff --check`                                                                                                                                                                                                                                                                                                                         | Passed with no whitespace errors.                                                                   |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent && if OCENTRA_PARENT_LINUX_BASELINE_VERSION=24.04 bash scripts/release/linux/build-agent-package.sh >/tmp/ocentra-linux-baseline-negative.log 2>&1; then exit 1; else cat /tmp/ocentra-linux-baseline-negative.log; fi"` | Passed as a negative proof; the build guard refused the mismatched baseline.                        |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent && bash scripts/release/linux/build-agent-package.sh"`                                                                                                                                                                   | Passed; built versioned and latest DEBs plus `linux-baseline.json`.                                 |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent && OCENTRA_PARENT_LINUX_SMOKE_PORT=4587 bash scripts/smoke/linux-deb-smoke.sh target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb"`                                                                | Passed; sidecar, metadata, payload, extracted launch, health, and install/remove smoke passed.      |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent/target/release-packages/linux && sha256sum --check ocentra-parent-agent-linux-amd64-latest.deb.sha256 && sha256sum --check ocentra-parent-agent-linux-amd64-v0.1.1.deb.sha256"`                                          | Passed; both sidecars returned `OK`.                                                                |
| `cmd /c npm run validate`                                                                                                                                                                                                                                                                                                                  | Passed. Existing source-shape warnings and Vite chunk-size warning remain warnings, not failures.   |
| `cmd /c npm run lanes:status` / `cmd /c npm run hub:status` / `cmd /c npm run lanes:guard` / `cmd /c npm run hub:guard`                                                                                                                                                                                                                    | Passed; B lane remains on `codex/linux-package-baseline-and-package-proof` with the expected locks. |

## Owner-Ready Next Proof

Run after this branch is pushed, or after merge on current `main`:

```bash
# CI should now show the Linux DEB preview running on ubuntu-22.04.
gh run view <branch-or-main-ci-run> --repo ocentra/OcentraParent --json headSha,conclusion,status,jobs

# Artifact inspection after downloading the branch/main Linux preview artifact.
cd target/release-packages/linux
sha256sum --check ocentra-parent-agent-linux-amd64-latest.deb.sha256
dpkg-deb --field ocentra-parent-agent-linux-amd64-latest.deb \
  Package Version Architecture Depends X-Ocentra-Linux-Baseline X-Ocentra-Min-GLIBC X-Ocentra-Build-GLIBC

# Full Linux host lifecycle proof on a real Ubuntu 22.04 VM/device.
sudo dpkg -i ocentra-parent-agent-linux-amd64-latest.deb
systemctl status ocentra-parent-agent.service --no-pager
curl -i --max-time 10 http://127.0.0.1:4477/health
sudo reboot
systemctl status ocentra-parent-agent.service --no-pager
sudo dpkg -r ocentra-parent-agent
sudo dpkg -P ocentra-parent-agent
```

Record distro, kernel, glibc, package fields, service logs, health payload,
install/remove output, reboot behavior, and unsupported Linux capability labels.
Do not upgrade desktop capture, network attribution, enforcement, signing,
stores, or device-management claims from package smoke alone.
