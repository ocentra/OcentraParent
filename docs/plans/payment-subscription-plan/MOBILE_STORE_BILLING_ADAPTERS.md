# Mobile Store Billing Adapters

Purpose: define how Apple and Google store billing are mapped into the same billing truth as the web control plane.

## Adapters

| Adapter             | When used                                                          | What it must normalize                                                | Notes                                           |
| ------------------- | ------------------------------------------------------------------ | --------------------------------------------------------------------- | ----------------------------------------------- |
| Apple App Store     | When iOS distribution policy requires in-app purchase billing.     | Receipt status, renewal status, cancellation, and entitlement state.  | Store events are inputs, not product authority. |
| Google Play Billing | When Android distribution policy requires in-app purchase billing. | Purchase status, renewal status, cancellation, and entitlement state. | Keep server-side receipt validation mandatory.  |

## Rules

- Store billing is a channel adapter, not a product branch.
- Store receipts must normalize into the same app-owned billing and entitlement ledgers.
- Store metadata must stay privacy-safe and minimal.
- If a region or surface does not require store billing, do not add it.

## Failure conditions

- Do not let a store receipt bypass ledger materialization.
- Do not place child data in store metadata.
- Do not let mobile store adapters own the product pricing model.
