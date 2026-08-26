# Workpack 04 - Client Broker Anchor Transport

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Workpack: `04-client-broker-anchor-transport`
> Kind: protected client/IPC source-authorable route.
> Proves: ownership and expected source/test boundaries only.
> Does not prove: owner enrollment, operational transport, tests, proof, READY, or DONE.

<!-- /agent-capsule -->

## Purpose

Define the single fixed-pipe client admission boundary and the retained
OS-derived broker anchor. The base lifecycle is source-present for this bounded
fail-closed packet after the 2026-08-26 audit. The WP01 dependency remains
`reviewed-implementation`; WP02, WP03, and Parent WP12 are
`implementation-independent` for source phase only. Normal derived state and
completion remain blocked until those operational predecessors are DONE.

## Audited source truth — 2026-08-26

At source head `cbd974291`, all 21 expected production roots below are present,
registered by their Rust module hosts, and composed through the broker/client
transport. The bounded Windows FFI owns the unsafe boundary and now provides
retained process handles opened with limited-query plus synchronize access,
process creation/liveness/image observations, process and impersonated-thread
token handles, SID/integrity/session observations, bounded `TokenGroups`, and
service virtual-SID resolution from the retained SCM service name. The private
core anchor and peer admission retain and revalidate those observations; the
broker performs peer admission before it creates the fixed listener or reports
`Running`, and the client rechecks the kernel-reported server PID/session before
bootstrap and broker-hello acceptance.

This is reviewed production-source truth only. It does not establish external
enrollment, a protected monotonic provider, an owner-bound parent caller,
operational readiness, tests, proof, READY, or DONE. No bounded dependency-legal
WP04 source gap remains in this checkout.

## Expected production roots

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/client_anchor.rs
crates/protected-capability-custody-client/src/lib.rs
crates/protected-capability-custody-client/src/windows_ipc.rs
crates/protected-capability-custody-client/src/windows_ipc_connect.rs
crates/protected-capability-custody-client/src/windows_ipc_io.rs
crates/protected-capability-custody-client/src/windows_ipc_peer.rs
crates/protected-capability-custody-client/src/windows_ipc_session.rs
crates/protected-capability-custody-client/src/admission.rs
crates/protected-capability-custody-broker/src/authority.rs
crates/protected-capability-custody-broker/src/windows_ipc.rs
crates/protected-capability-custody-broker/src/windows_ipc/service.rs
crates/protected-capability-custody-broker/src/windows_ipc/peer.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/process.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/process_token.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/process_token_sid.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/process_token_groups.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/service_sid.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/token_groups.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/service_sid.rs
```

The existing registration hosts are part of this source boundary: the core
`platform/windows.rs` module must register `client_anchor`, `token_groups`,
and `service_sid`. In Windows FFI, `src/windows.rs` registers the `process`
path module, `src/windows/process.rs` registers the `process_token` path
module, and the existing `src/windows/process_token.rs` path module must
register `process_token_groups`; `service_sid` remains an explicit Windows
observation module. The client `src/lib.rs` and existing `windows_ipc.rs`
hosts must declare the fixed-pipe connect/io/peer/session sibling files with
path-module declarations. The broker `src/windows_ipc.rs` host registers the
actual startup/request hosts `service.rs` and `peer.rs`; those hosts own the
authenticated service lifetime and peer/request composition. These host edits
are required so the mapped leaves cannot be accepted as orphan files; they do
not create an owner, caller, or runtime authority.

## Expected test source

```text
crates/protected-capability-custody-broker/tests/windows_broker_custody.rs
crates/protected-capability-custody-client/tests/admission.rs
crates/protected-capability-custody-client/tests/windows_ipc_authentication.rs
```

## Anchor and fail-closed boundary

The transport retains and revalidates the OS-derived server process, token,
image, SCM, and protected enrollment anchor for the fixed pipe. `sysinfo`,
path-only observations, request fields, and caller identity are not authority.
Peer mismatch, stale enrollment, missing owner state, replay, or unavailable
transport fails closed; no client can mint an admission, key, lease, or
success result. The existing interprocess boundary supplies the OS server
PID/session, and existing RAII process/token/image/SCM/enrollment APIs remain
the owner-approved observation source. No broker/protocol handshake redesign,
spawned child, stdin bootstrap, nonce pipe, caller PID/SID/path, `sysinfo`, or
`dunce` fallback is part of this route.

The shared anchor source now includes bounded FFI/core observation for
`TokenGroups` and a service-SID resolver based on `LookupAccountNameW`. It
returns OS-derived observations only; it does not accept caller SDDL/SID or
replace service identity with broad SYSTEM/BA authority. The service SID is
derived from the retained fixed SCM service name, and the core compares the
resolved SID against the broker token's OS-reported group SIDs. This closes the
bounded source gap without making an operational authorization claim.

## Dependencies and state

Normal completion depends on WP01, WP02, WP03, and Parent WP12. WP04 remains
open for the owner-bound production caller, three expected tests, proof, and
DONE; source-phase authorization does not claim any of them. The current
source-only packet has no missing bounded adapter, but external enrollment,
monotonic currentness, package/lifecycle invocation, and operational anchor
state remain upstream owner work. Account issuer signing/store authority is
the distinct typed WP05 contract; lifecycle operation bytes and
`OpaquePreparedToken` are not a substitute.
