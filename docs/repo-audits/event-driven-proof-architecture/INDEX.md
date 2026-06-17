# Event-Driven Proof Architecture Index

This folder defines the architectural standard that must be applied before per-plan implementation expands.

It complements `docs/repo-audits/lane-manager-coordination/`: the lane-manager docs say **who goes when**; this folder says **what architecture each thread must enforce while doing it**.

## Read order

| Step | File | Purpose |
| ---: | --- | --- |
| 1 | [AGENTS.md](AGENTS.md) | Local router for this architecture layer. |
| 2 | [EVENT_BOUNDARY_STANDARD.md](EVENT_BOUNDARY_STANDARD.md) | Cross-responsibility event/request/read-model rules. |
| 3 | [LOGGER_USAGE_PATTERN_STANDARD.md](LOGGER_USAGE_PATTERN_STANDARD.md) | Per-file logger pattern adapted from Ocentra Games. |
| 4 | [LOGGED_PROOF_CHAIN_STANDARD.md](LOGGED_PROOF_CHAIN_STANDARD.md) | Controlled logging and proof-chain requirements. |
| 5 | [DEV_UI_PLAYWRIGHT_PROOF_STANDARD.md](DEV_UI_PLAYWRIGHT_PROOF_STANDARD.md) | Dev command UI and Playwright proof pattern. |
| 6 | [TEST_PLACEMENT_AND_PROOF_LAYERS.md](TEST_PLACEMENT_AND_PROOF_LAYERS.md) | Where unit/contract/integration/e2e/proof tests belong. |
| 7 | [COMMON_FAILURES.md](COMMON_FAILURES.md) | Repeated mistakes to block in every plan. |
| 8 | [thread-instructions/INDEX.md](thread-instructions/INDEX.md) | Per-plan architectural instructions. |

## Existing repo surfaces to reuse

| Surface | Current role |
| --- | --- |
| `@ocentra-parent/logging-domain/core/logger` | TypeScript controlled logger. |
| `@ocentra-parent/logging-domain/core/stackTrace` | Stack trace helper and type. |
| `@ocentra-parent/logging-domain/test-log/*` | NDJSON/test-log/proof-log infrastructure. |
| `@ocentra-parent/logging-domain/app-log/*` | App-log storage/writer infrastructure. |
| `@ocentra-parent/logging-domain/transport/*` | Bridge log transport. |
| `crates/ocentra-eventing` | Rust event bus, envelope, journal, queue, request, replay, topology, and testkit foundation. |

## Rule

Do not start a plan slice that crosses responsibility boundaries until the slice states:

1. command/event/request contract;
2. owner that emits it;
3. owner that consumes it;
4. read model or response contract;
5. log/proof trace points;
6. test layer that is allowed to prove the chain;
7. logger/test-log/event-journal instrumentation strategy.
