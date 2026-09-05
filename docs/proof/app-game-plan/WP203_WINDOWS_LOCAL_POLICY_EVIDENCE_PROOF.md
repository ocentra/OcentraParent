# App + Game WP203 Durable Proof Manifest

Workpack: `WP-app-game-plan-203-app-game-windows-local-policy-evidence-proof`

Reviewed checkout head: `dd65d5d1318db7e7c02d1c6c747b8e5ce9d28c4c`

Retained generated root:

```text
output/app-game-plan-proof/203-app-game-windows-local-policy-evidence-proof/
```

The generated root is intentionally ignored by Git. This checked-in manifest
records the exact retained bundle reviewed in the canonical lane on 2026-09-02
without treating checkout-local command output as source-controlled evidence.

## Retained bundle

| Artifact | Result | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `00-scope-summary.md` | passed | 1192 | `52182156c063c1d5b55be8444dfc2ce97bdc20ebbb13a2eb6b3492c747c3909c` |
| `01-negative-case-proof.md` | passed | 989 | `dd2a0b40c7fae6826ab5beaf8b27a7cc0a34ad735a6ea57a16d523c50b2873d3` |
| `02-no-claim-boundary.md` | passed | 895 | `6af726817c808baf072a70c4aa4fb388c01fa51ae203fcac903ce50a6bf862e1` |
| `16-validation-commands.log` | passed | 1790 | `89941a50d876d49c682893586d745bc3674ab6d3ed81d35884bb577bb1770358` |

## Recorded commands

The retained evidence-wrapper log records exit code 0 for:

```text
cargo test -p ocentra-app-game-windows-local-policy-ffi --all-targets
cargo test -p ocentra-parent-agent-protocol --test contract app_game_platform_proof_status
```

Evidence wrapper run `run-20260902101545-401c6e37` passed one real
no-argument Windows observation test and four parser/negative tests. Run
`run-20260902101617-79b0b62c` passed four focused protocol tests. The tests
cover unknown/private fields, invalid UTF-8, the 4 KiB bound, impossible
counts and states, partial-query results, non-Windows typed unavailability,
unknown protocol fields, required non-claim gaps, and serialization without
enforcement authority.

## Proof conclusion

This bundle proves the dedicated Windows observer and strict protocol boundary
for bounded AppIDSvc, AppLocker, Device Guard, and App Control booleans and
counts. It captures no raw AppLocker XML, executable paths, publisher rules,
service names, or private diagnostics.

Agent Service still keeps `windows_local_policy_evidence` absent in the
fail-closed status row. This proof does not claim Agent Service composition,
runtime readiness, broad launch blocking, policy enforcement, adapter dispatch,
rollback, audit custody, provider delivery, or child-device delivery.
