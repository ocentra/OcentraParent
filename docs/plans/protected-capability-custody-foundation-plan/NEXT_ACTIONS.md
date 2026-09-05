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
5. Reopen WP05 for the exact P0 live-call defect: preserve the still-current
   `BrokerAuthorizedClientTranscript` through broker Account dispatch, derive
   one opaque non-cloneable request-scoped Account admission from the external
   enrollment/service/key-lineage owner, and consume it in
   `account-issuer-owner` plus the single family transaction facade. The fixed
   mount and `authorize_protected_request` must remain fail-closed until this
   path is real. Do not add raw fields, booleans, closures, caller selectors,
   static keys, a second database, or a family-to-Protected crate cycle.
6. Complete the seven existing Protected/Account test roots for transcript
   retention, correlation/idempotency, currentness/generation drift,
   revocation, restart reconciliation, and owner unavailability. They are
   present but unexecuted. Cloudflare current-v2 tests belong to Cloudflare
   WP06. Keep WP02/WP03/WP04 operational absence fail-closed; tests must not
   invent an owner, signer, service identity, or deployment success.
7. Keep Parent WP12 installer-only and package-focused. No package success may
   mint protected authority. Tests, proof, validation, and DONE remain later
   gates for every split row.
