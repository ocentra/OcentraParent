# Policy Feature Expectations

Policy features define what parents want the local child-device AI evaluator to allow, limit, warn about, block, or send back for parent approval.

## Expected Deliverables

- Parent/family/child/device contracts where needed.
- Policy rule contracts.
- Schedule/time-window contracts.
- Category/app/site/domain target contracts.
- Permission request contracts.
- Decision reason codes.
- Local AI decision input and output contracts when the policy is context-heavy.
- Dry-run evaluator before enforcement.

## Acceptance

- Invalid rules fail schema validation.
- Conflicting rules have deterministic resolution.
- Policy decisions reference evidence.
- Policy decisions reference the local AI output when AI contributed.
- Decision events are journaled.
- Parent-facing explanation is stable and testable.
- Dry-run mode can explain what would happen without enforcing it.
- Explicit parent rules override ambiguous AI output.

## Non-Goals

- Do not enforce policy until the evaluator is trusted.
- Do not make untyped or untraceable AI the source of a policy decision.
- Do not mix billing entitlements into policy logic.

## Done Signal

Given real or contract-valid activity evidence, parent rules, and local AI output where needed, the evaluator returns a deterministic typed decision with reason codes and evidence references. Tests cover allow, limit, block, ask-parent, unknown, and conflict behavior.
