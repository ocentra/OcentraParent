# Route Index

Use [AGENTS.md](AGENTS.md), then [WORKPACK_INDEX.md](WORKPACK_INDEX.md), then [PROOF_AND_TEST_INVENTORY.md](PROOF_AND_TEST_INVENTORY.md). Provider and auth/security choices must route here before setup, policy, or remote plans claim account authority.

Read [AUTH_PROVIDER_DECISION.md](AUTH_PROVIDER_DECISION.md) and [IDENTITY_AUTHORITY_MODEL.md](IDENTITY_AUTHORITY_MODEL.md) before any setup, policy, or remote plan claims account authority.

## Current source route - 2026-08-17

```text
Account WP08 sealed schema/local repository
  -> Account WP02 target-aware actor/action authority
  -> Cloudflare WP06 authoritative D1 writer/currentness/revocation/CAS and provider caller
  -> Device Trust WP03 live Account + Device Trust ceremony composition
```

WP02 owns the Account action decision boundary; Cloudflare WP06 owns provider
composition and durable Cloudflare writes; Device Trust WP03 owns the parent
ceremony. No edge points back upstream, and no historical proof or mapped file
closes the missing source or expected-test work.
