# eventing-plan

## Normalized Header

- plan/thread name: `eventing-plan`
- source thread label: `eventing-plan`
- source thread id: `019ed328-d310-7b00-bcc2-d18bdff11ad6`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: `WP10 open; reusable eventing foundation real; household-mesh protocol/bridge slice partial; no DONE/PR_READY claim`
- claimed source files/crates/packages: `crates/agent-protocol/src/household_mesh.rs`, `crates/agent-protocol/src/constants/household_mesh.rs`, `crates/agent-protocol/src/lib.rs`, `crates/agent-core/src/household_mesh_event_bridge.rs`, `crates/agent-core/src/household_mesh_bridge_runtime.rs`, `crates/agent-core/src/household_mesh_bridge_runtime_refs.rs`, `crates/agent-core/src/household_mesh_bridge_runtime_state.rs`, `scripts/test/eventing-household-mesh-consumer-proof.mjs`, `docs/plans/eventing-plan/*`, authoritative `docs/plans/lan-plan/*` snapshot/index`
- claimed tests: `missing crate-level WP10 tests under crates/agent-protocol/tests/contract and crates/agent-core/tests/unit + tests/integration; existing inline src household-mesh tests are real but insufficient as closure evidence`
- claimed proof commands/artifacts: `required next: cargo test -p ocentra-parent-agent-protocol --test contract household_mesh; cargo test -p ocentra-parent-agent-core --test unit household_mesh_event_bridge; cargo test -p ocentra-parent-agent-core --test integration household_mesh_bridge_runtime; cargo clippy -p ocentra-parent-agent-protocol --all-targets -- -D warnings; cargo clippy -p ocentra-parent-agent-core --all-targets -- -D warnings; cargo lint-architecture on touched Rust files/tests; canonical WP10 artifacts still missing at output/eventing-plan-proof/12-household-mesh-consumer/proof-summary.json and test-results/eventing-household-mesh-consumer-proof/proof.json`
- claimed blockers: `no hard external blocker for WP10-A; local runtime wiring, crate-level tests, and stale proof harness are the real remaining gaps; lan-plan authoritative 01-20 contract surfaces matter for final proof wording only`
- claimed next actions: `finish WP10-A protocol/bridge runtime wiring; add crate-level contract/unit/integration tests; update household-mesh consumer proof script; generate canonical proof artifacts; reconcile plan docs`
- obvious missing evidence fields: `no fresh cargo/clippy/architecture results for the current partial slice; no generated WP10 proof root; no crate-level household-mesh test files landed yet`
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

Executive summary: `eventing-plan` is not closeable yet. The reusable eventing crate is already real, but WP10 is still open because the typed household-mesh contract is only partially landed: `crates/agent-protocol` now has a real Rust transport module and `crates/agent-core/src/household_mesh_event_bridge.rs` now has explicit replay/stale/family/device/authority rejection logic, but `crates/agent-core/src/household_mesh_bridge_runtime.rs` is still on the old untyped envelope shape, crate-level `tests/contract`, `tests/unit`, and `tests/integration` coverage for WP10 is still missing, and the canonical proof artifacts do not exist. I stopped widening implementation after a coherent checkpoint in `crates/agent-core/src/household_mesh_bridge_runtime_refs.rs` and `crates/agent-core/src/household_mesh_bridge_runtime_state.rs`.

Plan closure definition: this plan is only actually done when the Rust-first consumer bridge is fully wired through the runtime path, the first real crate-level tests live under the proper top-level `tests/` categories, the WP10 proof script is updated to prove the current bridge shape instead of stale inline-test optics, the canonical proof root exists at `output/eventing-plan-proof/12-household-mesh-consumer/`, and the final proof explicitly says physical LAN/provider execution remains manual and separately owned under `lan-plan`.

| Read surface | Exact files |
| --- | --- |
| Plan docs read | `docs/plans/eventing-plan/AGENTS.md`, `docs/plans/eventing-plan/WORKPACK_INDEX.md`, `docs/plans/eventing-plan/05-implementation-workpacks.md`, `docs/plans/eventing-plan/source-index.md`, `docs/plans/eventing-plan/current-eventing-snapshot.md`, `docs/plans/eventing-plan/TEST_PROOF_EXPECTATIONS.md` |
| LAN dependency docs read | `docs/plans/lan-plan/current-lan-snapshot.md`, `docs/plans/lan-plan/source-index.md` |
| Rule files read | `.ocentra-ai/rules/ocentra-parent-validation.mdc`, `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`, `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`, `.ocentra-ai/rules/ocentra-parent-rust-service.mdc`, `.ocentra-ai/rules/ocentra-parent-test-rules.mdc` |
| Source read | `crates/agent-protocol/src/lib.rs`, `crates/agent-protocol/src/constants/household_mesh.rs`, `crates/agent-protocol/src/household_mesh.rs`, `crates/agent-core/src/lib.rs`, `crates/agent-core/src/household_mesh_event_bridge.rs`, `crates/agent-core/src/household_mesh_bridge_runtime.rs`, `crates/agent-core/src/household_mesh_bridge_runtime_refs.rs`, `crates/agent-core/src/household_mesh_bridge_runtime_state.rs` |
| Test/source-test read | `crates/agent-protocol/tests/contract.rs`, `crates/agent-core/tests/unit.rs`, `crates/agent-core/tests/unit/tracking_read_model.rs`, `crates/agent-core/src/household_mesh_event_bridge_tests.rs`, `crates/agent-core/src/household_mesh_bridge_runtime_tests.rs` |
| Proof read | `scripts/test/eventing-household-mesh-consumer-proof.mjs`; existence checked for `output/eventing-plan-proof/12-household-mesh-consumer/proof-summary.json` and `test-results/eventing-household-mesh-consumer-proof/proof.json` and both are absent |

| Current truth | Surface | Exact files | Truth |
| --- | --- | --- | --- |
| done | Typed protocol contract exists | `crates/agent-protocol/src/household_mesh.rs`, `crates/agent-protocol/src/constants/household_mesh.rs`, `crates/agent-protocol/src/lib.rs` | Real Rust transport envelope, auth/policy enums, family/device/idempotency/timestamp fields, and new rejection constants are present. |
| partial | Bridge validator exists | `crates/agent-core/src/household_mesh_event_bridge.rs` | Export/import now use the typed envelope and explicit `ReplayedMessage`, `StaleMessage`, `FamilyMismatch`, `WrongTargetDevice`, and authority rejection logic. |
| partial | Runtime prep landed | `crates/agent-core/src/household_mesh_bridge_runtime_refs.rs`, `crates/agent-core/src/household_mesh_bridge_runtime_state.rs` | I landed LAN-message mapping and expanded rejection enums, but the runtime itself is still not wired to the typed envelope. |
| false-green | Runtime bridge still old | `crates/agent-core/src/household_mesh_bridge_runtime.rs` | Still validates a legacy untyped inbound envelope with `authenticated/authorized/direct_remote_publish_attempted/contains_raw_screenshot` booleans. |
| false-green | Inline tests still carry the household-mesh surface | `crates/agent-core/src/household_mesh_event_bridge_tests.rs`, `src/household_mesh_bridge_runtime_tests.rs` | These are real tests, but WP10 cannot honestly close with inline-only coverage. |
| false-green | Proof script is stale against the desired end state | `scripts/test/eventing-household-mesh-consumer-proof.mjs` | It asserts inline `src/*_tests.rs` names, checks `crates/ocentra-eventing` source shape instead of the WP10 bridge surface, and does not require crate-level `tests/contract`, `tests/unit`, or `tests/integration` files. |
| false-green | Workpack status text is stale | `docs/plans/eventing-plan/WORKPACK_INDEX.md` | The open condition still mentions `remote-access` handoff verification; coordinator already de-authorized that as a blocker for WP10 closure. |
| missing | Crate-level protocol contract test | `crates/agent-protocol/tests/contract/household_mesh.rs` | Missing. |
| missing | Crate-level core unit test | `crates/agent-core/tests/unit/household_mesh_event_bridge.rs` | Missing. |
| missing | Crate-level core integration harness | `crates/agent-core/tests/integration.rs`, `crates/agent-core/tests/integration/household_mesh_bridge_runtime.rs` | Missing. |
| missing | Canonical WP10 proof artifacts | `output/eventing-plan-proof/12-household-mesh-consumer/proof-summary.json`, `test-results/eventing-household-mesh-consumer-proof/proof.json` | Missing. |

| Code surface and ownership | Owner | Exact files | Role |
| --- | --- | --- | --- |
| Protocol wire contract | `crates/agent-protocol` | `src/household_mesh.rs`, `src/constants/household_mesh.rs`, `src/lib.rs` | Owns typed wire schema, constants, serde contract, and string/enum boundary. |
| Consumer bridge logic | `crates/agent-core` | `src/household_mesh_event_bridge.rs` | Owns selected local-event export and validated incoming-message local republish boundary. |
| Consumer runtime path | `crates/agent-core` | `src/household_mesh_bridge_runtime.rs`, `src/household_mesh_bridge_runtime_refs.rs`, `src/household_mesh_bridge_runtime_state.rs` | Owns event-chain/runtime integration and rejection mapping around the bridge. |
| Proof harness | `scripts/test` | `scripts/test/eventing-household-mesh-consumer-proof.mjs` | Must prove the current consumer-boundary implementation honestly. |
| LAN authority dependency | `lan-plan` docs plus LAN source owners | `docs/plans/lan-plan/current-lan-snapshot.md`, `docs/plans/lan-plan/source-index.md`, authoritative `01-20` set | Supplies the signed-peer / assignment / revocation / event authority model WP10 must cite, but does not block the current Rust-first slice. |

| Test surface inventory | Current state | Honest read | Required change |
| --- | --- | --- | --- |
| `contract` | `crates/agent-protocol/tests/contract.rs` exists; no `contract/household_mesh.rs` | Applicable and missing. | Add `crates/agent-protocol/tests/contract/household_mesh.rs` and register it in `contract.rs`. |
| `unit` | `crates/agent-core/tests/unit.rs` exists; only `unit/tracking_read_model.rs` is real | Applicable and missing for WP10. | Add `crates/agent-core/tests/unit/household_mesh_event_bridge.rs` and register it in `unit.rs`. |
| `integration` | `crates/agent-core/tests/integration/` is placeholder-only; `integration.rs` absent | Applicable and missing for WP10 runtime wiring. | Add `crates/agent-core/tests/integration.rs` and `tests/integration/household_mesh_bridge_runtime.rs`. |
| inline `src` tests | `crates/agent-core/src/household_mesh_event_bridge_tests.rs`, `src/household_mesh_bridge_runtime_tests.rs` | Real but wrong as sole closure evidence. | Keep as supplemental or trim, but do not count them as the only WP10 coverage. |
| `security` / `replay` | `crates/agent-core/tests/security/` and `tests/replay/` are placeholder folders | Applicable to final WP10 claims because replay/auth/authority abuse is first-class scope. | Either add real files there in a later slice or keep final claims scoped to unit/integration plus proof-script negatives only. |
| `e2e` / `property` / `load` | Placeholder-only | Not required for the current consumer-boundary closure because live transport and physical LAN execution are explicitly out of scope here. | Do not invent them for optics. |

| Proof inventory | Status | Truth |
| --- | --- | --- |
| `output/eventing-plan-proof/12-household-mesh-consumer/` | missing | This is still the canonical WP10 proof root. |
| `test-results/eventing-household-mesh-consumer-proof/proof.json` | missing | Required companion artifact is absent. |
| `scripts/test/eventing-household-mesh-consumer-proof.mjs` | present but stale | It proves old inline-test/source-shape assumptions and must be updated before final closure. |
| `docs/proof/eventing-plan/slice-01-envelope-version.md`, `slice-02-ordering-replay.md`, `slice-03-consumer-boundary.md` | visible in worktree as untracked docs | These are not canonical WP10 proof until reconciled with the real output/test-results artifacts. |

| Scoped validation inventory | State | Command / note |
| --- | --- | --- |
| already pass | none re-run in this checkpoint | I have not run any scoped validation after stopping implementation. |
| fail | none observed | No new validation commands were run, so there is no fresh fail output yet. |
| required next | unrun | `cargo test -p ocentra-parent-agent-protocol --test contract household_mesh` |
| required next | unrun | `cargo test -p ocentra-parent-agent-core --test unit household_mesh_event_bridge` |
| required next | unrun | `cargo test -p ocentra-parent-agent-core --test integration household_mesh_bridge_runtime` |
| required next | unrun | `cargo test -p ocentra-parent-agent-core household_mesh` |
| required next | unrun | `cargo clippy -p ocentra-parent-agent-protocol --all-targets -- -D warnings` |
| required next | unrun | `cargo clippy -p ocentra-parent-agent-core --all-targets -- -D warnings` |
| required next | unrun | `cargo lint-architecture` on the touched household-mesh files and new `tests/` paths; there is pre-existing risk because `crates/agent-protocol/src/lib.rs` still contains old `pub use` structure unrelated to this slice. |

| Dependencies | Bucket | Exact dependency | Effect on closure |
| --- | --- | --- | --- |
| local implementation | `local-now` | `crates/agent-core/src/household_mesh_bridge_runtime.rs` | Must be converted to the typed envelope before any honest WP10 validation or proof run. |
| local test debt | `local-now` | `crates/agent-protocol/tests/contract/household_mesh.rs`, `crates/agent-core/tests/unit/household_mesh_event_bridge.rs`, `crates/agent-core/tests/integration/household_mesh_bridge_runtime.rs` | Missing real crate-level coverage is the biggest false-green gap. |
| local proof debt | `local-now` | `scripts/test/eventing-household-mesh-consumer-proof.mjs` | Must be updated to require the new crate-level tests and current runtime surface. |
| sequencing risk | `needs-coordinator-sequencing` | very dirty shared worktree | Repo-wide validation is unsafe/noisy; keep this lane on focused file and crate scopes only. |
| sibling contract | `needs-sibling-plan-contract` | authoritative `lan-plan` `01-20` surfaces, especially the signed-peer / event / assignment-revocation class referenced by the current LAN snapshot | Needed for final WP10 proof wording and authority claims, but not for the current Rust-first runtime slice. |
| non-blocker | `needs-sibling-plan-contract` | `remote-access-plan` | Explicitly not required for WP10 closure. |
| host limit | `host-platform-limited` | Apple-host physical iOS/mac proof | Not required for this plan on this Windows host. |
| not a blocker here | `host-platform-limited` | physical two-device LAN/provider execution | Coordinator already scoped this to manual/separate `lan-plan` ownership; it must be documented, not waited on. |

| Platform feasibility | What is feasible now |
| --- | --- |
| Windows host | Full WP10 Rust implementation, cargo tests, clippy, proof-script execution, source-shape and diff checks. |
| Android Studio / emulator / synced Samsung device | Available if a later LAN/child-agent proof slice genuinely needs Android-backed evidence, but not required for this consumer-boundary closure. |
| Linux via WSL / Docker | Feasible for any later cross-platform Rust/service proof harness, but not required for the current WP10 base slice. |
| Apple-only | Real iOS/macOS execution proof only, which is outside the current WP10 done bar on this host. |

| Ordered slices | Files / domains to touch | Validation / proof | Exit criteria |
| --- | --- | --- | --- |
| 1. WP10-A protocol/bridge runtime wiring | `crates/agent-core/src/household_mesh_bridge_runtime.rs`, `src/household_mesh_bridge_runtime_refs.rs`, `src/household_mesh_bridge_runtime_state.rs`, `src/household_mesh_event_bridge_tests.rs`, `src/household_mesh_bridge_runtime_tests.rs`, `crates/agent-protocol/tests/contract.rs`, new `crates/agent-protocol/tests/contract/household_mesh.rs`, `crates/agent-core/tests/unit.rs`, new `crates/agent-core/tests/unit/household_mesh_event_bridge.rs`, new `crates/agent-core/tests/integration.rs`, new `crates/agent-core/tests/integration/household_mesh_bridge_runtime.rs` | focused cargo tests, clippy, Rust architecture gate | Runtime import/export path uses the typed envelope end-to-end; crate-level `contract`, `unit`, and `integration` tests exist and go green. |
| 2. WP10-B proof harness alignment | `scripts/test/eventing-household-mesh-consumer-proof.mjs`, any touched README/checklist references under `docs/plans/eventing-plan` | run the proof script only after slice 1 is green | Proof script checks the real crate-level tests and current household-mesh bridge/runtime files, not stale inline-only optics. |
| 3. WP10-C consumer proof generation | `output/eventing-plan-proof/12-household-mesh-consumer/`, `test-results/eventing-household-mesh-consumer-proof/`, optionally `docs/proof/eventing-plan/*` if used | `node scripts/test/eventing-household-mesh-consumer-proof.mjs` plus any required focused cargo reruns | Canonical proof artifacts exist and explicitly state that physical LAN/provider execution remains manual and separately owned under `lan-plan`. |
| 4. WP10-D plan/doc truth reconciliation | `docs/plans/eventing-plan/WORKPACK_INDEX.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `PLAN_HEALTH.md`, relevant WP10 workpack/proof-manifest docs | `git diff --check -- docs/plans/eventing-plan docs/proof/eventing-plan` | No stale `remote-access` blocker wording remains; WP10 state matches actual source/tests/proof. |
| 5. Optional final hardening if claims expand | real `tests/security/*` or `tests/replay/*` files if we want distinct category claims | focused cargo test targets only | Only needed if final WP10 proof wants separate replay/security category claims rather than unit/integration negative coverage. |

Strict done bar:
- `crates/agent-protocol/src/household_mesh.rs` is consumed by the `agent-core` runtime path, not just present as dead code.
- `crates/agent-core/src/household_mesh_bridge_runtime.rs` delegates import validation to the typed bridge contract and maps replay, stale, family, device, authority, direct-publish, raw-payload, and mismatched-ref rejections explicitly.
- Real crate-level tests exist at `crates/agent-protocol/tests/contract/household_mesh.rs`, `crates/agent-core/tests/unit/household_mesh_event_bridge.rs`, and `crates/agent-core/tests/integration/household_mesh_bridge_runtime.rs`.
- WP10 proof script is updated so it cannot pass on stale inline-test or wrong-scope source-shape checks.
- `output/eventing-plan-proof/12-household-mesh-consumer/proof-summary.json` and `test-results/eventing-household-mesh-consumer-proof/proof.json` exist and are generated from the current source.
- Final proof text says physical household LAN/provider execution remains manual and separately owned under `lan-plan`.
- Focused cargo/clippy/architecture validation is green for the touched crates and files.
- No placeholder `tests/` folders or untracked proof notes are counted as completion.

**COORDINATOR_DECISION_REQUEST**

- Recommended next slice: `WP10-A protocol/bridge runtime wiring`.
- Recommended predecessor plans: none before `WP10-A`; before `WP10-C` final proof/doc closure, only ensure `lan-plan` authoritative `01-20` contract surfaces are stable enough to cite.
- Estimated risk: medium.
- Estimated proof difficulty: medium.
- Continue immediately or pause for sequencing: continue immediately for `WP10-A`; pause only before final proof/doc closure if you know `lan-plan` signed-peer / assignment / authority surfaces are still moving in another active thread.

