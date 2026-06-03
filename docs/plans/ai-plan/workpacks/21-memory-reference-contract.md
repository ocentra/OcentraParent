# 21 - Memory Reference Contract

## Target State

Memory refs are typed, local, source-cited, confidence-scored where needed, and
invalidatable.

## Where We Are

Local AI memory graph contracts and core activity memory graph pieces exist.
The source-citation guard must be enforced before memory influences decisions.

## Checklist

- [ ] Define memory reference kinds.
- [ ] Require source evidence refs.
- [ ] Include policy/action refs where applicable.
- [ ] Include generated time, expiry, confidence, and index version.
- [ ] Reject unsourced memory for decisioning.

## Proof

- Memory contract tests.
- Unsourced memory rejected test.
- Expired memory ignored test.
