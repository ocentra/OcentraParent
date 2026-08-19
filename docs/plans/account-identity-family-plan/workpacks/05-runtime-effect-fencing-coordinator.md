# 05A Runtime Effect Fencing Coordinator

<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Workpack: `05A Runtime Effect Fencing Coordinator`
> Kind: planned production-source route; owner-specific reservation/coordinator seam.
> Proves: only the coordinator and Account-side reservation interface stated here.
> Does not prove: Account authority, Device Trust, parent step-up, provider runtime,
> data custody execution, policy execution, tests, proof, PR readiness, or DONE.
> Stop rule: do not implement until every hard owner dependency has a reviewed
> participant contract; do not replace an unavailable owner with a snapshot, fake,
> fixture, DTO, or in-process bus.

<!-- /agent-capsule -->

## Intent

Provide the smallest honest coordinator for high-risk runtime effects that
require Account authority, Device Trust currentness, parent step-up, a scoped
capability, and a controller lease at the same time.

## Ownership

This workpack owns only:

- the opaque operation identity and durable coordinator ledger;
- owner reservation ordering and exact target/generation binding;
- prepare/commit/abort/recover coordination;
- crash/restart recovery and exact committed-outcome replay;
- A private Account participant adapter that consumes sealed WP02/WP08
  authority, plus Account-side capability and controller-lease reservation
  adapters.

It does not own or copy:

- Account household/member/session/device authority source truth (Account
  WP02/WP08); WP05A may consume it only through a private adapter and may not
  create a duplicate repository;
- Device Trust binding or signer currentness (Device Trust WP01);
- parent presence, passkey/OS verification, nonce, or sign-count custody
  (Device Trust WP03);
- policy source truth or policy delivery (Policy WP01 and its consumers);
- data custody/provider/executor behavior (Data Custody WP08-WP11).

## Planned source and test roots

The source packet is intentionally absent at this routing checkpoint. The
smallest planned owner modules are:

- `crates/family-identity-core/src/household_authority_runtime_fence_coordinator.rs`
- `crates/family-identity-core/src/household_authority_runtime_fence_schema.rs`
- `crates/family-identity-core/src/household_authority_runtime_fence_recovery.rs`
- `crates/family-identity-core/src/household_authority_runtime_fence_account.rs`
- `crates/family-identity-core/src/household_authority_runtime_fence_capability.rs`
- `crates/family-identity-core/src/household_authority_runtime_fence_lease.rs`

The expected focused test root is:

- `crates/family-identity-core/tests/unit/household_authority_runtime_fence.rs`

These paths are routing obligations, not evidence that source or tests exist.

## Owner protocol

Each required owner must privately reserve the same opaque effect target and
current-generation set. The coordinator may commit only when every required
reservation is prepared. It must call abort on a failed prepare/commit where
the owner supports it and must remain manual-required when any owner is
unavailable or recovery is uncertain.

Recovery may reopen only an exact committed outcome for the same operation
identity and binding. It must reject stale generation, target substitution,
revocation, expiry, duplicate identity with a different target, partial
commit, and ambiguous restart. It must never mint a replacement receipt from
a persisted snapshot.

The protocol does not claim distributed transaction atomicity. The interface
must make uncertainty explicit and fail closed rather than hiding a partial
owner commit behind an Account-local CAS row.

## Hard dependencies and handoffs

- Account WP02/WP08: sealed Account authority and actor-versus-target binding;
  WP05A owns only the private adapter over that source.
- Account WP03: current session/revocation and freshness participant.
- Device Trust WP01: trusted-device current binding participant.
- Device Trust WP03: parent-step-up reservation participant; it remains the
  ceremony owner and must not be moved into Account.
- Account WP05: capability/lease consumer and downstream authorization owner.
- Policy WP01: typed consumer only; no policy storage or authority duplication.

Data WP08/WP09/WP10/WP11 remain blocked until this coordinator and all required
owner participants have reviewed source. They may consume only an opaque,
owner-derived committed outcome.

## Acceptance and no-claim boundary

- [ ] Account, Device Trust, step-up, capability, and lease reservations bind
      one exact operation target and current-generation set.
- [ ] Prepare/commit/abort/recover are private owner seams with durable
      idempotency and no caller-minted authority.
- [ ] Restart ambiguity fails closed; exact committed replay is the only
      positive recovery path.
- [ ] Data and Policy handoffs consume opaque outcomes without re-owning truth.
- [ ] Expected tests, focused validation, proof, CI, PR, and DONE are still
      open until a real source packet lands.
