# device-trust-bootstrap-plan Event Architecture Instruction

## Owns

- trusted-device vocabulary;
- step-up, QR approval, reset-required, and trust-rebuild semantics;
- trust-state transitions on top of account/setup/custody inputs.

## Must not own

- account/household/session authority source;
- custody persistence or key sealing substrate;
- LAN transport implementation;
- payment/device-subject consumption.

## Required chain

```text
setup/account producer emits bootstrap or approval input
-> device-trust owner evaluates trust transition
-> trust event/read model records result
-> consumers use typed trust state, not local guesses
```

## Logging/proof

Log trust input, actor/device scope, step-up reason, accepted/rejected transition, reset-required state, and no-claim custody boundary.

## Tests

Trust semantics belong in family/setup/LAN consumer contracts and any Rust parity crate that owns trust-state behavior. Cross-plan setup/trust/custody proof belongs in integration/proof runner, not in a single domain unit test.

## First architecture slice

Continue with step-up/QR approval. Do not start key sealing until data-custody has repaired the persistence/encryption substrate.
