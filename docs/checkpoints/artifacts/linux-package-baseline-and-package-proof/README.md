<!-- agent-capsule -->

> Agent Capsule
> Doc: Linux Package Baseline Proof Artifacts
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Linux Package Baseline Proof Artifacts

The Linux baseline proof generates local binaries and smoke logs that remain
ignored by git.

Generated artifact paths:

- `target/release-packages/linux/ocentra-parent-agent-linux-amd64-v0.1.1.deb`
- `target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb`
- `target/release-packages/linux/*.sha256`
- `target/release-packages/linux/linux-baseline.json`
- `test-results/linux-package-smoke/*`

The tracked proof record is
`docs/checkpoints/linux-package-baseline-and-package-proof-2026-05-25.md`.
Do not commit the generated DEB files or smoke outputs.
