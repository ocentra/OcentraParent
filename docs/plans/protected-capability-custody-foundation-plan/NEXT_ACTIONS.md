# Protected Capability Custody Foundation Plan Next Actions

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Next Actions
> Kind: resume queue.
> Proves: routing only; it is not a completion certificate.

<!-- /agent-capsule -->

1. Add the planned `ocentra-protected-capability-custody-windows-ffi` package
   as the only package-scoped unsafe module. Its manifest must use package-local
   lint tables, not `[lints] workspace = true`, set `unsafe_code = "allow"` and
   `unsafe_op_in_unsafe_fn = "deny"`, and manually mirror every workspace
   Rust/Clippy deny except `unsafe_code`. Core (including the safe Windows
   adapter) and the broker continue inheriting `[lints] workspace = true`. The
   FFI package may contain only raw Win32, TBS, TPM2, and owned-handle wrappers;
   it must not own custody decisions, enrollment, persistence, or a caller
   interface.
2. Add the private `cfg(windows)` core adapter modules at
   `broker_admission/platform/windows.rs` and its `enrollment.rs`, `peer.rs`,
   `scm.rs`, and `monotonic.rs` children. They must retain/revalidate
   pipe/process/token handles, SID/integrity/session, image+SCM identity, exact
   registry owner/protected DACL/ACE/ancestor chain, nonce/expiry/replay, and
   TPM2 NV/TBS monotonic generation while constructing only the core's private
   opaque proofs. Do not widen visibility of the sealed core authority.
3. Wire those private modules through the existing core runtime methods at one
   dispatch/open-session seam. Parent Client Runtime Distribution WP12 owns the
   parent-side MSI/WiX/custom-action provisioner, build wiring, and
   upgrade/rollback/uninstall contract; Protected WP01 owns acceptance of the
   installer-provisioned record and the opaque proof. The broker retains the
   pipe stream/handle for the request; pipe IDs are re-queried immediately
   before transcript authorization. No second helper process/protocol,
   caller-supplied identity/attestation, raw `authValue`, or disk generation
   restore is allowed. The first real enrolled production caller is a later
   distinct source handoff after both owner boundaries exist.
4. Preserve fail-closed behavior before any storage, registry, journal,
   listener, bootstrap, or service-ready mutation for missing identity, wrong
   owner, revoked or stale generation, path escape, replay, restart ambiguity,
   unavailable broker, and unsupported platforms.
5. After all production source is complete, write the full expected test family
   listed in `TEST_PROOF_EXPECTATIONS.md`: retain the 11 core/protocol/broker/
   client roots, then add the absent core-private Windows adapter and TPM2
   NV/TBS monotonic-counter roots. Tests must exercise the real private module
   seams and must not bless a disconnected helper, fake authority, or caller
   assertion.
6. Run focused core/protocol/broker/client/FFI compilation and core Windows
   adapter architecture checks, source-shape/Enforcer checks, and the selected
   tests. Then
   update the checklist and retained proof; repo-wide Enforcer, pre-commit, one
   PR, long CI, and promotion remain final gates.

## Explicit no-go actions

- Do not implement an in-process broker or a same-process DPAPI/file-lock
  substitute.
- Do not accept caller-supplied attestation, key choice, capability, lease, or
  success flags.
- Do not mark the route READY/DONE because the integrated source compiles or the
  graph observes its planned roots.
