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

This plan owns remote capability sessions: remote screen/live view, remote desktop/control, relay fallback, capability grants, consent/disclosure, remote input authority, abuse controls, and proof.

It does not own screen capture primitives, LAN discovery internals, account identity, or package build mechanics. Those remain handoffs.

Start with [AGENTS.md](AGENTS.md).
