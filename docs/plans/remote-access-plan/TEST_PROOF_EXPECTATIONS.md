<!-- agent-capsule -->

> Agent Capsule
> Plan: `remote-access-plan`
> Doc: `Remote Access Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Remote Access Plan Test Proof Expectations

## Proof root

```text
output/remote-access-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
cargo test -p ocentra-parent-agent-protocol remote
cargo test -p ocentra-parent-agent-service remote
npm run test --workspace @ocentra-parent/portal -- remote
npm run lint:architecture -- --files packages/screen-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/remote-access-plan
```

## Required negative states

```text
expired grant denied
revoked grant denied
wrong household denied
wrong role denied
missing device-trust handoff blocked
transport unavailable visible
manual stop visible
private payload not exposed by default
UI-only proof not product proof
```
