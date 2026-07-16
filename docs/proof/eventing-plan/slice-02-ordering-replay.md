# Slice 02 Ordering Replay

## Scope

Journal append/replay, replay cursor/filter behavior, and version-skew checks
for the reusable crate.

## Evidence

- `cargo test -p ocentra-eventing --test journal_replay`
- `cargo test -p ocentra-eventing --test version_skew`
- `cargo test -p ocentra-eventing --tests`
- `cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests`

## What This Proves

- The reusable crate still passes focused ordering/replay/version-skew coverage
  in this checkout.
- Removing the last source-side `src/tests` layout did not break the external
  journal/replay harnesses.
- The local test harness boundary now lives only under
  `crates/ocentra-eventing/tests`.

## Negative / Not Proved

- This slice does not restore the historical
  `output/eventing-plan-proof/36-41-journal-replay/proof-summary.json` file.
- This slice does not prove platform durability, retention policy, or
  cross-process replay behavior beyond the local reusable crate surface.

## Remaining Gaps

- Route closure still depends on honest WP10 status even though WP11/WP12 are
  now locally proved.
