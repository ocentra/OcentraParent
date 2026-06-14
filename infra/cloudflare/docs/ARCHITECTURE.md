# Parent Cloudflare Architecture

This module is the shared Cloudflare control-plane scaffold for Parent.

Current state:

- repo-local scaffold exists
- safe worker entrypoint exists
- route manifest exists
- auth boundary exists as an adapter placeholder
- storage bindings exist as interface declarations
- runtime handlers, real auth wiring, and proof do not exist yet

Ownership:

- shared module shape: `docs/plans/cloudflare-control-plane-plan/`
- payment semantics: `docs/plans/payment-subscription-plan/`
- auth provider decision: `docs/plans/account-identity-family-plan/`
- trusted-device gate consumption: `docs/plans/device-trust-bootstrap-plan/`
