<!-- agent-capsule -->

> Agent Capsule
> Doc: Release and Update Architecture
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Release and Update Architecture

Ocentra Parent needs releases before product beta because the first useful test loop is install-on-another-PC, start the headless agent, and keep it current without manual file copying.

## Version Authority

The repository uses one release version across:

- root `package.json`
- root `package-lock.json`
- app and package `package.json` files
- Cargo workspace crates
- Android `versionName`
- iOS `MARKETING_VERSION`

`npm run release:version` validates that all sources are aligned and use SemVer. A push to `production` is expected to carry a new version when publishing is intended. The production workflow first checks whether tag `vX.Y.Z` already exists; if it exists, the release publish job is skipped instead of creating duplicate assets. This lets the branch absorb scaffold-only workflow updates without accidentally republishing the same installer.

## Branch and Release Flow

```text
push to main
  -> CI Gate
  -> fail fast
  -> secret scan
  -> dependency policy and SBOM
  -> validate and build
  -> package previews and smoke checks for Windows, Linux, macOS, Android, and iOS simulator
  -> no GitHub Release
  -> no trusted update manifest

push to production
  -> Production Release Gate
  -> fail fast
  -> secret scan
  -> dependency policy and SBOM
  -> validate and build
  -> package previews and smoke checks for Windows, Linux, macOS, Android, and iOS simulator
  -> release decision checks for a missing vX.Y.Z tag
  -> signed Windows release package when a release is required
  -> git tag vX.Y.Z
  -> GitHub Release assets
```

The release assets are:

- `install-ocentra-parent-agent-windows.ps1`
- `ocentra-parent-agent-windows-x64-vX.Y.Z.msi`
- `ocentra-parent-agent-windows-x64-vX.Y.Z.msi.sha256`
- `latest-windows.json`

Preview artifacts are CI build outputs for testing. They are not store submissions and are not the trusted production update channel.

Current platform package preview status:

- Windows: real x64 MSI with WinSW services and updater scaffold; CI installs and uninstalls it.
- Linux: real amd64 `.deb` with a systemd service; CI installs and removes it.
- macOS: real `.pkg` with a launchd daemon; CI expands and validates the payload.
- Android: real debug APK with a foreground service scaffold; CI installs and launches it in an emulator.
- iOS: real simulator app build from an Xcode project; CI installs and launches it in a simulator.

Android device-owner policy, iOS Family Controls entitlements, macOS notarization, Apple App Store submission, Google Play submission, Linux rpm packaging, and non-Windows updater installers are intentionally not claimed yet.

The bootstrap installer downloads signed `latest-windows.json`, verifies that the manifest has a signature envelope, verifies the MSI hash from the signed payload, and runs Windows Installer in passive mode.

## Windows Service Strategy

The v0 MSI uses WinSW as the Windows Service wrapper. This keeps the Rust agent binary headless and console-friendly for local dev while still installing it as an automatic Windows service on test machines.

The release builder pins WinSW `v2.12.0`, verifies its SHA256 while building the MSI, and embeds the verified wrapper in the installer. The service identity is stable:

- service id: `OcentraParentAgent`
- service name: `Ocentra Parent Agent`
- default bind: `127.0.0.1:4477`
- install root: `%ProgramFiles%\Ocentra\Ocentra Parent Agent`
- app data root: `%ProgramData%\Ocentra\Ocentra Parent Agent`

The MSI owns service registration, start-on-install, stop-on-uninstall, major upgrades, and uninstall registration in Windows Installed Apps.

The MSI also installs a separate updater service:

- service id: `OcentraParentUpdater`
- service name: `Ocentra Parent Updater`
- executable: `ocentra-parent-agent-updater.exe`
- mode: `run-loop`
- default cadence: initial delay of 120 seconds, then every 3600 seconds

The updater is separate from the main agent so the main service does not replace its own executable in-process.

## Signed Update Manifests

`latest-windows.json` is a signed envelope:

```json
{
  "payload": {
    "schemaVersion": 1,
    "version": "0.1.0",
    "target": "windows-x64",
    "installer": { "type": "msi" },
    "artifact": { "sha256": "..." }
  },
  "signature": {
    "algorithm": "Ed25519",
    "keyId": "...",
    "value": "..."
  }
}
```

The production release builder requires `OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64` in CI. Local builds can generate a local-only development signing key under `target/release-packages`; package previews can generate an explicit ephemeral signing key; neither key is trusted for production releases.

The updater binary is compiled with the trusted public key from `OCENTRA_PARENT_UPDATE_PUBLIC_KEY_BASE64`. It refuses to process update manifests without a valid Ed25519 signature from that key.

## Signing and Store Secrets

The current production release requires one live secret:

- `OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64`

The workflow also documents but does not require future signing secrets for Windows Authenticode, macOS Developer ID, Apple App Store submission, and Android release keystores. Those remain planned until the matching signing and distribution workflows are real.

## Dependency Policy

CI runs `npm audit --audit-level=high`, `cargo audit --deny warnings`, an npm license allowlist, and SBOM metadata generation before package previews. SBOM outputs are uploaded from `target/security/*.json`.

Rust uses the version pinned in `rust-toolchain.toml`. Android package builds use `platforms/android/agent/gradlew` so the preview build does not depend on a globally installed Gradle version.

## Install Command

After the first release exists, a Windows test machine can run this from an elevated PowerShell session:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://github.com/ocentra/OcentraParent/releases/latest/download/install-ocentra-parent-agent-windows.ps1 | iex"
```

The MSI can also be downloaded from the GitHub Release and installed directly:

```powershell
msiexec /i ocentra-parent-agent-windows-x64-vX.Y.Z.msi
```

## Auto-Update Target

The MSI is upgrade-ready through `MajorUpgrade`, so a newer signed-and-verified MSI can replace the old install. The durable automatic update shape is:

```text
agent service startup
  -> keeps serving local parent-control APIs

updater service loop
  -> fetch signed update manifest
  -> verify Ed25519 manifest signature
  -> compare installed version
  -> verify artifact policy and SHA256
  -> start quiet MSI upgrade
  -> Windows Installer stops/replaces/restarts services
```

The current production release scaffold creates the MSI, signed manifest, checksum, bootstrap installer, and dedicated updater service. Package previews also build Linux, macOS, Android, and iOS simulator artifacts so platform breakage is visible early. Later hardening should add release channels, certificate-backed Windows MSI signing, macOS Developer ID signing/notarization, mobile store signing, non-Windows updater installers, and parent-visible update policy.
