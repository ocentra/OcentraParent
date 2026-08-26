# Protected Capability Custody Foundation Checklist Index

Status: reset for the new neutral route.

- [ ] Read the route, plan state, workpack, and current source boundary.
- [ ] Land real Cargo manifests/targets for the neutral protocol, broker, and
  client, register them in the active workspace only after the source targets
  exist, and add the narrow core admission/facade seam without unsealing owner
  traits.
- [ ] Write the complete isolated broker/client production source.
- [ ] Write all expected core-unit, protocol-contract, broker authority/race/
  process, client admission/IPC, restart, replay, and Windows integration tests.
- [ ] Run focused core/protocol/broker/client compilation and
  architecture/Enforcer gates.
- [ ] Execute the complete WP01 test family and repair failures.
- [ ] Verify broker-owned ACL/path/key/watermark/write-lease authority on Windows.
- [ ] Collect retained proof and update the proof index.
- [ ] Reconcile checklist and graph state from current evidence.
- [ ] Prepare one coherent PR only after the repo-wide final gate is assigned.

The existing core source does not check these rows. Mapped paths and graph
ownership are routing evidence, not execution evidence.

## Split workpack obligations — 2026-08-25

These rows are deliberately unchecked. They select ownership and expected
evidence; they do not authorize fake providers, external-owner substitution,
runtime readiness, or DONE.

- [ ] WP02: obtain the external OEM/firmware/MDM Enrollment/SCM/TPM owner
  transaction and write the fixed protected handoff under the mapped core and
  provisioner roots.
- [ ] WP02: add `tests/unit/windows_adapter.rs` and
  `provisioner/tests/integration/owner_handoff.rs` against the real owner
  boundary.
- [ ] WP03: implement the core Windows monotonic and platform anti-rollback
  provider only with hardware-backed currentness; add
  `tests/security/tpm_nv_counter.rs`.
- [ ] WP04: implement the fixed-pipe client admission and retained OS-derived
  broker anchor; add broker custody, client admission, and IPC-authentication
  tests.
- [ ] WP04: retain the reviewed WP01 edge and implementation-independent
  WP02/WP03/Parent WP12 edges for bounded fixed-pipe/client-anchor source
  shaping; normal completion dependencies remain hard and derived state stays
  blocked.
- [ ] WP05: route the Account-owned `crates/account-issuer-owner` statically
  linked into the existing broker, with WP01 reviewed-implementation and
  WP02/WP03/WP04 implementation-independent edges. Preserve family-core's
  VerifiedAccountIdentityAuthority, authority repository/source of truth,
  existing family-owned handoff contract remains a separate historical/input
  boundary and is never embedded, re-signed, or duplicated inside P-256 v2;
  preserve one opaque `BEGIN IMMEDIATE` host; do not open a second Account
  connection or merge with `custody.sqlite`.
- [ ] WP05: implement the selected self-contained TPM-native P-256 inner and
  outer v2 shape: producer `ocentra.account-authority-producer.v2`, inner
  domain `ocentra.account-authority-producer.signing.v2\0`, audience
  `ocentra.account.authority.v2`, algorithm `ecdsa-p256-sha256-p1363`, outer
  domain `ocentra.account-issuer.transport.v2\0`, service
  `ocentra.account-authority-producer.cloudflare.v2`, and
  `sha256:ecdsa-p256:<hex>` v2-derived key IDs. Keep Ed25519 v1 historical
  parse/verify-only with no new signing or fallback.
- [ ] WP05 source packet 1 freezes canonical 65-byte SEC1, exact 64-byte
  low-S P1363, FFI low-S canonicalization, Rust/Cloudflare high-S rejection,
  protocol envelope kinds 6/7 are AccountIssuerRequest/AccountIssuerResponse carrying the inner operations IssueCurrentAuthority and AcknowledgeReceipt; Verify remains owner-local and is not a protected protocol message,
  `Verify`, enrollment metadata, and authority/key-generation binding before
  parallel owner/FFI/facade code. Map the exact schema, family
  producer/parser/envelope, protocol codec, and Cloudflare contract/transport
  roots; only shared module wiring is serialized here.
