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
