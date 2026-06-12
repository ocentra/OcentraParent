# 05 Custody And Source Labels

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `05 Custody And Source Labels`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Platform docs require clear data source and custody labels. Package surfaces
must not imply hosted child activity storage.

## Where We Want To Be

Desktop states label live local, LAN, relay, parent cache, parent-owned storage,
Ocentra-hosted non-activity metadata, or unavailable.

## Decision Tree

| If the assignment touches... | Read next                                      | Required proof                         |
| ---------------------------- | ---------------------------------------------- | -------------------------------------- |
| Activity/evidence custody    | `../../data-custody-storage-plan/AGENTS.md`    | custody/export/delete/encryption proof |
| Local service or LAN source  | WP02/WP03 and selected service route           | source/freshness proof                 |
| Public account/metadata      | `../../account-identity-family-plan/AGENTS.md` | account/metadata boundary proof        |
| Support diagnostics          | WP11 support diagnostics and redaction         | redacted bundle proof                  |

## Required Labels

- `liveLocal`: generated on this machine from local service proof.
- `liveLan`: generated from LAN route proof for the selected device.
- `liveRelay`: generated through remote relay proof and explicit session grant.
- `parentCache`: locally cached parent-visible state; show age and source.
- `parentOwnedCloud`: parent-owned storage provider state; show provider and encryption/export status.
- `ocentraHostedMetadata`: account/subscription/release/support metadata only; not child activity/evidence.
- `manualRequired`: proof, permission, platform, or provider gap blocks claim.
- `unavailable`: no source is currently usable.

## Requirement Checklist

- [ ] Include source labels in command/proof output.
- [ ] Keep activity custody local/parent-owned by default.
- [ ] Label relay/cache unavailable states.
- [ ] Avoid hosted child data claims.
- [ ] Update docs if custody state changes.
- [ ] Show freshness/age and proof tier for each source.
- [ ] Prove support diagnostics redact secrets, tokens, and child-private data.
- [ ] Route cloud/export/delete questions to data custody instead of inventing desktop behavior.

## Acceptance And Proof

Package/runtime output makes custody source visible to parent surfaces and
support diagnostics.

Expected proof names:

- `parent-desktop.custody-label.live-local`
- `parent-desktop.custody-label.parent-cache`
- `parent-desktop.custody-label.parent-owned-cloud`
- `parent-desktop.custody-label.ocentra-hosted-metadata-boundary`
- `parent-desktop.custody-label.redaction-proof`
- `parent-desktop.custody-label.unavailable-manual-required`

Proof must include sample state rows, redacted diagnostic output, freshness/age fields, and explicit no-hosted-child-activity claim.

## Failure Conditions

- Do not imply Ocentra stores child activity/evidence unless a data-custody proof explicitly says so.
- Do not show cached/stale data as live.
- Do not expose account tokens, child identifiers, raw logs, raw screenshots, packet captures, or private evidence in diagnostics.

## Parallel Ownership Notes

C renders these labels; D proves package/runtime availability.
