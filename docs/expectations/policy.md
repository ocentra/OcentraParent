# Policy Feature Expectations

Policy features define what parents want the system to allow, limit, warn about, or block.

## Expected Deliverables

- Parent/family/child/device contracts where needed.
- Policy rule contracts.
- Schedule/time-window contracts.
- Category/app/site/domain target contracts.
- Permission request contracts.
- Decision reason codes.
- Dry-run evaluator before enforcement.

## Acceptance

- Invalid rules fail schema validation.
- Conflicting rules have deterministic resolution.
- Policy decisions reference evidence.
- Decision events are journaled.
- Parent-facing explanation is stable and testable.
- Dry-run mode can explain what would happen without enforcing it.

## Non-Goals

- Do not enforce policy until the evaluator is trusted.
- Do not make AI the only source of a policy decision.
- Do not mix billing entitlements into policy logic.

## Done Signal

Given real or contract-valid activity evidence, the evaluator returns a deterministic decision with reason codes and evidence references, and tests cover allow, limit, block, unknown, and conflict behavior.
