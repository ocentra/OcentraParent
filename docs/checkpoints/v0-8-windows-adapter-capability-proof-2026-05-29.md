# V0.8 Windows Adapter Capability Proof

Date: 2026-05-29
Owner lane: codex-b
Branch: codex/v0-8-windows-adapter-capability-proof

## Scope

This checkpoint adds a Rust-facing protocol and service read-model proof that ties the merged broad OS adapter readiness matrix to the merged host identity read model.

Covered capability surfaces:

- Windows app targets
- Windows domain and network targets
- managed-browser service-command targets
- unmanaged-browser process-only targets
- unsupported OS fallback states
- rollback and audit custody gates

## Product Truth

This slice is not broad app blocking, network/domain blocking, managed-browser exact URL control, unmanaged browser exact evidence, admin hardening, anti-tamper, or rollback enforcement.

The proof keeps:

- app target enforcement manual-required until installed inventory, process lineage, executable identity, package identity, publisher/signature, inventory/process join, apply, rollback, and audit artifacts exist;
- domain and network targets manual-required until a host network filter proves apply and rollback behavior;
- managed-browser service commands manual-required and separate from exact URL enforcement;
- unmanaged-browser support process-only, with exact URL, active tab, title, download source, page text, HTTPS content, and intent not claimed;
- unsupported OS targets unavailable instead of borrowing Windows adapter claims;
- rollback and audit readiness manual-required until same-identity apply, rollback, and custody evidence exists.

## Counts

- Read-model entries: 6
- Linked broad readiness rows: 8
- Linked host identity rows: 9
- Exact URL claimed rows: 0
- Broad blocking claimed rows: 0

## Focused Proof

Required command:

```powershell
node scripts/test/v0-8-windows-adapter-capability-proof.mjs
```

The proof command runs the Windows adapter capability Rust protocol and service read-model tests, the broad OS adapter readiness proof harness, the host identity read-model proof harness, and the pre-AI proof matrix validation. It writes:

```text
test-results/v0-8-windows-adapter-capability-proof/proof.json
```

## Manual Gaps Before Claim Upgrade

- Run real Windows app identity, block apply, rollback, and audit custody proof for the same package or executable identity.
- Run host network filter or DNS/VPN adapter apply and rollback proof before domain/network blocking can upgrade.
- Run managed-browser active-tab and exact URL enforcement proof before exact URL control can upgrade.
- Preserve unmanaged-browser exact URL, active tab, title, download source, page text, HTTPS content, and intent as not claimed without explicit browser integration proof.
- Keep unsupported OS, Android device-owner, iOS Family Controls, signing, stores, admin hardening, anti-tamper, and bypass resistance manual-required or unavailable until real artifacts exist.
