# Protected Capability Custody Foundation Plan Next Actions

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Next Actions
> Kind: resume queue.
> Proves: routing only; it is not a completion certificate.

<!-- /agent-capsule -->

1. Preserve the accepted CNG/TPM mechanics, private `cfg(windows)` core
   adapter, the WP01-owned BIN-only provisioner source, and the reviewed WP04
   OS-observation primitives. Do not preserve the superseded claim that WP04
   implementation is complete. The graph
   records 114 implementation files, 0 tests,
   and no workspace requirement gaps. The provisioner is only a read-only
   preflight: it revalidates enrollment and always returns
   `ExternalProvisioningRequired`; it cannot create or publish enrollment.
   This is not READY or DONE evidence.
2. Obtain the external protected runtime authority that remains unavailable:
   OEM/firmware/MDM authorization for `TPM_RH_PLATFORM` plus NV define/undefine
   lifecycle, authenticated owner handoff, protected registry/SCM mutation,
   enrolled counter generation, operational use of the independent
   broker/client/token observations, and the core monotonic provider. The
   bounded WP04 observation adapters are source-present, but startup must remain
   fail-closed `DeploymentRequired` before DB/state/listener mutation; there is
   no reachable success path in the current checkout. Parent Runtime WP12 owns
   only package invocation and lifecycle.
3. Repair only the two authorized WP04 internal defects: obtain fresh fallible
   broker platform-session state before every broker hello, and revalidate
   readiness/currentness through listener lifetime so owner/currentness drift
   drops the listener and reports SCM `Stopped` nonzero. Keep ordinary malformed
   peer failures connection-local. Do not use this repair to create enrollment,
   monotonic authority, caller identity, or a new handshake.
4. Connect and verify the real owner-bound broker/client transport caller only
   after the protected owner and WP12 package boundaries exist. The WP04 fixed
   pipe, retained OS anchor, and fail-closed broker/client composition are
   source-present; no second helper process/protocol, caller-supplied
   identity/attestation, raw `authValue`, disk generation restore, or
   caller-minted authority is allowed.
5. After production source is stable, write and execute all 13 expected test
   roots listed in `TEST_PROOF_EXPECTATIONS.md`, including the core-private
   Windows adapter and TPM2 NV/TBS monotonic-counter roots. Tests must exercise
   the real private seams and must not bless a disconnected helper, fake
   authority, or caller assertion.
6. Run the selected focused source/tests and Enforcer/architecture checks,
   then update checklist and retained proof. Repo-wide Enforcer, pre-commit,
   one PR, long CI, and promotion remain final gates.

## Explicit no-go actions

- Do not implement an in-process broker or a same-process DPAPI/file-lock
  substitute.
- Do not accept caller-supplied attestation, key choice, capability, lease, or
  success flags.
- Do not mark the route READY/DONE because the integrated source compiles or the
  graph observes its planned roots.

## Split routing queue — 2026-08-25

1. Keep WP01 as the neutral foundation and retain its fail-closed preflight.
2. Resolve the external OEM/firmware/MDM owner transaction before authorizing
   WP02 Windows enrollment source. The fixed Enrollment/SCM/TPM transaction,
   `tests/unit/windows_adapter.rs`, and
   `provisioner/tests/integration/owner_handoff.rs` remain
   open.
3. Route WP03 only after WP01 and the WP02 owner transaction. Its bounded roots
   are `core windows/monotonic.rs` and `platform/anti_rollback.rs`; the
   `tests/security/tpm_nv_counter.rs` obligation remains absent.
4. Keep WP04 normal-blocked but implementation-repair-authorized only for the
   fresh per-hello platform-state load and listener-lifetime fatal-currentness
   handling across the broker custody/runtime/peer/service roots. Its WP01 edge
   remains reviewed-implementation; WP02/WP03/Parent WP12 edges are
   implementation-independent for this source phase. No sysinfo, caller
   identity, handshake redesign, external-owner implementation, or runtime
   readiness claim is authorized. The three transport tests, owner-bound
   caller, operational dependencies, proof, and DONE remain open.
5. Route WP05 as planned/source-authorable for the Account-owned
   `crates/account-issuer-owner` statically linked into the existing broker.
   It absorbs issuer/key-registry/outbox/delivery/startup/recovery/signing/RPC
   mechanics while family-core retains `VerifiedAccountIdentityAuthority`, the
   authority repository/source of truth, and one opaque `BEGIN IMMEDIATE`
   transaction/currentness host. The existing family-owned handoff contract
   remains a separate historical/input boundary and is never embedded,
   re-signed, or duplicated inside P-256 v2. The broker mounts
   the owner for service lifetime and retains protected signer custody; the
   owner receives opaque Account-specific capabilities and never opens a second
   connection or accesses `custody.sqlite`.
6. Freeze WP05 source packet 1 before parallel implementation: the
   self-contained P-256 inner/outer v2 producer, inner/outer domains, audience,
   service, algorithm-aware `sha256:ecdsa-p256:<hex>` key ID, canonical 65-byte
   SEC1 public key, exact 64-byte low-S P1363 signature, FFI low-S
   canonicalization, Rust/Cloudflare high-S rejection,
   protocol envelope kinds 6/7 are AccountIssuerRequest/AccountIssuerResponse carrying the inner operations IssueCurrentAuthority and AcknowledgeReceipt; Verify remains owner-local and is not a protected protocol message,
   `Verify`, enrollment metadata, and authority/key-generation binding. Map the
   exact schema, family producer/parser/envelope, protocol codec, and
   Cloudflare contract/transport roots; serialize only shared contract/module
   wiring. Then implement the selected TPM-native P-256 Account issuer shape:
   runtime `NCryptIsAlgSupported`/`EnumAlgorithms`, unique non-exportable
   signing-only PCP key with service ACL, canonical 65-byte SEC1 export,
   exact 64-byte low-S P1363 signature over SHA-256 of original canonical
   bytes, schema/D1 v2, and Cloudflare original-byte verification without
   digest/double-hash. Keep Ed25519 v1 historical parse/verify-only; it is not a
   newly signed inner wire, fallback, or downgrade. Unsupported TPM/manual
   enrollment fails closed. Attestation, rotation,
   recovery, provider binding, service-binding lineage, tests, proof, and DONE
   remain open.
   Keep service-specific key custody fail-closed: external provisioning must
   create/set the service ACL, while broker revalidates only. Retain the WP04
   FFI/core TokenGroups and LookupAccountNameW service-SID observation roots
   now present in source, and add only the WP05 CNG security-descriptor
   revalidation root. Use locked ring
   0.17.14 ECDSA_P256_SHA256_FIXED after explicit low-S precheck; sha2 is only
   for key-ID hashing and no p256/ecdsa dependency is permitted.
7. Keep Parent WP12 installer-only and package-focused. No package success may
   mint protected authority. Tests, proof, validation, and DONE remain later
   gates for every split row.
