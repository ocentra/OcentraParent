# AI Agent Handoff Expectations

AI agents should leave the next agent with exact context, not a vague story.

## Start-Of-Feature Checklist

When an AI agent starts a feature, it should identify:

- Current branch and worktree cleanliness.
- Relevant roadmap milestone.
- Feature expectation sections that apply.
- Existing contracts and tests.
- Exact focused gates for iteration.
- Final gate before merge.
- Product claims being added or changed.

## End-Of-Feature Report

When an AI agent finishes a feature, it should report:

- What changed.
- What parent-visible behavior exists now.
- What product claim is now true.
- What remains intentionally out of scope.
- Exact validations run.
- Current git state.

## Blocker Handling

If blocked, report:

- The exact blocker.
- Whether it is technical, account/credential, legal/product, platform-policy, or destructive-action related.
- What independent work can continue.
- What decision or access is needed.

## Done Signal

Another agent can resume without rediscovering branch state, product intent, active milestone, validation state, or known blockers.
