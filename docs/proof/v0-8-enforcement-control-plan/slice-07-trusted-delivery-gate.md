# WP04 Trusted-Delivery Gate

## Scope

This proof records a fail-closed agent-service safety boundary for the WP04
owned-process path. A raw envelope without a persisted matching delivery record
is rejected before the enforcement journal and adapter. A consumed delivery
cannot be replayed; process and evidence mismatches reject without a journal
side effect. Complete payload parsing happens before receipt consumption, so a
malformed candidate cannot burn a persisted delivery; the matching valid
command can still consume it once.

## No-claim boundary

The local record is not produced by an authenticated parent-runtime issuer or
transport. Production therefore has no valid issuer for the record and remains
manual-required. This does not close WP04, prove parent-to-agent delivery,
prove a sequence-bound receipt, or claim broad app blocking, platform parity,
or rollback execution.

## Required follow-up

Wire an authenticated parent-runtime delivery producer that persists a
delivery identity and ordered sequence before agent-service receipt consumption,
then prove the real producer-to-agent path and rollback/manual-required states.
