# Ocentra Parent CI

The pipeline uses the same modular shape as other Ocentra repositories, scaled to the current scaffold.

```mermaid
graph LR
  FF["Fail Fast<br/>format, lint, types, Rust check"] --> SS["Secret Scan<br/>custom scanner + Gitleaks"]
  SS --> V["Validate<br/>tests, Rust, local smoke, LAN smoke"]
  SS --> B["Build<br/>portal and packages"]
  V --> R["Release<br/>Windows agent MSI"]
  B --> R
```

## Files

- `ci.yml`: orchestrates the gate.
- `fail-fast.yml`: catches broken formatting, lint, TypeScript, and Rust check failures early.
- `secret-scan.yml`: runs the repo scanner plus Gitleaks.
- `validate.yml`: runs `npm run validate`.
- `build.yml`: runs `npm run build`.
- `.github/actions/setup-ci`: shared Node/Rust/npm setup.

On pushes to `main`, the orchestrator creates a GitHub Release only after validation and build pass. The release job requires the repository version to be unique and aligned across npm and Cargo sources, then publishes the Windows x64 MSI, checksum, signed update manifest, and bootstrap installer.

Documentation-only pushes do not run this workflow. Changes limited to Markdown files or `docs/**` are ignored by `ci.yml`, so README and planning updates cannot accidentally trigger a Windows release. Use `workflow_dispatch` when a manual CI run is still wanted for a docs-only change.

For emergency or intentional bypasses on a code-touching commit, GitHub's native skip marker is also available. Include `[skip ci]` in the commit message only when you are deliberately bypassing the gate and do not want a release from that push.

The release job requires `OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64` as a repository secret. The updater binary is built with the matching public key and rejects unsigned or incorrectly signed manifests.

## Local Parity

Before pushing, run:

```powershell
cmd /c npm run ci:local
```

The pre-commit hook runs the commit-time subset:

```powershell
cmd /c npm run hooks:install
```

Check release version alignment directly:

```powershell
cmd /c npm run release:version
```
