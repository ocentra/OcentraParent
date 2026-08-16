<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.7 LAN And Cross-Platform Manual Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.7 LAN And Cross-Platform Manual Proof

## Scope

This record captures the worker B LAN and cross-platform proof pass for current
`main` after PR #97 and PR #98. It records CI/package mechanical proof, local
Windows LAN proof, available WSL/Linux package proof, local Android emulator
package proof, and explicit manual-required gaps for real household LAN,
macOS, iOS, physical devices, signing, entitlements, package lifecycle, stores,
and reboot/autostart behavior.

This record does not add product behavior, portal UI, runtime code, package
scripts, workflow changes, roadmap text, proof-matrix updates, A-owned Windows
controlled proof, or C-owned portal work.

## Run Metadata

| Field               | Value                                                                                                           |
| ------------------- | --------------------------------------------------------------------------------------------------------------- |
| Proof date          | 2026-05-25                                                                                                      |
| Worktree            | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`                                          |
| Branch              | `codex/v0.7-lan-and-cross-platform-manual-proof`                                                                |
| Baseline commit     | `b9ed9dc11849a02eb76134887e4ee64b08b072af`                                                                      |
| Baseline subject    | `Record V0.7 PR96 checkpoint evidence package (#98)`                                                            |
| Package/app version | `0.1.1`                                                                                                         |
| GitHub Actions run  | <https://github.com/ocentra/OcentraParent/actions/runs/26415925682>                                             |
| CI workflow         | `CI Gate`                                                                                                       |
| CI final status     | `completed`                                                                                                     |
| CI final conclusion | `success`                                                                                                       |
| CI created UTC      | 2026-05-25 19:11:03 UTC                                                                                         |
| CI updated UTC      | 2026-05-25 19:27:09 UTC                                                                                         |
| Local proof host    | `GAMEDEV`, Windows 11 Pro `10.0.26200`, x64, Gigabyte Technology Co., Ltd. X570 AORUS MASTER                    |
| Local LAN IPv4      | `192.168.2.10/24` on `Ethernet 2`; WSL virtual adapters `172.26.128.1/20` and `172.19.48.1/20` were not counted |
| Tooling             | Node `v22.22.2`, npm `11.7.0`, cargo `1.90.0`, rustc `1.90.0`, gh `2.40.1`, adb `35.0.2-12147458`               |
| Sensitive data      | No child private activity, screenshots, browser history, decrypted evidence, local tokens, or secrets recorded. |

## Source Inputs

- `AGENTS.md`
- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `docs/architecture/worktree-lanes.md`
- `docs/architecture/primary-coordinator-reminder.md`
- `docs/product-roadmap.md`
- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- `docs/architecture/local-lan-manual-proof-runbook.md`
- `docs/architecture/v07-cross-platform-proof-gap-tracker.md`
- `docs/expectations/pre-ai-proof-matrix.json`
- `docs/expectations/lan-pairing.md`
- `docs/expectations/platform-deliverables.md`
- `docs/expectations/platforms.md`
- GitHub Actions CI Gate run:
  <https://github.com/ocentra/OcentraParent/actions/runs/26415925682>

## Current-Main CI Job Ledger

Run `26415925682` targeted `main` at
`b9ed9dc11849a02eb76134887e4ee64b08b072af` and completed successfully.

| Evidence area       | Job id        | Job name                                               | Result    | Mechanical proof recorded                                                                 |
| ------------------- | ------------- | ------------------------------------------------------ | --------- | ----------------------------------------------------------------------------------------- |
| Fail-fast gate      | `77760276786` | `fail-fast / Format, Lint, Types, Rust Check`          | `success` | Format check, release-version policy, lint, TypeScript type-check, and Rust check passed. |
| Secret scan         | `77760638797` | `secret-scan / Secrets and Sensitive Files`            | `success` | Repository secret scanner and Gitleaks passed.                                            |
| Production build    | `77760663259` | `build / Production Build`                             | `success` | Production build gate passed.                                                             |
| Pre-AI matrix       | `77760663270` | `validate / Pre-AI Proof Matrix`                       | `success` | Pre-AI proof matrix accepted 11 claims and 7 checkpoint scenarios.                        |
| Dependency/SBOM     | `77760663275` | `dependency-policy / Dependency Audit, Licenses, SBOM` | `success` | Dependency/license policy, cargo-audit setup, SBOM metadata write/upload passed.          |
| Full validation     | `77760769243` | `validate / Full Validation Gate`                      | `success` | Full validation passed in CI after Playwright Chromium install.                           |
| Real portal-to-Rust | `77760769252` | `validate / Real Portal To Rust E2E (macos-latest)`    | `success` | Hosted macOS real portal-to-Rust E2E passed against the real Rust service.                |
| Real portal-to-Rust | `77760769256` | `validate / Real Portal To Rust E2E (ubuntu-latest)`   | `success` | Hosted Ubuntu real portal-to-Rust E2E passed against the real Rust service.               |
| Real portal-to-Rust | `77760769258` | `validate / Real Portal To Rust E2E (windows-latest)`  | `success` | Hosted Windows real portal-to-Rust E2E passed against the real Rust service.              |
| Package preview     | `77761334851` | `package-preview / macOS PKG Preview`                  | `success` | macOS PKG build, payload smoke, and artifact upload passed.                               |
| Package preview     | `77761334860` | `package-preview / Windows MSI Preview`                | `success` | Windows MSI build, CI install/uninstall smoke, and artifact upload passed.                |
| Package preview     | `77761334872` | `package-preview / iOS Simulator App Preview`          | `success` | iOS simulator build, simulator install/launch smoke, and artifact upload passed.          |
| Package preview     | `77761334879` | `package-preview / Linux DEB Preview`                  | `success` | Linux DEB build, CI install/remove smoke, and artifact upload passed.                     |
| Package preview     | `77761334888` | `package-preview / Android APK Preview`                | `success` | Android APK build, emulator install/launch smoke, and artifact upload passed.             |

CI also emitted a hosted-runner notice: `windows-latest` requests are being
redirected to `windows-2025-vs2026` by June 15, 2026. That is not a failure, but
future proof records should keep the exact Windows runner image visible.

## Current-Main Artifact Ledger

`gh api repos/ocentra/OcentraParent/actions/runs/26415925682/artifacts` returned
these artifact records. Artifact digests are GitHub artifact archive digests,
not the package-file sidecar hashes listed later.

| Artifact name                          | Artifact id  | Size in bytes | Digest                                                                    | Created UTC             | Expired | Proof label           |
| -------------------------------------- | ------------ | ------------: | ------------------------------------------------------------------------- | ----------------------- | ------- | --------------------- |
| `ocentra-parent-windows-x64-preview`   | `7203840936` |    19,036,264 | `sha256:d4b693804274d92403fec32adb5e223a535d5bf5617e9fbb76c3302135eae13f` | 2026-05-25 19:27:03 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-ios-simulator-preview` | `7203805306` |        86,193 | `sha256:67cf39f41c8ebdac1db7e20c685d4321039c98f9d1f412d7e27b8ccb0d388826` | 2026-05-25 19:23:42 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-linux-amd64-preview`   | `7203805123` |     4,250,727 | `sha256:c51a07ca02ef57351d0f01c42e946f65b48fa027f9d8dca9f9860196fa938d2f` | 2026-05-25 19:23:41 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-android-preview`       | `7203805010` |        11,831 | `sha256:6b7944441e7fbb05d1529a0efcb3ae549b5dc39a224117fd1781906df581d1a8` | 2026-05-25 19:23:40 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-macos-preview`         | `7203797581` |     4,560,867 | `sha256:783178253d550e4d2650453aafd40a1410477e131848bfece1b3bac9f61b699f` | 2026-05-25 19:23:00 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-security-sbom`         | `7203749676` |       176,724 | `sha256:8f72f535b9a8224c94545e9caec19e9db01ac3ef27d339c95090a441d64ba5b0` | 2026-05-25 19:18:41 UTC | `false` | `ci-mechanical-proof` |

## Downloaded Artifact Inspection

Artifacts were downloaded to
`%TEMP%\ocentra-parent-v07-platform-artifacts-26415925682` with
`gh run download 26415925682 --repo ocentra/OcentraParent --dir <temp-dir>`.
The binary artifacts were not committed.

| Artifact payload                                                                                     |      Size | Local sidecar/file hash result                                                             | Local inspection result                                                                                                                                             | Proof label           |
| ---------------------------------------------------------------------------------------------------- | --------: | ------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- |
| `ocentra-parent-agent-windows-x64-v0.1.1.msi`                                                        | 9,781,248 | Sidecar matched `sha256:920cc683445a991b8bce8874138342acb5bb9b80ab982dc616740a3ce0185c79`. | `Get-AuthenticodeSignature` returned `NotSigned`; `latest-windows.json` names MSI, per-machine scope, WinSW service ids, and Ed25519 metadata signature.            | `ci-mechanical-proof` |
| `ocentra-parent-agent-linux-amd64-v0.1.1.deb`                                                        | 2,124,276 | Sidecar matched `sha256:95f8eaaaa0adba5761b56a583ec12b950153bc342454fdf851d00c14af0b6cfa`. | WSL `dpkg-deb --field` read package `ocentra-parent-agent`, version `0.1.1`, architecture `amd64`; contents include systemd service, binary, lib, and log paths.    | `ci-mechanical-proof` |
| `ocentra-parent-agent-macos-v0.1.1.pkg`                                                              | 2,281,440 | Sidecar matched `sha256:793c82e2fb3f96b98dfc34f75d611f06ea65b27a093a3efafd981fb526c229c2`. | Windows `tar -tf` listed `Bom`, `Payload`, `Scripts`, `PackageInfo`; `PackageInfo` identifier `ca.ocentra.parent.agent`, version `0.1.1`, `auth="root"`.            | `ci-mechanical-proof` |
| `ocentra-parent-agent-android-debug-v0.1.1.apk`                                                      |     9,329 | Sidecar matched `sha256:1fd9f0414e7fd94490a77c214730e3af39d4bdaa7acd4e1e23fff6df8f649c58`. | APK archive contains Android manifest, resources, metadata, and dex files; local emulator install/launch smoke is recorded below.                                   | `ci-mechanical-proof` |
| `ocentra-parent-agent-ios-simulator-v0.1.1.zip`                                                      |    44,065 | Sidecar matched `sha256:89878181bc78cff114262360e444efb36b360d8f2f0d52118f40dec08b961577`. | ZIP contains `OcentraParentAgent.app`, executable, dylibs, `Info.plist`, and `PkgInfo`; binary plist includes bundle id `ca.ocentra.parent.agent`, version `0.1.1`. | `ci-mechanical-proof` |
| `ocentra-parent-security-sbom/cargo-metadata.json`, `ocentra-parent-security-sbom/npm-sbom.cdx.json` | 1,793,886 | GitHub artifact digest recorded above; SBOM files were downloaded and listed.              | Download proves SBOM artifact retrieval and file presence only. It is not a threat-model, compliance, or production release signoff.                                | `ci-mechanical-proof` |

## Local Windows LAN Proof

### Focused Integration Proof

Command:

```powershell
cmd /c npm run test:integration
```

Result:

- `build:contracts` completed for schema, logging, activity,
  agent-protocol, text, and portal domain packages.
- `cargo build -p ocentra-parent-agent-service` completed.
- Local smoke reported:
  `websocket-local-smoke-ok:agent.connection.ready,agent.health.reported,agent.activity.ingest.status.reported`.
- LAN smoke reported:
  `websocket-lan-smoke-ok:agent.connection.ready,agent.command.rejected,agent.lan-pairing.status.reported,agent.lan-pairing.status.reported,agent.health.reported`.

Proof label:

- `implemented` for current dev WebSocket mechanics that reject an unpaired LAN
  command, accept direct proof submission, select a route, and then accept a
  paired health command through the real Rust service.
- `implemented` only for the one-host dev WebSocket mechanics named above.
  Household two-device proof remains `manual-required`.
- `scaffold-only` for unsupported HTTP discovery, challenge, proof, control,
  and registry endpoints reported by the LAN status payload.

### Manual Bind, Origin, Wrong-Port, And Offline Proof

The local Windows proof launched the built Rust service directly with:

```powershell
$env:OCENTRA_PARENT_AGENT_ADDR = "0.0.0.0:4677"
$env:OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS = "http://127.0.0.1:4678,http://localhost:4678,http://192.168.2.10:4678"
$env:OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED = "true"
.\target\debug\ocentra-parent-agent-service.exe
```

Observed command output:

| Check                | Observed result                                                                                                                                                                                                                                              | Proof label      |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------- |
| Loopback health      | `curl.exe -i http://127.0.0.1:4677/health` returned `HTTP/1.1 200 OK`; health payload named device id `local-dev-agent`, hostname `GAMEDEV`, platform `windows`, service version `0.1.1`, pid `47400`, `captureEnabled: true`, `policyEngineEnabled: false`. | `implemented`    |
| LAN health           | `curl.exe -i http://192.168.2.10:4677/health` returned `HTTP/1.1 200 OK` with the same real service payload.                                                                                                                                                 | `implemented`    |
| Allowed origin       | `curl.exe -i -H "Origin: http://192.168.2.10:4678" http://192.168.2.10:4677/health` returned `access-control-allow-origin: http://192.168.2.10:4678`.                                                                                                        | `implemented`    |
| Wrong origin         | `curl.exe -i -H "Origin: http://example.invalid" http://192.168.2.10:4677/health` returned `HTTP/1.1 200 OK` for health but did not include an `access-control-allow-origin` header for the unrelated origin.                                                | `implemented`    |
| Wrong port negative  | `curl.exe -i --max-time 3 http://192.168.2.10:4679/health` failed with curl exit `7`.                                                                                                                                                                        | `implemented`    |
| Offline after stop   | After stopping the service, `curl.exe -i --max-time 3 http://192.168.2.10:4677/health` failed with curl exit `7`.                                                                                                                                            | `implemented`    |
| Service log location | `%TEMP%\ocentra-parent-v07-lan-proof-b9ed9dc`; temp logs were not committed.                                                                                                                                                                                 | `not-applicable` |

This proves current-host LAN bind/origin mechanics and negative wrong-port /
offline behavior. It does not prove firewall/router behavior from another
household parent device, production LAN authentication, or persistent pairing.

## WSL And Linux Proof

Available local Linux tooling:

- `wsl.exe --list --verbose` reported `Ubuntu-22.04` and `docker-desktop`
  distributions stopped.
- `docker version` failed because the Docker CLI is not installed in this
  Windows environment.
- `wsl.exe -d Ubuntu-22.04 -- bash -lc "ldd --version | head -n 1; uname -a"`
  reported Ubuntu glibc `2.35` and Linux kernel
  `5.15.167.4-microsoft-standard-WSL2`.

Commands and observations:

| Check                  | Command summary                                                                                               | Observed result                                                                                                                                                                           | Proof label                               |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| WSL pre-AI matrix      | `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/.../OcentraParent && npm run test:pre-ai-proof"`              | Passed: `pre-ai-proof-ok: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                                                                                          | `ci-mechanical-proof`                     |
| WSL worktree git state | `git status --short --branch` inside WSL worktree                                                             | Failed because the Windows worktree `.git` metadata points at a Windows absolute path: `fatal: not a git repository: ... E:/OcentraParent/.git/worktrees/OcentraParent1`.                 | `blocked`                                 |
| DEB metadata           | `dpkg-deb --field <deb> Package Version Architecture Maintainer Description`                                  | Read `Package: ocentra-parent-agent`, `Version: 0.1.1`, `Architecture: amd64`, `Maintainer: Ocentra <support@ocentra.ca>`.                                                                | `ci-mechanical-proof`                     |
| DEB contents           | `dpkg-deb --contents <deb>`                                                                                   | Listed `/lib/systemd/system/ocentra-parent-agent.service`, `/opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service`, `/var/lib/ocentra/...`, and `/var/log/ocentra/...`.      | `ci-mechanical-proof`                     |
| Extracted DEB launch   | Extracted the DEB under WSL and tried to run `ocentra-parent-agent-service` on `127.0.0.1:4681`, then health. | Failed before binding: `/lib/x86_64-linux-gnu/libc.so.6: version GLIBC_2.39 not found`; WSL Ubuntu 22.04 has glibc `2.35`; curl health failed with exit `7` because no service was bound. | `blocked` for Ubuntu 22.04 package launch |

Original Linux conclusion from this current-main proof pass:

- Hosted CI proves Linux DEB build and install/remove smoke on its runner.
- WSL proves the pre-AI matrix still runs locally on Ubuntu 22.04 and can
  inspect the package metadata and contents.
- This host cannot prove the downloaded DEB runtime on Ubuntu 22.04 because the
  artifact requires glibc `2.39`. A Linux package owner needs to decide whether
  the preview target is Ubuntu 24.04+ only or whether the package should build
  against an older glibc baseline.
- Docker proof is `unavailable` on this host because the Docker CLI is absent.

Follow-up Linux baseline implementation:

- Branch `codex/linux-package-baseline-and-package-proof` selects the older
  supported target instead of making the Linux package Ubuntu 24.04+ only:
  Ubuntu 22.04 `amd64` with glibc `2.35`.
- The follow-up record is
  `docs/checkpoints/linux-package-baseline-and-package-proof-2026-05-25.md`.
- Local WSL Ubuntu 22.04.5 proof on that branch built
  `ocentra-parent-agent-linux-amd64-v0.1.1.deb`, recorded
  `X-Ocentra-Linux-Baseline: ubuntu-22.04`,
  `X-Ocentra-Min-GLIBC: 2.35`, and `X-Ocentra-Build-GLIBC: 2.35`, launched the
  extracted service on `/health`, and ran DEB install/remove with passwordless
  `sudo`.
- This follow-up narrows the Linux package baseline gap only. Docker, real
  Linux reboot/autostart, desktop capture, network attribution, enforcement
  adapters, and managed-device behavior remain unavailable, manual-required, or
  not-yet-proven until separate host/device proof exists.

## Android Emulator Proof

Available local Android tooling:

- `adb.exe` exists at
  `C:\Users\sujan\AppData\Local\Android\Sdk\platform-tools\adb.exe`.
- `adb version` reported Android Debug Bridge `1.0.41`, version
  `35.0.2-12147458`.
- `adb devices` initially listed no connected devices.
- Android emulator exists at
  `C:\Users\sujan\AppData\Local\Android\Sdk\emulator\emulator.exe`.
- `emulator.exe -list-avds` reported `Pixel_9_Pro_XL_API_35`.

Command summary:

```powershell
emulator.exe -avd Pixel_9_Pro_XL_API_35 -no-window -no-audio -no-snapshot-save -no-boot-anim -gpu swiftshader_indirect
adb -s emulator-5554 install -r ocentra-parent-agent-android-debug-v0.1.1.apk
adb -s emulator-5554 shell cmd package resolve-activity --brief ca.ocentra.parent.agent
adb -s emulator-5554 shell monkey -p ca.ocentra.parent.agent 1
adb -s emulator-5554 shell pidof -s ca.ocentra.parent.agent
adb -s emulator-5554 shell dumpsys package ca.ocentra.parent.agent
```

Observed result:

| Check                 | Observed result                                                                                               | Proof label           |
| --------------------- | ------------------------------------------------------------------------------------------------------------- | --------------------- |
| Emulator boot         | Headless AVD reached `emulator-5554` with `sys.boot_completed=1`.                                             | `ci-mechanical-proof` |
| APK install           | `Performing Streamed Install` followed by `Success`; install exit `0`.                                        | `ci-mechanical-proof` |
| Activity resolution   | Resolved `ca.ocentra.parent.agent/.MainActivity`; resolve exit `0`.                                           | `ci-mechanical-proof` |
| Launch smoke          | `adb shell monkey -p ca.ocentra.parent.agent 1` injected 1 event; launch exit `0`.                            | `ci-mechanical-proof` |
| Long-lived process    | `pidof -s ca.ocentra.parent.agent` returned exit `1`; no long-lived foreground or child-agent process proven. | `not-yet-proven`      |
| Package metadata      | `versionCode=2`, `targetSdk=35`, `versionName=0.1.1`, install/update time 2026-05-25 15:33:48 local.          | `ci-mechanical-proof` |
| Physical device proof | No physical Android device was connected.                                                                     | `manual-required`     |

Android conclusion:

- Current CI and this local emulator pass prove APK package install, activity
  resolution, and launch scaffold mechanics.
- They do not prove Android child-agent behavior, foreground-service
  durability, notification permission, UsageStats, accessibility, VPN/DNS,
  device-owner, managed-profile, background behavior, or physical-device
  lifecycle.

## macOS And iOS Proof

This worker ran on Windows, not on a Mac.

| Area       | Local command or tooling check                                                       | Observed result                                                                                                                                                                   | Proof label           |
| ---------- | ------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- |
| macOS PKG  | Windows `tar -tf ocentra-parent-agent-macos-v0.1.1.pkg`                              | Listed `Bom`, `Payload`, `Scripts`, and `PackageInfo`; `PackageInfo` identifier `ca.ocentra.parent.agent`, version `0.1.1`, install location `/`, `auth="root"`, 8 payload files. | `ci-mechanical-proof` |
| Mac tools  | `Get-Command pkgutil,xcrun,spctl`                                                    | `pkgutil`, `xcrun`, and `spctl` are missing on this Windows host.                                                                                                                 | `unavailable`         |
| iOS ZIP    | Windows `tar -tf ocentra-parent-agent-ios-simulator-v0.1.1.zip`                      | Listed `OcentraParentAgent.app/`, executable, debug dylibs, `Info.plist`, and `PkgInfo`.                                                                                          | `ci-mechanical-proof` |
| iOS plist  | Extracted ZIP and searched binary `Info.plist`                                       | Found bundle id `ca.ocentra.parent.agent`, supported platform `iPhoneSimulator`, SDK `iphonesimulator18.5`, and version `0.1.1`.                                                  | `ci-mechanical-proof` |
| Xcode path | `Get-Command xcrun,simctl,codesign`                                                  | `xcrun`, `simctl`, and `codesign` are missing on this Windows host.                                                                                                               | `unavailable`         |
| TestFlight | Not runnable from this host; no Apple signing, provisioning, or entitlement context. | TestFlight/device/provisioning/Family Controls/DeviceActivity/Screen Time/Network Extension proof remains blocked until a Mac/iOS owner runs it with credentials and devices.     | `manual-required`     |

macOS and iOS conclusion:

- Current CI proves hosted macOS package payload smoke and iOS simulator
  install/launch smoke.
- This local Windows pass can only inspect archive/package metadata.
- Real Mac install, launchd, signing, notarization, Screen Recording,
  Accessibility, Network Extension, iOS simulator rerun, device install,
  TestFlight, provisioning, and entitlement behavior remains manual-required or
  blocked by missing host/tooling.

## Explicit Non-Claims

- No household two-device LAN proof ran. The LAN bind/origin checks used one
  Windows host and its own LAN IPv4 address.
- No real parent device connected to a separate child device across a home
  router/firewall.
- No production LAN discovery, challenge, HTTP proof/control, persistent
  trusted-device registry, or multi-device route durability is claimed.
- No privileged Windows browser URL, foreground window, network/domain, screen
  queue, app/game duration, installed-service lifecycle, reboot, autostart,
  uninstall cleanup, update, or data-retention proof is claimed by this branch.
- No production signing, macOS notarization, store distribution, TestFlight,
  entitlement, device-owner, managed-profile, VPN/DNS, UsageStats,
  accessibility, notification, foreground-service durability, background
  execution, or physical-device proof is claimed.
- No local AI model execution, V0.8 enforcement adapter, blocking, timer, or
  remote/cloud relay behavior is claimed.
- No proof-matrix label is upgraded by this record.

## Owner-Ready Remaining Proof

### LAN Household Pairing

Run this on two real devices on the same household network:

```powershell
# Child Windows device
$env:OCENTRA_PARENT_DEV_NETWORK = "lan"
$env:OCENTRA_PARENT_LAN_HOST = "<child-lan-ip>"
$env:OCENTRA_PARENT_AGENT_PORT = "4677"
$env:OCENTRA_PARENT_PORTAL_PORT = "4678"
cmd /c npm run dev:lan
```

```powershell
# Parent device
curl.exe -i --max-time 10 http://<child-lan-ip>:4677/health
curl.exe -i -H "Origin: http://<child-lan-ip>:4678" --max-time 10 http://<child-lan-ip>:4677/health
curl.exe -i -H "Origin: http://example.invalid" --max-time 10 http://<child-lan-ip>:4677/health
curl.exe -i --max-time 5 http://<child-lan-ip>:4679/health
```

Then run the WebSocket paired/unpaired flow from a parent device or a
parent-device script over the LAN address, and record:

- parent and child OS/device names;
- parent and child IP addresses/subnet;
- firewall/router state;
- selected ports and allowed origins;
- unpaired rejection payload;
- proof submit, route select, and paired accepted payload;
- service logs and parent-visible output;
- explicit unsupported HTTP endpoint and durable registry gaps.

Do not mark household LAN implemented until the parent and child are separate
real devices.

### Linux Package Baseline

The follow-up branch
`codex/linux-package-baseline-and-package-proof` rebuilds the Linux DEB against
Ubuntu 22.04/glibc `2.35` and records that baseline in the package metadata.
After that branch lands, use an Ubuntu 22.04 VM/device or Docker image first.
Use Ubuntu 24.04+ only as a newer-runtime compatibility check, not as the
minimum baseline.

```bash
grep -E '^(PRETTY_NAME|VERSION_ID|ID)=' /etc/os-release
getconf GNU_LIBC_VERSION
sha256sum --check ocentra-parent-agent-linux-amd64-latest.deb.sha256
dpkg-deb --field ocentra-parent-agent-linux-amd64-latest.deb \
  Package Version Architecture Depends X-Ocentra-Linux-Baseline X-Ocentra-Min-GLIBC X-Ocentra-Build-GLIBC
sudo dpkg -i ocentra-parent-agent-linux-amd64-v0.1.1.deb
systemctl status ocentra-parent-agent.service --no-pager
curl -i --max-time 10 http://127.0.0.1:4477/health
sudo reboot
systemctl status ocentra-parent-agent.service --no-pager
sudo dpkg -r ocentra-parent-agent
sudo dpkg -P ocentra-parent-agent
```

Record distro, kernel, glibc, systemd state, install/remove output, service
status before and after reboot, health payload, and unsupported desktop
capture/network/enforcement capability labels. Do not upgrade Linux desktop,
network, enforcement, managed-device, signing, store, or update claims from
package smoke alone.

### macOS Host

Run on a real Mac:

```bash
pkgutil --expand ocentra-parent-agent-macos-v0.1.1.pkg pkg-expanded
sudo installer -pkg ocentra-parent-agent-macos-v0.1.1.pkg -target /
launchctl print system/ca.ocentra.parent.agent
spctl --assess --verbose ocentra-parent-agent-macos-v0.1.1.pkg
curl -i --max-time 10 http://127.0.0.1:4477/health
```

Record signing/notarization, launchd behavior, service health, permissions,
uninstall cleanup, and unsupported capability states.

### Android Physical Device

Run after connecting a real Android device:

```powershell
adb devices
adb install -r ocentra-parent-agent-android-debug-v0.1.1.apk
adb shell cmd package resolve-activity --brief ca.ocentra.parent.agent
adb shell monkey -p ca.ocentra.parent.agent 1
adb shell dumpsys package ca.ocentra.parent.agent
adb shell dumpsys notification
```

Record emulator and physical-device results separately. Do not claim
device-owner, managed-profile, accessibility, VPN/DNS, UsageStats,
notification, foreground-service, or background behavior unless those product
paths exist and are exercised on the device.

### iOS Simulator, Device, TestFlight, And Entitlements

Run on a Mac with Xcode:

```bash
xcrun simctl list devices available
xcrun simctl boot "<simulator-name>"
xcrun simctl install booted OcentraParentAgent.app
xcrun simctl launch booted ca.ocentra.parent.agent
codesign -d --entitlements :- OcentraParentAgent.app
```

Separately record provisioning, signing team, TestFlight status, real iOS device
install, Family Controls, DeviceActivity, Screen Time, Network Extension,
notification, and background execution entitlement availability.

## Known Gaps And Risks

- CI run `26415925682` is current-main green for `b9ed9dc`, but it remains
  CI-mechanical proof for privileged OS/device behavior.
- The historical WSL Ubuntu 22.04 package launch failure is resolved by the
  follow-up branch `codex/linux-package-baseline-and-package-proof`, which
  selects Ubuntu 22.04/glibc `2.35` as the explicit preview baseline and proves
  a local WSL build/smoke. Treat this as branch evidence until branch CI or
  post-merge CI uploads a new Linux preview artifact from `ubuntu-22.04`.
- Current LAN WebSocket proof is real service code, but it is still one-host
  dev-mode proof unless a separate parent and child device run it over the LAN.
- Android emulator install/launch proof does not prove a long-lived agent,
  foreground service, child-device behavior, or physical-device policy.
- Mac/iOS proof cannot be completed from this Windows worker. It needs a Mac,
  Xcode, signing/provisioning context, and any required Apple entitlement
  approval.

## Roadmap Slice

V0.7 LAN and cross-platform manual proof checkpoint before additional AI or
enforcement work. This branch makes current-main CI/package proof and available
manual platform proof reviewable while preserving unsupported product claims as
manual-required, blocked, unavailable, scaffold-only, or not-yet-proven.

## Historical PR Body Outline For This Proof Record

```text
Scope
- Added the V0.7 LAN and cross-platform manual proof package for current main b9ed9dc.
- Recorded final green CI Gate run 26415925682, job ids, package-preview artifacts, artifact ids, and digests.
- Ran local Windows LAN bind/origin/wrong-port/offline proof and npm run test:integration for real WebSocket paired/unpaired service mechanics.
- Downloaded and inspected Windows/Linux/macOS/Android/iOS/SBOM artifacts without committing binaries.
- Ran WSL Ubuntu proof, recording pre-AI proof pass plus a real Linux DEB launch blocker on GLIBC_2.39 vs Ubuntu 22.04 glibc 2.35.
- Ran local Android emulator install/activity launch smoke from the CI APK.
- Kept macOS/iOS/TestFlight/entitlement/physical-device/two-device LAN/package lifecycle claims manual-required or blocked where proof was not available.

Touched files
- docs/checkpoints/v0-7-lan-and-cross-platform-manual-proof-2026-05-25.md
- docs/checkpoints/artifacts/v0-7-lan-and-cross-platform-manual-proof-2026-05-25/README.md
- docs/architecture/v07-cross-platform-proof-gap-tracker.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- cmd /c npm run test:integration
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- Real two-device household LAN is still manual-required.
- Linux DEB launch was blocked on WSL Ubuntu 22.04 by GLIBC_2.39 requirement in this historical proof; the follow-up Linux baseline branch owns the fix and new proof record.
- macOS host, iOS simulator/device/TestFlight/entitlements, Android physical device, package lifecycle, signing, stores, reboot, and autostart remain manual-required or blocked.
- No proof-matrix, roadmap, portal, package script, runtime, or workflow code changed.

Roadmap slice
- V0.7 LAN and cross-platform pre-AI/enforcement proof checkpoint.
```
