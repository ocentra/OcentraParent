# Event-Driven Proof Architecture Router

Use this folder before source work that crosses plan, domain, crate, service, UI, AI, logging, eventing, or proof boundaries.

## Route

1. `INDEX.md`
2. `EVENT_BOUNDARY_STANDARD.md`
3. `LOGGED_PROOF_CHAIN_STANDARD.md`
4. `DEV_UI_PLAYWRIGHT_PROOF_STANDARD.md`
5. `TEST_PLACEMENT_AND_PROOF_LAYERS.md`
6. `COMMON_FAILURES.md`
7. `thread-instructions/<plan>.md`
8. Assigned plan docs, feature docs, expectation docs, and exact source files.

## Direct import rule

Allowed direct imports:

- schemas;
- typed contracts;
- enums and constants;
- parser/decoder helpers;
- local helpers inside the same owner.

Cross-responsibility behavior must use typed commands, events, requests, responses, journals, or read models.

## Required source-slice report

Every assigned slice reports:

- owner package/crate;
- emitted command/event/request names;
- consumed event/read-model names;
- log/proof correlation strategy;
- test layer used;
- proof artifacts generated;
- boundaries intentionally not proven.
