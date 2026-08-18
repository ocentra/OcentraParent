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
merged Windows-only source contains DPAPI-protected record/registry-epoch
custody code in `storage-custody-core`, the current-authority seam in
`family-identity-core`, and an opaque parent-runtime staged-handle facade.
`require_authenticated_parent_authority()` is permanently unavailable, so the
custody operations fail closed before record/epoch mutation. The parent desktop
now registers a native command that accepts only an opaque staged-ceremony
reference and generates trust material inside the facade. Its result is
custody-sealed-only and does not claim family lifecycle activation, but no
ceremony issuer or custody-to-lifecycle startup composition reaches that source;
no end-to-end Windows seal, activation, unseal, or revocation execution is
claimed. Existing lifecycle/custody tests are stale against this boundary and
are not current DPAPI proof.

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

## Production-code reachability audit (2026-08-16)

This source audit is refreshed against the canonical reconciled tree. The
two patch-unique commits on remote `codex/device-trust-wp01-source-wave` at
`914d06b6a` are superseded/rejected and are not integrated. It does not promote tests, proof artifacts, graph topology, or
typed DTOs into runtime authority. WP01's bounded repository hardening is
accepted source; no shipped caller yet owns the missing cryptographic/device
authority.

| Workpack | Reachable production code | Missing production authority / caller |
| --- | --- | --- |
| WP01 | `crates/family-identity-core` has the parent-presence store, lifecycle sidecar/transition journal, current device/signer binding, durable local event journal, and the integrated trusted-device/signer-key registration packet. Public household signer/verifier mint paths are removed; current authority is re-resolved from owner state. | Production custody still fails closed before path creation on unsupported/untrusted providers; no shipped platform/passkey ceremony issuer or complete trust-state composition owner. Independent source review accepted the bounded packet as implementation evidence only. Expected-test migration, functional validation, proof, production caller integration, and completion remain open. |
| WP02 | `crates/storage-custody-core` has Windows DPAPI/registry-epoch source, `crates/family-identity-core` has current-authority and lifecycle-activation seams, and `crates/parent-runtime-core` plus the parent desktop have an opaque staged-handle facade and mounted custody-sealing command. | `require_authenticated_parent_authority()` is permanently unavailable before custody mutation; no ceremony issuer or custody-to-lifecycle startup composition reaches the source. Windows custody-open platform failures are typed unavailable; unsupported non-Windows startup is typed manual-required; the later authenticated-parent gate remains manual-required. The command does not perform lifecycle activation. Non-Windows custody and end-to-end sealing remain unavailable/manual-required; stale lifecycle/custody tests are not proof. |
| WP03 | `crates/family-identity-core` and policy consumers have typed step-up receipt/proof boundaries, a five-minute lifetime gate, and bounded atomic challenge/intent plus receipt/credential custody. | The graph is blocked on Device Trust WP01, Account Identity WP08, and Cloudflare WP06; target-aware Account WP02 is transitive through WP06. Planned `parent_step_up_target_authority.rs` and `parent_step_up_runtime.rs` do not exist. Actor parent-controller and target child/profile/device are not authoritatively separated for `RegisterLanSignerAnchor`; the authoritative Account writer/provider caller, passkey/OS-native signature provider, durable sign counter, shipped parent runtime, expected tests, proof, LAN handoff, and DONE remain open. |
| WP04 | Typed QR challenge/response contracts and an unavailable-by-default verifier port exist. | No issuer, phone ceremony, nonce consumer, or transport runtime owner. |
| WP05 | The independently reviewed source-repair wave on `codex/device-trust-wp05-source-wave` adds `crates/entitlement-core` signed transport, bound authority generation/channel, strict weak-key-rejecting verifier, typed Grace rejection, and fail-closed read-only signed revocation custody. Authority verification is crate-private and the child-runtime entitlement module is not exported; there is no public capability selector, unlock, or final-consumption route. Active-window/session decisions remain behind a crate-private owner-controlled trusted-time/currentness boundary, and no snapshot or revocation mutation writer is present without a verified owner transition. | No concrete owner repository composition is present: real issuer/HSM or platform key provider, installed-package authority, billing/currentness owner, trusted-time/configured-grace policy, live Account/Device Trust re-resolution caller, handle-safe cache custody, signed revocation delivery caller, child entitlement action owner/startup mount, expected tests, runtime execution, proof, CI, and completion remain open; raw authority/issuer DI stays crate-private and no activation is claimed. |
| WP06 | `crates/storage-custody-core` provides restore preflight; the public raw installation-generation repair stub is removed, bundle construction is fail-closed without encryption custody, preflight requires an exact owner-supplied tombstone cursor, and the dead apply/authority seam is unconditionally blocked until an owner-bound cursor token exists. | No encrypted bundle/key-custody runtime, durable current tombstone owner, authorized re-pair producer, real executor, or shipped restore caller. The storage source overlaps the Data WP05 candidate and requires post-acceptance rebase/reconciliation. |
| WP07 | `crates/child-runtime` durably records tamper/removal evidence, binds readiness to current trust, and keeps ingress blocked across restart while unresolved; the Android bridge carries the fail-closed health state. | Package/device-owner removal, attestation, parent transport, and a real platform removal caller are absent; state remains manual-required. |
| WP08 | Research/dependency review only. | No runtime dependency adoption owner or trust-root caller. |
| WP09 | Route aggregation/documentation only. | No runtime trust behavior; completion remains downstream of WP01-WP08 evidence and authority. |

The smallest honest result is therefore a durable gap map, not a synthetic
issuer, test bridge, proof adapter, or dead DTO caller. WP01 owns the persistent
trusted-device/signer-key lifecycle source. Account Identity WP08 supplies the
canonical binding foundation; Account Identity WP02 must correct actor/target
action authority; Cloudflare WP06 must own authoritative writes/currentness and
resolve it from a shipped provider caller. WP03 retains bounded custody source,
but its missing target-authority and parent-runtime owners keep the graph
blocked until that chain and a real platform/passkey ceremony issuer provide
the one-time `RegisterLanSignerAnchor` authorization. Device Trust WP02 is
conditional only if the implementation demonstrates a private-key/install
custody need; it is not added as a blanket WP26 dependency. When selected, the
reviewed WP26 -> WP02 gate must be promoted and complete before the LAN/child
consumer route proceeds; when not selected, that edge remains absent. Subsequent
WP04-WP07 work remains dependent on its authority and platform owners.

The repository graph records the bounded WP01 packet as reviewed implementation
evidence only. Graph state remains non-authoritative for completion until its
source, tests, validation, caller integration, and proof contracts are actually
satisfied.

## Accepted source consolidation — 2026-08-17

The independently reviewed Device Trust continuation at source branch
`914d06b6a` is superseded/rejected and is not integrated. The canonical
reconciliation review
also reconciled the overlapping Payment entitlement boundary: it retained the
unsigned entitlement projection and fail-closed crate-owned context and removed
the incompatible public signed snapshot/verifier modules. This is accepted
production-source evidence only.

The current WP01 owner paths include:

- `crates/family-identity-core/src/device_trust_signer_registration.rs`
- `crates/family-identity-core/src/device_trust_signer_registration_schema.rs`
- `crates/family-identity-core/src/device_trust_signer_registration_current_authority.rs`
- `crates/family-identity-core/src/device_trust_current_binding.rs`
- `crates/family-identity-core/src/device_trust_lifecycle.rs`
- `crates/family-identity-core/src/device_trust_lifecycle_revocation.rs`
- `crates/family-identity-core/src/household_authority_proof.rs`
- `crates/family-identity-core/src/lib.rs`

Independent source review found no remaining internal P0/P1 in the accepted
continuation. Focused source formatting, architecture, Enforcer, and diff gates
passed after reconciliation. The WP05 source wave on the separate
`codex/device-trust-wp05-source-wave` branch adds the authority packet described
below; it is not part of the historical `68717b5b7` consolidation. WP06 has no
real restore executor/custody owner; and WP07 has no platform removal or
parent-transport caller.

## WP05 source-wave reachability — 2026-08-18 (independently reviewed source repairs; tests open)

The independently reviewed source-repair packet is present on
`codex/device-trust-wp05-source-wave`; expected tests, runtime execution,
proof, and completion remain open:

- `crates/entitlement-core/src/entitlement_snapshot_issuer.rs`: opaque owner
  issuance boundary and manual-required signing custody; the module and its
  raw owner-composition path remain crate-private until a concrete owner
  repository is mounted.
- `crates/entitlement-core/src/entitlement_snapshot.rs` plus
  `entitlement_snapshot_shape.rs`, `entitlement_snapshot_signing.rs`,
  `entitlement_snapshot_derivation.rs`,
  `entitlement_snapshot_wire_names.rs`, and
  `entitlement_snapshot_capability_wire_names.rs`: bounded signed envelope,
  authority-generation/channel binding, and owner-side projection derivation.
- `crates/entitlement-core/src/entitlement_snapshot_authority.rs` plus
  `entitlement_snapshot_authority_ports.rs`,
  `entitlement_snapshot_authority_currentness_ports.rs`,
  `entitlement_snapshot_authority_verifier.rs`,
  `entitlement_snapshot_authority_verifier_request.rs`,
  `entitlement_snapshot_authority_verifier_binding.rs`,
  `entitlement_snapshot_authority_verifier_signature.rs`,
  `entitlement_snapshot_authority_verifier_currentness.rs`,
  `entitlement_snapshot_authority_currentness.rs`, and
  `entitlement_snapshot_authority_revocation.rs`: Ed25519 verification,
  weak-key rejection, pinned key-id and authority-generation binding,
  installed-package/currentness seams, live Account/Device Trust re-resolution,
  current account/device binding, and a crate-private verifier result.
  Active-window and session expiry decisions belong to the crate-private
  owner-controlled trusted-time/currentness port; no caller-injected clock is
  exposed. The typed ports and raw dependency-injection constructor are
  crate-private. No concrete owner currently reaches the verifier, so no
  capability selector or positive unlock/consume operation is production
  reachable.
- `crates/entitlement-core/src/entitlement_snapshot_cache.rs` plus
  `entitlement_snapshot_cache_path.rs`,
  `entitlement_snapshot_cache_revocation.rs`: read-only signed revocation state
  custody with secure path checks. No snapshot or revocation mutation writer
  exists in this packet: the removed receipt and raw signed-update paths had
  no real owner transition caller. The current path-based implementation is
  not a platform handle-safe reparse defense and remains manual-required until
  a platform custody owner mounts that adapter.
- `crates/child-runtime/src/service.rs` and `service_recovery.rs`: the shipped
  child service owner does not construct an entitlement authority or own the
  Account/Billing/package/currentness composition. The prior
  `runtime_entitlement_license.rs` wrapper and its public authorize/consume
  APIs were removed, so no child entitlement action owner or startup caller
  reaches this source wave.

This does not promote the independently reviewed source repair to DONE or
runtime-accepted behavior.
External key custody/signing,
installed-package identity, billing/currentness/revocation owners and their
startup composition are not present in this branch. Expected WP05 tests and
proof roots remain missing/open, and no activation or broad capability
completion claim follows from the source reachability.

The full expected-test wave, functional validation, proof, production caller
integration, repo-wide Enforcer/architecture acceptance, platform custody,
broader lifecycle composition, and DONE state remain open. WP01 is a
foundation/source-only route, not a shipped authority or production-caller
route; its graph state must remain validation/open rather than READY for a
missing issuer. WP03 remains BLOCKED. Its bounded custody source is retained,
but its planned target-authority and parent-runtime roots are missing. Account
WP02's target-aware correction is consumed transitively through Cloudflare
WP06; no ceremony, provider, runtime, test, proof, or completion claim follows.
The Account WP08 implementation input and Cloudflare read adapter are reviewed,
while authoritative Cloudflare writes/provider composition remain open. Device
Trust WP02 is a conditional downstream sealing/composition route, and WP26 is
ordered after WP03 for current-binding/revocation consumption. The default graph
does not force WP02 into that route; when a platform sealing/lifecycle-revocation
path is selected, its reviewed conditional `WP26 -> WP02` gate must be promoted
before the consumer route can proceed.

## WP06 source-wave reachability — 2026-08-18 (source integrated; tests open)

The independently reviewed WP06 source packet is integrated on
`codex/eventing-wp09-production` through `f656a80a1` from final candidate
`1b3593319`. This is a source-only fail-closed repair; expected-test rewrites,
runtime execution, proof, and workpack acceptance remain open:

- `crates/family-identity-core/src/device_trust_lifecycle_revocation.rs` no
  longer exports `repair_with_new_installation`, which accepted raw identity
  strings and an installation generation. Durable revoke/reset remains inside
  the lifecycle owner; no authorized re-pair ceremony or producer exists.
- `crates/storage-custody-core/src/export_import_backup_recovery_build.rs`
  refuses to construct the encrypted wire bundle with
  `EncryptionCustodyUnavailable`. The prior caller-supplied encryption and
  support-decrypt flags cannot create metadata that claims ciphertext.
- The import preflight requires the bundle tombstone cursor to exactly equal
  the owner-supplied current cursor. Missing or mismatched currentness returns
  a `TombstoneConflict` with `tombstones_preserved: false`; apply is now
  unconditionally blocked because the caller-held context is not a durable
  currentness authority. The removed parent-authority/custom-executor path
  cannot invoke side effects until an owner-bound cursor token is reread and
  consumed at apply time.

No production caller constructs `ImportBundleContext` or
`CurrentVerifiedHouseholdAuthority`; no encrypted key-custody owner, durable
revocation/tombstone owner, authorized parent re-pair producer, real restore
executor, or parent/child startup route reaches this source. The five expected
WP06 test roots and the proof root remain missing/open. These storage files
overlap Data WP05 candidate `e91bb3de1`; that candidate must be rebased and
semantically reconciled on top of this integrated fail-closed boundary before
its own integration claim.

## Conditional WP02 sealing gate — 2026-08-17

The graph's reviewed dependency model is completion-gated by default, with
`implementationGate: "reviewed-implementation"` available for a separately
reviewed source phase. It has no always-on optional dependency switch. The
default Device Trust/LAN route therefore keeps WP02 out of WP26's hard
dependency list when no private-key/install custody path is selected.

If that platform path is selected, the owner must add the reviewed, acyclic
edge from LAN WP26 to Device Trust WP02, carry the matching WP26 dependency
review, and regenerate the graph before assigning the consumer:

```text
WP26 --depends_on (reviewed; implementationGate only for the reviewed source
                  phase)--> WP02 --depends_on (reviewed)--> WP01
```

The selected route is then blocked until WP02's sealing, lifecycle-generation,
and revocation handoff is complete; the edge never points back to WP03 and
cannot create a cycle. If the platform path is not selected, that conditional
edge remains absent, so the Account WP08 -> Account WP02 -> Cloudflare WP06 -> WP03 -> LAN/child
route is not forced through WP02. This is routing/authorization only; it does
not claim WP02 implementation, tests, proof, runtime reachability, or DONE.
The current WP02 source cannot satisfy the selected gate: its authenticated
parent-authority requirement is permanently unavailable, the desktop/native
mount and ceremony issuer are absent, and no custody-to-lifecycle startup
caller exists. A selected route must deliver those missing owners and fresh
runtime proof before the promoted edge can clear.

## WP03 target-authority owner correction — 2026-08-17

Live source inspection found a sealed Account binding and local SQLite
repository/CAS plus a Cloudflare D1 read adapter, but no complete live authority
chain. Account WP02 still hard-codes same-family authority, derives device scope
from the actor role, and accepts capability/lease facts from the caller instead
of resolving the target child/profile/device independently. Account WP08 owns
the canonical cross-boundary contract. Cloudflare WP06 is the designated
authoritative D1 writer/currentness/revocation/CAS and provider-caller owner,
but those source roots do not exist.

WP03 therefore has explicit hard dependencies on Device Trust WP01, Account
Identity WP08, and Cloudflare WP06; target-aware Account WP02 is intentionally
transitive through WP06 rather than a duplicate direct edge. The local LAN
registry cannot substitute for account/household authority. The current WP03
Rust draft may retain its atomic custody and recovery work, but
`RegisterLanSignerAnchor` must keep the actor parent-controller device distinct
from the target child/profile/device and consume both live Account and Device
Trust currentness. A native/passkey provider, parent-runtime composer, and
durable sign-counter owner also remain absent. The
implementation-only routing is authorization only, not a runtime, test, proof,
or completion claim.

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
- The Windows-only WP02 source has an opaque parent-runtime facade and a
  parent-desktop Tauri command mount. The startup state is typed manual-required
  only for unsupported non-Windows startup and typed unavailable for Windows
  platform errors, corruption, or other custody-open failures; neither state
  accepts sealing traffic without an available facade. The command reports
  custody sealing only and does not activate the family lifecycle. Android, Linux,
  iOS, and macOS custody implementations and their proof remain absent.
- `require_authenticated_parent_authority()` is permanently unavailable, so
  Windows custody seal/unseal/revoke/reset fails closed before record or epoch
  mutation. No platform ceremony issuer exists in this lane, so local
  revocation remains manual-required; existing lifecycle/custody tests are
  stale and not current DPAPI proof.
- WP03 now rejects parent step-up receipts with a lifetime over five minutes
  before any external verifier call; its independently static-reviewed bounded
  source adds atomic ceremony custody, recovery, and linked-challenge lifecycle
  validation, but independent target binding, real signature
  verification, durable sign-count ownership, one-time nonce consume, and
  OS/passkey ceremony composition remain manual-required.
- WP04 now has a typed QR challenge/response boundary with action, household,
  parent, approving-device, desktop, target, nonce, audit, expiry, and replay
  bindings. Its authority verifier is unavailable by default until a real
  issuer, phone ceremony, nonce consumer, and transport owner exist.
- WP04 binds the response to a trusted expected approving-device identity and
  requires response timestamps to remain inside the issued challenge interval;
  a response `Fresh` field remains an untrusted claim until nonce consumption.
- WP05 now contains crate-private entitlement account, household,
  trusted-device, package, signature, generation, and grace-shape verifier
  boundaries. The real key/revocation/currentness owners and child-runtime
  action consumer are absent, so no positive entitlement path is exported or
  reachable and no capability unlock is claimed.
- WP06 now blocks confirmation-only restore and the dead apply/authority seam
  unconditionally. A caller-held preflight/context cannot provide durable
  currentness or project applied/partial state; a future owner-bound cursor
  token must be reread and consumed at apply time. Encryption/key custody,
  revocation preservation, authorized re-pair, a real executor, and runtime
  proof remain manual-required.
- Recovery/reset/re-pair remains unproven without encrypted bundle handling and wrong-household/device/key negatives.
- Child tamper/uninstall now has a code-drafted child-runtime boundary: durable
  tamper evidence forces a separate manual-required readiness state, while durable revocation
  requires a verified, identity-bound parent authority. Platform package or
  device-owner removal, attestation, transport, and proof remain open.
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
- Windows custody source includes DPAPI/registry-epoch persistence and a
  current-authority seam. The authenticated-parent gate is permanently
  unavailable, while the parent-desktop command mount is now reachable and
  reports custody-sealed-only success or rejected/manual-required state without
  an issuer. It does not perform family lifecycle activation and is not a
  complete trust lifecycle, activation, recovery, or cross-platform custody
  implementation.
- Parent-presence decisions are correlated and redacted, inserted transactionally into the canonical parent-presence SQLite outbox, and delivered fail-closed into an `ocentra-eventing` hash-chained NDJSON journal. Pending rows drain on restart, and stable event identities make recovery idempotent. This is durable local journal evidence only; it does not claim subscriber delivery, a broader event-bus runtime, or complete device-trust lifecycle integration.
- No complete device-trust state machine exists yet beyond that narrow parent-presence bootstrap boundary.
- No merged cross-platform local key sealing implementation exists; the merged
  DPAPI/registry-epoch vertical slice is Windows-only.
- A fail-closed record-backed parent-step-up authority and receipt contract are
  merged, but phone-QR approval, encrypted recovery bundles, entitlement
  binding, and platform child-uninstall handoff remain absent. The child
  runtime now owns only the verified revocation/evidence boundary; it does not
  claim platform removal or anti-tamper enforcement.
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
