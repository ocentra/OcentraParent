# WP06 Validation Commands

| Command | Exit | Result | Notes |
| --- | ---: | --- | --- |
| `cargo test -p ocentra-eventing --test journal_replay` | 0 | pass | 22 passed; append/hash-chain/recovery, corruption, cursor/filter, projection-only safety, journal policy. |
| `cargo test -p ocentra-eventing --test contract topology_manifest` | 0 | pass | 4 passed; deterministic manifest and explicit topology classifications. |
| `cargo test -p ocentra-eventing --test contract compatibility_matrix` | 0 | pass | 3 passed; lineage semantics, deviations, and manual-required scope. |
| `cargo test -p ocentra-eventing --test unit production_shutdown` | 0 | pass | 5 passed; drain/dead-letter, cancellation, and shutdown lifecycle. |
| `cargo test -p ocentra-eventing --test version_skew` | 0 | pass | 2 passed; reject newer and older stored envelope schema versions. |
| `npm run lint:architecture -- --files crates/ocentra-eventing/src crates/ocentra-eventing/tests` | 0 | pass | Current repository architecture-policy gate. |
| `rg -n --glob '*.rs' 'EventBus::global|OnceLock<.*EventBus|Lazy<.*EventBus|static .*EventBus' crates/ocentra-eventing/src crates/ocentra-eventing/tests` | 1 | expected zero-match | No hidden global EventBus pattern found. |

## Validation boundary

These commands prove only Eventing WP06 generic journal/replay/topology
mechanics. They do not prove enforcement adapter dispatch, authorization,
rollback, side effects, transport, retention, deletion, or WP10.
