<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# AI Plan Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

For a selected workpack, use proof root:

```text
output/ai-plan-proof/<workpack-file-stem>/
```

## Common command families

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/ai-domain
npm run test --workspace @ocentra-parent/ai-domain
npm run build --workspace @ocentra-parent/text-domain
npm run test --workspace @ocentra-parent/text-domain
cargo test -p ocentra-parent-agent-protocol ai
cargo test -p ocentra-parent-agent-service ai
npm run test --workspace @ocentra-parent/portal -- ai
npm run lint:architecture -- --files packages/ai-domain packages/text-domain packages/evidence-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/ai-plan
```

## Required proof themes

Any selected workpack must state which of these apply:

```text
contract/schema proof
fixture/replay proof
local no-model deterministic proof
provider adapter proof
timeout/degraded proof
invalid output proof
redaction/custody proof
source-reference/citation proof
journal/replay proof
parent-visible UI proof
performance/resource proof
security/privacy proof
```

## Required negative states

```text
invalid provider output rejected
timeout returns degraded/manual-required state
missing evidence refs block claim
private payload not included in prompt/output/logs
mock provider does not prove product readiness
memory/reference without source proof is rejected
assistant action cannot bypass policy/account/device authority
model unavailable state is visible
```

## Failure conditions

- Do not mark DONE or PR_READY until code, tests, validation, and proof are complete for the selected slice.
- Do not store proof inventories inside this plan folder.
- Do not claim AI product readiness from docs-only work, mock-only proof, or happy-path-only tests.
