# Dispatch Packet Template

Use this exact packet when assigning a thread. Do not dispatch from a vague sentence.

```text
Thread: <plan/thread>
Assigned slice: <one slice only>
Read scope: R0/R1/R2/R3/R4/R5 from READ_SCOPE_BUDGET.md
Validation budget: V0/V1/V2/V3/V4/V5 from VALIDATION_BUDGET_LADDER.md
Allowed paths:
- <exact file or subtree>
Forbidden paths:
- <exact file or subtree>
Owner boundary:
- owns: <contracts/events/read models>
- consumes: <contracts/events/read models>
- must not own: <runtime/product behavior>
Event/request chain:
- producer:
- consumer:
- schema/contract:
- read model/result:
Logging/proof chain:
- run id source:
- correlation id source:
- log points:
- proof artifacts:
Tests/proof:
- required checks:
- skipped higher validation:
Stop conditions:
- <condition>
Report back:
- files changed
- commands run
- validation level
- proof outputs
- blockers
- next proposed slice
```

## Packet rules

- One packet assigns one slice only.
- Allowed paths must be exact enough to prevent broad package edits.
- Forbidden paths must include any shared frontage package not assigned to the worker.
- V4/V5 validation must be explicitly granted by the lane manager.
- Cross-owner chains must name producer, consumer, schema, and read model.
- If the worker discovers the packet is wrong, it stops and reports; it does not widen itself.
