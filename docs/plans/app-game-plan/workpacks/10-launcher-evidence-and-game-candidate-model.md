# 10 Launcher Evidence And Game Candidate Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `10 Launcher Evidence And Game Candidate Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Game launchers and launcher-game candidates are first-class states that do not
overclaim active game play.

## Scope

- Steam, Epic, Xbox, Riot, Battle.net, EA, Ubisoft, GOG, Roblox, Minecraft,
  itch.io, and native cloud-game client evidence where available.
- Launcher installed, launcher running, launcher foreground, child game process,
  manifest game id, and launcher-game candidate states.
- Parent-facing explanation for launcher-only versus active game.

## Tests And Proof

- Launcher installed is inventory only.
- Launcher running is runtime only.
- Launcher foreground is launcher foreground only.
- Launcher-game candidate is not known game.
- Known game requires deterministic or classifier-backed child-game proof.

## Done Signal

Launchers can help identify games without becoming fake game-session proof.

## Implementation

- Added `AppGameLauncherEvidenceSchema` and
  `AppGameLauncherGameProofStateSchema` in `packages/activity-domain` for
  launcher-only, launcher manifest candidate, child process candidate,
  deterministic child-game proof, classifier-backed child-game proof,
  permission-limited, adapter-error, and not-claimed states.
- Mirrored the launcher evidence row in `crates/agent-protocol` so the Rust
  boundary carries launcher-only and child-game proof fields explicitly.
- Added a staged `agent-core` Windows launcher evidence parser that downgrades
  launcher overclaims to candidate or launcher-only states unless a child-game
  evidence claim is present.
- Product scope remains unchanged: this is contract/protocol/parser proof, not
  live launcher crawling, service events, portal launcher rows, game-budget
  policy, or broad blocking.

## AI Worker Checklist

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-game-snapshot.md), [main checklist](../implementation-checklist.md), [app/game evidence expectation](../../../expectations/app-game-evidence.md), and this workpack.
- [ ] Hub lock covers this workpack and exact implementation/docs/proof paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] TypeScript Effect Schema contract added before Rust/core consumers.
- [ ] Rust protocol parity added for the new launcher evidence row.
- [ ] Staged Windows parser proof added without claiming live launcher crawling.
- [ ] Tests prove launcher installed/running/foreground/candidate states do not become known-game proof without child-game evidence.
- [ ] Required proof pack exists under `output/app-game-plan-proof/10-launcher-evidence-and-game-candidate-model/`.
- [ ] Feature/plan docs updated; product capability checklist unchanged because product status did not move and A owns that file.

## Completion Notes

- Branch: `codex/app-game-launcher-candidate-model`
- Proof: `output/app-game-plan-proof/10-launcher-evidence-and-game-candidate-model/`
- Product-doc decision: feature current-state and app-game plan docs updated;
  product capability checklist unchanged because this proof does not move the
  app/game control product row to complete.

## 2026-08-17 Source-Wave Truth

Production source is accepted on the integration branch. The recurring Windows
capture now takes one process snapshot and shares its process-start generation
across process, launcher, generic foreground, and app/game foreground evidence.
Launcher child processes remain candidates unless deterministic or
classifier-backed proof promotes them. Generic foreground evidence retains its
window identity while app/game sessionization consumes the bound
generation-safe process identity exactly once.

This is source acceptance only. The expected-test wave must still refresh or
add the launcher/candidate, missing-generation, targeted-PID, shared-snapshot,
foreground-join, duplicate-session, and no-false-known-game cases before any
focused execution or proof. None of the checklist or proof rows below are
closed by this source integration.

## 2026-08-28 Test-Source Wave Truth

Canonical `51d9819a9` adds the six mapped real Rust test roots for this bounded
workpack. They cover launcher/candidate states that do not become known games,
deterministic and classifier-backed promotion, missing PID and process
generation, a shared PID-start generation identity, foreground joins, PID
reuse session separation, and invalid known-game ingest:

- `crates/agent-core/tests/unit/activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs`
- `crates/agent-core/tests/unit/activity_store_app_game/app_game_windows_foreground_source_tests.rs`
- `crates/agent-core/tests/unit/activity_store_app_game/app_game_windows_launcher_tests.rs`
- `crates/agent-core/tests/unit/activity_store_app_game/app_game_windows_process_runtime_tests.rs`
- `crates/agent-core/tests/unit/activity_store_app_game/app_game_windows_process_source_tests.rs`
- `crates/agent-protocol/tests/unit/app_game_tests.rs`

No test, build, proof, pre-commit, CI, or PR was run in this code/test-source
phase. Live launcher manifest/catalog crawling, the external
publisher/classifier proof owner, focused execution, retained proof, checklist
acceptance, READY, and DONE remain open.

Use the standard checklist in [workpacks README](README.md).
