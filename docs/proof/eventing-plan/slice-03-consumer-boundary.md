# Slice 03 Consumer Boundary

## Scope

Route reconciliation for consumer-boundary claims: what eventing can prove
locally, what remains consumer-owned, and which workpacks are still open.

## Evidence

- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts`
- `cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet`
- `npm run type-check --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- policy-control-audit-redaction.test.ts policy-control-delivery-read-model.test.ts contracts.test.ts`
- `npm run lint:architecture -- --files packages/agent-protocol-domain/src/contracts.ts packages/agent-protocol-domain/src/policy-control-audit-redaction.ts packages/agent-protocol-domain/src/policy-control-delivery-read-model.ts`
- `output/eventing-plan-proof/13-test-folder-layout-regression-audit/proof-summary.json`

## What This Proves

- Focused downstream protocol mirrors still pass the eventing-adjacent contract
  surfaces exercised in this checkout.
- WP13 is locally proved and no longer blocks route truth.
- WP11 is now locally proved, so the remaining open eventing-plan slice is WP10
  rather than WP11/WP12/WP13.

## Negative / Not Proved

- No claim that WP10 LAN household mesh proof is complete.
- No claim that broker-backed delivery, relay-hub delivery, portal-owned
  business event publishing, or broader consumer-plan runtime readiness is
  proved here.
- No `PR_READY` claim is made by this route-proof bundle.

## Remaining Gaps

- WP10 remains open because its expected local proof roots are absent and its
  owning consumer-plan handoff still needs exact verification.
