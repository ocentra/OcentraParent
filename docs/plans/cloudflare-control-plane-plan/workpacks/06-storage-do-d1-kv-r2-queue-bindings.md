# Workpack 06 - Storage DO D1 KV R2 Queue Bindings

> **Status:** BLOCKED on Protected WP05 -> Account WP09; current-v2
> Cloudflare verifier/store source is present, authenticated owner delivery and
> eight expected tests remain open.

## Agent capsule

- Route: Cloudflare control-plane WP06 only.
- Own the private current-v2 Account issuer consumer, current-key/D1 custody,
  migration, runtime mount, and Cloudflare-side tests.
- Consume only an owner-authenticated Account delivery and current public-key
  record. Never derive Account authority from headers, provider claims, D1
  rows, environment keys, or caller scalars.
- Do not edit Account or Protected Custody source from this workpack.

## Goal

Receive the bounded P-256 v2 Account authority wire through a private
owner-authenticated handoff, verify the original bytes against the exact current
Account public key, persist only public verifier currentness/CAS and inbound
idempotency in D1, and expose the existing caller/runtime/writer path only after
that binding is current.

## Current production truth - 2026-08-29

The repository already contains the current-v2 Cloudflare substrate:

- canonical P-256 v2 outer/inner verification, key-ID derivation, low-S and
  bounded-time checks;
- D1 migration `0008` and current verifier/inbound receipt custody;
- the existing Account caller, runtime, store, and writer hosts;
- a Worker internal route that accepts only the internal-queue header plus a
  shared secret.

That internal route is not Account authority. There is no shipped
Account-authenticated service-binding delivery caller, no positive owner-issued
current-key registration path, and no production delivery owner. Protected
WP05 must first preserve the broker transcript and authorize the Account owner;
Account WP09 must then compose the family lifecycle. Until then, WP06 remains
manual-required and must not expose mutation readiness.

## Current-v2 production roots

```text
infra/cloudflare/src/auth/account-identity-authority-issuer-v2.ts
infra/cloudflare/src/auth/account-identity-authority-producer-v2-contract.ts
infra/cloudflare/src/auth/account-identity-authority-producer-v2-transport.ts
infra/cloudflare/src/auth/account-identity-authority-caller.ts
infra/cloudflare/src/auth/account-identity-authority-runtime.ts
infra/cloudflare/src/storage/account-identity-authority-issuer-v2.ts
infra/cloudflare/src/storage/account-identity-authority-store.ts
infra/cloudflare/src/storage/account-identity-authority-writer.ts
infra/cloudflare/migrations/account-identity/0008_account_identity_authority_issuer_v2.sql
```

The old unversioned
`account-identity-authority-issuer-{transport,key-registry,runtime}.ts` names
are retired. The live v2 hosts above must be completed; duplicating or renaming
them would split the authority boundary.

## Expected real test source

One current-v2 negative test exists but is not registered by the Cloudflare
test runner:

```text
infra/cloudflare/tests/account-identity-authority-issuer-v2.test.ts
```

The remaining real test roots are:

```text
infra/cloudflare/tests/unit/account-identity-authority-issuer-v2-transport.test.ts
infra/cloudflare/tests/unit/account-identity-authority-issuer-v2-key-registry.test.ts
infra/cloudflare/tests/integration/account-identity-authority-issuer-v2-runtime.test.ts
infra/cloudflare/tests/unit/account-identity-authority-caller.test.ts
infra/cloudflare/tests/unit/account-identity-authority-runtime.test.ts
infra/cloudflare/tests/integration/account-identity-authority-currentness.test.ts
infra/cloudflare/tests/integration/account-identity-authority-restart-cas.test.ts
infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts
```

They must exercise the real private adapter and real D1/migration boundary:
valid owner delivery/current key, malformed and high-S wire, stale/unknown/
revoked key, outer/inner binding mismatch, duplicate delivery, restart,
compare-and-swap, rotation/revocation, migration mismatch, unavailable owner,
and fail-closed mutation reachability. A fixture key, public route, test-double
authority, or source-text assertion is not valid.

## Dependency route

```text
Protected WP01 -> WP02 -> WP03 -> WP04 -> WP05
-> Account WP08 -> Account WP09
-> Cloudflare WP06
-> Cloudflare WP08 runner/proof
```

WP06 remains implementation-blocked on Account WP09, which is itself blocked
on Protected WP05. Mapping current-v2 roots does not bypass that gate.

## Storage ownership

Account SQLite remains authoritative for Account key lineage, issue
reservation, receipts, and outbox. Cloudflare D1 owns only public verifier
currentness/CAS, inbound idempotency, and the selected Account mapping needed by
the Worker. Account DO/KV remain absent/manual-required; billing storage and R2
remain separate owners. Placeholder Wrangler IDs are not deployment evidence.

## Exit conditions

Exit requires the owner-authenticated delivery/current-key adapter, production
runtime composition, all nine real test roots registered and executable,
applied migration/restart/CAS/revocation results, retained proof, checklist
acceptance, pre-commit, CI, deployment/review, and normal merge. No READY or
DONE claim follows from verifier/store file presence.

Expected proof root:
`output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`.
