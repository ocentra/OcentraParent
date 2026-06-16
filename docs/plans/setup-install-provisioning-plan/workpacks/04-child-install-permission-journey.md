<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `WP04 Child Install Permission Journey`
> Kind: assigned implementation/research workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not implement child service runtime, platform adapter, package artifact, device trust, or LAN protocol here.
> Proves: parent-visible child install/permission journey state only after proof artifacts exist.
> Does not prove: child runtime readiness, platform permission readiness, package readiness, trust, pairing, or policy readiness.
> Proof rule: before DONE, write all WP04 proof artifacts and command log.

<!-- /agent-capsule -->

# WP04 Child Install Permission Journey

## Goal

Define the child-agent install and permission journey from the parent/setup perspective with honest installed/running/permissioned/paired/trusted/policy-ready separation.

## Required inputs

```text
RESEARCH_AND_DECISIONS.md
docs/features/child-agent-local-service.md
docs/expectations/platforms.md
docs/expectations/tamper-uninstall-protection.md
docs/plans/app-plan/AGENTS.md
docs/plans/child-agent-runtime-distribution-plan/AGENTS.md
docs/plans/setup-install-provisioning-plan/SETUP_STATE_MACHINE.md
docs/plans/setup-install-provisioning-plan/CHILD_PERMISSION_MATRIX.md
```

## Owned scope

```text
child bootstrap code/link state
child install instruction states
permission readiness labels
disclosure/consent state expectations
unsupported/manual-required states
reinstall recovery labels
handoffs to child runtime/package/platform/device-trust/LAN owners
```

## Out of scope

```text
child service runtime implementation
platform adapter implementation
package build/signing/store delivery
device-owner or managed-profile implementation
LAN signed hello protocol
policy/enforcement behavior
```

## Required output

```text
child bootstrap state machine
platform matrix
permission matrix
installed/running/permissioned/paired/trusted/policy-ready separation
disclosure checklist
reinstall/recovery state
handoff matrix to owning plans
```

## Required proof root

```text
output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/
```

Required artifacts:

```text
00-child-bootstrap-code-state-proof.md
01-child-platform-matrix-proof.md
02-permission-matrix-proof.md
03-missing-permission-degraded-proof.md
04-child-disclosure-proof.md
05-reinstall-recovery-proof.md
06-child-install-ui-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Child bootstrap code/link state machine exists.
- [ ] Child platform matrix exists.
- [ ] Permission matrix exists.
- [ ] Installed/running/permissioned/paired/trusted/policy-ready states are separated.
- [ ] Missing permission degraded state exists.
- [ ] Disclosure/consent expectations are visible.
- [ ] Reinstall recovery state exists.
- [ ] Owner handoffs are explicit.
- [ ] Focused commands pass or blocker recorded.

## Focused commands

```bash
node -e "console.log('child-install-permission-journey-handoff')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan docs/plans/child-agent-runtime-distribution-plan docs/plans/app-plan
```

If UI routes exist later:

```bash
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
```

## Negative states

- Installed process is shown as ready.
- Running service is shown as permissioned.
- Permissioned service is shown as paired/trusted.
- Missing platform capability looks successful.
- Missing permission state is hidden.
- Reinstall path creates a second uncontrolled device identity.

## Manual-required gaps

Real platform permission proof remains owned by app/runtime plans. Real device trust and LAN pairing remain owned by device-trust and LAN plans.

## Fill before DONE

```text
Workpack id and branch:
Child install/permission journey changes:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```
