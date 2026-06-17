<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `Device Trust Bootstrap Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Test Proof Expectations

## Proof root

```text
output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/
```

## Test layout

```text
test/device-trust-bootstrap-plan/<major-category>/
```

Major categories:

- `unit`
- `contract`
- `integration`
- `e2e`
- `security`

Current device-trust coverage starts in:

- `test/device-trust-bootstrap-plan/unit/local-key-sealing.test.mjs`
- `test/device-trust-bootstrap-plan/contract/parent-step-up-auth.test.mjs`
- `test/device-trust-bootstrap-plan/integration/recovery-re-pair-boundary.test.mjs`

These plan-local tests currently prove document and route alignment only. They do
not prove runtime key sealing, passkey ceremony, QR approval, recovery bundle
execution, or child uninstall execution by themselves.

Implementation-adjacent coverage currently lives in:

- `packages/family-domain/tests/unit/household-authority.test.ts`
- `packages/family-domain/tests/unit/setup-lifecycle.test.ts`
- `packages/family-domain/tests/unit/invite-recovery-lifecycle.test.ts`
- `packages/lan-domain/tests/unit/lan-pairing.test.ts`
- `packages/lan-domain/tests/unit/household-device-spine.test.ts`
- `packages/lan-domain/tests/unit/device-roles.test.ts`
- `crates/agent-protocol/src/lan_pairing_tests.rs`
- `crates/agent-service/src/lan_pairing_tests.rs`
- `crates/agent-service/src/lan_pairing_multidevice_tests.rs`

## Common commands

Docs-only truth sync:

```powershell
$tests = Get-ChildItem test/device-trust-bootstrap-plan -Recurse -Filter *.test.mjs |
  Sort-Object FullName |
  Select-Object -ExpandProperty FullName
node --test $tests
```

Family authority and recovery:

```powershell
npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/setup-lifecycle.test.ts tests/unit/invite-recovery-lifecycle.test.ts
```

LAN domain trust-adjacent contracts:

```powershell
npm run test --workspace @ocentra-parent/lan-domain -- tests/unit/lan-pairing.test.ts tests/unit/household-device-spine.test.ts tests/unit/device-roles.test.ts
```

Rust protocol and service LAN pairing seams:

```powershell
cargo test -p ocentra-parent-agent-protocol lan_pairing
cargo test -p ocentra-parent-agent-service lan_pairing
```

Scoped architecture gates:

```powershell
npm run lint:architecture -- --files packages/family-domain/src packages/lan-domain/src test/device-trust-bootstrap-plan docs/plans/device-trust-bootstrap-plan
cargo lint-architecture crates/agent-protocol/src/lan_pairing.rs crates/agent-service/src/lan_pairing.rs
```

If the touched slice includes `packages/parent-domain` frontage or
`tamper-uninstall-artifact-status`, run focused architecture gates there too.

## Host and platform proof expectations

- Windows proof is expected where the touched runtime slice is Windows-relevant.
- Android proof is expected where the touched runtime slice is Android-relevant, including Android Studio/emulator and the already-synced Samsung device when needed.
- Linux proof is expected where the touched runtime slice is Linux-relevant, including WSL and Docker where appropriate.
- Real iOS and macOS proof is an external-platform constraint from this Windows host. Record it as such when relevant; do not treat it as a local blocker.

## Blocker classification

When recording blocked validation or missing proof, classify each item as one of:

- `real dependency blocker`
- `external platform constraint`
- `avoidable local execution gap`

## Required proof states

```text
trust source-of-truth
local key custody
parent approval step
phone approval bridge
entitlement snapshot
recovery/reset/re-pair
child-device removal/tamper state
dependency adoption review
route gate
```

## Required negative states

```text
login alone not trust proof
license alone not unlock proof
wrong household/device blocked
revoked/expired state visible
manual-required state visible
mock proof not product proof
```

## No fake-green rule

- Document assertions and route-alignment tests may prove plan honesty, but they do not close runtime workpacks.
- Use mocks only when justified by the workpack risk surface and call them out explicitly in proof notes.
- Prefer real contract, integration, and end-to-end behavior over mock-only coverage whenever the plan risk requires it.
