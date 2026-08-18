# Next Actions

## Ordered runtime-owner routing (audit truth, not completion)

1. Account Identity WP08: keep the Rust-owned canonical household/child/device/
   pairing/install/route/lifecycle/revocation binding as the contract producer.
2. Cloudflare WP06: make the provider-gated durable repository/caller the
   current-authority bridge; no fixture, provider-subject mapping, or caller
   selector may substitute for it.
3. Device Trust WP03: after WP01 plus Account/Cloudflare are reachable, own the
   parent step-up/passkey ceremony, one-time `RegisterLanSignerAnchor`
   authorization, signature verification, sign-count, and nonce consumption.
4. Device Trust WP02 (conditional): when a real private-key/install custody
   path is selected, compose parent-runtime/platform sealing, lifecycle
   generation, and revocation. The selected route must carry the reviewed WP26
   -> WP02 gate to completion before LAN/child consumers proceed; the default
   non-sealing route does not add WP02 as a hard dependency. WP02 cannot issue
   ceremony authority.
5. LAN WP26 and child/runtime consumers: only after WP03, and after the selected
   WP02 gate when applicable, consume current binding/revocation for signed
   ingress; never register or infer signer authority locally.

This order is intentionally non-circular: WP01 is the foundation/source owner,
Account WP08 and Cloudflare WP06 bridge current authority before WP03, WP03 is
the ceremony owner, and downstream LAN/child consumers do not unlock WP03. The
optional WP02 gate points only into the downstream consumer route and never back
to WP03.

1. Finish the repository-wide production-source wave and integration review before starting this plan's expected-test migration. Do not interleave one-line source edits with test execution.
2. Treat merged PR #605 and its fresh 60-job CI as narrow unissued-parent-challenge test evidence only; do not promote it into workpack or plan closure.
3. Choose the real runtime owners for each missing slice before implementation: `family-domain`, `lan-domain`, billing entitlement, Rust protocol/service, and any new trust-bootstrap runtime module.
4. Re-open WP02, WP03, WP04, and WP06 around actual runtime seams: platform key sealing, parent step-up, QR approval, and encrypted recovery bundles.
5. Re-open WP05 and WP07 around real trust binding and parent-controlled uninstall/tamper execution instead of contract-only frontage. WP07 has a code-drafted local evidence/manual-required boundary; platform removal, attestation, transport, tests, and proof remain open.
6. After source is complete, write the full expected-test delta for WP01/WP05/WP06/WP07, then run focused crate/domain validation and Enforcer. Proof remains a later phase under `output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/`.

## Accepted source checkpoint (2026-08-17)

The independently accepted Device Trust branch `914d06b6a` is integrated
through `68717b5b7`. WP01 now preserves owner-resolved current device/signer
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

WP01 source is integrated through `68717b5b7`. It has crash-safe,
process-serialized lifecycle-authority sidecar writes, fail-closed corruption
and missing-authority handling, strict no-repair validation for existing
databases, concurrency-tolerant first-open initialization, a SQLite busy
timeout, and crate-private raw enrollment/revoke/reset mutation entrypoints.
Independent source review and focused static gates accepted the bounded packet.
Platform custody, trusted ceremony issuance, expected-test migration,
functional validation, proof, and broader lifecycle integration remain open.

The WP02 Windows custody revoke/reset path now refuses local unauthenticated
removal and preserves manual-required truth until a trusted parent authority
provider and ceremony issuer are owned by the correct runtime.

WP03 now has a bounded receipt lifetime gate and an independently static-reviewed
remote-safe integration source packet for atomic ceremony custody/recovery plus strict
linked-challenge lifecycle validation. It remains blocked on Device Trust WP01,
the Account WP08 canonical household/child/device/pairing contract, the
Cloudflare WP06 durable repository/caller, a real passkey/OS-native authority
adapter, durable sign-count ownership, signature verification, nonce
consumption, focused tests, and retained runtime proof.

WP04 now has a typed challenge/response boundary and fail-closed verifier port,
but remains blocked on the external issuer/signature authority, phone
ceremony, one-time nonce consumer, transport, and retained runtime proof.

WP05 now has a device-bound entitlement verifier boundary, but remains blocked
on the real signature/revocation provider and retained runtime proof; the
unavailable default keeps capability unlock manual-required.

WP06 now blocks the untrusted confirmation-only restore path and exposes a
verified-parent re-pair gate plus an unavailable-by-default restore executor;
only a coherent execution receipt can project applied/partial state.
Encrypted bundle/key custody, revocation preservation, and runtime proof
remain open.

## Production-code audit boundary (2026-08-16)

### LAN WP26 routing correction

Device Trust WP01 owns the persistent trusted-device/signer-key lifecycle
source, but it is not sufficient by itself. Account Identity WP08 must define
the canonical household/child/device/pairing authority binding, and Cloudflare
WP06 must extend its durable store and production caller to persist and resolve
that binding. The existing provider-subject mapping, Account WP05 pure
evaluator, and local LAN registry are not substitutes.

Device Trust WP03 remains blocked in the default graph on all three owners, but
its bounded source packet is now authorized in the implementation-only phase by
reviewed-implementation gates for WP01, Account WP08, and Cloudflare WP06. This
does not authorize a real parent ceremony, provider, runtime caller, tests,
proof, or completion. Normal WP03 readiness remains blocked until a real parent
ceremony resolves the target authoritatively, provides one-time
`RegisterLanSignerAnchor` authorization, verifies the signature, owns the
durable sign counter, and consumes the nonce/receipt. LAN WP26 must remain
blocked on the Account WP08 -> Cloudflare WP06 authority bridge and WP03
ceremony; WP01 supplies the current-binding foundation and no shipped service
route can legally register a signer anchor today. If the platform
sealing/lifecycle-revocation path is selected, promote the reviewed WP26 ->
WP02 gate before assigning that consumer route; otherwise WP02 is not a hard
dependency. WP02 is conditional only when a demonstrated private-key/install
custody requirement exists.

The consolidated source audit found no legal production-code slice to add:

- WP01/WP02 require a shipped ceremony issuer, platform custody provider, and
  registered parent-runtime/desktop composition. The current source remains
  fail-closed/manual-required where those owners are absent.
- WP03 first requires the Account WP08 canonical target-binding contract and
  Cloudflare WP06 durable authoritative repository/caller; it then requires a
  real passkey/OS ceremony, durable sign counter, signature verification, and
  one-time nonce consumption. WP04 separately requires its phone ceremony and
  transport callers. Typed receipts, request-bound IDs, and QR contracts are
  not authority.
- WP05 requires a real entitlement signature/revocation provider; WP06 requires
  encrypted recovery/key custody and a real restore executor; WP07 requires
  platform removal/attestation and parent transport. None may be invented in
  this plan as a test, proof, fixture, generic JSON, or DTO-only bridge.
- WP08 and WP09 have no production behavior to implement in this pass.

The next dependency chain is Account WP08 canonical binding -> Cloudflare WP06
durable repository/caller -> WP03 trusted target resolution and platform/passkey
ceremony, alongside completion of the WP01 trust lifecycle source. Until those
owners are dependency-legal and reachable from shipped entrypoints, preserve
the manual-required outcomes and do not claim trust bootstrap, device sealing,
recovery, entitlement unlock, or child uninstall.
