<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `Device Trust Bootstrap Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: implementation completion or PR readiness.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Execution Blueprint

## Execution rule

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Proof root

```text
output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/
```

## Focused commands

```powershell
$tests = Get-ChildItem tests/device-trust-bootstrap-plan -Recurse -Filter *.test.mjs |
  Sort-Object FullName |
  Select-Object -ExpandProperty FullName
node --test $tests
npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/setup-lifecycle.test.ts tests/unit/invite-recovery-lifecycle.test.ts
npm run test --workspace @ocentra-parent/lan-domain -- tests/unit/lan-pairing.test.ts tests/unit/household-device-spine.test.ts tests/unit/device-roles.test.ts
cargo test -p ocentra-parent-agent-protocol lan_pairing
cargo test -p ocentra-parent-agent-service lan_pairing
npm run lint:architecture -- --files packages/family-domain/src packages/lan-domain/src tests/device-trust-bootstrap-plan docs/plans/device-trust-bootstrap-plan
cargo lint-architecture crates/agent-protocol/src/lan_pairing.rs crates/agent-service/src/lan_pairing.rs
```

Choose only the smallest subset that covers the touched risk surface. Do not run
all of these by habit.

## Host and platform rule

- Windows proof is expected where relevant.
- Android proof is expected where relevant, including emulator and the synced Samsung device when needed.
- Linux proof is expected where relevant via WSL or Docker.
- Real iOS and macOS proof is an external-platform constraint from this host and must be recorded that way rather than as a local blocker.

## Proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
03-platform-proof-status.md
16-validation-commands.log
17-blockers.md
```

## DONE rule

One workpack is DONE only after focused commands or blockers are recorded, proof artifacts exist under that workpack root, blocker categories are separated explicitly, and no runtime claim is closed by fake-green document checks alone.
