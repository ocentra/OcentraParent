# WP05 DRY Common-Core Candidate Map

## Objective

Identify repeated code patterns that should either become common-core helpers or be explicitly kept separate because product behavior differs.

## Scope

Start with known repeated patterns:

- child-domain observed/evidence/AI/policy event-chain assembly;
- runtime decision enum/action/handoff shape across app, app-game, browser, network, tracking, and screen;
- repeated proof-gate/manual-required state machines;
- repeated test fixture builders and assertion helpers.

## Required rule

Do not extract common code until existing behavior is protected by real tests. The first slice is inventory and proof requirements, not refactor.

## Output table

| Pattern | Current copies | Similarity | Difference that may block extraction | Preferred owner | Required pre-extraction tests | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| Child-domain observed/evidence/AI/policy event-chain assembly | `crates/app-core/src/lib.rs`<br>`crates/app-game-core/src/lib.rs`<br>`crates/browser-core/src/lib.rs`<br>`crates/network-core/src/network_runtime.rs`<br>`crates/screen-core/src/lib.rs`<br>`crates/child-runtime/src/child_domain_runtime_flow.rs` | Each domain maps a local intent into `ChildRuntimeDomain::<domain>.observed_profile(...)`, then walks the same helper ladder: `child_domain_observed_event(...)`, `child_domain_evidence_recorded_event(...)`, optional AI request, optional policy request. `child-runtime` then republishes the same chain across domains. | `network-core` already wraps the flow in `NetworkRuntimeEventChain`; `screen-core` converts capture scheduling/capability into runtime input first; `child-runtime` owns subscriber/publisher orchestration rather than just helper assembly. A shared helper must not hide domain-specific intent mapping or event-bus ownership. | Extract later into a small shared helper on the `child-runtime` / focused-eventing boundary, while keeping per-domain intent enums and input classification local. WP04 now treats broad runtime roots as frontage/composition only, and WP07 confirms the sampled runtime-shadow surfaces are already wired to `ocentra_eventing`; the only remaining provisional note is whether this helper should live directly under `crates/child-runtime/src/**` or in a narrower eventing-adjacent helper. | Existing protecting tests: `crates/app-core/tests/unit/runtime_decision.rs`, `crates/app-game-core/tests/unit/runtime_decision.rs`, `crates/browser-core/tests/unit/runtime_decision.rs`, `crates/network-core/tests/unit/runtime_flow.rs`, `crates/screen-core/tests/unit/runtime_decision.rs`, `crates/child-runtime/tests/integration/child_domain_event_flow.rs`, `crates/child-runtime/tests/integration/child_domain_runtime_flow_intent.rs` | `extract-later` |
| Runtime decision envelope shape | `crates/app-core/src/runtime_decision.rs`<br>`crates/app-game-core/src/runtime_decision.rs`<br>`crates/browser-core/src/runtime_decision.rs`<br>`crates/screen-core/src/runtime_decision.rs`<br>`crates/network-core/src/network_runtime.rs`<br>`crates/child-runtime/src/runtime_gate.rs` | All six surfaces expose typed input plus typed decision data, deterministic evaluation helpers, action/manual-or-policy state enums, and event/id scaffolding for runtime decisions or preflight gates. | The state axes are intentionally different: app/app-game/browser record different foreground semantics, screen models capture suppression/degraded capture, network omits a `DomainEvent` decision-record type and exposes an event-chain helper, and `child-runtime` composes four upstream decisions instead of one child-domain observation. | Keep behavior local. Only a tiny helper for naming, id/event-contract boilerplate, or comparison conventions is a realistic later candidate. | Existing protecting tests: `crates/app-core/tests/unit/runtime_decision.rs`, `crates/app-game-core/tests/unit/runtime_decision.rs`, `crates/browser-core/tests/unit/runtime_decision.rs`, `crates/screen-core/tests/unit/runtime_decision.rs`, `crates/network-core/tests/unit/runtime_flow.rs`, `crates/child-runtime/tests/unit/runtime_gate.rs` | `keep-separate for behavior; extract-later for naming only` |
| Tracking runtime observation/report compared with the child-domain flow | `crates/tracking-core/src/runtime_flow.rs` compared with `crates/child-runtime/src/child_domain_runtime_flow.rs` | Both surfaces build deterministic runtime reports from observed inputs, record evidence, and optionally request AI work. Both are attractive over-abstraction targets because they look like "observe -> evidence -> optional AI". | `tracking-core` then branches into geofence transition, expected-place evaluation, parent acknowledgement, child check-in, and portal-notification semantics without using the `ChildDomainObservedEvent` helper stack. Pulling it into the child-domain helper would erase tracking-specific contracts. | `crates/tracking-core/src/runtime_flow.rs` should stay the owner. | Existing protecting tests: `crates/tracking-core/tests/unit/read_model.rs`, `crates/tracking-core/tests/unit/geofence.rs`, `crates/tracking-core/tests/unit/expected_place.rs`, `crates/tracking-core/tests/contract/runtime_events.rs`, `crates/tracking-core/tests/ai_boundary.rs`, `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs` | `keep-separate` |
| Manual-proof / claim-boundary / no-claim protocol vocabulary | `crates/agent-protocol/src/app_game_adapter_dispatch_preflight.rs`<br>`crates/agent-protocol/src/enforcement_readiness.rs`<br>`crates/agent-protocol/src/notification_provider_status_boundary.rs`<br>`crates/agent-protocol/src/social_source_custody_mutation.rs`<br>`crates/agent-protocol/src/network_windows_firewall_lab_status.rs`<br>`crates/agent-protocol/src/lan_pairing_support.rs` | These read models all publish the same kind of truth surface: manual-required state, explicit claim boundary, required artifacts/proof gaps, and not-claimed execution boundaries. The repeated concept is real even when the field names differ. | The containers are materially different: some use free-text `claim_boundary`, some use typed enums, some use `manual_proof_requirements`, some use `manual_proof_gaps`, some use booleans or counts, and some carry no-claim labels instead. A shared struct would likely flatten domain meaning. | Extract later only for shared vocabulary/constants under `crates/agent-protocol/src/constants/**`; keep each read-model schema local. WP04 now treats the broad protocol roots as frontage, and WP03 fixes the migration rule: narrow module ownership only, with no new re-export debt. | Existing protecting tests: `crates/agent-protocol/src/enforcement_readiness_tests.rs`, `crates/agent-protocol/src/notification_provider_status_boundary_tests.rs`, `crates/agent-protocol/src/social_source_custody_mutation_tests.rs`, `crates/agent-protocol/src/network_windows_firewall_lab_status_tests.rs`, `crates/agent-protocol/src/lan_pairing_tests.rs`, `crates/agent-protocol/src/lan_pairing_provider_selection_tests.rs`, `packages/agent-protocol-domain/tests/unit/app-game-adapter-dispatch-preflight.test.ts`; add a Rust-side contract test for `crates/agent-protocol/src/app_game_adapter_dispatch_preflight.rs` before any helper extraction. | `extract-later for vocabulary only` |
| Agent-protocol fixture and test-support builders | `crates/agent-protocol/src/network_flow_event_fixtures.rs`<br>`crates/agent-protocol/src/child_agent_event_fixtures.rs`<br>`crates/agent-protocol/src/parent_controller_event_fixtures.rs`<br>`crates/agent-protocol/src/lan_pairing_browser_add_device_state_tests/source_matrix_test_support.rs`<br>`crates/agent-protocol/src/lan_pairing_browser_add_device_state_tests/production_household_proof_test_support.rs`<br>`crates/agent-protocol/src/lan_pairing_browser_add_device_state_tests/signed_discovery_relay_spine_test_support.rs` | All six files create deterministic fixtures and JSON assertion helpers for protocol/read-model serialization tests. They are the clearest repeated "test helper" pattern inside the allowed source budget. | The helpers are already heavily domain-coupled: each one bakes in event ordering, proof labels, or schema expectations that are meaningful only inside that suite. The LAN helpers already reuse one another; extracting them further would mostly move constants around. | Keep local to each suite. Only a tiny future testkit for generic timestamp/id seeds or JSON assertion wrappers would be worth sharing. | Existing protecting tests: `crates/agent-protocol/src/network_flow_tests.rs`, `crates/agent-protocol/src/child_agent_event_tests.rs`, `crates/agent-protocol/src/parent_controller_event_tests.rs`, `crates/agent-protocol/src/lan_pairing_browser_add_device_state_tests.rs`; any future shared helper needs its own cross-suite contract test. | `keep-separate` |

## Comparison rules used in this pass

- Treat a source family as a DRY candidate only when at least two copies expose the same public chain or decision shape, not just similar field names.
- Treat a row as `extract-later` only when the repeated part can move without erasing domain-specific safety semantics, event ownership, or proof boundaries.
- Treat a row as `keep-separate` when the repeated code is mostly schema-local wording, domain-specific branching, or suite-local fixtures whose meaning would be obscured by a generic helper.
- Require named protecting tests before any extraction recommendation. WP01 is now concrete evidence for existing crate-level tests; where a candidate lacks direct Rust-side coverage, this workpack names the missing test explicitly.

## Dependency notes

- WP01 is usable evidence input for real test locations and weak-surface warnings.
- WP04 is landed and now supplies concrete ownership/frontage evidence. Only the first row still carries a true owner-boundary note because this bounded slice does not fully decide `child-runtime` versus a narrower eventing-adjacent helper.
- WP03 is landed and now fixes the architecture stance for this workpack: no new re-exports, no repo-wide clean claim from this slice, and no broad protocol/root frontage should be treated as the owner.

## Command log

- `git branch --show-current`
- `git rev-parse HEAD`
- `npm run ledger:workers`
- `Get-Content -Raw '.ocentra-ai/rules/ocentra-parent-rules.mdc'`
- `Get-Content -Raw 'docs/agent/TASK_ROUTER.md'`
- `Get-Content -Raw 'docs/agent/WORKER_LANE_FLOW.md'`
- `Get-Content -Raw 'docs/repo-audits/AGENTS.md'`
- `Get-Content -Raw 'docs/repo-audits/INDEX.md'`
- `Get-Content -Raw 'docs/repo-audits/WORKPACK_INDEX.md'`
- `Get-Content -Raw 'docs/repo-audits/2026-06-17-structural-truth-audit.md'`
- `Get-Content -Raw 'docs/repo-audits/NEXT_ACTIONS.md'`
- `Get-Content -Raw 'docs/repo-audits/lane-manager-coordination/READ_SCOPE_BUDGET.md'`
- `Get-Content -Raw 'docs/repo-audits/lane-manager-coordination/VALIDATION_BUDGET_LADDER.md'`
- `Get-Content -Raw 'docs/repo-audits/workpacks/01-test-topology-inventory.md'`
- `Get-Content -Raw 'docs/repo-audits/workpacks/04-ownership-drift-map.md'`
- `Get-Content -Raw 'docs/repo-audits/workpacks/05-dry-common-core-candidate-map.md'`
- `Get-Content -Raw 'docs/repo-audits/workpacks/07-orphaned-legacy-surface-inventory.md'`
- `rg -n --glob 'crates/app-core/src/**' --glob 'crates/app-game-core/src/**' --glob 'crates/browser-core/src/**' --glob 'crates/network-core/src/**' --glob 'crates/tracking-core/src/**' --glob 'crates/screen-core/src/**' --glob 'crates/agent-protocol/src/**' --glob 'crates/child-runtime/src/**' "manual|required|no[_-]?claim|observed|evidence|policy|handoff|decision|runtime status|runtime_status" crates`
- `rg --files crates/app-core/src crates/app-game-core/src crates/browser-core/src crates/network-core/src crates/tracking-core/src crates/screen-core/src crates/agent-protocol/src crates/child-runtime/src`
- `rg -n --glob 'crates/agent-protocol/src/**' '\#\[serde\(rename = "manual-required"\)\]|manual_required|manual-proof|manual_proof|manual action|required_artifact|policy_engine_execution_claimed|no_claim|claim_boundary' crates/agent-protocol/src`
- `rg -n --glob 'crates/agent-protocol/src/**' 'event_fixtures|test_support|fixture' crates/agent-protocol/src`
- `rg --files crates/app-core/tests crates/app-game-core/tests crates/browser-core/tests crates/network-core/tests crates/screen-core/tests crates/child-runtime/tests crates/tracking-core/tests | rg 'runtime|decision|flow|gate|tracking'`
- `rg -n 'app-game-adapter-dispatch-preflight|AppGameAdapterDispatchPreflight' crates/agent-protocol/src packages/agent-protocol-domain/tests`
- Targeted file reads within the allowed crate/package budget for every file named in the rows above.

## Starting candidates

| Candidate | Current copies | Possible owner | Pre-extraction requirement |
| --- | --- | --- | --- |
| Child-domain event-chain assembly | `app-core`, `app-game-core`, `browser-core`, `network-core` | Rust protocol/runtime helper or focused child-domain runtime helper | Tests proving each domain's observed signal, AI handoff, policy handoff, and evidence-recorded behavior. |
| Runtime decision state naming | App/app-game/browser/network/tracking/screen runtime crates and modules | Naming convention or small shared primitives only | Confirm product-specific differences before abstraction. |
| Manual-required/no-claim proof states | Many proof helper modules | Shared proof-state vocabulary only if behavior is identical | Tests proving no accidental product-claim upgrade. |

## Acceptance

- Each candidate lists concrete files or crates.
- Each candidate states whether it is extract-now, extract-later, or keep-separate.
- No extraction happens before tests and ownership are clear.

## Failure conditions

- Creating generic abstractions that erase product-specific safety semantics.
- Extracting from broad frontage packages instead of true owner crates/packages.
- Duplicating a fifth copy while auditing the first four.
