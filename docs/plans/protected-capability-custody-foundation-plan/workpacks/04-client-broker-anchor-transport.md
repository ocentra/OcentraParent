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
OS-derived broker anchor. The Windows observation primitives are source-present,
and canonical `e0a410368` closes the two bounded internal currentness gaps found
by the independent review at `8f8cdf39e`. Canonical `597098eea` adds the three
expected typed test roots; they are unexecuted. The WP01
dependency remains `reviewed-implementation`; WP02, WP03, and Parent WP12 are
`implementation-independent` for source phase only. Normal derived state and
completion remain blocked until those operational predecessors are DONE.

The transport does mint and revalidate `BrokerAuthorizedClientTranscript`, but
the current Account issuer peer path drops that opaque transcript before owner
dispatch. Preserving and consuming it as an Account-bound admission is WP05
composition work. WP04 must not reinterpret OS provenance as Account identity.

## Corrected source truth — 2026-08-26

The original 21 production roots remain present and registered, and this route
now also maps the broker custody facade and runtime that own the platform
session state consumed by the transport. The bounded Windows FFI owns the
unsafe boundary and provides
retained process handles opened with limited-query plus synchronize access,
process creation/liveness/image observations, process and impersonated-thread
token handles, SID/integrity/session observations, bounded `TokenGroups`, and
service virtual-SID resolution from the retained SCM service name. The private
core anchor and peer admission retain and revalidate those observations, and
the client rechecks the kernel-reported server PID/session before bootstrap and
broker-hello acceptance.

The 2026-08-26 independent review identified two internal P1 defects:

1. Broker custody caches `BrokerPlatformSessionState` at startup, and later
   broker hellos reuse that cached key/writer/watermark after protected custody
   transitions may have advanced currentness. Every broker hello must obtain a
   fresh, fallible platform-session state from `BrokerCustodyRuntime` before it
   signs or returns the hello; stale or unavailable currentness must reject the
   connection before request processing.
2. The service validates once before creating the listener and reporting SCM
   `Running`, then swallows peer failures indefinitely. The listener lifetime
   must revalidate broker readiness and fresh platform currentness on a bounded
   cadence around accepts. Enrollment, SCM, process, owner, or currentness drift
   is service-fatal: drop the listener and report `Stopped` with a nonzero exit.
   Ordinary malformed or unauthenticated peer traffic remains connection-local
   and must not become a remote service-stop primitive.

## Internal repair and test-source checkpoint — 2026-08-28

Canonical `e0a410368` now obtains fresh fallible platform-session state before
each broker hello and revalidates fatal owner/currentness drift during listener
lifetime while keeping malformed peers connection-local. Canonical
`597098eea` adds the three exact test modules for the fixed pipe, session
currentness/expiry, process/session drift, request lifetime, protocol codec,
and typed non-Windows unavailability. They have not been executed.

This is reviewed production-and-test-source truth only. It does not establish external
enrollment, a protected monotonic provider, an owner-bound parent caller,
operational readiness, test results, proof, READY, or DONE. No broader
handshake, identity, enrollment, or provider work is authorized by this route.

The existing transcript is nevertheless a required input to WP05. The broker
peer must pass the still-valid opaque value into that private composition; it
must not replace it with a boolean, request field, process ID, SID, or a newly
public constructor.

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
crates/protected-capability-custody-broker/src/custody.rs
crates/protected-capability-custody-broker/src/custody/runtime.rs
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
authenticated service lifetime and peer/request composition. The broker
`src/custody.rs` facade and `src/custody/runtime.rs` own the fresh platform
session-state read required by peer admission. These hosts are part of the
reviewed repair boundary so the mapped leaves cannot be accepted as orphan
files; they do not create an owner, caller, or runtime authority.

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
resolved SID against the broker token's OS-reported group SIDs. Those retained
observations remain required, but they do not close the cached-state or
listener-lifetime currentness gaps above.

## Dependencies and state

Normal completion depends on WP01, WP02, WP03, and Parent WP12. WP04 remains
open for the owner-bound production caller, execution of its three expected
tests, proof, and DONE; source-phase integration does not claim any of them.
The fresh per-hello platform-state read and service-lifetime
currentness/fatal-drift handling described above are source-integrated. External
enrollment, monotonic provider availability, package/lifecycle invocation, and
operational anchor state remain upstream owner work. Account issuer
signing/store authority is the distinct typed WP05 contract; lifecycle
operation bytes and `OpaquePreparedToken` are not a substitute. The current
peer path's discarded transcript is therefore an explicit WP05 blocker, not a
claim that WP04 owns or can mint Account authorization.
