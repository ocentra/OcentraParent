<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `Native Apps Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Native Apps Plan Test Proof Expectations

## Proof root

```text
output/app-plan-proof/<workpack-file-stem>/
```

## Common command families

```bash
cargo test -p ocentra-parent-agent-service
cargo test -p ocentra-parent-agent-protocol
npm run build --workspace @ocentra-parent/agent-protocol-domain
npm run test --workspace @ocentra-parent/agent-protocol-domain
npm run test --workspace @ocentra-parent/portal -- app
npm run lint:architecture -- --files crates/agent-service crates/agent-protocol packages/agent-protocol-domain apps/portal docs/plans/app-plan
```

## Required proof themes

Selected workpacks must state which apply:

```text
contract/schema proof
platform capability proof
service lifecycle proof
permission/degraded-state proof
install/package handoff proof
portal/request proof
negative case proof
manual-required gap proof
```

## Required negative states

```text
scaffold is not runtime support
package preview is not product readiness
missing OS permission is visible
unsupported platform is visible
portal cannot bypass service boundary
child-agent package/runtime claims remain in owning plan
```
