# Policy Feature Expectations

Policy features define what parents want the local child-device AI evaluator to allow, limit, warn about, block, or send back for parent approval.

Parent surfaces author policy rules and expectations. The child-device agent owns policy evaluation, conflict resolution, local AI integration, timers, and enforcement handoff.

## Expected Deliverables

- Parent/family/child/device contracts where needed.
- Policy rule contracts.
- Parent rule-authoring contracts separated from child-device evaluator contracts.
- Schedule/time-window contracts.
- Category/app/site/domain target contracts.
- Permission request contracts.
- Decision reason codes.
- Local AI decision input and output contracts when the policy is context-heavy.
- Dry-run evaluator before enforcement.

## Acceptance

- Invalid rules fail schema validation.
- Portal-authored rules are inert configuration until the child-device agent validates and evaluates them.
- Conflicting rules have deterministic resolution.
- Policy decisions reference evidence.
- Policy decisions reference the local AI output when AI contributed.
- Decision events are journaled.
- Parent-facing explanation is stable and testable.
- Dry-run mode can explain what would happen without enforcing it.
- Explicit parent rules override ambiguous AI output.

## Non-Goals

- Do not enforce policy until the evaluator is trusted.
- Do not evaluate or enforce policy in the portal/browser.
- Do not make untyped or untraceable AI the source of a policy decision.
- Do not mix billing entitlements into policy logic.

## Done Signal

Given real or contract-valid activity evidence, parent rules authored through typed contracts, and local AI output where needed, the child-device evaluator returns a deterministic typed decision with reason codes and evidence references. Tests cover allow, limit, block, ask-parent, unknown, and conflict behavior.
