<!-- agent-capsule -->

> Agent Capsule
> Plan: `remote-access-plan`
> Doc: README
> Kind: token-efficient entry point.
> Read when: remote desktop, remote screen, relay, remote input/control, or remote capability grants are in scope.
> Stop rule: Open AGENTS, then one workpack.
> Proves: route ownership only.

<!-- /agent-capsule -->

# Remote Access Plan

This plan owns remote capability sessions: remote screen/live view, relay fallback, capability grants, standing paired access, revocation, abuse controls, and proof.

Remote input/control is a deferred extension. The current pass is live view plus standing paired access, not repeated permission prompts.

It does not own screen capture primitives, LAN discovery internals, account identity, or package build mechanics. Those remain handoffs.

Start with [AGENTS.md](AGENTS.md).
