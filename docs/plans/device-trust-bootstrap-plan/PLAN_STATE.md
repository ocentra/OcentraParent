<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `PLAN_STATE.md`
> Kind: plan state and current gap summary.
> Read when: After this plan is selected and before opening workpacks.
> Stop rule: Use this to choose one next action; do not scan historical docs.
> Proves: current planning state only.
> Does not prove: product completion or implementation readiness.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan State

Status: blocked / audit-truth-synced / not complete.

## PR disposition

PR #605 merged to `main` as `43a473f17` after fresh 60-job CI. It is narrow
unissued-parent-challenge test evidence only: it does not close a device-trust
workpack or change this plan's partial/open state. Platform key sealing,
step-up, recovery, tamper, and trusted-device product-chain work remain open.

## WP02/WP03 merged-slice disposition — 2026-08-06

PR [#616](https://github.com/ocentra/OcentraParent/pull/616) remains rejected
branch evidence; its review findings must not be used as WP02 proof. Its
replacement, [#623](https://github.com/ocentra/OcentraParent/pull/623), merged
as `46bb53da4d0dfbdd8d1b40937abfd67262aac8c3` after required CI passed. The
merged Windows-only slice owns DPAPI-protected record/registry-epoch custody in
`storage-custody-core`, binds the accepted ceremony through
`family-identity-core`, and exposes the selected parent-runtime/desktop bridge
path with focused Windows and unsupported-platform tests. Record-before-epoch
activation and epoch-first revocation ensure a restored app-data record is not
accepted after revocation.

PR [#627](https://github.com/ocentra/OcentraParent/pull/627) also merged as
`1ce56056c8c233addafe89feec7008c2bdda7059`, adding the fail-closed
record-backed parent-step-up authority and canonical receipt contract. These
are merged WP02/WP03 slices, not device-trust-plan closure: cross-platform
custody, recovery/reset/re-pair, phone-QR approval, entitlement binding, and
child tamper/uninstall runtime proof remain open.

## Current Truth

This plan owns the one-time trust bootstrap layer for parent and child devices. The product model is still: pair once, trust once, seal locally, and keep that trust until a parent revokes, removes, or resets the device.

Current direction from research and the pasted plan set:

- WebAuthn and passkeys are the right parent-presence proof foundation, including QR and hybrid cross-device flows.
- Biometric verification stays inside the authenticator or OS prompt; the relying party does not get biometric data.
- Local trust material must be sealed with platform-backed stores, not custom app-managed plaintext keys.
- Platform-backed local sealing is documented in `LOCAL_KEY_SEALING_MODEL.md` and `PLATFORM_KEY_CUSTODY_MATRIX.md`.
- Plan-local test folders now live under `tests/device-trust-bootstrap-plan/<major-category>/`.
- Current top-level categories are `unit`, `contract`, `integration`, `e2e`, and `security`.
- Recovery must use an encrypted bundle or equivalent sealed backup artifact; account recovery is not the same thing as data or device recovery.
- Device trust is separate from account login, subscription entitlement, policy delivery, and remote-access grant state.
- RustDesk is useful as architecture reference material for remote-desktop patterns, but not as embedded trust-root product code by default.
- Android Play Integrity is a supporting signal only; it is not the trust root.
- A narrow Rust parent-presence custody slice is present in `crates/family-identity-core` and is exercised by visible crate tests. Generated command logs may be written below `output/device-trust-bootstrap-plan-proof/` for a local run, but no generated proof file is committed as product truth.
- The current plan-local tests are mostly doc-shape and route-alignment checks, not runtime trust-bootstrap proof.

## Current ownership interpretation

```text
crates/schema or the owning Rust crate:
  Canonical shared trust state, device registration, parent step-up assertion, QR approval, recovery, entitlement binding, tamper/uninstall, and route-handoff shapes when they cross package, crate, app, or plan boundaries.

schema-domain:
  Temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.

family-domain:
  Household/role/action authorization helper surface consuming Rust-owned/generated contracts. It is trust-adjacent, not full device-trust runtime.

lan-domain:
  LAN pairing and selected-device proof consumer. It is transport/pairing-adjacent, not the trust root.

agent-protocol and agent-service:
  Protocol/service proof only when the selected workpack names runtime or wire behavior.

setup-install-provisioning-plan:
  Install/setup journey owner and first-run handoff producer.

account-identity-family-plan:
  Account, household, role, session, invite, and membership authority owner.

data-custody-storage-plan:
  Encrypted storage, export/import, restore, and recovery artifact custody after device trust exists.

payment-subscription-plan:
  Subscription entitlement policy and billing state owner.

parent/child runtime distribution plans:
  Package build, signing, update, rollback, installer, and child package mechanics owners.

remote-access-plan and policy-control-plane-plan:
  Standing access grant and policy delivery consumers after device trust exists.
```

## Current coupling risks

```text
- A partial parent-presence custody repository now exists in `crates/family-identity-core`. Production custody deliberately returns unavailable on every platform until a trusted custody provider can exclude same-user challenge-store writers. Its lifecycle authority sidecar now uses a process lock, reload-before-update, and atomic synchronized persistence; the broader device-trust runtime state machine remains open.
- `family-domain` contains trust-adjacent authority helpers but not platform key sealing, QR approval runtime, recovery bundle runtime, or trust-root state machine.
- `lan-domain` and LAN Rust seams contain pairing/selected-device proof consumers, but LAN pairing is not trust root proof.
- Current plan-local tests prove document and route shape only, not runtime trust.
- Login/session proof, LAN pairing proof, package install proof, and license proof are all insufficient for device trust.
- The merged Windows-only WP02 local-custody slice has a visible parent-desktop
  command and parent-runtime source/test vertical path; Android, Linux, iOS,
  and macOS custody implementations and their proof remain absent.
- The Windows custody revoke/reset path now fails closed behind the same
  authenticated-parent authority gate as sealing; no platform ceremony issuer
  exists in this lane, so local revocation remains manual-required.
- WP03 now rejects parent step-up receipts with a lifetime over five minutes
  before any external verifier call; real signature verification, one-time
  nonce consume, and OS/passkey ceremony ownership remain manual-required.
- WP04 now has a typed QR challenge/response boundary with action, household,
  parent, approving-device, desktop, target, nonce, audit, expiry, and replay
  bindings. Its authority verifier is unavailable by default until a real
  issuer, phone ceremony, nonce consumer, and transport owner exist.
- Recovery/reset/re-pair remains unproven without encrypted bundle handling and wrong-household/device/key negatives.
- Child tamper/uninstall remains unproven without parent-authorized revocation and package/runtime handoff proof.
```

## Current proof interpretation

```text
Document assertions are not runtime trust proof.
Route-alignment tests are not runtime trust proof.
WebAuthn/passkey schema proof is not platform ceremony proof.
QR challenge shape is not phone approval bridge proof.
Key-custody model proof is not platform-backed sealing proof.
Entitlement snapshot proof is not product unlock proof.
Recovery docs are not recovery execution proof.
Child uninstall/tamper docs are not parent-authorized uninstall proof.
WP09 can aggregate only accepted proof roots plus exact carried blockers.
```

## Proof Coverage

- WP01 has visible Rust source and focused tests for its parent-presence slice, but no committed generated proof artifact and no full workpack closure. The remaining planned proof roots are absent.
- Device-trust tests now live under `tests/device-trust-bootstrap-plan/<major-category>/`.
- Current top-level categories are `unit`, `contract`, `integration`, `e2e`, and `security`.
- The legacy `docs/proof/device-trust-bootstrap-plan/*` path is also absent on disk.

## Verified implementation boundary

- `packages/family-domain` contains typed trust-adjacent authority and recovery contracts, including `DeviceTrustState`, privileged device actions, setup invite rules, and recovery authorization boundaries.
- `packages/lan-domain` plus the Rust LAN pairing runtime contain trusted-route and selected-device registry contracts, restart behavior, and explicit manual proof gaps for LAN pairing.
- `packages/parent-domain` is mostly frontage for this slice and currently fails the repo re-export architecture gate on the named LAN/tamper bridge files.
- `crates/family-identity-core` has durable explicit-path SQLite issuance/consumption for debug/test parent-presence challenges, exact pre-initialization allowlisting of integrity-critical schema objects, global nonce uniqueness, opaque OS-random receipt capabilities, atomic first publication, and concurrent process/restart replay proof. Windows file and ancestor custody checks remain exercised only through the explicit debug/test seam; they are not production custody proof.
- Production parent-presence custody is fail-closed before path creation on every platform. A debug-only test seam exercises owner-private creation, path checks, and permission rejection without making an operational production claim.
- The merged WP02 vertical slice introduces a specific `SealParentDeviceTrust`
  authority action. It permits only a fresh parent-controller ceremony in
  pending/reset state, rejects child-scoped and low-risk ceremonies, and does
  not make login or ordinary parent authority a sealing capability.
- Merged Windows custody coordinates sealed-key persistence with a
  current-authority source through the parent-runtime/desktop bridge. It is a
  bounded Windows vertical slice, not a complete trust lifecycle, recovery, or
  cross-platform custody implementation.
- Parent-presence decisions are correlated and redacted, inserted transactionally into the canonical parent-presence SQLite outbox, and delivered fail-closed into an `ocentra-eventing` hash-chained NDJSON journal. Pending rows drain on restart, and stable event identities make recovery idempotent. This is durable local journal evidence only; it does not claim subscriber delivery, a broader event-bus runtime, or complete device-trust lifecycle integration.
- No complete device-trust state machine exists yet beyond that narrow parent-presence bootstrap boundary.
- No merged cross-platform local key sealing implementation exists; the merged
  DPAPI/registry-epoch vertical slice is Windows-only.
- A fail-closed record-backed parent-step-up authority and receipt contract are
  merged, but phone-QR approval, encrypted recovery bundles, entitlement
  binding, and child-uninstall authorization runtime remain absent.
- Login alone does not create trust, child devices do not own the trust root, and revocation must win over stale state.

## Latest selected slice (2026-08-09)

WP08 dependency adoption review was replayed on the consolidated E: branch.
The contract test passed 1/1, the scoped architecture gate passed, and Enforcer
guard passed. The tracked manifest is
`docs/proof/device-trust-bootstrap-plan/slice-08-dependency-adoption.md`.

This is a validation slice only. It does not claim runtime dependency adoption,
platform ceremony, key sealing, recovery execution, device-trust closure, CI,
review, or main merge.

## Execution Gate

- Route and implementation continue from [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
- Update this plan only through the blueprint and the selected workpack.
- Do not mark this plan complete from checklist deltas alone.
- Use [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack owner/proof family is unclear.
- Generated command output may be collected in the designated local artifact path or a crate-local ignored proof folder, not inside this plan folder and not as a tracked repository file. Source, visible tests, and current CI or harness results remain the reviewable evidence.
- True completion remains blocked until the runtime ownership split is resolved across the actual source owners and real proof artifacts exist for the missing trust-bootstrap slices.
