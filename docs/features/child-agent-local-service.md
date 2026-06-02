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
- Windows installer/updater scaffolding exists.
- Production service hardening and all adapter paths are not complete.

## Current Gap

The service is real enough for local/LAN proof and local hardware visibility,
but not yet a fully hardened consumer child-agent across signed LAN
advertisement, capture, enforcement, notifications, updates, tamper/integrity,
and support diagnostics.

## Checklist

- [ ] Installed service health and restart behavior.
- [ ] Local/LAN command validation and origin checks.
- [ ] LAN child-agent identity advertisement, heartbeat, and pairing proof
      across a second physical child device.
- [ ] Evidence capture and journal writes.
- [ ] Policy and AI read paths.
- [ ] Enforcement adapter dispatch with audit.
- [ ] Capability and degraded-state reporting.
- [ ] Updater status and rollback.
- [ ] Support diagnostics with redaction.

## Next AI Instructions

Do not add child-device authority to the portal. For any runtime capability,
add the TypeScript contract, Rust protocol parity, service/core behavior, real
tests, and portal read-state only after the service state exists.
