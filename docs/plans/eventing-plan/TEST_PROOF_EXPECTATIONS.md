<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Eventing Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Eventing Plan Test Proof Expectations

## Proof root

```text
output/eventing-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
cargo test -p ocentra-parent-eventing-core
cargo test -p ocentra-parent-agent-protocol event
npm run build --workspace @ocentra-parent/event-domain
npm run test --workspace @ocentra-parent/event-domain
npm run lint:architecture -- --files crates/eventing-core crates/agent-protocol packages/event-domain docs/plans/eventing-plan
```

## Required states

```text
envelope schema
idempotency
ordering/replay
retry/dead-letter
consumer contract
redaction
manual-required blockers
```
