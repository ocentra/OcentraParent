# WP195 App/game platform proof status preflight detail refs

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP195 App/game platform proof status preflight detail refs`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Carry the latest Windows and Android platform preflight evidence into the shared
app/game platform proof status read model instead of leaving those proof rows as
separate one-off artifacts.

This workpack keeps app and game under the same low-level evidence spine:

- Windows broad-blocking authority preflight refs become parent-visible status
  refs and open gaps.
- Android Device/Profile Owner authority preflight refs become parent-visible
  status refs without treating `not-proved` as authority.
- Android Accessibility overlay preflight refs become parent-visible status
  refs without leaking service/component names or claiming overlay runtime.
- Linux WSL runtime and foreground-capture readiness continue to use the same
  status surface.

## Non-Goals

- No AppLocker/App Control enforcement claim.
- No Android Device Owner/Profile Owner enrollment claim.
- No Android Accessibility overlay execution claim.
- No Linux active foreground capture or policy enforcement claim.
- No macOS/iOS local runtime claim from this Windows host.
- No adapter dispatch, provider delivery, child-device delivery, raw target,
  raw path, raw service-name, or private diagnostic claim.

## Files

- `packages/parent-domain/src/app-game-platform-proof-status.ts`
- `packages/parent-domain/tests/app-game-platform-proof-status.test.ts`
- `scripts/test/app-game-platform-proof-status-proof.mjs`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/features/app-game-control.md`

## Validation

Focused validation should run the platform proof status test/proof chain and the
shared hygiene gates:

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-platform-proof-status`
- `cmd /c node --check scripts/test/app-game-platform-proof-status-proof.mjs`
- `cmd /c node scripts/test/app-game-platform-proof-status-proof.mjs`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## Done Criteria

- Platform proof status rows include Windows, Android, and Linux.
- Android status refs include authority and Accessibility overlay preflight refs
  when supplied.
- Windows status refs include broad-blocking authority preflight refs when
  supplied.
- Open gaps remain explicit for Windows broad blocking, Android authority,
  Android overlay runtime, Linux foreground/policy mechanisms, and
  cross-platform child delivery.
- Summary counts match rows and enforcement-ready count remains zero.
