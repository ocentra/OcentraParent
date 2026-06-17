# Device Trust Bootstrap Plan Tests

Top-level test organization for the device-trust bootstrap plan.

## Major categories

- `unit`
- `contract`
- `integration`
- `e2e`
- `security`

## Current coverage

- `unit/local-key-sealing.test.mjs`
- `contract/parent-step-up-auth.test.mjs`
- `integration/recovery-re-pair-boundary.test.mjs`

## Layout rule

Keep tests under the major category folders above. Do not add per-test-name root folders outside the category tree.
