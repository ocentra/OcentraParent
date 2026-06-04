# Authority Tier Proof

WP30 adds protocol serialization coverage for app/game platform authority rows.

Proofed states:

- Android hide row can serialize `authorityTier=device-owner`,
  `proofState=runtime-proof-attached`, Device Owner proof, and rollback proof.
- Windows broad block-launch row can serialize `authorityTier=manual-required`,
  `proofState=manual-required`, proof needed for AppLocker/App Control and
  rollback, and `canExecuteAdapter=false`.

No authority tier moved up in product status.
