# Protected Capability Custody Foundation Plan State

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan State
> Kind: current state and open gaps.
> Proves: only the current protected-custody route and its stated boundaries.
> Does not prove: implementation, tests, proof, runtime reachability, or release.

<!-- /agent-capsule -->

## Current status

## WP04 code-and-test-source checkpoint — 2026-08-28

Canonical `e0a410368` closes WP04's two internal currentness defects with fresh
fallible platform-session loading per broker hello and bounded listener-lifetime
fatal owner/currentness revalidation. Canonical `597098eea` adds all three
expected typed WP04 test roots. The tests have not been executed. External
enrollment/SCM ownership, the hardware monotonic provider, Parent WP12 package
lifecycle composition, a real owner-bound caller, proof, CI, READY, and DONE
remain blocked.

## Source-consolidation checkpoint — 2026-08-25

The independently reviewed Protected WP01 source packet and the subsequent
read-only provisioner preflight are integrated at canonical `a6d7d9adf`. This
includes the CNG/TPM mechanics, the private core adapter, and the active
BIN-only provisioner package (`Cargo.toml`, `src/main.rs`, and
`src/provisioning/`). The graph topology records 114 implementation files and
0 tests, with no Cargo workspace requirement gaps for the mapped packages.
The provisioner is a preflight only: it reads and revalidates enrolled state,
then always returns `ExternalProvisioningRequired`; it cannot create or publish
enrollment. Focused source/compile, architecture/source-shape, Enforcer, and
lane/hub guards passed for the source packets. This is source truth only: no
tests, proof, pre-commit, CI, PR, READY, or DONE claim is made here.

The plan remains an active neutral foundation route with one workpack in
`validation`. Protected custody is still not operational:

- startup remains `DeploymentRequired` before DB/state/listener mutation and
  there is no reachable success path;
- external OEM/firmware/MDM authority for `TPM_RH_PLATFORM` and NV
  define/undefine lifecycle, authenticated owner handoff, protected
  registry/SCM mutation, and enrolled counter generation remain unavailable;
- independent broker/client/token current observations and the core monotonic
  provider, plus the owner-bound production broker/client caller, remain open;
  the bounded WP04 process/token/service-SID observation adapters, fresh
  per-hello platform state, and listener-lifetime fatal-drift revalidation are
  source-present, while operational enrollment and hardware currentness remain
  unavailable.
- SQLite remains a checked replica, not protected authority;
- the three WP04 expected test roots are present but unexecuted; other expected
  plan test roots, retained proof, and release evidence remain open.

The graph records the route as `validation`. Accepted mechanics and a
read-only preflight do not derive READY or DONE while the external authority,
caller, test, proof, and release gates remain open.

## WP04 source route correction — 2026-08-26

Independent review at canonical `8f8cdf39e` confirmed that the retained Windows
FFI/process/token/service-SID observations and client anchor remain real, but
rejected the earlier conclusion that WP04 implementation was complete. The
reviewed ownership map now includes 23 production roots, adding the broker
custody facade and runtime that own platform-session state. Two internal P1s
remain: broker hellos reuse startup-cached `BrokerPlatformSessionState` after
custody currentness can advance, and the listener reports SCM `Running` after a
one-time check while swallowing later peer/currentness failures without a
service-lifetime revalidation path.

WP04 is therefore implementation-repair-authorized only to load fresh fallible
platform-session state before every broker hello and to make owner/currentness
drift service-fatal during listener lifetime, dropping the listener and
reporting `Stopped` nonzero. Malformed or unauthenticated peers remain
connection-local failures so the repair cannot become a remote stop primitive.
No handshake redesign or external-owner substitute is authorized.

The retained source has no caller-supplied identity/path, `sysinfo`
substitution, weak same-user/session trust, public adapter constructor, or
unsafe code outside the Windows FFI owner. This correction withdraws the prior
reviewed implementation evidence; it does not revoke the accepted observation
primitives. The owner-bound production caller, external enrollment/SCM
authority, hardware monotonic provider, three WP04 transport tests, retained
proof, operational readiness, READY, and DONE remain open.

## Installer-side ownership checkpoint — 2026-08-25

Parent Client Runtime Distribution WP12
(`12-protected-broker-provisioner-package`) is now the routed owner for the
parent-side Windows MSI/WiX package, elevated custom-action/provisioner
invocation, build/release wiring, and upgrade/rollback/uninstall contract. Its
expected package roots are `scripts/release/windows/parent-protected-custody/`,
`scripts/release/windows/parent-protected-custody.wxs`, and
`scripts/release/windows/build-parent-protected-custody-package.ps1`. WP12
  invokes and packages the fixed installer-owned provisioner binary but does not
  own its source. The BIN-only provisioner package, its Cargo manifest, `src/main.rs`,
  and private `src/provisioning/` implementation directory are now present as
  Protected WP01 source roots recorded in the graph. They implement only a
  read-only preflight and never establish enrollment. Generated
MSI/checksum/signing outputs belong under `target/release-packages/` and are not
source truth.

Protected WP01 remains the sole owner of the private core/FFI Windows adapter,
enrollment provenance, registry/SCM/peer authority, TPM policy and
non-exportable-handle validation, opaque admission/transcript proofs, and the
fixed BIN-only provisioner source boundary. The parent package may invoke an
approved elevated provisioner but may not expose
or accept a raw TPM `authValue`, TPM index/policy, SID, path, image identity,
generation, lease, capability, or success flag from MSI properties, command
line, setup, or a production caller. This is routing only: the package roots,
installer ceremony, real caller, expected tests, proof, CI, PR, READY, and DONE
remain absent/open, and the 24-plan program count is unchanged while this plan
gains one additional workpack.

## ADR-PCC-002 routing checkpoint — 2026-08-25

ADR-PCC-002 selects one Rust Windows front-door process: the existing protected
broker continues to depend on core and protocol, while core depends on one tiny
Windows FFI crate. The safe adapter is private `cfg(windows)` core modules;
there is no second helper process, helper protocol, or public adapter crate.
The FFI/private-core source roots below are integrated at `a6d7d9adf` and are
reviewed source truth, not merely absent planned roots:

```text
crates/ocentra-protected-capability-custody-windows-ffi/Cargo.toml
crates/ocentra-protected-capability-custody-windows-ffi/src/lib.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/enrollment.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/peer.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/scm.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/monotonic.rs
crates/ocentra-protected-capability-custody-provisioner/Cargo.toml
crates/ocentra-protected-capability-custody-provisioner/src/main.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/
```

The FFI package is limited to raw Win32/TBS/TPM calls and safe owned-handle RAII
wrappers. Its manifest must use package-local lint tables, not
`[lints] workspace = true`, set `unsafe_code = "allow"` and
`unsafe_op_in_unsafe_fn = "deny"`, and manually mirror every workspace
Rust/Clippy deny except `unsafe_code`. Core and broker continue inheriting
`[lints] workspace = true`. The private core modules preserve construction of
`BrokerPeerAdmissionObservation` and `BrokerAuthorizedClientTranscript`, the
`pub(crate)` sealed platform traits/guards, `BrokerPlatformOwner`, and the
existing broker-facing runtime methods. Installer-only immutable enrollment
must pin broker/client image+SCM identity, exact protected registry
owner/DACL/ACE/ancestor chain, and a TPM2 NV counter/index reached via TBS.
The fixed NV index/policy defines the expected object but is distinct from
`TPM_RH_PLATFORM` hierarchy authority. LocalSystem, elevation, TBS, or a PCP
signer does not grant platform hierarchy authorization; there is no empty-auth
or caller-supplied-auth fallback. The broker retains the pipe stream/handle for the request lifetime; core retains
process/token/image observations, and pipe process/session IDs are re-queried
immediately before transcript authorization. TPM reset, missing/deleted NV
index, TBS failure, or enrollment mismatch fails closed and requires re-pair;
disk state never restores the generation.

This is graph implementation-phase routing only. If the graph derives
implementation authorization, it authorizes only the remaining production
roots; the normal WP01 lifecycle remains `validation`, not READY or DONE. The
broker/client/FFI/private-core and read-only provisioner source is present,
  including the WP04 fixed-pipe/retained-observation primitives and bounded
  internal currentness repairs. Runtime still remains
blocked by external platform/installer authority, authenticated owner handoff,
operational current observations, monotonic provider, and real transport caller
boundaries. The three WP04 roots are present but unexecuted; other expected
roots remain absent, including these core-private adapter/TPM expectations:

```text
crates/protected-capability-custody-core/tests/unit/windows_adapter.rs
crates/protected-capability-custody-core/tests/security/tpm_nv_counter.rs
```

Account WP05A, Device Trust WP01/WP03, and Browser WP06 remain blocked
downstream consumers.

## Owning boundary

The neutral owner is the protected-custody plan and its future broker/client
surface. Account, Device Trust, Data Custody, Cloudflare, policy, and provider
owners may consume typed opaque results; none may mint authority, open the
broker, select a key, or treat a SQLite row as authoritative.

The existing production roots are mapped in `docs/engineering-graph/code-map.json`:

```text
crates/protected-capability-custody-core/Cargo.toml
crates/protected-capability-custody-core/src/lib.rs
crates/protected-capability-custody-core/src/authority.rs
crates/protected-capability-custody-core/src/binding.rs
crates/protected-capability-custody-core/src/binding/
crates/protected-capability-custody-core/src/custody.rs
crates/protected-capability-custody-core/src/custody/
crates/protected-capability-custody-core/src/path_security.rs
crates/protected-capability-custody-core/src/path_security/
crates/protected-capability-custody-core/src/platform.rs
crates/protected-capability-custody-core/src/platform/
crates/protected-capability-custody-core/src/storage.rs
crates/protected-capability-custody-core/src/storage/
```

The accepted source packet maps and activates this concrete package topology:

```text
crates/protected-capability-custody-core/src/broker_admission.rs
crates/protected-capability-custody-protocol/Cargo.toml
crates/protected-capability-custody-protocol/src/lib.rs
crates/protected-capability-custody-broker/Cargo.toml
crates/protected-capability-custody-broker/src/lib.rs
crates/protected-capability-custody-broker/src/main.rs
crates/protected-capability-custody-broker/src/windows_ipc.rs
crates/protected-capability-custody-broker/src/authority.rs
crates/protected-capability-custody-broker/src/custody.rs
crates/protected-capability-custody-client/Cargo.toml
crates/protected-capability-custody-client/src/lib.rs
crates/protected-capability-custody-client/src/windows_ipc.rs
crates/protected-capability-custody-client/src/admission.rs
```

The integrated implementation roots are one FFI manifest/lib, the private
`cfg(windows)` core module tree named in the ADR, and the expected WP01-owned
BIN-only provisioner manifest, `main` target, and private provisioning
directory. No second adapter manifest or public target is present. Parent
Runtime WP12 owns only the package invocation/lifecycle roots and consumes the
WP01-owned binary.

`broker_admission.rs` is a narrow core-owned facade seam: it may expose only
typed broker-entry/request/result operations after authenticated process/IPC
validation. Its private Windows modules construct the opaque peer observation
and transcript while retaining the `pub(crate)` sealed traits/guards and
`BrokerPlatformOwner`; they must not expose `CustodyAdmission`,
`CurrentBindingPort`, `PlatformCustodyOwner`, `PlatformDatabaseGuard`, or any
constructor that lets a caller implement or mint those sealed owner traits.
The neutral protocol crate owns the wire contract consumed by both broker and
client; the broker binary does not own a client-visible copy of that contract.

## Consumer routing

The reviewed graph records Account WP05A and Device Trust WP01/WP03 as
downstream consumers of this neutral boundary. These are source-order/unlock
relationships only. They do not transfer ownership, create a caller, or close
their existing authority and platform gaps. No edge points from this neutral
plan back to Account or Device Trust, so the route does not create a dependency
cycle. Browser WP06 is likewise downstream and remains blocked on this owner;
its persisted profile/path state cannot become protected authority.

## Exit conditions

The workpack can leave validation only after the integrated source is backed by
external OEM/firmware/MDM `TPM_RH_PLATFORM` authority and NV lifecycle,
authenticated owner handoff, protected registry/SCM provisioning, independent
broker/client/token observations, a core monotonic provider, and a production
transport caller; focused source and boundary validation are green; all 13
expected tests are written and executed;
the broker owns the protected operation; restart/recovery and concurrent
reservations are covered; and retained proof/checklist/merge evidence is
current. Until then, keep the state open and report the exact missing adapter
or caller.

## Routing split — 2026-08-25

WP01 remains the accepted neutral foundation. The graph now separates the
operational owner work that must follow it:

- **WP02 Windows Enrollment Owner Handoff** is blocked on an external
  OEM/firmware/MDM owner transaction. It owns the fixed protected
  Enrollment/SCM/TPM handoff and the provisioner-side package boundary. The
  current checkout has only fixed constants/read-only preflight, CNG
  existing-key open, NV public/read/increment for an already-defined object,
  and Registry/SCM read-only observation. TPM2 NV define/undefine codecs and
  allowlist, TPM_RH_PLATFORM owner ceremony, protected Registry write/ACL,
  fixed SCM create/config/security/delete, enrolled generation, and independent
  observations are absent. TBS/LocalSystem/elevation is not platform owner.
  WP02 is ACCEPT-for-source-design only; no implementation is authorized until
  the external owner is available, and no setup field, MSI property, caller
  identity, or synthetic enrollment result may substitute.
- **WP03 Monotonic Anti-Rollback Provider** owns the core Windows monotonic
  provider and platform anti-rollback boundary. WP01 and the WP02 owner
  transaction remain normal hard prerequisites; disk, SQLite, rollback, and
  caller counters are not authority.
- **WP04 Client Broker Anchor Transport** owns the client-side fixed-pipe
  admission and retained OS-derived broker anchor. Its 23-root ownership map
  retains the private Windows `client_anchor`, fixed-pipe transport/session,
  custody platform-session state, interprocess PID/session, and RAII
  process/token/image/SCM/enrollment observations, including TokenGroups and
  service-SID resolution. The reviewed WP01 plus implementation-independent
  WP02/WP03/Parent WP12 edges authorized only the bounded fresh per-hello state
  and listener-lifetime fatal-currentness repair, now source-integrated at
  `e0a410368`. Normal derived state remains
  blocked and normal completion depends on WP01, WP02, WP03, and Parent WP12.
  The route cannot use sysinfo or caller-supplied identity and does not redesign
  the broker/protocol handshake. The three expected test roots are checked in
  but unexecuted; the owner-bound caller, proof, operational enrollment, READY,
  and DONE remain open.
- **WP05 Account Issuer Key and Store Custody** has independently reviewed
  bounded production source integrated through canonical `f6d6dcf542ff` for
  Account WP09. WP01's `Seal`/`Rotate`/`Revoke`/`Recover` protocol
  actions and opaque prepared token are not signer/store authority. The new
  Account-owned `crates/account-issuer-owner` is statically linked into the
  existing broker and absorbs issuer/key-registry/outbox/delivery/startup,
  recovery, signing, and typed RPC mechanics. The broker mounts it for service
  lifetime and retains protected signer custody; family-core retains
  `VerifiedAccountIdentityAuthority`, the authority repository/source of truth,
  and one opaque `BEGIN IMMEDIATE` transaction/currentness host. The existing
  family-owned handoff contract remains a separate historical/input boundary
  and is never embedded, re-signed, or duplicated inside P-256 v2. The owner receives opaque Account-specific
  transaction and signer capabilities, never raw path, SQL, generic signer,
  second SQLite connection, or `custody.sqlite` access. WP05 uses the reviewed
  WP01 edge plus implementation-independent WP02/WP03/WP04 edges, so normal
  derived state remains blocked.

  The graph maps 140 production files across the route, with 128 exact files
  retained as reviewed implementation evidence; the remaining mapped paths are
  shared/historical integration roots and manifests. The reviewed files span
  the Account owner, family transaction/currentness
  host, protected core/protocol/client/broker, Windows FFI/provisioner, Parent
  Runtime facade, schema, and Cloudflare verifier/storage composition. The
  final repairs close durable reservation/idempotency, restart reconciliation,
  exact service/receipt/outbox lineage, owner admission, signer custody, and
  bounded v2 time arithmetic. This is implementation evidence only: all eight
  expected tests are absent, and no proof, pre-commit, CI, PR, READY, DONE, or
  deployed owner/provider/runtime claim is made.

  The selected issuer is a self-contained TPM-native ECDSA P-256 inner and
  outer v2: producer ocentra.account-authority-producer.v2, inner domain
  ocentra.account-authority-producer.signing.v2\\0, audience
  ocentra.account.authority.v2, algorithm ecdsa-p256-sha256-p1363, outer
  domain ocentra.account-issuer.transport.v2\\0, service
  ocentra.account-authority-producer.cloudflare.v2, and
  sha256:ecdsa-p256:<hex> key IDs derived with a v2 derivation domain. It
  freezes exact 65-byte SEC1, exact 64-byte low-S P1363, FFI low-S
  canonicalization, and Rust/Cloudflare high-S rejection. Protocol envelope
  message kinds 6 and 7 are AccountIssuerRequest and AccountIssuerResponse;
  they carry the operations IssueCurrentAuthority and AcknowledgeReceipt.
  Verify remains owner-local and is not a protected protocol message. Ed25519
  v1 is historical parse/verify only, not a v2 inner-wire wrapper, new signing
  path, fallback, or downgrade. Family-core retains the authority DTO,
  VerifiedAccountIdentityAuthority, authority repository/source of truth, the
  v1 historical verifier, and one opaque BEGIN IMMEDIATE transaction/currentness
  host. The existing family-owned handoff contract remains a separate
  historical/input boundary and is never embedded, re-signed, or duplicated
  inside P-256 v2. Account SQLite owns authoritative
  key/receipt/outbox state; D1 owns public verifier currentness/CAS and inbound
  idempotency receipt only.

  Service-specific key custody is REJECT/runtime-blocked: the existing key ACL
  is SYSTEM GenericAll, SCM exposes only SID type, and CNG security-descriptor
  revalidation is absent. The shared WP04 TokenGroups and LookupAccountNameW
  service-SID observations are source-present but do not provision or authorize
  the service-specific ACL.
  External provisioning must create/set the service-specific ACL; the broker
  only opens and revalidates the descriptor and token/service observations.
  Caller SDDL/SID and broad SYSTEM/BA authority are forbidden. WP04 owns the
  shared FFI/core service-SID and TokenGroups observation roots; WP05 owns CNG
  security-descriptor revalidation. Rust v2 verification uses locked ring
  0.17.14 ECDSA_P256_SHA256_FIXED over original bytes after explicit low-S
  precheck; sha2 is only for key-ID hashing, with no new p256/ecdsa dependency.
  Pinned windows-sys 0.61.2 API availability is implementation evidence only,
  not provider, ACL, provisioning, runtime, test, or proof evidence.
  Unsupported TPM/manual enrollment fails closed. The reviewed source now
  binds `service_binding_id` through registry, reservation, receipt, outbox,
  acknowledgement, and recovery queries. External attestation, rotation and
  recovery ceremonies, provider/ACL deployment, production composition, all
  eight tests, proof, and DONE remain open.

The graph records these as phase-specific implementation routing only. WP02,
WP03, WP04, WP05, and WP01 remain open for tests, runtime callers, proof, and
DONE; none of the new rows is READY or DONE. WP04 transport alone does not
unblock Account WP09.
