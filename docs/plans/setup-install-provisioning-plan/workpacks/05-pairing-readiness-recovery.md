# Workpack 05: Pairing Readiness Recovery

Goal: define first-run pairing and readiness as product state, not scattered protocol notes.

## Ownership boundary

```text
setup-install-provisioning-plan owns setup pairing journey labels, readiness matrix, recovery UX, redacted setup diagnostics, and no-fake-ready proof.
lan-plan owns discovery, signed hello, LAN pairing protocol, local transport, and physical LAN proof.
account-identity-family-plan owns household/device authority and parent role proof.
device-trust-bootstrap-plan owns trusted-device approval and trust/key proof.
data-custody-storage-plan and policy-control-plane-plan own custody and policy-baseline readiness inputs.
```

Owns: pairing journey, setup status model, recovery UX, stale/revoked/offline states, and final readiness checklist.

Handoff: `lan-plan` owns local pairing protocol; `account-identity-family-plan` owns household/device authority; `portal-ux-household-surfaces-plan` owns UI rendering.

Expected shape:

- Pairing is a two-stage flow: parent portal creates pairing authority, child bootstrap redeems pairing authority, and parent portal confirms the detected child device.
- Pairing code/link/QR state has expiry, revocation, household binding, and replay rejection.
- Readiness separates account, parent app, child app, permissions, network reachability, custody sync, and policy baseline.
- Recovery handles lost parent device, child reinstall, revoked child, wrong account, offline device, and permission loss.

## Required proof fields

The selected proof must name, at minimum:

```text
pairing_lifecycle_state
parent_authority_state
child_bootstrap_redeem_state
parent_confirmation_state
pairing_code_state
expiry_state
revocation_state
replay_state
wrong_household_state
wrong_device_state
stale_signed_hello_state
anonymous_device_state
revoked_device_state
offline_child_state
permission_missing_state
policy_baseline_state
data_custody_state
lost_parent_recovery_state
child_reinstall_recovery_state
redacted_log_state
lan_handoff_state
device_trust_handoff_state
first_run_complete_state
no_lan_protocol_claim
no_device_trust_claim
no_setup_complete_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

Expected proof:

- Success and negative pairing states.
- Wrong household, stale code, replay, revoked device, and offline child proof.
- Readiness checklist artifact.
- Logs/traces with redaction.

Failure: claiming first-run complete when only LAN discovery or UI rendering is proven.

## Execution Detail

Minimum context:

- `docs/plans/lan-plan/AGENTS.md`
- `docs/plans/account-identity-family-plan/AGENTS.md`
- `docs/expectations/lan-pairing.md`
- `docs/features/family-setup-device-roles.md`
- `docs/plans/setup-install-provisioning-plan/SETUP_STATE_MACHINE.md`
- `docs/plans/setup-install-provisioning-plan/PAIRING_READINESS_MODEL.md`

Agent decision tree:

- If the task is signed hello, local discovery, or protocol detail, route to `lan-plan`.
- If the task is household/device authority, route to `account-identity-family-plan`.
- If the task is readiness UX and recovery state, stay here.
- If remote pairing outside LAN is in scope, route to `remote-access-plan`.

Required output:

- Pairing lifecycle: generated, displayed, accepted, expired, revoked, replayed, wrong household, trusted, untrusted, recovered.
- Readiness model: account, parent app, child agent, permissions, pairing, policy baseline, data custody, network reachability.
- Recovery flows: lost parent, child reinstall, revoked child, stale code, offline device, permission loss.
- Audit and support diagnostics expectations.

Expected tests/proof names:

- `setup.pairing.lifecycle-state-machine`
- `setup.pairing.code-generated-state`
- `setup.pairing.code-expired-rejected`
- `setup.pairing.code-revoked-rejected`
- `setup.pairing.replay-rejected`
- `setup.pairing.wrong-household-rejected`
- `setup.pairing.wrong-device-rejected`
- `setup.pairing.anonymous-device-rejected`
- `setup.pairing.revoked-device-rejected`
- `setup.pairing.stale-signed-hello-rejected`
- `setup.pairing.parent-role-required`
- `setup.readiness.matrix`
- `setup.readiness.no-fake-ready-state`
- `setup.readiness.offline-child-degraded`
- `setup.readiness.permission-missing-degraded`
- `setup.readiness.policy-baseline-missing`
- `setup.readiness.data-custody-unavailable`
- `setup.recovery.lost-parent-device`
- `setup.recovery.child-reinstall`
- `setup.recovery.revoked-child`
- `setup.recovery.permission-loss`
- `setup.recovery.offline-device`
- `setup.observability.redacted-pairing-logs`
- `setup.guided.parent-portal-generates-child-pairing`
- `setup.guided.parent-sees-child-pending-confirmation`
- `setup.guided.child-not-trusted-until-parent-confirmed`
- `setup.guided.no-fake-ready-after-install`
- `setup.guided.no-child-data-public-site`
- `setup.guided.redacted-bootstrap-logs`

Proof artifact expectations:

- `05-pairing-state-machine-proof.md`
- `05-pairing-negative-proof.md`
- `05-readiness-matrix-proof.md`
- `05-no-fake-ready-state-proof.md`
- `05-recovery-flow-proof.md`
- `05-redacted-pairing-log-proof.md`
- `guided-parent-child-pairing-proof.md`
- `no-fake-ready-after-install-proof.md`
- `redacted-bootstrap-logs-proof.md`

## Failure conditions

- Do not claim setup complete from LAN discovery, UI rendering, or pairing-code generation alone.
- Do not claim LAN protocol implementation or device-trust readiness from setup journey proof.
- Do not show child data on public pages.
- Do not omit redaction state for pairing/bootstrap logs.

## Fill before DONE

```text
Workpack id and branch: WP05 Pairing Readiness Recovery / codex/tracking-plan-full-continuation-a
Pairing journey changes: repaired provisioning/runtime validation drift, removed the provisioning-core crate re-export in scope, and wrote the first real WP05 proof root from current source and scoped command evidence.
Touched files: crates/provisioning-core/src/lib.rs, crates/provisioning-core/src/provisioning_install.rs, crates/provisioning-core/tests/unit/readiness.rs, crates/provisioning-core/tests/unit/readiness_flow.rs, crates/child-runtime/src/runtime_gate.rs, crates/child-runtime/tests/unit/runtime_gate.rs, crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-pairing-state-machine-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-pairing-negative-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-readiness-matrix-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-no-fake-ready-state-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-recovery-flow-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-redacted-pairing-log-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/guided-parent-child-pairing-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/no-fake-ready-after-install-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/redacted-bootstrap-logs-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/16-validation-commands.log, docs/plans/setup-install-provisioning-plan/workpacks/05-pairing-readiness-recovery.md
Validation commands and results: `npm run build --workspace @ocentra-parent/setup-domain` PASS; `npm run test --workspace @ocentra-parent/setup-domain` PASS (43 tests); `npm run test --workspace @ocentra-parent/child-runtime-domain` PASS (146 tests); `npm run lint:architecture -- --files packages/setup-domain packages/family-domain packages/child-runtime-domain` PASS; `cargo test -p ocentra-provisioning-core` PASS (29 tests); `cargo test -p ocentra-child-runtime runtime_gate` PASS (9 scoped tests); `cargo lint-architecture crates/provisioning-core` PASS; `cargo lint-architecture crates/child-runtime/src/runtime_gate.rs crates/child-runtime/tests/unit/runtime_gate.rs crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs` PASS.
Proof artifacts: output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-pairing-state-machine-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-pairing-negative-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-readiness-matrix-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-no-fake-ready-state-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-recovery-flow-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-redacted-pairing-log-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/guided-parent-child-pairing-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/no-fake-ready-after-install-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/redacted-bootstrap-logs-proof.md, output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/16-validation-commands.log
Known gaps/manual-required states: real LAN/device-trust proof remains owned by `docs/plans/lan-plan`; pairing-token redaction is now proved through the `@ocentra-parent/family-domain` session-lifecycle contracts and tests, and bootstrap audit projection is now proved through the `ocentra-provisioning-core` readiness-event/action-event chain; no physical-device LAN proof, service restart proof, or broader portal runtime proof is claimed here.
No-claim boundaries: no LAN protocol implementation, no production device-trust claim, no anonymous LAN control, no broad support export, no first-run complete claim from LAN discovery alone.
```
