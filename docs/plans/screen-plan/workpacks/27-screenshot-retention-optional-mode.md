# 27 Screenshot Retention Optional Mode

## Target State

Separate opt-in raw screenshot retention design exists with custody, TTL, disclosure, export, and delete proof.

## Current State

Raw screenshot retention is not default and not product-complete.

## Checklist

- [ ] Record product decision.
- [ ] Keep default `retainRawImage=false`.
- [ ] Define explicit opt-in setting if approved.
- [ ] Define custody and TTL.
- [ ] Define export/delete behavior.
- [ ] Define disclosure/audit.
- [ ] Add separate proof.

## Proof

- Feature/checklist update.
- Tests proving retention cannot silently enable.
