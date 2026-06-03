# 17 - Degraded Timeout Invalid-Output Handling

## Target State

AI failure is visible, safe, auditable, and policy-consumable.

## Where We Are

Runtime status and provider states exist, but model failure handling must be
consistent across all AI routes.

## Checklist

- [ ] Define degraded reason codes.
- [ ] Map timeout to degraded/unknown.
- [ ] Map overload to degraded/unknown.
- [ ] Map invalid output to rejected/degraded.
- [ ] Map unavailable model to deterministic fallback.
- [ ] Record failure in journal/read model.

## Proof

- Timeout tests.
- Overload/unavailable tests.
- Invalid-output tests.
- Portal degraded-state screenshot if UI changes.
