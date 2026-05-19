# Release and Update Architecture

Ocentra Parent needs releases before product beta because the first useful test loop is install-on-another-PC, start the headless agent, and keep it current without manual file copying.

## Version Authority

The repository uses one release version across:

- root `package.json`
- root `package-lock.json`
- app and package `package.json` files
- Cargo workspace crates

`npm run release:version` validates that all sources are aligned and use SemVer. A push to `main` is expected to carry a new version. If the matching release tag already exists, the release job fails and the next main push must bump the version first.

## Main-Branch Release Flow

```text
push to main
  -> CI Gate
  -> fail fast
  -> secret scan
  -> validate and build
  -> Windows release package
  -> git tag vX.Y.Z
  -> GitHub Release assets
```

The release assets are:

- `install-ocentra-parent-agent-windows.ps1`
- `ocentra-parent-agent-windows-x64-vX.Y.Z.msi`
- `ocentra-parent-agent-windows-x64-vX.Y.Z.msi.sha256`
- `latest-windows.json`

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

The release builder requires `OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64` in CI. Local builds can generate a local-only development signing key under `target/release-packages`; that key is ignored and must not be used for production releases.

The updater binary is compiled with the trusted public key from `OCENTRA_PARENT_UPDATE_PUBLIC_KEY_BASE64`. It refuses to process update manifests without a valid Ed25519 signature from that key.

## Install Command

After the first release exists, a Windows test machine can run this from an elevated PowerShell session:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://github.com/SujanMishra/OcentraParent/releases/latest/download/install-ocentra-parent-agent-windows.ps1 | iex"
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

The current scaffold creates the MSI, signed manifest, checksum, bootstrap installer, and dedicated updater service. Later hardening should add release channels, certificate-backed MSI signing, and parent-visible update policy.
