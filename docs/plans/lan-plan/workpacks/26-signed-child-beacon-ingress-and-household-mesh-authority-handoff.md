# 26 Signed Child Beacon Ingress And Household Mesh Authority Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `26 Signed Child Beacon Ingress And Household Mesh Authority Handoff`
> Kind: assigned workpack; open production integration and proof packet.
> Read when: this exact workpack is selected from `WORKPACK_INDEX.md`.
> Stop rule: do not claim code, tests, proof, household readiness, or Eventing
> authorization until this workpack has its own implementation, organized tests,
> and current proof artifacts.
> Proves: only the real child/runtime ingress, LAN authority composition, durable
> custody, and private Eventing WP10 handoff named here.
> Does not prove: portal authority, portal-owned LAN truth, fake transport,
> physical two-device readiness, or Eventing plan completion.
> Proof rule: an absent artifact keeps the matching row open/manual-required.

<!-- /agent-capsule -->

## Scope

WP26 owns the missing real signed-child beacon ingress and household-mesh
authority handoff. The packet must receive a shipped child/runtime peer message
from the real local transport, validate it through the existing signed hello and
heartbeat boundary, compose household and route authority, and durably custody
accepted and rejected messages before any downstream authorization handoff.

The LAN-owned composition consumes:

- W15 household device persistence, restart continuity, stale/offline state, and
  canonical known-device custody;
- W18 signed hello/heartbeat verification, nonce/replay/expiry checks, family and
  device binding, and child/runtime transport authority;
- W19 selected-route, controller-lease, assignment, revocation, audit, and
  wrong-target authority.

The private downstream handoff is the typed LAN-to-Eventing WP10 authorization
boundary after LAN validation and authority succeed. Eventing remains the owner
of local bus semantics; it does not become the LAN transport or household
authority owner. Portal/UI remains projection-only and cannot authorize, accept,
or publish a child beacon or enforcement/business event on this path.

No fixture, mock, fake socket, synthetic receiver, controlled observation command,
or portal transport may stand in for the shipped child/runtime peer ingress.

## Current state

- Code: partial/code-drafted. The current branch contains a bounded custody,
  registry, and signed-transport draft, but no completion claim is made until
  the legal trust authority and shipped ingress route exist.
- Tests: open. No real ingress, restart, custody, authority, or handoff suite is
  claimed by this workpack.
- Proof: open. The canonical generated root is:
  `output/lan-plan-proof/26-signed-child-beacon-ingress-and-household-mesh-authority-handoff/`.
- Dependencies: W15, W18, and W19 must expose the durable household, signed
  child, and route/revocation composition required by this boundary. Device
  Trust WP01 must first provide the persistent trusted-device/signer-key
  registration/current-binding source, and Device Trust WP03 must provide the
  one-time parent `RegisterLanSignerAnchor` authorization before this consumer
  bridge runs. WP02 is conditional only if a demonstrated private-key/install
  custody need exists.
- Ordering: Account WP08 -> Cloudflare WP06 -> Device Trust WP03 -> this LAN /
  child current-binding consumer. WP26 consumes current binding and revocation
  state after the ceremony; it cannot register a signer, infer authority from
  pairing, or create a reverse WP03 dependency.
- Unlocks: private Eventing WP10 authorization handoff only after LAN validation,
  route authority, custody, and negative-path proof are complete.

## Required production behavior

- Accept a real signed child/runtime hello or heartbeat only after family,
  device, route, parent, nonce, timestamp, schema/message kind, and revocation
  checks succeed.
- Persist accepted and rejected ingress outcomes with atomic message identity,
  nonce/replay state, event/message reference, household/device reference, route
  reference, and idempotency custody before downstream authorization.
- Recover custody and replay/idempotency state across service restart without
  re-authorizing a duplicate or resurrecting stale, revoked, or offline authority.
- Compose W15 canonical household state, W18 signed-child trust, and W19 selected
  route/controller authority without allowing weak discovery evidence to promote
  a child or route.
- Emit only the private typed handoff required by Eventing WP10 after all LAN
  guards pass; rejected or manual-required states do not enter that handoff.
- Keep provider-policy or provider-route constraints fail-closed and separate
  from portal/UI presentation.

## Expected real tests

Tests must live in organized Rust crate test groups owned by the actual ingress,
protocol, service, and runtime surfaces. The expected packet includes real
transport/runtime callers and real persistence paths, not mocks or replacement
transports:

- signed child beacon/hello and heartbeat ingress acceptance from the shipped
  child/runtime peer;
- atomic custody and restart recovery for accepted, rejected, and in-flight
  messages;
- duplicate message, duplicate nonce, and duplicate idempotency rejection
  without a second authorization or downstream handoff;
- stale, expired, missed-heartbeat, and offline transitions without deleting
  canonical household records;
- revoked device, revoked route, expired controller lease, unpaired child, and
  wrong-household rejection;
- wrong family, wrong device, wrong parent, wrong route, and wrong target
  rejection;
- provider-policy/provider-route denial before any authorization handoff;
- accepted route and custody composition from W15 + W18 + W19 after restart;
- proof that portal/UI has no authority to accept the ingress or publish the
  downstream authorization/business event.

## Expected proof

The workpack proof root must retain a source snapshot, exact validation command
log, real ingress/custody negative-case output, restart/duplicate evidence, and
the private Eventing WP10 handoff reference. Expected generated test output is:

```text
output/lan-plan-proof/26-signed-child-beacon-ingress-and-household-mesh-authority-handoff/
test-results/26-signed-child-beacon-ingress-and-household-mesh-authority-handoff/proof.json
```

Until those artifacts are regenerated on the current branch, WP26 remains open
and no household mesh readiness or Eventing WP10 authorization claim is released.

## Failure conditions and boundaries

- A controlled WebSocket observation command is not a production peer ingress.
- A signed schema or unit fixture is not packet/runtime or two-device proof.
- Portal rendering or portal transport is not LAN authority.
- Eventing WP10 handoff is not valid until LAN trust, route, revocation, custody,
  idempotency, and provider-policy checks have passed.
- Weak discovery, MAC vendor, ICMP, stale restart state, or provider metadata
  cannot establish child authority.
- Missing child/runtime transport, missing W15/W18/W19 composition, absent real
  tests, or absent proof keeps the workpack open/manual-required. No shipped
  service route currently establishes signer registration, so WP26 remains
  blocked on Device Trust WP01 and WP03; a code draft, typed receipt, or local
  test cannot bypass that authority gate.
