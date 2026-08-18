# Next Actions

## Ordered runtime-owner routing (audit truth, not completion)

1. Account Identity WP08: keep the Rust-owned canonical household/child/device/
   pairing/install/route/lifecycle/revocation binding, sealed capability, and
   local repository as the contract foundation.
2. Account Identity WP02: add target-aware action authority that keeps the
   actor parent-controller device distinct from the target child/profile/device
   and derives same-family, capability, lease, and step-up from owned state.
3. Cloudflare WP06: implement authoritative D1 create/update/revoke/currentness/
   CAS plus the shipped Firebase/provider-to-sealed-authority caller; its
   current read adapter is not a live current-authority bridge.
4. Device Trust WP03: after WP01 plus Account/Cloudflare are reachable, own the
   parent step-up/passkey ceremony, one-time `RegisterLanSignerAnchor`
   authorization, signature verification, sign-count, and nonce consumption
   while preserving actor/target separation.
5. Device Trust WP02 (conditional): when a real private-key/install custody
   path is selected, compose parent-runtime/platform sealing, lifecycle
   generation, and revocation. The selected route must carry the reviewed WP26
   -> WP02 gate to completion before LAN/child consumers proceed; the default
   non-sealing route does not add WP02 as a hard dependency. WP02 cannot issue
   ceremony authority.
6. LAN WP26 and child/runtime consumers: only after WP03, and after the selected
   WP02 gate when applicable, consume current binding/revocation for signed
   ingress; never register or infer signer authority locally.

This order is intentionally non-circular: WP01 is the foundation/source owner,
Account WP08, target-aware Account WP02, and Cloudflare WP06 bridge current
authority before WP03; WP02 is transitive through WP06 rather than a duplicate
direct WP03 edge. WP03 is the ceremony owner, and downstream LAN/child
consumers do not unlock WP03. The
optional WP02 gate points only into the downstream consumer route and never back
to WP03.

1. Finish the repository-wide production-source wave and integration review before starting this plan's expected-test migration. Do not interleave one-line source edits with test execution.
2. Treat merged PR #605 and its fresh 60-job CI as narrow unissued-parent-challenge test evidence only; do not promote it into workpack or plan closure.
3. Choose the real runtime owners for each missing slice before implementation: `family-domain`, `lan-domain`, billing entitlement, Rust protocol/service, and any new trust-bootstrap runtime module.
4. Re-open WP02, WP03, WP04, and WP06 around actual runtime seams: platform key sealing, parent step-up, QR approval, and encrypted recovery bundles.
5. Re-open WP05 and WP07 around real trust binding and parent-controlled uninstall/tamper execution instead of contract-only frontage. WP07 has a code-drafted local evidence/manual-required boundary; platform removal, attestation, transport, tests, and proof remain open.
6. After source is complete, write the full expected-test delta for WP01/WP05/WP06/WP07, then run focused crate/domain validation and Enforcer. Proof remains a later phase under `output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/`.

## Accepted source checkpoint (2026-08-17)

The independently reviewed branch `914d06b6a` is superseded/rejected and is
not integrated because its authority path was forgeable. The canonical
reconciled source preserves owner-resolved current device/signer
authority without a public household-authority mint path. The integration
reconciliation keeps WP05's entitlement state unsigned and fail-closed, removes
caller-minted restore authority from WP06, and binds WP07 durable removal state
to service readiness across restart. No real platform/passkey ceremony,
entitlement issuer/revocation provider, restore executor/key-custody owner,
parent transport, or platform removal caller was invented.

These are production-source checkpoints only. The complete expected-test wave,
focused execution/fixes, proof, precommit, PR, and CI remain deliberately later.

## Latest code/test checkpoint (2026-08-09)

WP08's review matrix and boundary test are current, but this workpack remains
research-level. Continue with the runtime-owned gaps in WP02/WP03/WP04/WP06
before treating any dependency candidate as an adopted trust-root component.

## Production-code checkpoint (2026-08-16)

The canonical WP01 source has crash-safe,
process-serialized lifecycle-authority sidecar writes, fail-closed corruption
and missing-authority handling, strict no-repair validation for existing
databases, concurrency-tolerant first-open initialization, a SQLite busy
timeout, and crate-private raw enrollment/revoke/reset mutation entrypoints.
Independent source review and focused static gates accepted the bounded packet.
Platform custody, trusted ceremony issuance, expected-test migration,
functional validation, proof, and broader lifecycle integration remain open.

WP02 Windows custody source is present, but
`require_authenticated_parent_authority()` is permanently unavailable before
record/epoch mutation. No ceremony issuer, registered desktop/native mount, or
custody-to-lifecycle activation/startup caller exists; sealing, unsealing, and
revocation therefore remain manual-required. Existing lifecycle/custody tests
and synthetic parent-presence probes are stale/non-authoritative and must not
be used as DPAPI proof.

WP03 has a bounded receipt lifetime gate and independently static-reviewed
remote-safe integration source packet for atomic ceremony custody/recovery plus strict
linked-challenge lifecycle validation. It remains blocked on Device Trust WP01,
the Account WP08 canonical binding, target-aware Account WP02 authority
transitively through Cloudflare WP06, the missing Cloudflare writer/provider
caller, a real passkey/OS-native authority adapter, durable sign-count
ownership, signature verification, nonce consumption, focused tests, and
retained runtime proof. Planned `parent_step_up_target_authority.rs` and
`parent_step_up_runtime.rs` owners do not exist yet.

WP04 now has a typed challenge/response boundary and fail-closed verifier port,
but remains blocked on the external issuer/signature authority, phone
ceremony, one-time nonce consumer, transport, and retained runtime proof.

WP05 now has a crate-private device-bound entitlement verifier boundary, but
remains blocked on the real signature/revocation provider, child-runtime action
owner/startup mount, and retained runtime proof; no entitlement startup or
capability unlock route is exported in the reviewed source-repair wave.

WP06 now blocks the untrusted confirmation-only restore path. The independently
reviewed source repair integrated through `f656a80a1` removes the raw
installation-generation repair stub, refuses to
construct encrypted bundle metadata without a real key-custody owner, and
requires an exact current tombstone cursor for preflight. Because the current
context is only a caller-held snapshot, the apply seam is now unconditionally
blocked and cannot dispatch an executor; a future owner must provide an
opaque cursor token that is reread and consumed at apply time. Encrypted
bundle/key custody, durable revocation currentness, authorized re-pair, a real
executor, production callers, expected tests, and runtime proof remain open.
Its storage boundary overlaps accepted Data WP05 source candidate `e91bb3de1`;
that candidate must rebase on the integrated WP06 safety boundary and preserve
the blocked restore contract before Data integration.

## Production-code audit boundary (2026-08-16)

### LAN WP26 routing correction

Device Trust WP01 owns the persistent trusted-device/signer-key lifecycle
source, but it is not sufficient by itself. Account Identity WP08 must define
the canonical household/child/device/pairing authority binding, and Cloudflare
WP06 must extend its durable store and production caller to persist and resolve
that binding. The existing provider-subject mapping, Account WP05 pure
evaluator, and local LAN registry are not substitutes.

Correction (superseding the earlier implementation-authorized wording): Device
Trust WP03 remains blocked in both the default graph and the implementation-only
phase. Its reviewed edges do not authorize bounded source work while Cloudflare
WP06 lacks the planned authoritative caller/writer evidence. Account WP02 alone
is currently eligible for implementation-only work. This route does not
authorize a real parent ceremony, provider, runtime caller, tests, proof, or
completion. Normal WP03 readiness remains blocked until a real parent
ceremony resolves the target authoritatively, provides one-time
`RegisterLanSignerAnchor` authorization, verifies the signature, owns the
durable sign counter, and consumes the nonce/receipt. LAN WP26 must remain
blocked on the Account WP08 -> Account WP02 -> Cloudflare WP06 authority bridge and WP03
ceremony; WP01 supplies the current-binding foundation and no shipped service
route can legally register a signer anchor today. If the platform
sealing/lifecycle-revocation path is selected, promote the reviewed WP26 ->
WP02 gate before assigning that consumer route; otherwise WP02 is not a hard
dependency. WP02 is conditional only when a demonstrated private-key/install
custody requirement exists.

The consolidated source audit found no legal production-code slice to add:

- WP01/WP02 require a shipped ceremony issuer, platform custody provider, and
  registered parent-runtime/desktop composition. WP02's current
  `require_authenticated_parent_authority()` boundary is permanently
  unavailable, so its Windows DPAPI/registry source remains
  fail-closed/manual-required until those owners and fresh runtime proof exist.
- WP03 first requires the Account WP08 canonical binding, target-aware Account
  WP02 authority transitively through Cloudflare WP06, and Cloudflare's
  authoritative writer/provider caller; it then requires a
  real passkey/OS ceremony, durable sign counter, signature verification, and
  one-time nonce consumption. WP04 separately requires its phone ceremony and
  transport callers. Typed receipts, request-bound IDs, and QR contracts are
  not authority.
- WP05 requires a real entitlement signature/revocation provider; WP06 requires
  encrypted recovery/key custody and a real restore executor; WP07 requires
  platform removal/attestation and parent transport. None may be invented in
  this plan as a test, proof, fixture, generic JSON, or DTO-only bridge.
- WP08 and WP09 have no production behavior to implement in this pass.

The next dependency chain is Account WP08 canonical binding -> Account WP02
target-aware action authority -> Cloudflare WP06 authoritative writer/provider
caller -> WP03 trusted target resolution and platform/passkey ceremony,
alongside completion of the WP01 trust lifecycle source. Until those
owners are dependency-legal and reachable from shipped entrypoints, preserve
the manual-required outcomes and do not claim trust bootstrap, device sealing,
recovery, entitlement unlock, or child uninstall.
