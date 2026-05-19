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

The bootstrap installer downloads `latest-windows.json`, verifies the MSI hash, and runs Windows Installer in passive mode.

## Windows Service Strategy

The v0 MSI uses WinSW as the Windows Service wrapper. This keeps the Rust agent binary headless and console-friendly for local dev while still installing it as an automatic Windows service on test machines.

The release builder pins WinSW `v2.12.0`, verifies its SHA256 while building the MSI, and embeds the verified wrapper in the installer. The service identity is stable:

- service id: `OcentraParentAgent`
- service name: `Ocentra Parent Agent`
- default bind: `127.0.0.1:4477`
- install root: `%ProgramFiles%\Ocentra\Ocentra Parent Agent`
- app data root: `%ProgramData%\Ocentra\Ocentra Parent Agent`

The MSI owns service registration, start-on-install, stop-on-uninstall, major upgrades, and uninstall registration in Windows Installed Apps.

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

The installed service should not replace its own executable in-process. The MSI is upgrade-ready through `MajorUpgrade`, so a newer signed-and-verified MSI can replace the old install. The durable automatic update shape is:

```text
agent service startup
  -> check update policy
  -> fetch signed update manifest
  -> compare installed version
  -> delegate to updater helper
  -> helper stops service
  -> helper verifies artifact hash/signature
  -> helper installs new package
  -> helper restarts service
```

The current scaffold creates the MSI, manifest, checksum, and installer link. The next update slice should add a dedicated updater helper boundary before the agent starts performing automatic updates.
