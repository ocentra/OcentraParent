# Workpack 03: Domain Policy Compilers

Goal: define how the control plane hands typed policy to domain owners.

Expected shape:

- Domain compilers produce target-specific policy artifacts for app/game, browser, network, tracking, screen, AI, and enforcement.
- Unsupported capability remains explicit, not silently ignored.
- Compilers are deterministic, versioned, and testable with fixtures.

Expected proof:

- Contract fixtures per domain.
- Unsupported/manual-required proof.
- Version compatibility proof.
- Consumer handoff notes.

Failure: parent policy directly manipulating domain runtime state without compiler/audit boundary.

## Execution Detail

Minimum context:

- `docs/plans/app-game-plan/AGENTS.md`
- `docs/plans/browser-plan/AGENTS.md`
- `docs/plans/network-plan/AGENTS.md`
- `docs/plans/tracking-plan/AGENTS.md`
- `docs/plans/screen-plan/AGENTS.md`
- `docs/plans/v0-8-enforcement-control-plan/AGENTS.md`

Required compiler outputs:

- App/game policy targets.
- Browser/site/social/video targets.
- Network/domain policy targets.
- Location/geofence policy targets.
- Screen/capture policy targets.
- AI evaluation policy context.
- Enforcement action hints with manual-required boundaries.

Rules:

- Unsupported capabilities stay explicit.
- Domain compiler output is deterministic and versioned.
- Enforcement authority remains in enforcement/domain plans.

Expected tests/proof names:

- `policy-compiler.app-game-fixture`
- `policy-compiler.browser-fixture`
- `policy-compiler.network-fixture`
- `policy-compiler.tracking-fixture`
- `policy-compiler.unsupported-manual-required`
- `policy-compiler.version-compat`

Proof artifact expectations:

- Fixture matrix.
- Domain handoff notes.
- Consumer contract references.
