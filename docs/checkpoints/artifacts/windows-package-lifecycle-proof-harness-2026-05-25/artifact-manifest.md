<!-- agent-capsule -->

> Agent Capsule
> Doc: Windows Package Lifecycle Harness Artifact Manifest - 2026-05-25
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Windows Package Lifecycle Harness Artifact Manifest - 2026-05-25

This manifest lists non-binary artifact references for
`docs/checkpoints/windows-package-lifecycle-proof-harness-2026-05-25.md`.
Downloaded MSI packages, `.sha256` sidecars, and proof JSON output are ignored
local artifacts under `test-results/`.

## Ignored Local Proof Output

| Artifact                                                                                                          | Purpose                                                                                                                                                               | Commit state           |
| ----------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| `test-results/windows-package-lifecycle-proof/2026-05-25T23-20-00-current-main-non-elevated/proof.json`           | Current-main non-elevated harness proof for run `26423129817`; records artifact verification, MSI metadata, elevation state, and `admin-required` lifecycle decision. | Ignored local artifact |
| `test-results/windows-package-lifecycle-proof/2026-05-25T23-20-00-current-main-non-elevated/downloaded-artifact/` | Downloaded `ocentra-parent-windows-x64-preview` artifact contents from run `26423129817`.                                                                             | Not committed          |

## GitHub Actions Artifact

| Field                | Value                                                                     |
| -------------------- | ------------------------------------------------------------------------- |
| Run id               | `26423129817`                                                             |
| Run URL              | <https://github.com/ocentra/OcentraParent/actions/runs/26423129817>       |
| Head SHA             | `0ebfb9e4ffa5352e0afd759725b226d3c6624e12`                                |
| Artifact name        | `ocentra-parent-windows-x64-preview`                                      |
| Artifact id          | `7206068857`                                                              |
| Artifact digest      | `sha256:20330dfb1a7bb98304a2bf99a384536b34dd29715811f7e22c561a84060c2580` |
| Artifact size        | `19040514` bytes                                                          |
| Artifact created UTC | `2026-05-25T23:15:02Z`                                                    |
| Artifact expires UTC | `2026-08-23T23:00:29Z`                                                    |

## Verified Downloaded Files

The harness verified the manifest, versioned MSI, latest MSI, both checksum
sidecars, and the bootstrap installer. Key file hashes from proof JSON:

| File                                          | SHA-256                                                            |
| --------------------------------------------- | ------------------------------------------------------------------ |
| `ocentra-parent-agent-windows-x64-v0.1.1.msi` | `F4C34A98BBD894035E2473640AB84C998A70A74A96C42242B63CD124CE529EB8` |
| `ocentra-parent-agent-windows-x64-latest.msi` | `F4C34A98BBD894035E2473640AB84C998A70A74A96C42242B63CD124CE529EB8` |

## MSI Metadata

Read-only Windows Installer metadata inspection returned:

| Property         | Value                                    |
| ---------------- | ---------------------------------------- |
| `ProductName`    | `Ocentra Parent Agent`                   |
| `ProductVersion` | `0.1.1`                                  |
| `Manufacturer`   | `Ocentra`                                |
| `ProductCode`    | `{4FE2C3E8-262A-4E87-81F4-DF5FD5F384D0}` |
| `UpgradeCode`    | `{0143F5A1-4C10-4C0F-97BE-55EDAF5012BB}` |
