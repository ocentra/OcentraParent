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

WP03 now has a bounded receipt lifetime gate, but remains blocked on the real
passkey/OS-native authority adapter, signature verification, nonce consumption,
and retained runtime proof.

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
