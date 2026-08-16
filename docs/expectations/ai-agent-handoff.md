<!-- agent-capsule -->

> Agent Capsule
> Doc: AI Agent Handoff Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# AI Agent Handoff Expectations

AI agents should leave the next agent with exact context, not a vague story.

## Start-Of-Feature Checklist

When an AI agent starts a feature, it should identify:

- Current branch and worktree cleanliness.
- Owning feature doc from `docs/feature-list.md`.
- Relevant roadmap milestone.
- Feature expectation files linked by the owning feature doc that apply.
- Relevant `docs/product-capability-checklist.md` row.
- Relevant module README for every touched app, package, crate, or platform.
- Whether the current work is local-only, pushed feature branch work, PR integration work, or explicit product release work.
- Existing contracts and tests.
- Exact focused gates for iteration.
- Final gate before merge.
- Product claims being added or changed.

The agent should not bulk-read unrelated feature, expectation, or checkpoint
docs. Historical checkpoints are read only when the feature doc, capability
checklist, roadmap, or hub assignment names them as proof.

## End-Of-Feature Report

When an AI agent finishes a feature, it should report:

- What changed.
- What parent-visible behavior exists now.
- What product claim is now true.
- What remains intentionally out of scope.
- Which owning feature doc changed, or why no feature-doc update was required.
- Which capability checklist row changed, or why no checklist update was
  required.
- Whether roadmap, expectation docs, module README, README, or competitor map
  changed.
- Exact validations run.
- Current git state.
- Whether anything was pushed, PR'd, merged, or released.

## Blocker Handling

If blocked, report:

- The exact blocker.
- Whether it is technical, account/credential, legal/product, platform-policy, or destructive-action related.
- What independent work can continue.
- What decision or access is needed.

## Done Signal

Another agent can resume without rediscovering branch state, product intent, active milestone, validation state, or known blockers.
