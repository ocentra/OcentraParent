<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `RESEARCH_AND_UI_GUIDANCE.md`
> Kind: research and UI guidance.
> Read when: When a workpack needs the combined research anchors and UI contract in one place.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file is guidance, not a claim that the platform work is already done.

<!-- /agent-capsule -->

# Data Custody Storage Plan Research And UI Guidance

## Purpose

This plan must become an execution contract. It should tell future agents what to build, what not to build, which docs to research, which states to expose, which events to emit, which proofs to create, and which claims must remain false until proven.

## Primary research anchors

- Windows: DPAPI, service account and user profile behavior, filesystem ACLs, Credential Manager only if secret storage is being considered.
- Android: Android Keystore, app-specific storage, backup exclusion, service limitations, OWASP MASVS Android storage and crypto checks.
- Apple: Keychain, Secure Enclave where applicable, iCloud Drive or CloudKit when provider work starts, backup and app group behavior, OWASP MASVS iOS storage and crypto checks.
- Linux: Secret Service, libsecret, GNOME Keyring, KWallet, systemd user and service behavior.
- Provider storage: Google Drive appDataFolder, Drive file or folder picker, OneDrive app folder and selected folder, iCloud Drive app container or selected location, Dropbox app folder, NAS and local folder.
- Security baseline: OWASP MASVS storage, crypto, auth, network, platform, code, privacy, and resilience sections.

## Core product rule

Ocentra-hosted infrastructure is not the default child-data store.

Allowed Ocentra-hosted metadata:

- account identity metadata
- subscription, billing, and entitlement metadata
- license, download, and update metadata
- device registration and pairing route metadata
- minimal notification routing metadata
- short-lived report compiler status
- support case metadata
- public website and release status

Disallowed by default:

- raw child evidence
- encrypted journal segments
- SQLite evidence/read-model databases
- screenshots or screen-analysis images
- browser URL history
- network/app/game/tracking evidence
- parent rules and approval history as source of truth
- generated long-term reports
- assistant child-evidence context
- parent-owned storage contents
- provider sync payloads
- support bundles containing raw child activity
- universal decrypt keys

## Correct custody mental model

Household-owned data is readable only by authorized household devices or components that hold valid role, pairing, and key material.

The trust domain may include:

- child device agent
- parent desktop portal
- parent mobile app
- future co-parent device
- future recovery device
- parent-owned cloud storage containing encrypted bundles

## Provider and UI guidance

- Keep hidden app-managed backup and parent-visible folder modes separate.
- Keep backup destination, export format, sync schedule, retention, restore permission, and human-readable report export separate.
- Never say backed up, synced, restored, deleted, encrypted, private, safe, or complete unless the corresponding proof state exists.
- Show explicit manual-required, provider error, provider revoked, quota exceeded, and offline queued states.
- Restore must be retrieve -> preview -> confirm apply.
- Delete must distinguish local delete, provider delete, disconnect, support bundle delete, and Ocentra metadata delete.
- Parent mobile can view storage status, connect provider accounts, approve authorization, view backup health, request export or import preview, and confirm apply-back after re-auth when required.
- Parent desktop is the primary near-term storage setup surface.
- Child mobile and child service support remain manual-required until platform proof exists.

## Workpack research hints

| Workpack | Research focus |
| --- | --- |
| WP01 | Which data exists already, which is planned, which is derived, which is local source of truth, and which may never be hosted by default. |
| WP02 | Who needs decrypt access, what survives reinstall, what survives revocation, and when recovery must stay manual-required. |
| WP03 | Hidden app backup versus parent-visible folder, metadata leakage, revocation, quota, and provider delete behavior. |
| WP04 | Delete as propagation, tombstone lifetime, replay protection, and what must stay as minimal audit only. |
| WP05 | Export, retrieve, preview, and apply as separate operations with wrong-household and wrong-key rejection. |
| WP06 | Report, query, AI, and notification derived truth, citation rules, and leak prevention. |
| WP08 | Parent storage choices, state cards, restore preview, delete and disconnect flow, and no-claim copy. |

## Required UI docs and artifacts

- `docs/plans/data-custody-storage-plan/UI_EXPECTATIONS.md`
- `docs/plans/data-custody-storage-plan/PARENT_STORAGE_PROVIDER_MATRIX.md`
- `docs/plans/data-custody-storage-plan/PLATFORM_KEY_CUSTODY_MATRIX.md`
- `docs/plans/data-custody-storage-plan/PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`
- `docs/plans/data-custody-storage-plan/RESEARCH_AND_UI_GUIDANCE.md`

## Final instruction

Do not leave this plan as abstract privacy writing. It must remain an execution contract.
