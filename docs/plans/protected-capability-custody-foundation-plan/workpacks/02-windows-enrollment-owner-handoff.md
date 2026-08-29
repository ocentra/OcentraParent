# Workpack 02 - Windows Enrollment Owner Handoff

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Workpack: `02-windows-enrollment-owner-handoff`
> Kind: external-owner and protected enrollment transaction route.
> Proves: ownership and expected source/test boundaries only.
> Does not prove: external authority, implementation, tests, proof, READY, or DONE.

<!-- /agent-capsule -->

## Purpose

Define the one protected Enrollment/SCM/TPM owner transaction needed after the
neutral WP01 foundation. The transaction is dependent on an external
OEM/firmware/MDM owner that can authorize `TPM_RH_PLATFORM` and the associated
NV/SCM/registry enrollment. It must also establish the immutable Account issuer
service-binding and current-key-lineage anchor consumed by WP05; request fields
or later broker code cannot invent that binding. That authority is not present
in this checkout.

## Ownership and fail-closed boundary

Protected Custody owns the fixed owner transaction and its authenticated
provenance. The provisioner package may invoke the fixed owner-approved
operation, but setup, MSI properties, environment values, caller identity,
SQLite/JSON rows, and generated receipts cannot mint or replace it. Missing,
contradictory, stale, revoked, or unavailable owner state remains
`ExternalProvisioningRequired`/manual-required and cannot be reported as
protected readiness.

The external record may bind the fixed Account issuer service and protected
key-generation/currentness anchor. It must not carry caller-selected household,
member, device, key, or provider claims. Those remain Account-owned runtime
truth and are resolved only after protected admission.

## Current feasibility audit

The checkout contains only fixed constants/read-only preflight, CNG
existing-key open, NV public/read/increment for an already-defined object, and
Registry/SCM read-only observation. It does not contain the hard P0 owner
transaction for `TPM2_NV_DefineSpace`/`TPM2_NV_UndefineSpace` codecs and
allowlist, the `TPM_RH_PLATFORM` hierarchy owner ceremony, protected Registry
write/ACL transaction, fixed SCM create/config/security/delete transaction,
enrolled generation, Account issuer service/key-lineage binding, or independent
current observations. TBS, LocalSystem, or process elevation is not the
platform owner.

This is **ACCEPT-for-source-design only**. No honest executable WP02 source
packet is authorized until the external OEM/firmware/MDM owner is available;
the route remains `ExternalProvisioningRequired`/manual-required and blocked.

## Expected production roots

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/enrollment.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/scm.rs
crates/ocentra-protected-capability-custody-provisioner/Cargo.toml
crates/ocentra-protected-capability-custody-provisioner/src/main.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/mod.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/owner_handoff.rs
```

## Expected test source

```text
crates/protected-capability-custody-core/tests/unit/windows_adapter.rs
crates/ocentra-protected-capability-custody-provisioner/tests/integration/owner_handoff.rs
```

Tests must use the real owner boundary and cover absent owner authority,
enrollment mismatch, protected registry/SCM custody, and rollback/re-pair
failure. No fake TPM, caller assertion, raw `authValue`, or no-op owner is
valid evidence.

## Graph state and exit conditions

WP02 has a hard WP01 foundation dependency and no implementation authorization
while the external owner is absent. It remains blocked, with source, tests,
proof, and DONE open. Exit requires the external owner transaction, real
production composition, the immutable Account issuer service/key-lineage
binding needed by WP05, focused validation, the two expected test roots, and
retained proof; graph topology alone cannot promote the route.
