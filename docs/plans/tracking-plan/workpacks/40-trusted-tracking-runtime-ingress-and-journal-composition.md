# WP40 Trusted Tracking Runtime Ingress And Journal Composition

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP40 Trusted Tracking Runtime Ingress And Journal Composition`
> Kind: dependency route and production-composition workpack.
> Read when: WP37 is selected and the trusted runtime/journal owner is required.
> Stop rule: This packet routes the missing owner; it does not claim that the
> owner exists or that WP37 is complete.
> Proves: only the required ownership boundary and dependency contract below.
> Does not prove: implementation, tests, proof, platform delivery, product
> readiness, or PR readiness.

<!-- /agent-capsule -->

## Purpose

Provide the missing production owner between the typed WP35/WP36 tracking
cascades and WP37's durable journal/replay/read-model contract. The owner must
be a shipped runtime/service composition, not a helper, DTO, fixture, proof
script, or test-only adapter.

## Why this workpack exists

The current tracking flows create fresh in-memory `EventBus` instances. The
existing `ActivityStore::ingest_journal` path consumes a separate encrypted
`ActivityEvent` journal, and no shipped owner maps tracking events into that
journal, configures its durable key/path, replays it at startup, or projects it
idempotently into SQLite. WP08/WP09 provide no platform ingress in this
checkout, and Eventing WP06 provides generic journal/replay mechanics only.

## Dependencies

This route consumes the following existing boundaries:

- Eventing WP06 for generic durable journal, recovery, and projection-only
  replay mechanics.
- Tracking WP32 for the ActivityStore/SQLite projection boundary.
- Tracking WP34 for canonical tracking event contracts and identity.
- Tracking WP36 for the typed detection cascade that this owner must compose.

The graph review for this workpack remains `planned` and review-required. No
dependency is promoted to implementation or completion by this document.

## Required production outcome

The eventual implementation must provide, through the actual child/service
lifecycle:

- a trusted tracking ingress with real event identity, correlation, causation,
  authority, and retention metadata;
- canonical tracking-event to durable journal-envelope mapping;
- configured durable journal/key/path ownership and journal-before-dispatch
  semantics where required;
- startup recovery and projection-only replay with corrupt/missing-event
  degraded state;
- idempotent projection into the existing ActivityStore/SQLite read model; and
- a runtime composition that does not use `EventBus::new()` as the production
  tracking persistence path.

## Explicit non-goals

This packet does not authorize synthetic tracking events, fabricated identity or
authority, generic JSON as a substitute for typed contracts, test-only ingress,
mock providers, static status/read-model panels, platform claims, notification
delivery, escalation lifecycle, portal completion, proof generation, or DONE.

## Validation and proof later

After production code exists, the owning worker must add real ingress,
durability, restart, corruption, idempotency, and projection tests, then run
the focused Enforcer route and regenerate proof. Until then WP40 remains open
and WP37 remains blocked behind it.
