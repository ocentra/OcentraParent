# Policy Control Plane Route Index

## Read route

1. Read [AGENTS.md](AGENTS.md).
2. Read [PLAN_STATE.md](PLAN_STATE.md).
3. Read [NEXT_ACTIONS.md](NEXT_ACTIONS.md).
4. Read [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
5. Read [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner/proof family is unclear.
6. Open exactly one selected workpack.
7. Read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) before choosing tests.
8. Read [PROOF_INDEX.md](PROOF_INDEX.md) and [PROOF_AND_TEST_INVENTORY.md](PROOF_AND_TEST_INVENTORY.md) before claiming proof.

## Owns

```text
policy source truth
parent authoring/preview contract
schedule/timezone/DST/time-budget semantics
conflict and precedence model
domain compiler handoff contracts
delivery/ack/audit contract
ask-parent/override contract
event family and audit linkage
rollout proof route
```

## Boundary split

```text
schema-domain owns canonical shared policy shapes.
policy-domain is a legacy/package anchor or proof-consumer unless public exports exist.
policy-control-core owns Rust control-plane helper behavior.
portal plans own rendered UI surfaces.
account plan owns actor/session/role authority.
device-trust plan owns parent presence and step-up gating.
data-custody plan owns export/delete/sync/retention custody.
eventing plan owns reusable event mechanics.
domain plans own runtime effects.
enforcement plan owns enforcement action authority and rollback.
AI plan owns AI runtime/draft behavior; policy accepts only typed, confirmed actions.
```

## Stop rule

- Open only one workpack at a time unless the chosen workpack names a handoff to another workpack or plan.
- Domain policy effect work routes to the owning domain only after this plan defines source truth and handoff.
- Do not treat UI preview, compiler output, event model, assistant draft, or focused contract tests as full policy readiness.
- WP02 and WP05 remain open until their dependency-owned surfaces have targeted proof or explicit blockers.
