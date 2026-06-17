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
cargo test -p ocentra-eventing --test unit
cargo test -p ocentra-eventing --test contract
cargo test -p ocentra-eventing --test journal_replay
cargo test -p ocentra-eventing --test integration
cargo test -p ocentra-eventing --test version_skew
cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests
npm run test --workspace @ocentra-parent/event-domain
npm run type-check --workspace @ocentra-parent/event-domain
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts
cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet
npm run type-check --workspace @ocentra-parent/agent-protocol-domain
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- policy-control-audit-redaction.test.ts policy-control-delivery-read-model.test.ts contracts.test.ts
npm run lint:architecture -- --files packages/agent-protocol-domain/src/contracts.ts packages/agent-protocol-domain/src/policy-control-audit-redaction.ts packages/agent-protocol-domain/src/policy-control-delivery-read-model.ts
node scripts/test/eventing-rollout-proof.mjs
git diff --check -- docs/proof/eventing-plan docs/plans/eventing-plan
```

## Current local proof roots

```text
docs/proof/eventing-plan/PLAN_PROOF_MANIFEST.md
docs/proof/eventing-plan/slice-01-envelope-version.md
docs/proof/eventing-plan/slice-02-ordering-replay.md
docs/proof/eventing-plan/slice-03-consumer-boundary.md
output/eventing-plan-proof/rollout-proof/
test-results/eventing-rollout-proof/
output/eventing-plan-proof/13-test-folder-layout-regression-audit/
test-results/eventing-test-folder-layout-regression-audit/
output/eventing-plan-proof/63-type-safety-source-gate/
test-results/eventing-type-safety-source-gate-proof/
output/eventing-plan-proof/66-76-source-safety/
output/eventing-plan-proof/67-lock-await/
output/eventing-plan-proof/68-fixture-parity/
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
proof-root presence
WP12 rollout-proof route restored without PR_READY claims
WP13 source-side test scaffold cleanup locally proved
WP11 scoped proof roots restored locally, package-wide agent-protocol-domain type-check passes again, and focused policy-control plus contracts validation is green
WP10 remains open until its proof roots and blocking validation exist
```
