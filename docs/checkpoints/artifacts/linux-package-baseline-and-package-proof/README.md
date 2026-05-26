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
