<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `DECISIONS.md`
> Kind: architecture decision record.
> Read when: When a workpack needs the current custody decisions and open decisions.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Decisions here must be reflected in the matching classification, key, provider, bundle, event, and UI docs.

<!-- /agent-capsule -->

# Data Custody Storage Plan Decisions

## Canonical decisions

| Decision | Current direction | Status | Proof needed |
| --- | --- | --- | --- |
| Default child-data store | Ocentra-hosted infrastructure is not the default child-data store. Household-owned local or parent-owned storage is the default custody path. | Decided | Custody matrix and route consistency proof |
| Encryption before upload | Any provider sync, backup, or export of sensitive child data must be encrypted before it leaves the custody boundary. | Decided | Bundle and sync proof |
| Restore semantics | Restore is always retrieve -> preview -> confirm apply. Provider data must never auto-apply. | Decided | Bundle and UI flow proof |
| Delete semantics | Delete is tombstone-first, replay-safe, and must block resurrection from old bundles or stale sync. | Decided | Retention/delete proof |
| Provider mode split | Hidden app-managed backup and parent-visible folder modes are different custody models and must not be conflated. | Decided | Provider matrix proof |
| UI language | The UI must not claim backed up, synced, restored, deleted, encrypted, private, safe, or complete without proof for that claim. | Decided | UI proof and copy review |
| Parent storage flow | Parent storage setup and restore preview live in the portal UX surface, but the custody rules come from this plan. | Decided | UI handoff proof |

## Open decisions

| Open question | Why it is still open | Current guardrail |
| --- | --- | --- |
| Zero-knowledge versus recoverable support | Product has not chosen whether support can ever help recover encrypted payloads. | Default to manual-required and no support decrypt path. |
| Provider defaults | Google Drive, OneDrive, iCloud, local folder, and NAS do not share the same custody tradeoffs. | Keep mode choice explicit. |
| Linux key store | The actual secret-store or keyring choice is not final. | Mark Linux manual-required until chosen. |
| Android and iOS proof level | Mobile key and restore claims need platform proof. | Treat mobile support as limited and manual-required until proven. |
| Parent deletion ergonomics | One-step versus staged delete/disconnect behavior is still a product choice. | Separate disconnect from delete and require explicit confirmation. |

## Claims that remain false until proof exists

- Ocentra is the storage owner for child evidence.
- A provider folder choice is the same as hidden app backup.
- Provider data can auto-apply into local truth.
- Delete is complete if a UI row disappears.
- Support can decrypt parent-held child evidence by default.

