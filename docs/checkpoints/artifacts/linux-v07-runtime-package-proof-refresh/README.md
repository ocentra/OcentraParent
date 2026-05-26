# Linux V0.7 Runtime Package Proof Refresh Artifact Index

This directory intentionally stores a text index only. The generated DEBs, CI
artifact downloads, package smoke logs, and extracted payloads remain ignored by
git.

## Temp Artifact Locations Used During The Proof Pass

| Temp or ignored path                                     | Purpose                                                                  | Committed |
| -------------------------------------------------------- | ------------------------------------------------------------------------ | --------- |
| `%TEMP%\ocentra-parent-linux-proof-26456009160`          | Downloaded GitHub Actions Linux preview artifact from run `26456009160`. | No        |
| `target/release-packages/linux/*`                        | Locally built Ubuntu 22.04/glibc 2.35 DEBs and sidecars.                 | No        |
| `test-results/linux-package-smoke/linux-deb-smoke-*.log` | Local Linux DEB smoke command logs.                                      | No        |
| `test-results/linux-package-smoke/*.health.json`         | Local extracted-service `/health` payloads.                              | No        |
| `test-results/linux-package-smoke/*.contents.txt`        | Local DEB payload listings from smoke proof.                             | No        |

The tracked proof record is
`docs/checkpoints/linux-v07-runtime-package-proof-refresh-2026-05-26.md`.
Do not commit generated package binaries, downloaded artifacts, or smoke output
files.
