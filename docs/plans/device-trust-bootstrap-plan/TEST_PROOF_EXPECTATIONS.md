<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `Device Trust Bootstrap Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Test Proof Expectations

## Proof root

```text
output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/
```

This root is local generated evidence only. Do not commit files below it.

## Test layout

```text
tests/device-trust-bootstrap-plan/<major-category>/
```

Major categories:

- `unit`
- `contract`
- `integration`
- `e2e`
- `security`

Current device-trust coverage starts in:

- `tests/device-trust-bootstrap-plan/unit/local-key-sealing.test.mjs`
- `tests/device-trust-bootstrap-plan/contract/parent-step-up-auth.test.mjs`
- `tests/device-trust-bootstrap-plan/integration/recovery-re-pair-boundary.test.mjs`

These plan-local tests currently prove document and route alignment only. They do not prove runtime key sealing, passkey ceremony, QR approval, recovery bundle execution, or child uninstall execution by themselves.

WP03's future owner-fence participant must add and focus:

```text
crates/family-identity-core/tests/unit/parent_step_up_runtime_fence_participant.rs
```

That test must prove action/target-bound prepare, one-time commit or abort,
sign-count and expiry enforcement, replay/restart recovery, and fail-closed
uncertainty through a private participant seam. It must not export raw receipt
or nonce state to Account WP05A, and its presence does not authorize source,
proof, platform ceremony, runtime reachability, or DONE before the real
passkey/OS authority and WP01 currentness owner exist.

`crates/family-identity-core/tests/unit/trust_bootstrap_probes.rs` is a
synthetic parent-presence/authority-boundary probe. It is not a Windows DPAPI
proof and must not be cited for same-device unseal, registry-epoch persistence,
revocation, or lifecycle activation. The current
`require_authenticated_parent_authority()` boundary is permanently
unavailable, so no current test establishes a reachable DPAPI custody path.
It does not close WP02 or claim Android, Linux, macOS, iOS, recovery, or
complete trust lifecycle coverage.

Implementation-adjacent coverage currently lives in:

- `packages/family-domain/tests/unit/household-authority.test.ts`
- `packages/family-domain/tests/unit/setup-lifecycle.test.ts`
- `packages/family-domain/tests/unit/invite-recovery-lifecycle.test.ts`
- `packages/lan-domain/tests/unit/lan-pairing.test.ts`
- `packages/lan-domain/tests/unit/household-device-spine.test.ts`
- `packages/lan-domain/tests/unit/device-roles.test.ts`
- `crates/agent-protocol/src/lan_pairing_tests.rs`
- `crates/agent-service/src/lan_pairing_tests.rs`
- `crates/agent-service/src/lan_pairing_multidevice_tests.rs`
- `crates/family-identity-core/tests/unit/trust_bootstrap.rs`
- `crates/family-identity-core/tests/unit/trust_bootstrap_cross_process.rs`
- `crates/family-identity-core/tests/unit/trust_bootstrap_nonce_process.rs`
- `crates/family-identity-core/tests/unit/trust_bootstrap_store_security.rs`

WP01 parent-presence runtime custody:

```powershell
cargo test -p ocentra-family-identity-core --test unit trust_bootstrap
cargo clippy -p ocentra-family-identity-core --tests -- -D warnings
npm run lint:architecture -- --files crates/family-identity-core/src crates/family-identity-core/tests/unit
```

This focused proof covers explicit-path SQLite custody, durable challenge and nonce identity, opaque receipt generation/redaction, concurrent process consumption/issuance, restart replay rejection, exact integrity-critical SQLite object allowlisting before initialization, and fail-closed path cases. Malformed or executable extra objects must be rejected before initialization can repair them, and isolated trigger, view, virtual-table, and structural fixtures must prove the existing database bytes remain unchanged.

On Windows, production custody requires retained handles for the final database file and every ancestor, all opened without delete sharing. A runtime probe must demonstrate that the active filesystem denies rename while such a handle is held; otherwise opening returns unavailable. The focused security test must prove both final-file and ancestor rename denial while custody is live.

On Unix, production custody currently returns unavailable before creating or opening the database because this boundary cannot exclude same-user pathname substitution. The explicit debug-only custody seam may exercise owner-private `0600` creation, atomic first publication, restart, concurrency, and permissive-existing-file rejection, but those tests are not a production custody claim.

Trust sealing must remain manual-required until the authority contract exposes a specifically authorized high-risk sealing action. `device_trust_ref` values must come from a CSPRNG and remain opaque and input-independent. Parent-presence decisions must be correlated and redacted, committed to a canonical transactional outbox with custody state, and delivered fail-closed into the owned `ocentra-eventing` hash-chained journal. Focused proof must cover accepted and rejected decisions, delivery failure, restart recovery, replay, and idempotent re-delivery. The no-claim boundary remains subscriber delivery, a broader event-bus runtime, and complete device-trust integration.

Windows DPAPI adapter validation:

**No current DPAPI proof command is authorized.** The Windows source and native
desktop command mount are present, but the authenticated-parent requirement is
permanently unavailable before custody mutation and no record-backed ceremony
issuer or custody-to-lifecycle startup caller exists. The command accepts only
an opaque staged handle and reports custody-sealed-only success or a typed
rejection; it does not claim family lifecycle activation or prove a live seal in
this issuer-less lane. Windows custody-open platform failures are unavailable,
while unsupported non-Windows startup is manual-required. A future selected platform route
must add a real Windows caller and retain proof for the exact authority,
current binding, registry epoch, ciphertext, activation, unseal, revocation,
wrong-user, wrong-device, and restart states. On non-Windows hosts, record
`unsupported-platform`; a passing compile, skipped `#[cfg(windows)]` test, or
synthetic probe is not DPAPI proof.

```powershell
cargo check -p ocentra-storage-custody-core
cargo clippy -p ocentra-storage-custody-core --lib -- -D warnings
cargo clippy -p ocentra-family-identity-core --all-targets -- -D warnings
npm run lint:architecture -- --files crates/storage-custody-core/src/windows_dpapi_key_sealing.rs crates/storage-custody-core/src/windows_device_trust_custody.rs crates/storage-custody-core/src/windows_device_trust_custody_platform.rs crates/family-identity-core/src/trust_bootstrap.rs crates/family-identity-core/src/trust_bootstrap/current_authority.rs crates/family-identity-core/src/device_trust_lifecycle_activation.rs crates/parent-runtime-core/src/device_trust_bootstrap_runtime.rs
```

These commands are source/static prerequisites only until the missing caller
and authority owners exist. The Windows adapter must fail closed when the
local machine binding cannot be read; no roaming, plaintext, or portable-key
fallback is permitted. The current sealing source derives subject/device
binding from the ceremony seam, while unseal requires a current runtime-owned
lifecycle authority source rather than a caller-deserialized snapshot.

## Common commands

Docs-only truth sync:

```powershell
$tests = Get-ChildItem tests/device-trust-bootstrap-plan -Recurse -Filter *.test.mjs |
  Sort-Object FullName |
  Select-Object -ExpandProperty FullName
node --test $tests
```

Family authority and recovery:

```powershell
npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/setup-lifecycle.test.ts tests/unit/invite-recovery-lifecycle.test.ts
```

LAN domain trust-adjacent contracts:

```powershell
npm run test --workspace @ocentra-parent/lan-domain -- tests/unit/lan-pairing.test.ts tests/unit/household-device-spine.test.ts tests/unit/device-roles.test.ts
```

Rust protocol and service LAN pairing seams:

```powershell
cargo test -p ocentra-parent-agent-protocol lan_pairing
cargo test -p ocentra-parent-agent-service lan_pairing
```

Scoped architecture gates:

```powershell
npm run lint:architecture -- --files packages/family-domain/src packages/lan-domain/src tests/device-trust-bootstrap-plan docs/plans/device-trust-bootstrap-plan
cargo lint-architecture crates/agent-protocol/src/lan_pairing.rs crates/agent-service/src/lan_pairing.rs
```

If the touched slice includes `packages/parent-domain` frontage or `tamper-uninstall-artifact-status`, run focused architecture gates there too.

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- `schema-domain` owns canonical trust/device/step-up/recovery/entitlement/tamper handoff shapes when contracts cross package/crate/app/plan boundaries.
- `family-domain` proves household/role/action authorization helpers only. It is not a platform device-trust runtime.
- `lan-domain` proves LAN pairing/selected-device contracts only. It is not the trust root.
- `agent-protocol` and `agent-service` are protocol/service proof only when selected.
- Setup, account, data custody, payment, package distribution, remote access, policy, and portal scopes run only when the selected workpack explicitly touches their typed handoff.
- Plan-local tests prove route and document truth unless the selected workpack names real runtime behavior and proof artifacts.

## Device Trust E2E meaning

Do not use one proof family to claim the whole device-trust path. For this plan, E2E has separate meanings:

```text
trust source-of-truth E2E: actor/account/household/device registration -> trust state -> revocation/expiry/no-child-control boundaries.
local key sealing E2E: trust subject -> platform store/wrapper -> sealed key lifecycle -> wrong user/device/key negatives -> no universal key.
parent step-up E2E: live Account currentness + live Device Trust currentness + parent-controller actor + independently resolved target child/profile/device + action -> platform approval assertion -> nonce/sign-count/expiry/audit proof.
phone QR approval E2E: desktop challenge -> phone approval -> action/household/parent/device/target binding -> replay/expiry rejection.
entitlement-device binding E2E: signed entitlement snapshot -> trusted device binding -> expiry/revocation/replay checks -> no license-only unlock.
recovery reset/re-pair E2E: encrypted recovery bundle -> wrong household/device/key negatives -> revocation preserved -> re-pair state.
child tamper/uninstall E2E: parent-authorized request -> trust revocation -> package/runtime handoff -> residual/manual-required state.
dependency adoption E2E: dependency candidate -> license/security/maintenance/supply-chain review -> adoption or rejection proof.
route gate E2E: accepted proof roots + carried blockers -> adjacent handoffs -> route/index sync -> manual-required gap register.
```

WP03 expected-test source must reject actor-device/target-device conflation,
cross-child and cross-household substitution, stale or revoked Account/Device
Trust state, provider/account mismatch, caller-supplied capability/lease/step-
up facts, stale sign counters, replayed nonces, restart reuse, and a
`RegisterLanSignerAnchor` call that bypasses the shipped parent runtime. These
tests are written only after the target-authority and runtime source packet is
stable; historical receipt/shape tests do not satisfy them.

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every device-trust proof slice must preserve product-safe logging and local harness logging.

Product/runtime-safe logging and artifacts:

```text
redact protected auth material, sealed key bytes, recovery payloads, QR private values, entitlement signing material, private device identifiers beyond opaque refs, and support-private diagnostics unless explicitly selected for proof
log trust subject, device role, actor role, trust state, sealed-key state, platform store, step-up state, QR challenge state, entitlement binding state, recovery state, tamper/uninstall state, revocation state, replay state, platform note, proof ref, manual-required note, and no-claim boundary when safe
separate login/session, setup, LAN pairing, package install, license, trust, key sealing, step-up, QR approval, recovery, tamper/uninstall, and route-gate states
never treat document tests, route tests, login logs, LAN logs, package logs, or license logs as trust proof without selected runtime proof or exact blocker
do not claim an artifact was emitted, published, journaled, or logged when the domain only constructed and returned it
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, trust subject, device role, platform, exit code, result, artifact pointer, diagnostics summary, blocker class, manual-required note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Host and platform proof expectations

- Windows proof is expected where the touched runtime slice is Windows-relevant.
- Android proof is expected where the touched runtime slice is Android-relevant, including Android Studio/emulator and the already-synced Samsung device when needed.
- Linux proof is expected where the touched runtime slice is Linux-relevant, including WSL and Docker where appropriate.
- Real iOS and macOS proof is an external-platform constraint from this Windows host. Record it as such when relevant; do not treat it as a local blocker.

## Blocker classification

When recording blocked validation or missing proof, classify each item as one of:

- `real dependency blocker`
- `external platform constraint`
- `avoidable local execution gap`

## Required proof states

```text
trust source-of-truth
local key custody
parent approval step
phone approval bridge
entitlement snapshot
recovery/reset/re-pair
child-device removal/tamper state
dependency adoption review
route gate
```

## WP05 expected test and proof debt (source review complete; tests open)

The WP05 source wave has independent source review and the Grace/revocation
repairs in this packet; its expected test roots and retained proof remain
missing/open. The later test wave must exercise the
trusted issuer/provider boundary without caller-built projections, weak-key
and key-id rejection, signed authority-generation and release-channel
binding, typed Grace rejection for every non-Tracking capability and inactive
offline-grace state, absent/manual-required snapshot and revocation
replacement, the restart-safe owner generation fence, ancestor symlink and
storage-error fail-closed behavior, explicit signed Grace restrictions,
release-channel mismatch, semantic effective-seat-limit equality, and the
owner-controlled account/session and Device Trust re-resolution contract for
any future grant/consume operation. It must also prove that no public
capability selector or forgeable final-consumption API is reachable without a
concrete child-runtime action owner.
The trusted-time/currentness owner must prove restart-safe time rollback
fencing and an owner-configured maximum grace interval; no caller clock or
crate-wide magic grace limit is acceptable. Because no real snapshot ingestion
owner exists in this packet, the expected source/test debt must keep snapshot
cache mutation absent rather than treating a receipt as an ingestion path.
Any future mutation proof must show that unverified wire data cannot advance
durable state and that platform custody handles reparse/TOCTOU safely, or
remain manual-required. A future production startup or integration proof must
also show real issuer/platform key custody, installed-package identity,
billing and currentness owners, signed revocation delivery, a concrete
child-runtime action owner, and its service startup caller; the current source
has no child-runtime entitlement consumer. Expected test debt remains in
`crates/entitlement-core/tests/unit/capability_gate.rs`,
`crates/entitlement-core/tests/unit/capability_access.rs`,
`crates/entitlement-core/tests/contract/signed_snapshot_delivery.rs`, and
`crates/child-runtime/tests/unit/runtime_gate.rs`; these files are not edited
in the source wave.

## WP06 expected test and proof debt (source repair integrated; tests open)

The WP06 source candidate is static-reviewed only. Its expected test roots and
retained proof remain missing/open and must be updated in the later test wave:

- `crates/family-identity-core/tests/contract/device_trust_lifecycle.rs` still
  calls the removed raw `repair_with_new_installation` API and expects a
  generation-only repair outcome; it must instead cover the missing authorized
  parent re-pair owner once that owner exists.
- `crates/schema/tests/contract/export_import_backup_recovery.rs` remains the
  recovery contract root and must retain wrong-household, wrong-key, corrupted,
  tombstoned, and redacted-shape negatives.
- `crates/storage-custody-core/tests/unit/export_import_backup_recovery.rs`
  still expects caller-built encrypted bundles and context literals without a
  current tombstone cursor; it must cover `EncryptionCustodyUnavailable`,
  missing/mismatched cursor rejection, and the unconditionally blocked apply
  seam. A future owner-bound cursor token must be tested for reread/consume
  semantics before any positive apply test is legal.
- `tests/device-trust-bootstrap-plan/contract/recovery-reset-re-pair.test.mjs`
  and `tests/device-trust-bootstrap-plan/integration/recovery-re-pair-boundary.test.mjs`
  must remain open until a real encrypted key-custody owner, parent
  authorization, durable revocation owner, restore executor, startup caller,
  and an owner-bound apply-time cursor token are mounted.

No WP06 tests, build/check, proof, precommit, CI, or runtime execution were run
in this source wave. The proof root remains
`output/device-trust-bootstrap-plan-proof/06-recovery-reset-re-pair/` and is
absent.

## Required negative states

```text
login alone not trust proof
license alone not unlock proof
LAN pairing not trust root
package install/copy not trust proof
wrong household/device blocked
wrong key blocked
revoked/expired state visible
manual-required state visible
surrogate proof not product proof
```

## No surrogate-green rule

- Document assertions and route-alignment tests may prove plan honesty, but they do not close runtime workpacks.
- Use local surrogates only when justified by the workpack risk surface and call them out explicitly in proof notes.
- Prefer real contract, integration, and end-to-end behavior over surrogate-only coverage whenever the plan risk requires it.
