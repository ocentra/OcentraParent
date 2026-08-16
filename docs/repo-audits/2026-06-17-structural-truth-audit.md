# OcentraParent Structural Truth Audit

Date: 2026-06-17
Branch: `codex/tracking-plan-full-continuation-a`
Purpose: compact repo-level audit reference before reviewing per-plan thread self-assessments.

This document records structural issues that can make plan reports look green when the repo is not actually done. It is intentionally indexed and table-driven so it can be reused during long coordination threads without turning into a 10-page narrative.

## Document index

| Section | Use |
| --- | --- |
| [Working rules](#working-rules) | Baseline rules for accepting/rejecting plan claims. |
| [Red finding index](#red-finding-index) | Short list of active structural risks. |
| [Test topology findings](#test-topology-findings) | Empty scaffolds, inline tests, and weak test gates. |
| [Architecture and validation findings](#architecture-and-validation-findings) | Re-export gate, CI segmentation, and omitted package/crate risks. |
| [Ownership and DRY map](#ownership-and-dry-map) | Where code appears duplicated or placed in broad frontage layers. |
| [Thread report review template](#thread-report-review-template) | Template for each pasted plan-thread report. |
| [Immediate execution queue](#immediate-execution-queue) | What should be fixed/audited first. |

## Working rules

| Topic | Rule |
| --- | --- |
| Proof artifacts | `output/` and `test-results/` are intentionally untracked. Missing generated artifacts in Git is not a defect by itself. A proof claim must name the tracked command, generated path, and local/CI run evidence. |
| Test folders | Empty `.gitkeep` folders and broad category scaffolds do not count as tests. Count only executable `.rs`, `.ts`, `.tsx`, `.mjs`, Playwright, or equivalent test files. |
| Rust source tests | Source-adjacent `#[cfg(test)]` modules are acceptable only for private seams or wiring that cannot be specified through public APIs. Public behavior should have crate-level `tests/` coverage where feasible. |
| Test gates | Passing a presence check does not prove feature/workpack coverage. Required-test gates must be treated as minimum hygiene only. |
| Architecture gates | A changed-file or package-scope pass does not prove repo-wide architecture clean. Reports must state the exact scope. |
| Plan reports | Treat thread reports as evidence, not truth. Source, tests, proof generators, and run evidence win over stale docs and checkmarks. |

## Red finding index

| ID | Status | Finding | Why it is red | Required action |
| --- | --- | --- | --- | --- |
| TEST-01 | active | Empty `.gitkeep` test scaffold folders exist. | Agents can mistake folder topology for risk coverage. | Inventory and delete placeholders or replace with real tests. |
| TEST-02 | active | `scripts/check-required-tests.mjs` is too weak. | It checks existence, not coverage, risk class, or workpack proof. | Add a stricter topology audit command. |
| TEST-03 | active | Rust inline tests are heavy in some crates. | Public behavior can be hidden in `src/*_tests.rs` instead of tested through crate APIs. | Classify inline tests as valid private seam or move candidate. |
| ARCH-01 | active | Rust `pub use` appears despite no-reexport policy. | Repo-wide architecture-clean claims are suspect. | Decide global cleanup vs explicit staged exception. |
| ARCH-02 | active | TypeScript/JavaScript re-export/barrel debt exists in shared surfaces. | Same as Rust: false architecture green and unclear ownership. | Remove barrels or add explicit allowed transition plan. |
| CI-01 | active | CI Rust coverage is segmented. | CI does not equal `cargo test --workspace`. | Build crate-to-CI matrix. |
| CI-02 | active | Domain package build/test coverage is hand-maintained. | Real packages can be omitted from `build:contracts` or `test:contract`. | Build package-to-command matrix and fail on omissions. |
| DRY-01 | active | Repeated child-domain runtime event handoff code exists. | Similar app/app-game/browser/network logic can diverge. | Add tests first, then extract shared event-chain assembly. |
| OWN-01 | active | Broad frontage packages hide owner drift. | `parent-domain`, `portal-domain`, and `agent-protocol-domain` can become dumping grounds. | Route source back to narrow owner packages/crates. |
| PROOF-01 | active | Generated proof paths are sometimes phrased as proof truth. | Local artifacts are intentionally ephemeral. | Reports must name generator command and local/CI evidence. |

## Test topology findings

| ID | Surface | Finding | What to check next |
| --- | --- | --- | --- |
| TEST-01A | `crates/agent-core/tests/**/.gitkeep` | Empty placeholder folders exist under risk-category names. | Generate a full list of `.gitkeep`-only folders and remove or replace. |
| TEST-01B | `crates/agent-core/tests/unit.rs` | Real crate-level test entry exists, but initial check found only a narrow tracking read-model module. | Count all real `.rs` files under `crates/agent-core/tests/` and compare against empty folder count. |
| TEST-02A | `scripts/check-required-tests.mjs` | Rust passes when either any inline `#[cfg(test)]` exists or any crate-level `.rs` test exists. | Add output that reports inline count, integration count, empty dirs, and uncovered crates. |
| TEST-02B | `scripts/check-required-tests.mjs` | TS passes when a package has at least one `.test.ts` or `.spec.ts`. | Add package-level test inventory and minimum mapping to package exports. |
| TEST-03A | `crates/agent-core/src/lib.rs` | Many `#[cfg(test)] mod ...` declarations. | Mark each as private seam, transport wiring, or move-to-`tests/`. |
| TEST-03B | `crates/agent-protocol/src/lib.rs` | Many `#[cfg(test)] mod ...` declarations. | Move protocol public serialization/contract behavior to crate-level tests where feasible. |

## Architecture and validation findings

| ID | Surface | Finding | Required distinction |
| --- | --- | --- | --- |
| ARCH-01A | Root rule + Rust lint tool | Rust `pub use` is banned by policy and by `tools/no-reexports`. | Repo-wide architecture clean is not credible while crate roots still use `pub use`. |
| ARCH-01B | Rust crate roots | `app-core`, `app-game-core`, `agent-core`, `agent-protocol`, `tracking-core`, `storage-custody-core`, `family-identity-core`, and `lan-core` were observed using `pub use`. | Treat any architecture pass as scoped unless full cleanup or exception exists. |
| ARCH-02A | TS shared domains | Prior first pass found TS barrel/re-export debt in shared surfaces such as screen/portal/parent domain files. | Remove export-forwarding or document staged cleanup. |
| CI-01A | `.github/workflows/ci-*.yml` | Rust CI jobs cover selected packages, not all workspace members. | Plan reports must name exact cargo command, not just “CI passed.” |
| CI-01B | Local validation | `npm run validate` includes `cargo test --workspace`, but CI full validation aggregates segmented jobs. | Local full validation and CI validation are not equivalent. |
| CI-02A | `package.json` | `build:contracts` is a manual chain. | Compare every `packages/*/package.json` against root build chain. |
| CI-02B | `test:contract` | Contract test script filters only selected packages. | Domain-package proof requires package-specific test command unless matrix says covered. |

## Ownership and DRY map

| ID | Pattern | Observed surfaces | Likely owner / target shape | Do not do |
| --- | --- | --- | --- | --- |
| DRY-01A | Child-domain observed/evidence/AI/policy event-chain assembly repeats. | `crates/app-core`, `crates/app-game-core`, `crates/browser-core`, `crates/network-core`. | Keep per-domain intent mapping local, but extract shared event-chain assembly into the protocol/runtime layer after tests lock behavior. Candidate shape: a common helper that accepts `ChildRuntimeDomain`, ref suffix, observed signal, AI requirement, and policy requirement. | Do not duplicate another copy in tracking/screen/app plans. |
| DRY-01B | Runtime decision crates have similar enum/state/action patterns. | App, app-game, browser, network, tracking. | Standardize naming and proof expectations; extract only if behavior is truly common. | Do not prematurely abstract product-specific decisions. |
| OWN-01A | `agent-protocol-domain` aggregates many plan surfaces. | Commands/events/read-model adapters for network, tracking, app-game, policy, assistant, LAN, screen. | Protocol domain should own transport schema, constants, codec, and Rust-crossing shapes only. Product decisions stay in narrow domains/crates. | Do not let protocol become product logic. |
| OWN-01B | `parent-domain` contains many cross-plan proof/frontage surfaces. | Billing, app install, app-game, parent mobile, tracking, AI, LAN/tamper bridges. | Parent domain should be parent-product frontage and composition, not child runtime authority or proof dumping ground. | Do not fix downstream compile by moving owner logic into parent-domain. |
| OWN-01C | `portal-domain` and `apps/portal` can blur UI proof with source truth. | Portal panels and route read models consume many domains. | Portal owns rendering, source-state labels, and UX proof. It does not own policy, custody, LAN, trust, or runtime truth. | Do not claim runtime readiness from portal screenshots. |
| OWN-01D | `agent-core` is large and plan-dense. | Browser, network, screen, enforcement, tracking, policy dry-run, household mesh, journal, activity store. | Agent core should own local runtime composition and private adapter helpers. Reusable domain engines should live in focused crates. | Do not keep growing agent-core for every plan if a focused crate exists. |
| OWN-01E | `ocentra-network-evidence` is broad but should remain reusable evidence/parsing. | Parsers, fixtures, cascade, platform-gate proof, risk budget, adapter proof helpers. | Keep reusable metadata/parsing/proof helpers here; live capture/service/UI/policy authority elsewhere. | Do not put live enforcement or product UI ownership here. |

## What should have gone where: current routing guide

| Concern | Preferred owner | Notes |
| --- | --- | --- |
| Shared TS runtime schema helpers | `packages/schema-domain` | Keep Effect Schema/decode helpers here. |
| Endpoint/path/header constants | `packages/endpoint-domain` | Should be in root build/test gates if actively used. |
| Family/account/role/device authority contracts | `packages/family-domain` plus Rust parity in `family-identity-core` when needed | Setup, payment, policy, and remote should consume, not redefine. |
| Setup journey/readiness contracts | `packages/setup-domain` | Setup orchestration only; not account/session/trust authority. |
| Data custody contracts | `packages/data-custody-domain` and `crates/storage-custody-core` | Reports, AI, sync, remote, and setup should consume custody rules. |
| Child runtime orchestration | `crates/child-runtime` | Composition boundary for child-side feature domains. |
| Transport/protocol shapes | `packages/agent-protocol-domain` and `crates/agent-protocol` | No product behavior. |
| Local runtime composition/adapters | `crates/agent-core` | Should shrink when focused reusable crates exist. |
| Network metadata parsers/proof helpers | `crates/ocentra-network-evidence` | No live enforcement or product claims. |
| Reusable event bus/journal/replay | `crates/ocentra-eventing` | No cross-device transport or plan-specific policy. |
| Portal rendering and route panels | `apps/portal` plus `packages/portal-domain` | UI proof only, not runtime authority. |

## Thread report review template

Use this template for every pasted thread self-assessment.

| Field | Finding |
| --- | --- |
| Plan |  |
| Claimed status |  |
| Actual source owners |  |
| Misplaced source/tests |  |
| Real executable tests |  |
| Empty scaffold folders |  |
| Inline `src` tests |  |
| Proof generator commands |  |
| Generated proof destinations |  |
| Local run evidence |  |
| CI coverage |  |
| Architecture gate status |  |
| DRY/common-core concerns |  |
| Dependency blockers |  |
| Verdict |  |
| Best next slice |  |

## Verdict vocabulary

| Verdict | Meaning |
| --- | --- |
| `done` | Source, tests, proof generator, run evidence, docs, and dependency boundaries agree. |
| `partial` | Real work exists, but closure is blocked by missing tests, proof, dependencies, or ownership drift. |
| `false-green` | Docs/checklists/report say done, but code/tests/proof do not support it. |
| `missing` | Claimed surface has no meaningful implementation or executable proof. |
| `blocked` | Work cannot honestly proceed until a predecessor contract/proof/decision lands. |

## Immediate execution queue

| Order | Slice | Output |
| ---: | --- | --- |
| 1 | Test topology inventory | Count real tests, inline tests, and empty scaffold folders for `crates/`, `packages/`, and `apps/`. |
| 2 | CI coverage matrix | Map every crate/package/app to local commands and CI jobs. |
| 3 | Architecture policy decision | Decide whether `pub use` / TS re-export cleanup is immediate, staged, or exception-based. |
| 4 | Ownership drift map | List files in broad frontage packages that should move or be treated as adapters only. |
| 5 | DRY candidate proof | Lock behavior with tests before extracting repeated child-domain runtime event-chain assembly. |
| 6 | Plan-thread review | Review each pasted report against this audit before accepting completion. |

## Current working rule

Do not accept a plan completion report from checklist counts, folder scaffolds, generated proof paths, or optimistic docs alone. Accept only source-backed, test-backed, proof-generator-backed, and locally/CI-run-backed claims.
