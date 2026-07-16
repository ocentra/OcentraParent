# cloudflare-control-plane-plan Event Architecture Instruction

## Owns

- Cloudflare worker route behavior;
- auth boundary classification;
- binding-backed billing/control-plane state;
- queue and failure-path handling inside the worker;
- payment handoff artifact after worker proof is current.

## Must consume

- account/session authority from account plan;
- trusted-device contract from device-trust plan;
- billing product contract from payment plan.

## Required chain

```text
worker request -> route/auth classification -> typed handler -> storage/queue update -> audit/read model -> handoff artifact
```

## Logging/proof

Log route id, caller class, handler decision, binding result, queue result, failure-path result, and handoff readiness.

## Tests

Keep worker tests under `infra/cloudflare/tests/*`. Proof artifacts must land under `output/cloudflare-control-plane-plan-proof/` before payment consumes the handoff.

## First architecture slice

Run CFCP-C1: worker queue/failure-path truth plus negative-path test hardening. Then materialize the current green proof scope.
