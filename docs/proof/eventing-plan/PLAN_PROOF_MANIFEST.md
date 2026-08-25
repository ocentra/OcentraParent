# Eventing Plan Proof Manifest

This tracked manifest retains the generic WP06 Eventing handoff for
`eventing-plan`. It does not claim WP10-WP13 closure, `PR_READY`, or closure of
consumer-owned workpacks.

## Current route-proof status

- WP12 route-proof harness/root is missing.
- WP13 test-folder-layout code is complete, but current validation/proof is open
  and must include the external `contract` harness.
- WP10 LAN Household Mesh Consumer is blocked on LAN WP26 and its canonical
  proof root is absent.
- WP11 Type Safety And Ownership Hardening is implementation-ready/open: the
  envelope/aggregate/idempotency negatives and retained proof are missing.

## Proof docs

- [slice-01-envelope-version.md](slice-01-envelope-version.md)
- [slice-02-ordering-replay.md](slice-02-ordering-replay.md)
- [slice-03-consumer-boundary.md](slice-03-consumer-boundary.md)
- [wp06-00-enforcement-wp11-handoff.md](wp06-00-enforcement-wp11-handoff.md)
- [wp06-01-journal-replay-proof.md](wp06-01-journal-replay-proof.md)
- [wp06-02-topology-lineage-proof.md](wp06-02-topology-lineage-proof.md)
- [wp06-16-validation-commands.md](wp06-16-validation-commands.md)

## Validation previously named by the route (not current retained proof)

- `cargo test -p ocentra-eventing --tests`
- `cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts`
- `cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet`
- `npm run type-check --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- policy-control-audit-redaction.test.ts policy-control-delivery-read-model.test.ts contracts.test.ts`
- `npm run lint:architecture -- --files packages/agent-protocol-domain/src/contracts.ts packages/agent-protocol-domain/src/policy-control-audit-redaction.ts packages/agent-protocol-domain/src/policy-control-delivery-read-model.ts`
- `node scripts/test/eventing-rollout-proof.mjs` (missing)

## Remaining gaps

- No claim that WP10 household mesh proof is complete. The expected local proof
  roots for that slice are still absent in this checkout.
- No claim that WP11, WP12, or WP13 proof is current; their generated roots are
  absent and WP13 still requires the `contract` harness.
- No claim that the full eventing plan is done, rollout-ready, or PR-ready.
