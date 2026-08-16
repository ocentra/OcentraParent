# Policy Conflict Precedence

Conflicts must resolve deterministically or produce explicit `manualRequired`. Silent last-write-wins is forbidden unless the choice is recorded as a conscious, versioned decision.

## Default precedence order

```text
emergency/safety block
parent owner explicit block
time budget exceeded
active override / bonus time
parent owner explicit allow
co-parent rule if authorized
template/default rule
domain unavailable/manualRequired
unknown
```

## Required model behavior

- Precedence must be explicit and testable.
- Conflicts must show the reason, target, scope, and version involved.
- A domain that cannot resolve safely must surface `manualRequired`.
- Conflict resolution must preserve audit refs and rollback refs.

## Negative cases

```text
conflict silently last-write-wins
manual-required conflict auto-applies
domain-specific default hides a higher-priority block
resolved conflict loses audit ref
resolved conflict cannot be rolled back
```

## Proof expectation

The conflict model is closed when the proof inventory shows deterministic precedence, manualRequired fallback, and rollback-aware audit references.
