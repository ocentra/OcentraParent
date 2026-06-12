<!-- agent-capsule -->

> Agent Capsule
> Doc: Windows Package Lifecycle Proof Harness - 2026-05-25
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Windows Package Lifecycle Proof Harness - 2026-05-25

## Scope

This checkpoint records the Worker A implementation slice for the Windows
package lifecycle proof harness on branch
`codex/windows-package-lifecycle-proof-harness`, based on current `main`
`0ebfb9e4ffa5352e0afd759725b226d3c6624e12`.

The change is implementation-first. It adds a reusable local proof harness for
Windows preview MSI artifacts instead of another doc-only proof pass. The
harness verifies the downloaded preview artifact shape, sidecars, manifest
metadata, read-only MSI metadata, host elevation state, and a typed lifecycle
decision. It writes sanitized machine-readable output under ignored
`test-results/` and never reboots automatically.

## Implemented Harness

| Path                                                      | Purpose                                                                                                                                                                                                                                                               |
| --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scripts/release/windows/package-lifecycle-proof.mjs`     | Direct CLI entrypoint. Accepts `--artifact-dir` for an already downloaded preview artifact or `--run-id` plus `--repo` to download a GitHub Actions artifact with `gh run download`. Writes `proof.json` under `test-results/windows-package-lifecycle-proof/<run>/`. |
| `scripts/release/windows/package-lifecycle-artifacts.mjs` | Artifact verifier for `latest-windows.json`, versioned MSI, latest MSI, `.sha256` sidecars, bootstrap installer policy strings, manifest payload policy, and signed-manifest envelope presence.                                                                       |
| `scripts/release/windows/package-lifecycle-host.mjs`      | Windows host operations. Reads elevation state, reads MSI metadata through Windows Installer COM, and supports an explicit elevated `--install` path for silent install, service start/health, uninstall, service absence, and process cleanup.                       |
| `scripts/release/windows/package-lifecycle-runner.mjs`    | Shared lifecycle decision/output helpers. Emits non-elevated `admin-required` and downloads GitHub Actions artifact metadata before the CLI writes proof JSON.                                                                                                        |
| `scripts/test/windows-package-lifecycle-proof.test.mjs`   | Focused node tests for artifact verification, sidecar failure behavior, checksum parsing, and lifecycle decisions.                                                                                                                                                    |
| `scripts/test/release-windows-assets.test.mjs`            | Extended release-asset guard so the harness remains wired to ignored proof output, admin-required state, silent MSI args, service health URL, and no automatic reboot command.                                                                                        |

`package.json` was intentionally not changed because C owns the package wiring
lock. Run the harness directly:

```powershell
cmd /c node scripts/release/windows/package-lifecycle-proof.mjs --run-id <run-id> --repo ocentra/OcentraParent --out-dir test-results/windows-package-lifecycle-proof/<label> --install
```

`--install` means "attempt lifecycle proof if the shell is elevated." In a
non-elevated shell, the harness does not invoke `msiexec`; it emits
`admin-required` with `reason: requires-elevated-shell`.

## Current-Main Artifact Proof

GitHub Actions run `26423129817` was the current-main run for
`0ebfb9e4ffa5352e0afd759725b226d3c6624e12`. It completed successfully on
2026-05-25 with Windows preview artifact metadata:

| Field                | Value                                                                     |
| -------------------- | ------------------------------------------------------------------------- |
| Run id               | `26423129817`                                                             |
| Workflow             | `CI Gate`                                                                 |
| Artifact name        | `ocentra-parent-windows-x64-preview`                                      |
| Artifact id          | `7206068857`                                                              |
| Artifact digest      | `sha256:20330dfb1a7bb98304a2bf99a384536b34dd29715811f7e22c561a84060c2580` |
| Artifact size        | `19040514` bytes                                                          |
| Artifact created UTC | `2026-05-25T23:15:02Z`                                                    |
| Artifact expires UTC | `2026-08-23T23:00:29Z`                                                    |

Local non-elevated harness command:

```powershell
cmd /c node scripts/release/windows/package-lifecycle-proof.mjs --run-id 26423129817 --repo ocentra/OcentraParent --out-dir test-results/windows-package-lifecycle-proof/2026-05-25T23-20-00-current-main-non-elevated --install
```

Result:

- `windows-package-lifecycle-status=ok`
- `windows-package-lifecycle-decision=admin-required`
- Proof JSON:
  `test-results/windows-package-lifecycle-proof/2026-05-25T23-20-00-current-main-non-elevated/proof.json`

Important proof JSON fields:

| Field                                   | Value                                                              |
| --------------------------------------- | ------------------------------------------------------------------ |
| `artifactSource.runId`                  | `26423129817`                                                      |
| `artifactSource.id`                     | `7206068857`                                                       |
| `artifact.status`                       | `verified`                                                         |
| `artifact.manifest.version`             | `0.1.1`                                                            |
| `artifact.manifest.signature.status`    | `present`                                                          |
| `artifact.manifest.signature.algorithm` | `Ed25519`                                                          |
| `artifact.files.versionedMsi.sha256`    | `F4C34A98BBD894035E2473640AB84C998A70A74A96C42242B63CD124CE529EB8` |
| `artifact.files.latestMsi.sha256`       | `F4C34A98BBD894035E2473640AB84C998A70A74A96C42242B63CD124CE529EB8` |
| `msiMetadata.status`                    | `read`                                                             |
| `msiMetadata.properties.ProductName`    | `Ocentra Parent Agent`                                             |
| `msiMetadata.properties.ProductVersion` | `0.1.1`                                                            |
| `msiMetadata.properties.Manufacturer`   | `Ocentra`                                                          |
| `msiMetadata.properties.ProductCode`    | `{4FE2C3E8-262A-4E87-81F4-DF5FD5F384D0}`                           |
| `msiMetadata.properties.UpgradeCode`    | `{0143F5A1-4C10-4C0F-97BE-55EDAF5012BB}`                           |
| `elevation.status`                      | `not-elevated`                                                     |
| `lifecycle.decision.status`             | `admin-required`                                                   |
| `lifecycle.decision.reason`             | `requires-elevated-shell`                                          |
| `lifecycle.decision.installAttempted`   | `false`                                                            |
| `lifecycle.decision.rebootAttempted`    | `false`                                                            |

The manifest signature envelope is checked for presence and expected fields.
Cryptographic signature verification was not claimed because the preview run
uses an ephemeral signing key and the public key is not committed with the
artifact.

## Elevated Lifecycle Path

The elevated path is implemented but was not run in this non-elevated Codex
session. It is intentionally gated by both `--install` and an administrator
shell. When those conditions are met, the harness:

1. Runs `msiexec.exe /i <msi> /qn /norestart /L*v <out>\windows-msi-install.log`.
2. Confirms `OcentraParentAgent` and `OcentraParentUpdater` are registered and
   running, starting them if registered but stopped.
3. Checks `http://127.0.0.1:4477/health` and stores only a SHA-256 of the
   response body.
4. Runs `msiexec.exe /x <msi> /qn /norestart /L*v <out>\windows-msi-uninstall.log`.
5. Confirms the services and service processes are absent after uninstall.
6. Leaves `lifecycle.reboot.attempted=false`; no reboot command exists in the
   harness.

Elevated command plan:

```powershell
cd C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent
cmd /c node scripts/release/windows/package-lifecycle-proof.mjs --run-id 26423129817 --repo ocentra/OcentraParent --out-dir test-results/windows-package-lifecycle-proof/elevated-26423129817 --install
```

Run this only from an administrator PowerShell window on a proof host where
installing and removing the per-machine services is acceptable. Reboot/autostart
survival remains a separate manual-required proof because the harness does not
reboot automatically.

## Validation Status

Focused validation already run during implementation:

| Command                                                                                                                                                                                                                            | Result                                                        |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| `cmd /c node --test scripts/test/windows-package-lifecycle-proof.test.mjs scripts/test/release-windows-assets.test.mjs`                                                                                                            | Passed, 12 tests.                                             |
| `cmd /c node scripts/release/windows/package-lifecycle-proof.mjs --run-id 26423129817 --repo ocentra/OcentraParent --out-dir test-results/windows-package-lifecycle-proof/2026-05-25T23-20-00-current-main-non-elevated --install` | Passed with `admin-required` non-elevated lifecycle decision. |

Final branch validation is recorded in the worker DONE report after format,
guards, and broader validation complete.

## Remaining Manual-Required Gaps

| Gap                                      | Reason                                                                                              | Next owner step                                                                                                                                         |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Elevated install/service/uninstall proof | Current Codex shell is not administrator; install would modify local services.                      | Run the elevated command plan from an admin proof window and attach the ignored proof JSON/log paths.                                                   |
| Reboot/autostart proof                   | User explicitly prohibited automatic reboot.                                                        | After an elevated install on a proof host, manually reboot and record post-reboot service/health state in a follow-up proof record.                     |
| Production signing/store proof           | Current artifact is a CI preview with ephemeral update signing.                                     | Release owner must run the production branch workflow with production signing credentials when that milestone is approved.                              |
| Manifest cryptographic verification      | Preview artifact does not include the public key needed to verify the ephemeral manifest signature. | Provide the trusted preview public key to the harness or switch preview signing to a known verification key if this is required before V0.7 acceptance. |

## Roadmap Slice

This closes the local Windows package lifecycle harness gap identified by the
V0.7 proof record: there is now a reusable implementation path for current-main
preview artifacts, typed non-elevated state, read-only MSI metadata proof, and
an intentionally gated elevated installer/service lifecycle proof path.

V0.7 remains the acceptance gate until the elevated lifecycle and reboot/autostart
proofs are run on an appropriate Windows proof host or explicitly accepted as
manual-required for this release checkpoint.
