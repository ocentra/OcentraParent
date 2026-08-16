# WP09 Network Consumer Event Chain

Scope: align network consumer use of reusable eventing with AI, policy, enforcement, audit, and read-model authority boundaries.

Source rows: `05-implementation-workpacks.md` rows 57-62.

Read next:

- `../../network-plan/AGENTS.md` after identifying exact network workpack
- `../05-implementation-workpacks.md` rows 57-62 only
- `../TEST_PROOF_EXPECTATIONS.md`

Expected outcome:

- Network plan consumes the reusable eventing crate instead of a private bus when implementation reaches this boundary.
- Network event chain distinguishes observation, classification, policy preview, enforcement command, adapter result, audit, and read-model events.
- Weak network evidence and AI classification cannot publish enforcement commands.
- Proof artifacts link back to both eventing and network plan state.

## Current production audit (2026-08-16)

The network event contracts exist in `agent-protocol`, but the shipped capture
path does not call a durable consumer chain. The service read API republishes
stored rows through `NetworkRuntimeDelivery`'s `OnceCell` and
`NetworkRuntimeSpine`/`EventBus::new`; the phase subscribers are no-op routing
surfaces, and `refs.rs`/`TEST_*` values manufacture or validate phase refs
without a production journal. There is no network-owned durable journal or
startup replay/recovery before readiness.

This workpack is therefore the single legal READY code packet for the missing
production foundation, not a historical validation row. Its owning boundary is
`agent-core` plus `agent-service`, consuming the typed `agent-protocol`
contracts and reusable `ocentra-eventing` journal/bus semantics.

## Ready implementation boundary (no completion claim)

The production slice must establish, at the owning boundary:

- one ingestion-time publish for each captured observation;
- deterministic event identity/idempotency so retries cannot mint duplicate
  network observations;
- a network-owned durable journal with replay semantics;
- startup recovery before the service reports the network path ready; and
- no read-time republish or other side effect in activity/read-model APIs.

The first shipped slice is capture-ingestion through durable journal/replay.
AI, policy, enforcement, audit, and portal consumers are downstream contracts;
they remain blocked and fail-closed until their owning plans provide real
authority, consumer, and handoff implementations. The current nested
`network_event_runtime` fixture/prove/`TEST_*` files are review/test material,
not shipped production behavior, and are excluded from the code-map production
topology.

The reviewed ownership map records the exact current implementation/test roots
for this boundary. Tests, retained proof, CI, review, and merge remain later
gates; this route does not mark WP09 done or claim live capture/enforcement.

Expected tests/proof:

- `eventing.network-consumer.chain-contract`
- `eventing.network-consumer.weak-evidence-negative`
- `eventing.network-consumer.ai-cannot-enforce`
- `eventing.network-consumer.policy-authority-required`
- `eventing.network-consumer.proof-linkage`
- Proof includes network-plan workpack, event family manifest, and denied-authority cases.

Failure conditions:

- Do not mark this complete from eventing-only tests.
- Do not claim live network capture, DNS/firewall enforcement, or child delivery from reusable eventing.
- Do not let AI or network observations become enforcement without policy/enforcement consumer proof.
- Do not treat read-time republish, no-op subscribers, `TEST_*` refs, or an
  in-memory `OnceCell`/`EventBus::new` spine as durable production composition.
