# Schedule Time Budget Model

This document defines the schedule and time-budget semantics that policy control depends on.

## Core model

Every schedule and time budget must carry:

```text
timezone id
local start/end
recurrence rule
exception dates
holiday/school-night/weekend mode if used
budget window
reset rule
grace period
effective start/end
expiry
clock source
DST ambiguous/nonexistent time behavior
```

## Required behavior

- Timezone is explicit; local times are never ambiguous by default.
- Recurrence and exceptions are modeled separately.
- Budget reset and carryover semantics are explicit.
- Grace periods and bonus time expire.
- Device clock source and recovery behavior are explicit.
- DST spring-forward and fall-back behavior is defined, not implied.
- Offline device recovery must not silently change policy meaning.

## Conflict and precedence hooks

- Schedule conflicts must be resolved by the policy conflict model, not by hidden local defaults.
- Bonus time, override windows, and exceptions need explicit precedence.
- ManualRequired is a valid output when the local time state cannot be trusted.

## Negative cases

```text
DST skip grants wrong time
DST repeat grants double time
clock skew bypasses bedtime
timezone missing
bonus time never expires
offline child loses timer state
conflict silently last-write-wins
manual-required conflict auto-applies
```

## Proof expectation

The schedule model is closed when the proof inventory covers timezone semantics, DST boundaries, budget reset, conflict precedence, bonus-time expiry, and offline timer recovery.
