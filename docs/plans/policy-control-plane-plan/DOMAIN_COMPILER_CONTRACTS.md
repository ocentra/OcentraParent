# Domain Compiler Contracts

Domain compilers consume parent policy source documents and produce versioned domain-specific policy artifacts. They do not own parent policy truth.

## Required compiler inputs

```text
policy version
rules
targets
schedules
exceptions
time budgets
child/device scope
capability state
domain support matrix
custody/evidence requirements
```

## Required compiler outputs

```text
compiled artifact id
source policy version
domain
target refs
supported capability state
manual-required state
delivery target
rollback ref
audit refs
no-claim flags
```

## Required domain coverage

```text
app/game policy compiler
browser policy compiler
network/domain policy compiler
tracking/location/geofence policy compiler
screen policy compiler
AI evaluator context compiler
enforcement handoff compiler
notification/ask-parent compiler
```

## Compiler rules

- Compilers are deterministic and versioned.
- Unsupported capability must be explicit.
- Compilers cannot silently ignore unsupported targets.
- Compilers cannot mutate runtime directly.
- Compilers cannot claim enforcement.
- Compilers cannot claim UI delivery.
- Compilers cannot claim platform support without proof.
- Compiled artifacts are never source truth.

## Negative cases

```text
compiler silently drops unsupported target
compiler mutates runtime directly
compiler output becomes source truth
same input produces different output
compiled artifact lacks source policy version
compiled artifact lacks rollback ref
manual-required state missing
```

## Proof expectation

The compiler contract closes only when the proof inventory shows the compiler matrix, fixtures, unsupported/manualRequired handling, version compatibility, deterministic output, and no-runtime-mutation behavior.
