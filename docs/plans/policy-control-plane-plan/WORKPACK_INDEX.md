# Policy Control Plane Workpack Index

Use this file to select exactly one workpack. Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not scan all workpacks by default.

| Workpack | Purpose | Status |
| ------------------------------------------------------------------------------- | --------------------------------------------- | ------- |
| [01-policy-source-of-truth](workpacks/01-policy-source-of-truth.md) | Typed policy source truth, versioning, ownership. | Checked |
| [02-parent-authoring-preview](workpacks/02-parent-authoring-preview.md) | Parent UI intent, templates, preview, confirmation. | Open / partial reviewed slice on PR [#615](https://github.com/ocentra/OcentraParent/pull/615); CI-active and not closure |
| [03-domain-policy-compilers](workpacks/03-domain-policy-compilers.md) | Domain compiler contracts and handoffs. | Checked |
| [04-delivery-ack-audit](workpacks/04-delivery-ack-audit.md) | Delivery, ack, retry, rollback, and audit. | Contract checked / runtime blocked |
| [05-ask-parent-overrides](workpacks/05-ask-parent-overrides.md) | Ask-parent, bonus time, overrides, approvals. | Open — parent-resolution contract/service/UI implementation and focused tests now exist; child delivery/artifact/adapter/proof remain open |
| [06-rollout-proof-and-route-gate](workpacks/06-rollout-proof-and-route-gate.md) | Proof pack, route sync, and no-overclaim gate. | Checked |
| [07-schedule-time-budget-conflict-model](workpacks/07-schedule-time-budget-conflict-model.md) | Timezone/DST, budgets, and conflict precedence. | Checked |
| [08-policy-event-model](workpacks/08-policy-event-model.md) | Event families, idempotency, replay, and audit linkage. | Checked |

## Status interpretation

```text
Checked: current route docs say the selected workpack has focused closeout artifacts under docs/proof/policy-control-plane-plan/ and matching validation notes.
Contract checked / runtime blocked: the policy-owned contract and negative proof are current, but a dependency-owned trusted adapter and real execution proof are still missing.
Open: workpack remains blocked or incomplete and cannot be promoted by related contract tests alone.
```

## Selection rules

- Select exactly one workpack.
- If owner/proof family is unclear, classify through `WORKPACK_FAMILIES.md`.
- Keep WP02 open until rendered authoring/preview/conflict/approval surfaces have targeted proof or explicit dependency blockers.
- Keep WP05 open until parent confirmation, assistant preview, child-agent validation, notification handoff, expiry, replay, and audit paths have targeted proof or explicit dependency blockers.
- Do not use compiler, event, or source-truth proof to close WP02/WP05.
- Do not claim full plan readiness while WP02 or WP05 remain open.
