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

The current production diff now carries the exact captured source observation
from ingestion into `NetworkRuntimeSpine`, derives deterministic phase IDs,
refs, and source correlation, and persists through a network-owned
`ProductionFileEventJournal`. Journal recovery is performed before listener
readiness; startup and recurring reconciliation retry retained ActivityStore
observations; and activity/read-model delivery and stream APIs consume the
durable projection without republishing. The production spine registers no
phase subscribers or handlers. It emits only facts this owner performs now:
`FlowObserved`, `DomainObserved` when a domain is present, and
`ActivityClassified`. It does not manufacture AI, policy, enforcement, audit,
or portal completion events.

This workpack is therefore the single legal READY code packet for the missing
production foundation, not a historical validation row. Its owning boundary is
`agent-core` plus `agent-service`, consuming the typed `agent-protocol`
contracts and reusable `ocentra-eventing` journal/bus semantics.

## Implemented production boundary (integration acceptance still open)

The production slice must establish, at the owning boundary:

- one ingestion-time publish for each captured observation;
- deterministic event identity/idempotency so retries cannot mint duplicate
  network observations;
- a network-owned durable journal with replay semantics;
- startup recovery before the service reports the network path ready; and
- no read-time republish or other side effect in activity/read-model APIs.

The current code slice implements the first shipped boundary: capture-ingestion
through deterministic durable journal/replay, startup/recurring reconciliation,
fail-closed persisted-row reconstruction, and projection-only reads. The portal
host bridge now rejects the four direct enforcement mutation commands before
either Tauri or dev-web serialization while still allowing enforcement
read-model commands. The parent-assistant service router is covered by a real
negative test that returns an assistant-answer event rather than an enforcement
event. Real expected tests are written and the focused local Eventing,
protocol, core, ActivityStore, service, parent-runtime, and portal families plus
changed-file architecture/Enforcer gates pass. The command-boundary follow-up
is included in pushed commit `4b7bf6e3f`, whose normal pre-commit passed. The
whole-plan integration acceptance, CI, review, and merge gates remain open. AI,
policy, enforcement, audit, and portal *consumer behavior*
remain downstream contracts; this workpack proves only their direct-command
authority boundary. The current nested `network_event_runtime` fixture/prove/
`TEST_*` files are review/test material, not shipped production behavior, and
are excluded from the code-map production topology.

The row 57-59 source audit also removed a false implementation target. The
production network consumer uses reusable Eventing for durable typed
observation publish/replay. Queue/drain and local request-response helpers have
no shipped network-consumer caller and are not wired merely to satisfy a proof
row. The production phase gate emits only `FlowObserved`, optional
`DomainObserved`, and `ActivityClassified`; it does not synthesize a full
AI-policy-enforcement chain. Weak evidence stays manual-review-required with
`AskParent`, emits no downstream authority phase, and leaves
`adapter_action_executed=false`.

The reviewed ownership map records the exact current implementation/test roots
for this boundary. Expected focused tests and current local gates are recorded
below; retained proof, normal pre-commit, CI, review, and merge remain later
gates. This route does not mark WP09 done or claim live capture/enforcement.

Current focused local evidence:

- Eventing journal replay: 41 passed.
- Protocol `network_runtime`: 10 passed; protocol `network_flow`: 17 passed.
- Agent-core network runtime: 43 passed; focused ActivityStore integrity: 6 passed.
- Agent-service network bridge/runtime: 39 passed, including real SQLite
  corruption through startup reconciliation with no journal mutation.
- Parent-runtime network-flow integration: 2 passed.
- Parent-assistant service-router target: 11 passed, including the AI
  no-enforcement-event negative.
- Portal: 38 files / 159 tests plus type-check, including parameterized
  pre-serialization rejection for all four enforcement mutations and an
  allowed enforcement read-model command.
- Changed-file architecture and routed Enforcer checks pass. Local ignored
  evidence is regenerated at
  `output/eventing-plan-proof/09-network-consumer-event-chain/proof-summary.json`.
  The final combined normal pre-commit, CI, review, and merge are not complete.

Expected tests/proof:

- `network_consumer_chain_contract_uses_durable_journal_and_exact_source_ids`
- `network_consumer_weak_evidence_requires_manual_review_without_enforcement`
- `parent_assistant_service_router_publishes_answer_event_not_enforcement_event`
- `apps/portal/tests/unit/portal-command-boundary.test.ts`
- [local proof summary](../../../../output/eventing-plan-proof/09-network-consumer-event-chain/proof-summary.json)
- Proof links the exact workpack, focused command artifacts, and denied-authority
  cases without claiming downstream consumer execution.

Failure conditions:

- Do not mark this complete from eventing-only tests.
- Do not claim live network capture, DNS/firewall enforcement, or child delivery from reusable eventing.
- Do not let AI or network observations become enforcement without policy/enforcement consumer proof.
- Do not treat no-op subscribers, `TEST_*` refs, or test-created paths as
  downstream authority or acceptance proof. The production spine is the
  recovered `ProductionFileEventJournal`; no in-memory fallback is used for
  production initialization, no production subscribers are registered, and
  read-time APIs remain projection-only.
- Do not mark this workpack DONE/PR_READY or use it to unlock Network WP04
  until retained proof, normal pre-commit, CI, review, and merge are complete.
