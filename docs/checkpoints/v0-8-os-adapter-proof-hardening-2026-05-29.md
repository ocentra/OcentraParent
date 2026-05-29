# V0.8 OS Adapter Proof Hardening - 2026-05-29

Branch: `codex/v0-8-os-adapter-proof-hardening`

## Scope

This checkpoint hardens worker B's V0.8 enforcement-adapter proof chain. It does
not add new OS blocking behavior. It proves the real service paths that exist
and records manual-required or unavailable states where broad product claims
would be dishonest.

## Proof Command

```powershell
node scripts/test/v0-8-os-adapter-proof-hardening.mjs
```

The command runs:

- `cmd /c npm run build:contracts`
- `cmd /c node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`
- `cmd /c node scripts/test/v0-8-production-enforcement-hardening.mjs`
- `cmd /c node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`

The wrapper validates generated artifacts and writes:

- `test-results/v0-8-os-adapter-proof-hardening/proof.json`
- `test-results/v0-8-windows-app-time-limit-adapter-mvp/*.json`
- `test-results/v0-8-production-enforcement-hardening/*.json`
- `test-results/windows-managed-unmanaged-browser-enforcement-proof/*.json`

## Proof Labels

Expected wrapper labels:

- `v0.8.app-time-limit.restart-cancel-expire-audit-proof`
- `v0.8.production-hardening.manual-required-service-boundaries`
- `v0.8.browser-boundary.pid-name-unmanaged-managed-nonclaim-proof`
- `v0.8.windows-capability-specific-os-adapter-states`
- `proof-matrix.v0-8-os-adapter-proof-hardening`

## Honest Boundaries

- Owned-process terminate and app time-limit lifecycle have real service proof
  where the host supports them.
- Broad app blocking stays manual-required or unavailable outside the
  owned-process paths.
- Network/domain blocking stays manual-required or unavailable until an
  OS-approved adapter proves real host behavior.
- Managed-browser service commands returning manual-required do not prove exact
  URL enforcement by themselves.
- Unmanaged-browser process evidence does not prove exact URL, active tab, page
  title, download source, page text, HTTPS content, or user intent.

## Manual Proof Still Required

Before upgrading V0.8 product claims:

1. Run the proof command on a real Windows child host and archive every generated
   proof JSON with the commit SHA.
2. Record Rust service logs for time-limit execute, restart recovery, parent
   cancel, unavailable recovery, expiry, audit, encrypted journal, and SQLite
   storage.
3. Record process-id-required and process-name-mismatch rejection logs showing
   that the service does not terminate an unverified process.
4. Record app/domain/managed-browser unavailable or manual-required adapter
   states without upgrading them to broad blocking claims.
5. Use managed-browser intervention proof, not service-command target strings,
   before claiming exact URL enforcement.
