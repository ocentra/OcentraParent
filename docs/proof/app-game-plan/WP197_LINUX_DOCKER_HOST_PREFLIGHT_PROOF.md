# App + Game WP197 Durable Proof Manifest

Workpack: `WP-app-game-plan-197-app-game-linux-docker-host-preflight`

Reviewed checkout head: `dd65d5d1318db7e7c02d1c6c747b8e5ce9d28c4c`

Retained generated root:

```text
output/app-game-plan-proof/197-app-game-linux-docker-host-preflight/
```

The generated root is intentionally ignored by Git. This checked-in manifest
records the exact retained bundle reviewed in the canonical lane on 2026-09-02
without treating checkout-local command output as source-controlled evidence.

## Retained bundle

| Artifact | Result | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `00-scope-summary.json` | passed | 1331 | `85db23594526151321e84c7080aa3db9765b4ddc68c44db96c2319962b89fe82` |
| `01-negative-case-proof.md` | passed | 407 | `acd8f0e97c66703a17948ada332734e46866968e1ed54da498af0fb45d9305d5` |
| `02-no-claim-boundary.md` | passed | 333 | `e51c8800ad64e0ef903a29d584b6a591a65ce2035d41090cb724f75b624c76e2` |
| `16-validation-commands.log` | passed | 1989 | `e3506b389bea1aef300d4f44d078553962310d957a819b94e06c9257564a8900` |
| `raw/01-graph-validate.log` | passed | 97 | `c713d084d939ef2effc213db7a967c70753b12c3954a794f3a533be68dd573d0` |
| `raw/02-protocol-contract.log` | passed | 119 | `082c986a96115bcd54ce162e120c05b0c480028b191efcc1c4acbcf0f76ca8a0` |
| `raw/03-service-preflight.log` | passed | 95427 | `cb8112a5ad2601ca856ee47650e927e89a6ab0471baf6522e7b940460fec654e` |
| `raw/04-architecture.log` | passed | 45 | `44c8964815633706ed02b6420aba6586187218495c143ce9ed717f842a9461bf` |

## Recorded commands

The retained validation log records exit code 0 for:

```text
npm run graph:validate
cargo test -q -p ocentra-parent-agent-protocol --test contract app_game_platform_proof_status -- --test-threads=1
cargo test -q -p ocentra-parent-agent-service --test app_game_linux_preflight -- --test-threads=1
npm run lint:architecture -- --files <WP197 protocol/service/tests/docs>
```

The protocol run passed 4 focused tests. The Agent Service preflight run passed
21 tests covering unavailable probes, cleanup-owner degradation, malformed and
oversized markers, invalid UTF-8, protected path boundaries, cache-unavailable
state, route rejection, and bounded count-only parent visibility.

The retained scope summary was regenerated after final graph completion and
records both `graphState` and `lifecycleState` as `done`. The current workpack,
checklist, generated graph, proof runner, and this portable manifest therefore
agree on the bounded DONE closure.

## Proof conclusion

This bundle proves the bounded Linux Docker CLI, daemon, context, image, and
container host preflight, protected path and cleanup behavior, cache state, and
parent-safe redacted-count projection. It stores no Docker context names,
image names, container identifiers, executable paths, raw daemon diagnostics,
or private target details.

It does not claim container policy execution, adapter dispatch, platform
enforcement, provider or child delivery, rollback, or audit authority.
