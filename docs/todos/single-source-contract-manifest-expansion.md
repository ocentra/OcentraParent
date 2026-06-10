# Single-Source Contract Manifest Expansion

Status: TODO after the current merge wave.

## Why This Exists

PR #546 added reusable single-source contract guard machinery, but only one
concrete contract is currently registered: the network evidence drawer proof ids.
That prevents drift for the immediate failure, not for every route id, event id,
proof id, UI surface id, domain field name, or Effect Schema identity in the
repo.

## Follow-Up

- Inventory domain/proof identities that must have exactly one owner.
- Move duplicated literals into the owning domain package, protocol package, or
  proof fixture.
- Expand `scripts/check-single-source-contracts.json` with each owner contract.
- Keep `npm run lint:schema-boundaries` failing when app/runtime/docs copy those
  values instead of importing or referencing the owner.
- Include known duplicate-truth classes such as `activity` versus
  `network-activity`, eventing runtime versus eventing UI, and other
  route/id/schema identity splits.

## Completion Bar

Do not claim whole-repo single-source enforcement until the inventory is
complete, the manifest covers the selected contracts, and the guard proves it
rejects copied or locally redefined values.
