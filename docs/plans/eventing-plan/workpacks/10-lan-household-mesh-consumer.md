# WP10 LAN Household Mesh Consumer

Scope: define eventing obligations for household mesh import/export without turning the local event bus into a remote shared bus.

Source rows: `05-implementation-workpacks.md` rows 79-87.

Read next:

- `../../lan-plan/AGENTS.md`
- `../../remote-access-plan/AGENTS.md` only if relay/remote access is the assignment
- `../05-implementation-workpacks.md` rows 79-87 only
- `../WORKPACK_FAMILIES.md` only if owner/proof family is unclear

## Ownership boundary

```text
eventing-plan owns selected event export/import validation and local republish semantics only.
lan-plan owns LAN mesh, pairing, transport, authority, and household route behavior.
remote-access-plan owns relay/remote access behavior when selected.
account/device-trust/data-custody plans own authority, trust, and custody facts consumed by the handoff.
policy/enforcement plans own decisions and actions; peer/provider devices must not publish those events directly.
```

Expected outcome:

- Household mesh bridge boundary is explicit.
- Selected event export/import uses typed LAN message envelopes and validates custody, source, family, idempotency, and authority.
- Incoming bridge messages republish locally only after validation.
- Provider advertisement, heartbeat, AI work claim/lease/result, and child-agent AI work ledger events have owner boundaries.
- Provider or peer devices cannot publish policy/enforcement events directly.
- The proof root records whether this is eventing-local validation, LAN consumer proof, or remote-access handoff proof.

## Current production-code pass

The production boundary is code-drafted but unvalidated in this pass. The
exact implementation roots are:

- `crates/agent-protocol/src/household_mesh.rs`
- `crates/agent-protocol/src/household_mesh/household_mesh_bridge_input.rs`
- `crates/agent-core/src/household_mesh_event_bridge.rs`
- `crates/agent-core/src/household_mesh_bridge_runtime_validation_import.rs`

The protocol side now produces only a structural validation brand after the
schema, exact local-event/LAN-message mapping, family/target, replay, stale,
payload, and custody checks. Agent-core owns the private peer-authorization
token and the only structural-to-republish conversion path. Its current
authority resolver is deliberately unavailable and returns a fail-closed
manual-required outcome until LAN pairing/transport and account/device trust
composition supplies a non-forgeable runtime token. The required dependency
  is LAN WP26; until that authority/transport composition is routed and
  proven, no caller-supplied
envelope field or fixture mints runtime authority.

Tests, validation, proof, checklist, and runtime authority composition remain
deferred; this note does not mark WP10 done.

## Required proof fields

The selected proof must name, at minimum:

```text
handoff_id
source_event_ref
source_event_type
export_scope
family_ref
publisher_device_ref
consumer_device_ref
transport_boundary
authority_state
custody_state
idempotency_state
replay_state
stale_message_state
republish_local_state
consumer_plan_ref
peer_policy_publish_state
manual_required_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

Expected tests/proof:

- `eventing.mesh.selected-event-export-import`
- `eventing.mesh.incoming-validation-negative`
- `eventing.mesh.no-remote-direct-publish`
- `eventing.mesh.cross-device-idempotency`
- `eventing.mesh.provider-cannot-policy-enforce`
- Proof includes LAN plan workpack reference, replay/duplicate case, stale message case, and rejected authority case.

Failure conditions:

- Do not claim remote relay, mobile parity, or cloud delivery here.
- Do not trust provider/peer claims without account/device authority proof.
- Do not bypass local validation before republishing imported events.
- Do not allow provider or peer devices to direct-publish policy/enforcement events.
- Do not claim LAN/remote readiness from eventing crate proof alone.
- Do not claim eventing PR_READY from WP12/WP13 proof while this workpack lacks accepted proof or exact blockers.

Expected proof artifacts:

- `output/eventing-plan-proof/10-lan-household-mesh-consumer/proof-summary.json`
- `output/eventing-plan-proof/10-lan-household-mesh-consumer/16-validation-commands.log`
- `output/eventing-plan-proof/10-lan-household-mesh-consumer/02-no-claim-boundary.md`

These paths are currently absent in this checkout. Keep WP10 blocked until LAN
WP26 supplies the authority/transport composition and this canonical root is
regenerated with the owning handoff reference.
