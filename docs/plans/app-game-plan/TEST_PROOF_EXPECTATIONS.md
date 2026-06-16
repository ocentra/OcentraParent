<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App Game Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# App Game Plan Test Proof Expectations

## Proof root

```text
output/app-game-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/app-game-domain
npm run test --workspace @ocentra-parent/app-game-domain
cargo test -p ocentra-parent-agent-protocol app_game
cargo test -p ocentra-parent-agent-service app_game
npm run test --workspace @ocentra-parent/portal -- app
npm run lint:architecture -- --files packages/app-game-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/app-game-plan
```

## Required negative states

```text
unsupported platform visible
unknown app/game state visible
stale evidence visible
manual-required state visible
mock evidence not product proof
historical checked row not used as new proof
UI cannot claim runtime action without service/protocol proof
```

## Failure conditions

- Do not mark DONE or PR_READY from happy-path-only proof.
- Do not store proof inventories inside this plan folder.
- Do not use generated long-name handoff rows as implementation scope without a fresh selected proof target.
