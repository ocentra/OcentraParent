# 09 - Local Evidence Context Builder V1

## Target State

The context builder assembles the smallest relevant stored evidence window,
parent rules, runtime refs, prompt refs, and evidence-backed memory refs.

## Where We Are

The architecture spec exists and parent-domain context builder contracts exist.
The next proof must use real stored evidence and rules.

## Checklist

- [ ] Build from SQLite/read-model evidence.
- [ ] Include parent rule context.
- [ ] Include runtime/provider refs.
- [ ] Include prompt/template version.
- [ ] Return ready, partial, insufficient, unavailable, or rejected.
- [ ] Reject raw sources and invalid custody.

## Proof

- Stored-evidence integration test.
- Context minimization test.
- Custody rejection test.
