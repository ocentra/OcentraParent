# WP10 LAN Household Mesh Consumer

Scope: define eventing obligations for household mesh import/export without turning the local event bus into a remote shared bus.

Source rows: `05-implementation-workpacks.md` rows 79-87.

Read next:

- `../../lan-plan/AGENTS.md`
- `../../remote-access-plan/AGENTS.md` only if relay/remote access is the assignment
- `../05-implementation-workpacks.md` rows 79-87 only

Expected outcome:

- Household mesh bridge boundary is explicit.
- Selected event export/import uses typed LAN message envelopes and validates custody, source, family, idempotency, and authority.
- Incoming bridge messages republish locally only after validation.
- Provider advertisement, heartbeat, AI work claim/lease/result, and child-agent AI work ledger events have owner boundaries.
- Provider or peer devices cannot publish policy/enforcement events directly.

Expected tests/proof:

- `eventing.mesh.selected-event-export-import`
- `eventing.mesh.incoming-validation-negative`
- `eventing.mesh.no-remote-direct-publish`
- `eventing.mesh.cross-device-idempotency`
- `eventing.mesh.provider-cannot-policy-enforce`
- Proof includes LAN plan workpack reference, replay/duplicate case, stale message case, and rejected authority case.

Failure conditions:

- Do not claim remote relay, mobile parity, or cloud delivery here.
- Do not trust provider/peer claims without account/device authority proof.
- Do not bypass local validation before republishing imported events.

Proof:

- `output/eventing-plan-proof/12-household-mesh-consumer/proof-summary.json`
- `test-results/eventing-household-mesh-consumer-proof/proof.json`
