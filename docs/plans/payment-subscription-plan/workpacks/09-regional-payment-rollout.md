# Workpack 09: Regional Payment Rollout

Purpose: define the region matrix, provider availability, currency/tax launch gating, and fallback behavior.

## Owns

- `REGIONAL_PAYMENT_MARKET_MATRIX.md`
- PSP-012 and the regional launch policy

## Must prove

- Region-specific default provider selection is explicit.
- Unsupported regions degrade to a supported fallback.
- Currency and tax handling are checked before launch.
- Region-disabled behavior is visible and safe.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp09/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if a region can be marked live without a matrix entry.
- The workpack fails if unsupported regions silently create payment claims.
- The workpack fails if tax or currency behavior is untested for the region.
