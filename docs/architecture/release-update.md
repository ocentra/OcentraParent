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

`npm run release:version` validates that all sources are aligned and use SemVer. A push to `production` is expected to carry a new version. If the matching release tag already exists, the release job fails and the next production push must bump the version first.

## Branch and Release Flow

```text
push to main
  -> CI Gate
  -> fail fast
  -> secret scan
  -> validate and build
  -> package previews for Windows, Linux, macOS, Android, and iOS simulator
  -> no GitHub Release
  -> no trusted update manifest

push to production
  -> Production Release Gate
  -> fail fast
  -> secret scan
  -> validate and build
  -> package previews for Windows, Linux, macOS, Android, and iOS simulator
  -> signed Windows release package
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

- Windows: real x64 MSI with WinSW services and updater scaffold.
- Linux: real amd64 `.deb` with a systemd service.
- macOS: real `.pkg` with a launchd daemon.
- Android: real debug APK with a foreground service scaffold.
- iOS: real simulator app build from an Xcode project.

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
