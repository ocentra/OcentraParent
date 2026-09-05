# Child Agent Runtime Distribution WP06 Durable Proof Manifest

Workpack: `WP-child-agent-runtime-distribution-plan-06-child-ios-agent-capability-package`

Reviewed checkout head: `dd65d5d1318db7e7c02d1c6c747b8e5ce9d28c4c`

Retained generated roots:

```text
output/child-agent-runtime-distribution-plan-proof/06-child-ios-agent-capability-package/
test-results/child-ios-entitlement-capability-proof/proof.json
```

These run artifacts are intentionally ignored by Git. This checked-in manifest
records the exact retained bundle reviewed in the canonical lane on 2026-09-02
without turning host-local XCTest output into source-controlled evidence.

## Retained bundle

| Artifact | Result | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `00-scope-summary.md` | capability boundary recorded | 562 | `8080582c3283fc134cabff85d5f987f62fb6adc8a0ed31aebb5865f4b1377c95` |
| `01-negative-case-proof.md` | passed | 382 | `4a6a4b3c2c7282bb14ce511e3e89686559ebdceac9ea3c88837c9f11186055d9` |
| `02-no-claim-boundary.md` | passed | 322 | `2aa55cb5862d94589056940321681fecc8e4ad7e2e7587ff1833bcb356d11d56` |
| `16-validation-commands.log` | bounded result recorded | 493 | `6238737139c1edef12b7496d7da59d8a0fa79eae192d0c827d3e0d0bf3964f89` |
| `test-results/child-ios-entitlement-capability-proof/proof.json` | host-blocked XCTest recorded | 19078 | `2a9b6d2214f4c172a366db1399346c89e348f5c631e63488d537f7a1df2efacd` |

## Recorded commands

The retained result records:

```text
cargo test -p ocentra-schema --test contract child_ios_entitlement_capability
  result: passed

node --test platforms/ios/tests/child_capability_identity.test.mjs
  process exit: 0
  XCTest result: skipped / platform-unavailable / manual-required
  reason: iOS XCTest UI runtime requires macOS with Xcode and an iOS Simulator
```

The runner rejects parent product, project, scheme, bundle, and artifact
identity. It also rejects capability contracts that claim external transport,
daemon behavior, supervision proof, or recovery implementation. A Windows
host skip is retained as explicit blocked evidence, not relabelled as a pass.

## Proof conclusion

This bundle proves the checked-in `OcentraChildAgent` identity, bundle ID
`ca.ocentra.child.agent`, Rust-owned capability contract, strict negative
boundaries, host-aware XCTest source, and explicit platform-unavailable state.
It closes only the bounded capability-package workpack represented by the
current graph.

It does not claim Apple signing, provisioning or entitlement approval,
physical-device launch, TestFlight or App Store distribution, supervision,
persistent background execution, daemon or hidden-control behavior, external
transport, recovery, tracking ingress, or parent-client parity.
