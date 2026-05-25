# V0.7 LAN And Cross-Platform Proof Artifact Index

This directory intentionally stores a text index only. Binary package previews,
emulator logs, LAN service logs, extracted packages, and screenshots were kept
out of git.

## Temp Artifact Locations Used During The Proof Pass

| Temp path                                                  | Purpose                                                                           | Committed |
| ---------------------------------------------------------- | --------------------------------------------------------------------------------- | --------- |
| `%TEMP%\ocentra-parent-v07-platform-artifacts-26415925682` | Downloaded GitHub Actions artifacts from run `26415925682` for local inspection.  | No        |
| `%TEMP%\ocentra-parent-v07-lan-proof-b9ed9dc`              | Local Windows LAN service stdout/stderr for bind/origin/wrong-port/offline proof. | No        |
| `%TEMP%\ocentra-parent-v07-android-proof-26415925682`      | Local Android emulator stdout/stderr for APK install/activity launch smoke.       | No        |
| `%TEMP%\ocentra-parent-v07-ios-zip-inspect-26415925682`    | Extracted iOS simulator ZIP for metadata inspection.                              | No        |
| WSL `/tmp/ocentra-parent-deb-proof-b9ed9dc`                | Extracted Linux DEB and failed service launch logs for the glibc blocker.         | No        |

## Reproduction Commands

The main proof record at
`docs/checkpoints/v0-7-lan-and-cross-platform-manual-proof-2026-05-25.md`
contains the exact command summaries, observed outputs, proof labels, and
remaining owner checklists.

Use this directory only for lightweight text indexes. Do not commit downloaded
MSI, DEB, PKG, APK, ZIP, SBOM, emulator, or service log binaries here.
