# cloudflare-control-plane-plan Instruction

## Verdict

`partial`. Runtime module and test suites are real; proof artifacts and CFCP-C hardening are missing. Blocks payment handoff.

## Assign first

`cfcp-c-queue-dead-letter-and-negative-hardening`:

- close queue/dead-letter runtime truth;
- strengthen negative-path security/property/fuzz coverage;
- rerun `infra/cloudflare` focused suites.

## Then

1. `cfcp-proof-materialization`: write proof artifacts under `output/cloudflare-control-plane-plan-proof/00-12/`.
2. `cfcp-auth-deployment-proof`: only after account/trusted-device/deployment env ownership is clear.
3. `cfcp-payment-handoff`: publish `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`.

## Coordinate with

- `payment-subscription-plan` for handoff consumer only after WP12 exists.
- `account-identity-family-plan` for final auth/provider contract.
- `device-trust-bootstrap-plan` for trusted-device auth.

## Do not

- Do not count placeholder `infra/cloudflare/src/*` subdirs as implementation.
- Do not start payment source edits in Cloudflare while CFCP-C is dirty.
- Do not claim deployment readiness from Wrangler config alone.
