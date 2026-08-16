# Eventing Plan Proof Manifest

This manifest restores the local WP12 route-proof bundle for `eventing-plan`.
It documents current proof-backed status only. It does not claim full plan
completion, `PR_READY`, or closure of consumer-owned workpacks.

## Current route-proof status

- WP12 route-proof bundle is restored in this checkout.
- WP13 test-folder-layout regression audit is locally proved.
- Remaining open workpacks in truth are:
  - WP10 LAN Household Mesh Consumer
- WP11 Type Safety And Ownership Hardening is now locally proved through the
  restored proof roots, package-wide `@ocentra-parent/agent-protocol-domain`
  type-check, focused policy-control/contract tests, and the touched-file
  architecture gate.

## Proof docs

- [slice-01-envelope-version.md](slice-01-envelope-version.md)
- [slice-02-ordering-replay.md](slice-02-ordering-replay.md)
- [slice-03-consumer-boundary.md](slice-03-consumer-boundary.md)
- [wp06-00-enforcement-wp11-handoff.md](wp06-00-enforcement-wp11-handoff.md)
- [wp06-01-journal-replay-proof.md](wp06-01-journal-replay-proof.md)
- [wp06-02-topology-lineage-proof.md](wp06-02-topology-lineage-proof.md)
- [wp06-16-validation-commands.md](wp06-16-validation-commands.md)

## Validation used for this bundle

- `cargo test -p ocentra-eventing --tests`
- `cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts`
- `cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet`
- `npm run type-check --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- policy-control-audit-redaction.test.ts policy-control-delivery-read-model.test.ts contracts.test.ts`
- `npm run lint:architecture -- --files packages/agent-protocol-domain/src/contracts.ts packages/agent-protocol-domain/src/policy-control-audit-redaction.ts packages/agent-protocol-domain/src/policy-control-delivery-read-model.ts`
- `node scripts/test/eventing-rollout-proof.mjs`

## Remaining gaps

- No claim that WP10 household mesh proof is complete. The expected local proof
  roots for that slice are still absent in this checkout.
- No claim that the full eventing plan is done, rollout-ready, or PR-ready.
