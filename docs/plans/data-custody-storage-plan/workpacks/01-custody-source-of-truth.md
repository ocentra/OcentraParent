# Workpack 01: Custody Source of Truth

Goal: define exactly what "data" means and who owns every data class.

Context to read:

- `docs/expectations/data-custody.md`
- `docs/expectations/evidence-storage.md`
- `docs/expectations/sync-export.md`
- `docs/features/evidence-store-query.md`
- `docs/features/reports-notifications-sync.md`

In scope:

- Data taxonomy: account, household, child profile, device, policy, evidence, reports, notifications, logs, diagnostics, billing references, setup state, AI outputs, screenshots, location, network/app/browser events.
- Custody authority: parent-owned, local-only, parent-cloud-owned, Ocentra relay/transient, Ocentra account metadata, provider-owned billing identity.
- Sensitive data classes and redaction obligations.
- Current truth path for each data class.

Out of scope:

- Event bus mechanics owned by `eventing-plan`.
- Payment subscription state owned by `payment-subscription-plan`.
- Account identity model owned by `account-identity-family-plan`.

Decisions required:

- Which data must remain local-first.
- Which data may sync to parent-owned cloud storage.
- Which metadata Ocentra may store to operate accounts, billing, updates, and support.
- Which data is forbidden from Ocentra-hosted storage unless parent explicitly enables a mode.

Expected artifacts:

- Data custody matrix.
- Data owner/source-of-truth table.
- Forbidden-data list.
- Adjacent-plan handoff notes.
- Product-language constraints for privacy claims.

Expected proof:

- Route consistency check across feature/expectation docs.
- Negative proof that child activity data is not routed to public website, payment metadata, or unrelated providers.
- Redaction rule inventory.

Failure conditions:

- Vague "we store nothing" claim when account, billing, update, or support metadata is actually required.
- Any plan treating evidence, screenshots, location, policy, or child profile data as generic app telemetry.
