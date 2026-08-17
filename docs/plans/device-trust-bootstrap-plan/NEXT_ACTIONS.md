# Next Actions

1. Keep the plan truth aligned: do not reintroduce `complete`, `proof-covered`, or existing-proof claims until artifacts and scoped validation exist on disk.
2. Treat merged PR #605 and its fresh 60-job CI as narrow unissued-parent-challenge test evidence only; do not promote it into workpack or plan closure.
3. Choose the real runtime owners for each missing slice before implementation: `family-domain`, `lan-domain`, billing entitlement, Rust protocol/service, and any new trust-bootstrap runtime module.
4. Re-open WP02, WP03, WP04, and WP06 around actual runtime seams: platform key sealing, parent step-up, QR approval, and encrypted recovery bundles.
5. Re-open WP05 and WP07 around real trust binding and parent-controlled uninstall/tamper execution instead of contract-only frontage. WP07 has a code-drafted local evidence/manual-required boundary; platform removal, attestation, transport, tests, and proof remain open.
6. Replace doc-only proof with runtime coverage and keep proof outside the plan folder under `output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/`.

## Latest code/test checkpoint (2026-08-09)

WP08's review matrix and boundary test are current, but this workpack remains
research-level. Continue with the runtime-owned gaps in WP02/WP03/WP04/WP06
before treating any dependency candidate as an adopted trust-root component.

## Production-code checkpoint (2026-08-16)

WP01 now has crash-safe, process-serialized lifecycle-authority sidecar writes
with fail-closed corruption and missing-authority handling. Platform custody,
trusted ceremony issuance, and the broader lifecycle integration remain open;
tests and proof are deferred.

The WP02 Windows custody revoke/reset path now refuses local unauthenticated
removal and preserves manual-required truth until a trusted parent authority
provider and ceremony issuer are owned by the correct runtime.

WP03 now has a bounded receipt lifetime gate and an independently static-reviewed
uncommitted source packet for atomic ceremony custody/recovery plus strict
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

Device Trust WP03 depends on all three owners and remains blocked until a real
parent ceremony resolves the target authoritatively, provides one-time
`RegisterLanSignerAnchor` authorization, verifies the signature, owns the
durable sign counter, and consumes the nonce/receipt. The existing WP03 -> WP01
implementation gate does not bypass the new strict Account WP08 and Cloudflare
WP06 dependencies. LAN WP26 must remain blocked on WP01 and WP03; it has no
shipped service route that can legally register a signer anchor today. WP02 is
conditional only when a demonstrated private-key/install custody requirement
exists.

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
