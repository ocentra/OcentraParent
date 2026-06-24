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

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

## Authoritative Execution Workpacks

| ID | State | Next class | Current truth |
| --- | --- | --- | --- |
| `01` | partial | local proof | Contract/export surface exists in `packages/lan-domain`; Slice A repaired the LAN export surface and proof-schema contract path. |
| `02` | partial | local proof | Service-backed add-device read model and evidence rows exist; canonical household device records now persist in trusted-registry JSON with merged evidence timing, while fuller all-source evidence closure remains open. |
| `03` | partial | local proof | Shared Rust local-identity selection now captures gateway, DNS, DHCP, broadcast, and IPv6-prefix fields and persists them into scan-plan metadata; manual override plus ignored-interface reason-code proof remain open. |
| `04` | partial | local proof plus platform proof | Windows and Linux neighbor normalization now include IPv6 row representation plus malformed/duplicate parser proof; timestamped evidence shape, broader fixture corpus, and macOS/manual live proof remain open. |
| `05` | open | local implementation | Targeted ARP refresh is not implemented. |
| `06` | partial | local proof | Bounded active IPv4 host stimulation exists with gateway/router suppression, current-MAC-confirmed durable active-refresh suppression for stored child truth, durable service-probe suppression truth reuse, and persisted scan-plan metadata; packet IO abstraction and duplicate reply proof remain open. |
| `07` | open | local implementation | Passive collectors for ARP, DHCP, mDNS, SSDP, WS-Discovery, LLMNR, NetBIOS, and allowed SNMP responses are not implemented. |
| `08` | open/manual | local implementation plus packet proof | mDNS and DNS-SD implementation/proof is still open. |
| `09` | open/manual | local implementation plus packet proof | SSDP, UPnP, and bounded descriptor parsing proof is still open. |
| `10` | partial | local proof | NetBIOS, LLMNR, and reverse DNS are represented as weak evidence only. |
| `11` | open | local implementation | Light service probing is defined as curated weak evidence only; safe-port probing, sanitized HTTP/TLS hints, and no-crawl proof remain open. |
| `12` | partial | local proof | Rust MAC parsing, local OUI/vendor evidence, randomized/private warning, and multicast/malformed rejection now exist; richer classifier closure remains open. |
| `13` | partial | local proof | Household device spine/source merge exists; broader dedupe closure remains open. |
| `14` | partial | local proof | Explainable labels, authority, and status exist; weighted evidence-fusion classification and install-eligibility honesty remain open. |
| `15` | partial | local proof then physical/manual proof | Durable household device store/read-model work exists; trusted-registry JSON now persists canonical known-household records, prior-scan continuity and durable scan-suppression truth reuse are wired, active refresh now refuses stale IP-only suppression for reused addresses, stale restart read-model recovery is tested, and scan-plan sidecar metadata persists, while broader restart and physical/manual proof remain open. |
| `16` | partial | local proof then physical/manual proof | LAN read-model and diagnostics work exists; parent desktop now emits typed host-subscription route snapshots into the portal shell without a product UI WebSocket, while canonical replay/event proof remains open. |
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

## Selection Rules

- Select exactly one authoritative workpack from `01-20` unless the assignment explicitly says to inspect frozen follow-on context.
- If the selected workpack owner/proof family is unclear, classify it through `WORKPACK_FAMILIES.md`; do not scan every family.
- Do not raise status from B1/B2 proof outside their explicit non-claims.
- Do not use frozen `21-25` as current completion scope or PR_READY evidence.
- Do not claim physical household, portal, service/runtime, signed hello/heartbeat, router/firewall, Android/mobile, or relay readiness from schema, unit, source-matrix, B1, or B2 proof alone.
