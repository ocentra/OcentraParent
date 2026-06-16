<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Browser Plan Test Proof Expectations

## Proof root

```text
output/browser-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/browser-domain
npm run test --workspace @ocentra-parent/browser-domain
cargo test -p ocentra-parent-agent-protocol browser
cargo test -p ocentra-parent-agent-service browser
npm run test --workspace @ocentra-parent/portal -- browser
npm run lint:architecture -- --files packages/browser-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/browser-plan
```

## Required negative states

```text
unsupported browser visible
unmanaged browser visible
missing extension/app bridge visible
stale evidence visible
private content not exposed by default
mock data not product proof
UI cannot claim exact URL/feed visibility without source proof
```

## Failure conditions

- Do not mark DONE or PR_READY from happy-path-only proof.
- Do not store proof inventories inside this plan folder.
- Do not claim browser enforcement or content inspection unless the selected proof root proves it.
