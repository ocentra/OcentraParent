# Workpack 01: Custody Source of Truth

Goal: define every data class, its owner, and its source of truth.

Context to read:

- `docs/plans/data-custody-storage-plan/DECISIONS.md`
- `docs/plans/data-custody-storage-plan/DATA_CLASSIFICATION.md`
- `docs/plans/data-custody-storage-plan/RESEARCH_AND_UI_GUIDANCE.md`
- `docs/expectations/data-custody.md`
- `docs/expectations/evidence-storage.md`
- `docs/features/evidence-store-query.md`
- `docs/features/reports-notifications-sync.md`

In scope:

- Data taxonomy for account, household, child profile, device, policy, evidence, reports, notifications, logs, diagnostics, billing references, setup state, AI outputs, screenshots, location, network/app/browser events, provider payloads, and support bundles.
- Custody authority for parent-owned, local-only, parent-cloud-owned, Ocentra relay/transient, Ocentra account metadata, and provider-owned billing identity.
- Sensitive data classes, redaction obligations, and forbidden-hosting boundaries.
- Current truth path for each data class.

Out of scope:

- Key custody.
- Provider selection and sync mechanics.
- Delete/tombstone protocol.
- UI implementation.

Acceptance:

- Every active data class has a source-of-truth row.
- Ocentra-hosted-by-default and must-never-host-by-default classes are explicitly separated.
- Derived data is marked as derived and not mistaken for source truth.
- The forbidden-data list is explicit and claim-safe.

Expected artifacts:

- Data custody matrix.
- Data owner and source-of-truth table.
- Forbidden-data list.
- Claim-safe language list.
- Adjacent-plan handoff notes.

Expected proof names:

- `data-custody.matrix.source-of-truth`
- `data-custody.matrix.no-hosting-default`
- `data-custody.matrix.redaction-rules`

Failure conditions:

- Vague "we store nothing" claims when account, billing, update, or support metadata is required.
- Any plan treating evidence, screenshots, location, policy, or child profile data as generic telemetry.
- A data class missing source-of-truth ownership or encryption/redaction rules.

