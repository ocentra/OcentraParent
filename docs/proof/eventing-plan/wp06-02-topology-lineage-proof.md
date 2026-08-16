# WP06 Topology And Lineage Proof

plan: eventing-plan
workpack: WP06 Journal Replay And Lineage
owner: ocentra-eventing
event_namespace: eventing.test
event_type: eventing.test.observed | eventing.test.other
schema_version: 1
delivery_route_state: local-only
consumer_handoff_state: manual-required
transport_boundary: not-applicable
redaction_state: redacted

## Topology manifest evidence

`cargo test -p ocentra-eventing --test contract topology_manifest` passed: 4
passed. It covers deterministic markdown, canonical entry keys, sorted
publisher/subscriber descriptors, family variants, and explicit covered,
no-subscriber, no-publisher, and accepted-one-sided statuses.

## Lineage and runtime ownership evidence

- `cargo test -p ocentra-eventing --test contract compatibility_matrix`: 3
  passed. It records 9 compatible entries, 3 intentional deviations, and 1
  manual-required entry from Ocentra Games lineage.
- `cargo test -p ocentra-eventing --test unit production_shutdown`: 5 passed.
  Covers drain/dead-letter, pending request cancellation, active-dispatch wait,
  and test-only clear boundaries.
- The focused no-global-bus `rg` search returned its expected zero-match status
  `1`; no hidden global EventBus pattern was found.

## No claim

The topology manifest is generic contract evidence. It does not prove a
product consumer is wired, a remote peer may publish locally, or an
enforcement chain has authority to run.
