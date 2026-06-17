# LAN Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Workpack Index`
> Kind: authoritative workpack selector.
> Read when: before opening any LAN workpack.
> Stop rule: use only one authoritative workpack from `01-20`.
> Proves: current execution model and workpack routing only.
> Does not prove: row completion or physical proof by itself.
> Proof rule: if a workpack state changes here, the checklist and proof index must match it.

<!-- /agent-capsule -->

## Authoritative Execution Workpacks

| ID | State | Next class | Current truth |
| --- | --- | --- | --- |
| `01` | partial | local proof | Contract/export surface exists in `packages/lan-domain`; Slice A repaired the LAN export surface and proof-schema contract path. |
| `02` | partial | local proof | Service-backed add-device read model and evidence rows exist; canonical all-source evidence store remains open. |
| `03` | partial | local proof | Selected-interface requirements are modeled; full adapter/interface proof remains open. |
| `04` | partial | local proof plus platform proof | Windows neighbor-table path exists; Linux/mac normalization is still open or manual-required. |
| `05` | open | local implementation | Targeted ARP refresh is not implemented. |
| `06` | open | local implementation | Bounded ARP sweep is not implemented. |
| `07` | open | local implementation | Passive packet listeners are not implemented. |
| `08` | open/manual | local implementation plus packet proof | mDNS and DNS-SD implementation/proof is still open. |
| `09` | open/manual | local implementation plus packet proof | SSDP and UPnP implementation/proof is still open. |
| `10` | partial | local proof | NetBIOS, LLMNR, and reverse DNS are represented as weak evidence only. |
| `11` | open | local implementation | Light service probing is not implemented. |
| `12` | partial | local proof | OUI/vendor data is represented as weak/manual-required only. |
| `13` | partial | local proof | Household device spine/source merge exists; broader dedupe closure remains open. |
| `14` | partial | local proof | Explainable labels, authority, and status exist; full classifier closure remains open. |
| `15` | partial | local proof then physical/manual proof | Durable household device store/read-model work exists; restart and physical proof remain open. |
| `16` | partial | local proof then physical/manual proof | LAN read-model and diagnostics work exists; replay/event proof remains open. |
| `17` | open | local implementation plus packet proof | Parent/child mDNS advertisements are not implemented. |
| `18` | partial/manual | physical/manual proof bound | Signed hello/heartbeat contract rows exist; real signed artifacts are missing. |
| `19` | partial | local proof then physical/manual proof | Assignment/revoke/audit surfaces exist; restart and negative-route proof remain open. |
| `20` | partial/manual | local proof then physical/manual proof | B1 regenerated the local proof pack under `output/lan-plan-proof/01-lan-b1-proof-regeneration/`; B2 keeps LAN test-category truth honest by treating placeholder test folders as non-coverage. |

## Frozen Follow-On Workpacks

These rows remain in the folder as draft follow-on material only. They are not authoritative for current `lan-plan` completion.

| ID | State | Current truth |
| --- | --- | --- |
| `21` | frozen | follow-on contract/domain-schema rewrite material; not part of current executable model |
| `22` | frozen | follow-on current-state and gap-map rewrite material; not part of current executable model |
| `23` | frozen | follow-on pairing and route proof rewrite material; not part of current executable model |
| `24` | frozen | portal/first-run handoff work; belongs to later household/setup/account execution, not current LAN completion |
| `25` | frozen | later rollout/PR gate wrapper; not part of current authoritative LAN workpack model |
