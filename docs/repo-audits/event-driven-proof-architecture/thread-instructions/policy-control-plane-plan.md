# policy-control-plane-plan Event Architecture Instruction

## Owns

- policy source, authority, compiler, conflict, delivery, preview, request, and policy-event contracts;
- policy audit/redaction contract where local to policy.

## Must not own

- portal authoring UX final rendering;
- enforcement runtime action authority;
- account/session step-up source;
- custody export/delete runtime.

## Required chain

```text
policy authoring command
-> policy owner validates and compiles
-> policy event/delivery request is recorded
-> domain runtime consumes policy read model
-> enforcement/app/browser/tracking owners decide their own runtime effect
```

## Logging/proof

Log source, compiler decision, conflict resolution, delivery request, ack/retry, audit redaction, and no-direct-enforcement boundary.

## Tests

Policy-domain and policy-control-core own unit/contract/version-skew. Portal authoring and approval flows are portal/app integration proof. Enforcement outcome proof belongs to enforcement/app/browser owners.

## First architecture slice

Repair WP06 route/proof truth, rebuild WP03 compiler proof, then add WP04 delivery/ack/audit bundle. Start portal authoring dependency in parallel only through portal-owned instructions.
