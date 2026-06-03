# Source Snapshot

Workpack: app-game WP14 journal and SQLite ingest

Branch: `codex/app-game-journal-sqlite-ingest`

Starting head while preparing proof: `16297b7c33f42ea221c8969258b62d724e4717e0`

Before-state gap:

- App/game sessionization could derive deterministic durations from stored
  SQLite observation rows.
- The typed inventory, runtime, foreground, and launcher evidence rows did not
  have a focused encrypted journal-file append plus SQLite replay proof.
- The read-model proof did not yet show replayed inventory, running-now,
  foreground-now, launcher, and daily rollup rows from the same journal source.

Locked source/proof paths:

- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs`
- `crates/agent-core/src/activity_store_app_game_rows.rs`
- `crates/agent-core/src/activity_store_app_game_observation.rs`
- `crates/agent-core/src/lib.rs`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `docs/plans/app-game-plan/*`
- `docs/plans/app-plan/*`
- `docs/features/app-game-control.md`
- `output/app-game-plan-proof/14-journal-and-sqlite-ingest`
- `output/app-plan-proof/13-journal-and-sqlite-app-ingest`

Source files inspected:

- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `.ocentra-ai/rules/ocentra-parent-rust-service.mdc`
- `.ocentra-ai/rules/ocentra-parent-logging-redaction.mdc`
- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/evidence-storage.md`
- `docs/architecture/app-game-evidence-sessions.md`
- `docs/plans/app-game-plan/README.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/14-journal-and-sqlite-ingest.md`
- `docs/plans/app-plan/README.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/13-journal-and-sqlite-app-ingest.md`
- `packages/activity-domain/README.md`
- `crates/agent-core/README.md`
- `crates/agent-protocol/README.md`

Current `git status --short` included this workpack's implementation and proof
files plus pre-existing untracked `.codex/` and `.playwright-cli/` artifacts
from earlier C-lane UI work. Those unrelated artifacts were preserved and are
not part of this proof.
