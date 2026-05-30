# V0.8 Broad OS Adapter Proof

Date: 2026-05-30
Worker: codex-b
Branch: codex/v0-8-broad-os-adapter-proof

## Scope

This checkpoint records the current V0.8 broad OS adapter proof boundary. It composes the already-proved managed-session intervention, owned-process pid/name guardrails, unmanaged process terminate/warn boundary, and app timer lifecycle without upgrading broad app/domain/browser, exact browser evidence, non-Windows, mobile, admin hardening, rollback, or anti-tamper claims.

## Captured States

- Windows managed-session intervention: implemented only for the managed browser path.
- Windows owned-process pid/name guardrail: implemented only as process-scoped control.
- Windows unmanaged browser boundary: implemented only as process terminate/warn behavior.
- Windows app time-limit lifecycle: implemented for timer custody, restart recovery, parent cancel, expiry, and audit.
- Windows broad installed-app blocking: manual-required until app identity, apply, rollback, and custody artifacts exist.
- Windows network/domain blocking: manual-required until host network filter or DNS/VPN apply, rollback, and custody artifacts exist.
- Windows managed browser exact URL: manual-required until active-tab and exact URL apply/rollback/audit artifacts exist.
- Windows unmanaged browser exact evidence: not claimed.
- Linux broad OS adapter: unavailable in this proof.
- macOS, Android, and iOS adapters: manual-required until target-platform permission, package, device-policy, entitlement, and rollback artifacts exist.

## Validation Command

```powershell
cmd /c node scripts/test/v0-8-broad-os-adapter-proof.mjs
```

The harness writes `test-results/v0-8-broad-os-adapter-proof/proof.json` with proof counts, linked commands, non-claims, target platform gaps, and the manual artifact checklist required before any support upgrade.
