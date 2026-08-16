# 17 Risk App Detection

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`docs/expectations/ai.md`.

## Where We Are

Native app category/risk taxonomy exists in the shared activity-domain layer.
This workpack adds the app-only parent-domain candidate proof for risk app
detection with confidence, source, policy routing, parent display, and
no-content boundaries.

Code-pass status (2026-08-16): Rust app-game-core now promotes explicit
upstream inventory risk-category rows and unknown-process rows into advisory
candidates. This is code-drafted and unvalidated; executable-name heuristics,
publisher/hash enrichment, local-AI quality, service consumption, tests, and
proof remain deferred.

Risk labels are parsed by the shared `agent-protocol` contract. Malformed
evidence is retained as an explicit invalid/manual-required state; it is never
silently discarded into an apparently proved candidate.

## Where We Want To Be

Risk app detection creates explainable candidates for parent review and policy
targets. It covers VPN/proxy, remote desktop, torrent/download, installer,
AI/chatbot, messaging/social/video, and unknown risk without directly enforcing.

## Scope

- Risk category contract and candidate state.
- Known app catalog risk mappings.
- Unknown name/publisher/hash-derived candidates.
- AI classification handoff when local AI proof exists.
- Parent override and display labels.
- Policy candidate and ask parent routing.

## Touched Paths

- `crates/app-game-core/src/app_game_risk_candidate_detection.rs`
- `crates/app-game-core/src/app_game_category_risk_policy_routing.rs`
- `crates/agent-protocol/src/app_game.rs`

The detector recognizes only explicit upstream category labels and emits
`AppRisk` candidates with `AskParent`; unknown/no-proof rows remain candidates
for review and are rejected by policy routing until category evidence exists.
It never asserts content knowledge or dispatches enforcement.

## Tests And Proof

- Known VPN/proxy, remote desktop, torrent/download, and AI chatbot fixtures
  classify with evidence refs.
- Unknown executable with risklike name remains candidate, not fact.
- Unknown publisher lowers confidence.
- Risk category cannot directly enforce.
- UI shows confidence, source evidence, and no content claim.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created. `packages/activity-domain` taxonomy is reused but not edited because it is locked by `codex-a`.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/17-riskapp-detection/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior: `packages/parent-domain/src/app-riskdetection.ts`, `app-riskdetection-rules.ts`, and `app-riskdetection-data.ts`.
- [ ] Rust/service/portal parity N/A for this slice because no runtime protocol, service read model, or portal payload changed.
- [ ] Raw evidence artifact captured: `output/app-plan-proof/17-riskapp-detection/03-runtime-evidence.json`.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented as contract/proof-harness evidence.
- [ ] Required fixtures are represented by proof-matrix rows for known catalog, unknown executable-name, unknown publisher/hash, local AI digest, and parent display override candidates.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots N/A; `output/app-plan-proof/17-riskapp-detection/06-ui-snapshots/ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured in `output/app-plan-proof/17-riskapp-detection/08-security-negative-proof.log`.
- [ ] Manual platform proof N/A for support claims; `09-manual-platform-proof.md` records that no live classifier, portal, or enforcement claim moved up.
- [ ] Platform limitations remain contract-only; live scanning, service events, portal rendering, AI provider quality, and enforcement stay later proof.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/checklist update decision recorded: feature doc and app-plan checklist updated; product capability checklist unchanged because the row remains in progress and `codex-a` owns that lock.
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Risk detection is advisory evidence. Parent policy and platform authority still
decide actions.

## Historical completion note - 2026-06-03

- Owner/lane: `codex-c`.
- Branch: `codex/app-game-read-model-service-events`.
- Proof root: `output/app-plan-proof/17-riskapp-detection/`.
- Contract: `packages/parent-domain/src/app-riskdetection.ts`.
- Rules: `packages/parent-domain/src/app-riskdetection-rules.ts`.
- Data: `packages/parent-domain/src/app-riskdetection-data.ts`.
- Test: `packages/parent-domain/tests/app-riskdetection.test.ts`.
- Harness: `node scripts/test/app-riskdetection-proof.mjs`.

Proof captured:

- Known VPN/proxy, remote desktop, torrent/download, and AI chatbot native app
  risk candidates classify with evidence refs.
- Unknown risklike names and hash-derived rows remain candidates instead of
  facts.
- Unknown publisher state lowers confidence.
- Local AI risk candidates require a digest and stay ask parent or review
  routed.
- Risk candidates cannot directly enforce, and risk-app policy targets require
  category proof.
- Parent-surface disclosure carries confidence percent, source evidence count,
  and no-content-captured state.

The historical package-domain contract/proof harness does not establish current
Rust runtime completion. No live OS scanning, catalog enrichment, service read
model, portal evidence drawer UI, local model quality, or platform enforcement
claim moves up in this code pass.
