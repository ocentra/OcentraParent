# WP05 Intervention Adapter Proof Gates

Scope: define network intervention proof gates for DNS proxy/block/redirect, Windows Firewall, WFP, Android VpnService, Apple Network Extension, and Linux mechanisms.

Source rows: `03-network-implementation-checklist-and-workpacks.md` rows 37-42.

Read next:

- `../02-network-tests-proof-and-validation-blueprint.md`
- `../../v0-8-enforcement-control-plan/AGENTS.md`
- platform-specific plan/docs only when the selected adapter names them

## Ownership boundary

```text
network-plan owns adapter capability, proof-state, unavailable-state, rollback, and no-production-overclaim boundaries.
ocentra-network-evidence owns Rust adapter proof helper shapes when selected.
v0-8-enforcement-control-plan owns enforcement authority and product action execution policy.
device-trust/account/setup/platform owners provide authority or device-enrollment proof when required.
```

## Expected outcome

- Every intervention path has capability status, authority tier, reversible action model, rollback/unavailable state, audit event, and parent-visible limitation.
- DNS proxy proof is separate from host firewall, WFP, VpnService, Network Extension, nftables/eBPF/TUN, or MDM/Device Owner proof.
- Platform claims remain manual-required until the required device/authority proof exists.

## Required proof fields

The selected proof must name, at minimum:

```text
adapter_kind
platform
capability_state
authority_state
target_kind
requested_action
proof_state
rollback_state
unavailable_state
audit_ref
manual_required_note
no_production_overclaim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Expected tests/proof

- `network.intervention.dns-proxy-proof`
- `network.intervention.windows-firewall-lab-proof`
- `network.intervention.wfp-manual-required`
- `network.intervention.android-vpnservice-physical-proof`
- `network.intervention.apple-network-extension-entitlement-proof`
- `network.intervention.linux-mechanism-specific-proof`
- `network.intervention.rollback-audit`
- Proof includes command log, rollback log, unavailable-state artifact, and no-production-overclaim note.

## Failure conditions

- Do not claim broad network blocking from contract or replay proof.
- Do not claim mobile or MDM authority without physical/authority-enrolled proof.
- Do not block Grade C evidence unless strict parent policy and adapter authority are proved.
- Do not upgrade adapter capability proof into product enforcement readiness.
- Do not upgrade Windows lab proof into production firewall policy readiness.
- Do not claim platform action support without rollback and unavailable-state proof.
