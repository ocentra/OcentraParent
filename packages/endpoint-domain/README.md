# @ocentra-parent/endpoint-domain

Historical endpoint-domain package identity after endpoint contract ownership moved to `@ocentra-parent/schema-domain`.

## Current Role

- Package identity metadata via `./package-info`.
- Packet-local proof/tests that verify the canonical central endpoint contracts.

## Canonical Contract Owners

- `@ocentra-parent/schema-domain/endpoint-brands`
- `@ocentra-parent/schema-domain/endpoint-constants`
- `@ocentra-parent/schema-domain/endpoint-lan-pairing`
- `@ocentra-parent/schema-domain/endpoint-billing-account`
- `@ocentra-parent/schema-domain/endpoint-sync-export`

## Must Not Own

- Local endpoint brands, decoders, route ids, API paths, headers, query params, or contract-version labels.
- WebSocket command payloads. Use `agent-protocol-domain`.
- Portal route/nav semantics. Use `portal-domain`.
- Product policy decisions. Use `parent-domain`.

## Packet-local proofs

- `billing-account-endpoint-contract-proof`
- `sync-export-endpoint-contract-proof`

## Connected Docs

- [Contract expectations](../../docs/expectations/contracts.md)
- [LAN pairing expectations](../../docs/expectations/lan-pairing.md)
- [Sync/export expectations](../../docs/expectations/sync-export.md)
- [Cloud expectations](../../docs/expectations/cloud.md)
- [Billing expectations](../../docs/expectations/billing.md)

## Gaps To Fill

- Keep central endpoint contracts aligned with Rust service paths and tests.
- Keep `sync-export-endpoint-contract-proof` as central route contract proof only until
  parent-owned storage connectors and transfer runtime are implemented.
- Keep `billing-account-endpoint-contract-proof` as central route contract proof only
  until billing provider, account backend, entitlement runtime, and
  updater/download handlers are explicitly assigned.
- Remove this package completely once the remaining packet-local proof placement is no longer needed.
