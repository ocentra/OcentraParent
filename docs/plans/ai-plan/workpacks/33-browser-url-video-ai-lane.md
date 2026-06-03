# 33 - Browser URL Video AI Lane

## Target State

Managed browser URL, title, metadata, transcript summaries, and video refs feed
AI as typed evidence. AI does not read browser state directly.

## Where We Are

Browser plan owns URL/video AI planning. This workpack links that slice into the
shared AI queue/context/result/policy spine.

## Checklist

- [ ] Consume managed browser evidence only.
- [ ] Add URL/video context mapping.
- [ ] Add deterministic URL/platform classification first.
- [ ] Route ambiguous cases to local text model.
- [ ] Return schema-valid category/support result.
- [ ] Feed parent policy, not direct enforcement.

## Proof

- Browser URL AI dry-run test.
- Video metadata classification test.
- Unmanaged browser exact-URL rejection test.
