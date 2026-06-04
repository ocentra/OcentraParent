# Child Agent Local Service

## Parent Outcome

The child device has a real local agent that can capture evidence, expose
health, evaluate policy, run local AI when configured, enforce supported rules,
and report honest capability status.

## Ocentra Requirement

Ocentra is not a dashboard-only product. The Rust child-agent/service path is
the product authority for capture, local evidence, local AI safety, timers,
policy execution, enforcement, audit, and capability state.

## Roadmap And Expectations

- Roadmap: V0.1 through V1.0, then V6/V8 for platform and production hardening.
- Expectations: [platforms](../expectations/platforms.md),
  [real evidence proof](../expectations/real-evidence-proof.md),
  [capture](../expectations/capture.md),
  [enforcement](../expectations/enforcement.md).
- Modules: `crates/agent-service`, `crates/agent-core`,
  `crates/agent-protocol`, `packages/agent-protocol-domain`.
- Supporting docs: [reusable Rust eventing plan](../plans/eventing-plan/README.md).

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
app block/app limits, multi-device household, tamper/uninstall resistance, and
production distribution.

Competitors ship installed device agents or ecosystem-native controls. Ocentra
must be at least as real: installed service, health, update, local authority,
and clear degraded states.

## Current Ocentra State

- Local/LAN Rust service and WebSocket command paths exist.
- The local service reports hostname, IP/MAC/interface, CPU, memory, GPU, and
  NVIDIA `nvidia-smi` inventory for the connected child agent when the platform
  exposes those details.
- Many read-model and proof paths are service-backed.
- Parent desktop package proof now reports Rust-service health/readiness,
  backend kind, package service-manager launch ownership, fixed loopback port
  ownership, connect-or-degrade behavior, route/source/custody labels, and
  degraded state when the service socket is unavailable.
- Windows installer/updater scaffolding exists.
- Parent desktop release-support proof now records child desktop service state
  separately from parent shell state, and keeps child Android/iOS agent behavior
  manual-required until real device proof exists.
- `mobile-child-agent-capability-proof` adds a shared parent-domain read model,
  focused test, and proof harness that reconcile child Android/iOS capability
  states across existing Android package/service/storage/permission/privileged
  proof gates and iOS entitlement proof without claiming real mobile parity.
- Production service hardening and all adapter paths are not complete.
- A reusable Rust eventing plan now defines the shared event bus target for
  parent/controller and child-agent Rust runtimes. It does not upgrade runtime
  behavior until `crates/ocentra-eventing` and its tests exist.
- E-D added the first reusable Rust eventing crate, `crates/ocentra-eventing`,
  plus an agent-core network runtime chain proof that uses typed live
  envelopes and stored-envelope boundaries. This proves the first shared bus
  spine.
- E-D extended the network runtime's reusable eventing consumption with
  no-subscriber queue/drain proof and a typed local review request-response
  proof. This proves local in-process network runtime usage of the reusable
  crate, not broker/family-hub delivery or broad parent/child transport
  adoption.
- E-D extended `crates/ocentra-eventing` with production shutdown lifecycle
  proof: runtime-owned shutdown can drain queued work, dead-letter queued work,
  cancel pending local requests, clear subscriptions and aggregate gates, and
  reject later publish/subscribe calls.
- E-D added the eventing type-safety source gate proof: public eventing error
  identity fields use validated event/request/subscriber/idempotency newtypes,
  stored JSON is wrapped behind `StoredEventPayload`, and the raw public
  `&str` dead-letter event-type export was replaced by a typed constructor.
- E-D hardened weak network evidence command routing: manual-required or
  unavailable network observations now skip enforcement command/result publish
  phases while still preserving audit and read-model visibility.
- E-D added an eventing UI typed-intent boundary proof: the portal serializes
  outbound messages only as validated `AgentCommandEnvelope` values, parses
  service events as read models, keeps `AgentEvent` values as result metadata,
  and does not own event-bus publish/subscribe code.
- E-D added Rust protocol-facing network/AI/policy/enforcement/audit/portal
  event contracts in `crates/agent-protocol`, preserving exact chain refs and
  no exact URL/content/adapter-action claim boundaries. This proves protocol
  shape only; service delivery, parent/controller transport, and adapter
  execution remain separate work.
- E-D added Rust protocol-facing parent/controller and child-agent event
  contracts in `crates/agent-protocol`, with namespace constants, duplicate
  checks, serde tests, required-ref negative tests, and proof artifacts for
  rows 42-44 of the eventing plan. This proves protocol shape only; validated
  parent-intent publishing and parent/child transport remain separate runtime
  work.
- E-D added service-backed enforcement journal-before-action proof: the
  enforcement API now writes a pre-action audit activity row immediately after
  typed authorization and before adapter execution, then writes the final
  adapter-result audit row. This proves local service ordering and audit/store
  projection only; it does not add parent/child transport or new adapter
  capabilities.

## Current Gap

The service is real enough for local/LAN proof and local hardware visibility,
but not yet a fully hardened consumer child-agent across signed LAN
advertisement, capture, enforcement, notifications, updates, tamper/integrity,
and support diagnostics. The reusable Rust eventing crate now has proof for
queue/retry/TTL, request-response, durable journal/replay, panic isolation,
typed envelopes, production shutdown, and runtime-owned bus lifecycle. The
network runtime now consumes the reusable crate for typed publish,
no-subscriber queue/drain, local typed request-response, and Rust
protocol-facing network event contracts. The open eventing gap is
parent/child runtime publishing, transport handoff, broker-backed delivery,
and broad runtime adoption. The
initial UI typed-intent proof keeps the Vite/TypeScript portal as a view/input
surface while Rust remains the business event publisher. The initial AI and
portal direct-enforcement negative proof now rejects portal-side enforcement
action commands and verifies parent-assistant/AI command routing does not
target enforcement handlers; weak network evidence no longer publishes
enforcement command/result events; the eventing source gate now rejects public
raw JSON/string constants, `Uuid`, and raw domain identifier fields.

## Checklist

- [ ] Installed service health and restart behavior.
- [ ] Local/LAN command validation and origin checks.
- [ ] LAN child-agent identity advertisement, heartbeat, and pairing proof
      across a second physical child device.
- [ ] Evidence capture and journal writes.
- [ ] Reusable Rust eventing crate shared by parent/controller and child-agent
      runtimes, with UI/Vite kept view/input only. First E-D proof exists for
      `crates/ocentra-eventing` typed envelopes, queue/retry/request-response,
      journal/replay, production shutdown, UI typed-intent-only boundary,
      portal/AI direct-enforcement negative proof,
      weak-network-evidence command-routing guard, type-safety source gate, and
      the network runtime chain plus queue/drain, local request-response, and
      Rust protocol-facing network event contract proof; parent/controller and
      child-agent protocol event contract proof; the service enforcement API
      now proves journal-before-action plus final adapter-result audit/store
      projection. Parent/child runtime publishing, transport, and broad runtime
      adoption remain.
- [ ] Policy and AI read paths.
- [ ] Enforcement adapter dispatch with audit.
- [ ] Capability and degraded-state reporting. Current mobile capability proof
      covers `mobile-child-agent-capability-proof` scaffold/manual-required/
      not-implemented rows for Android foreground service, UsageStats,
      Accessibility, VPN/DNS, Device Owner, managed profile, Play signing, and
      iOS Family Controls, DeviceActivity, Network Extension, notifications,
      background execution, signing, TestFlight, and device proof.
- [ ] Updater status and rollback.
- [ ] Support diagnostics with redaction. Current release-support proof covers
      support-safe diagnostic fields for package/runtime handoff; production
      support workflow and incident process remain.

## Next AI Instructions

Do not add child-device authority to the portal. For any runtime capability,
add the TypeScript contract, Rust protocol parity, service/core behavior, real
tests, and portal read-state only after the service state exists. The Vite/TS
surface can send typed parent intents and render service-backed read models,
but business event chains belong in Rust.
