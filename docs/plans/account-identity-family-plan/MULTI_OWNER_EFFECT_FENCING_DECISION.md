# AIF-009: Multi-owner Effect Fencing and Recovery

Status: routed architecture decision; no implementation, test, proof, or
runtime readiness claim.

## Decision

The runtime effect fence is a coordinator over owner-specific reservations. It
must not become an Account-local snapshot CAS that copies Device Trust,
parent-step-up, capability, or controller-lease truth.

Each owner keeps its own currentness and revocation source and exposes a small,
private owner interface for one exact effect target:

- Account owns household/member/role/session/target authority and its current
  generation.
- Device Trust owns trusted-device binding and signer/currentness generation.
- Parent Step-Up owns the action-bound, one-time challenge/receipt reservation
  and platform verification result.
- Account authorization owns capability and controller-lease reservations;
  those reservations remain action-, target-, generation-, and expiry-bound.
- The coordinator owns only the effect idempotency record, reservation
  ordering, commit/abort state, crash recovery, and exact outcome replay.

The coordinator may commit an effect only after every required owner has
prepared the same opaque target and generation set. It must abort or remain
manual-required when any owner is unavailable, mismatched, revoked, expired,
ambiguous, or uncertain after restart. A persisted snapshot is evidence for
recovery lookup, never authority to re-authorize an effect.

## Required owner protocol

The public surface is intentionally small and opaque: prepare, commit, abort,
and recover for one exact operation identity. Owner implementations must keep
their reservation handles private and non-serializable to callers. Recovery may
replay only an exact committed outcome; it may not mint a new receipt or infer
that an incomplete reservation succeeded.

The coordinator must not claim cross-database atomicity. If owners cannot
provide durable prepare/commit/abort/recover semantics, the operation remains
manual-required. No caller-selected authority scalar, DTO, proof record,
fixture, or in-process bus can satisfy the protocol.

## Ownership and routing

- Account WP02/WP08 remain the source of truth for sealed Account authority and
  actor-versus-target binding. WP05A owns the private Account participant
  adapter that consumes that authority; it does not create a duplicate
  repository.
- Account WP05 owns capability/lease authorization and consumes the new
  coordinator; it does not duplicate Device Trust or step-up state.
- Device Trust WP03 owns the parent-step-up target resolver, platform
  verification, nonce/sign-count custody, and its private coordinator
  participant.
- The new Account WP05A workpack owns the coordinator schema/recovery, the
  private Account participant adapter, and Account-side capability/lease
  reservation adapters. It is not a replacement for any owner repository.
- Policy WP01 consumes the resulting typed Account/Device Trust authority and
  remains the policy source owner; it does not store or fence account/device
  truth.
- Data WP08/WP09/WP10/WP11 remain blocked until the owner participants and
  coordinator recovery source are reviewed; they consume opaque outcomes only.

## Failure and test obligations

The later test wave must cover owner-unavailable, stale generation, revocation,
expiry, target substitution, prepare failure, commit failure, abort retry,
restart ambiguity, exact committed replay, duplicate operation identity, and
no-resurrection/no-new-receipt recovery. These are routing obligations, not
evidence that tests exist or passed.
