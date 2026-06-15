<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: README
> Kind: token-efficient entry point.
> Read when: `PLAN_INDEX.md`, `FEATURE_ROUTE_INDEX.md`, or a hub assignment selects this plan.
> Stop rule: Open `AGENTS.md`, then follow the local route. Do not scan sibling plans.
> Proves: route ownership only.
> Does not prove: shipped installer, deployed website, account auth, pairing, or production readiness.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan

This plan owns the end-to-end setup bootstrap graph from public family site to a paired household-ready state.

Use it for `family.ocentra.ca`, invite and code entry, parent install bootstrap, child install bootstrap, pairing, permissions, first-run readiness, degraded states, and recovery.

Do not use it for package internals, LAN protocol internals, portal component styling, or domain-specific enforcement. Those stay in their owning plans and are named only as handoffs.

Start with [AGENTS.md](AGENTS.md).
