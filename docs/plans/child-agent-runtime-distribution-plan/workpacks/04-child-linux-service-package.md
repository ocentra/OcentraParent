# Workpack 04 - Child Linux Service Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `04-child-linux-service-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child Linux package, service-manager lifecycle, and package proof boundary.

Current status: `production code drafted / test-deferred`.

Proof root: `output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/`

## Owns

- Linux child package shape
- service-manager install and restart truth
- package signing and distribution state
- uninstall and daemon cleanup behavior on Linux

## Must prove

- the package installs through the intended Linux distribution path
- the service manager start/stop/restart state is honest
- platform-specific package manager gaps are explicit
- respawn or recovery claims match Linux service-manager proof
- no generic "Linux support" claim hides distro limits

## Failure conditions

- respawn is claimed without service-manager proof
- package proof is used to claim macOS or Windows readiness
- distro-specific gaps are hidden
- manual-required states are omitted

## Live source truth

Status: source partial; implementation correction and all test/validation/proof gates remain open.

The Linux builder emits child-named `.deb` artifacts, child package metadata, child binary/install paths, and a child systemd unit destination. The checked-in unit source filename and current smoke/workflow expectations remain parent-labelled. Maintainer hooks tolerate `systemctl` failures, so install success does not establish service health. The package is unsigned and no repository/feed owner is implemented.

The systemd unit can start the binary, but default child-service startup supplies no current Device Trust source. No authenticated ingress or external health endpoint is composed. `Restart=always` is static policy, not bounded respawn or cleanup proof.

## Required production source outcome

- consume WP10's reviewed trusted-startup, authenticated-ingress, and external-health boundary;
- use canonical child-owned Linux package/unit identity end to end;
- expose fail-closed service-manager lifecycle outcomes instead of swallowing them;
- preserve declared distro scope and own signing/feed/manual-required states explicitly;
- own deliberate stop, disable/remove/purge, residual custody, and cleanup results.

Implementation dependency: Child WP10 reviewed implementation. Normal READY/DONE remains strict.

## Expected test-source gap

- canonical child `.deb`, unit, binary, artifact, and workflow identity;
- current/missing/revoked trust startup plus external health;
- declared-baseline install, service start, crash/restart, deliberate stop, disable/remove/purge, and cleanup;
- maintainer-hook failure propagation, restart-loop guard, and unsigned/feed rejection states;
- explicit non-`systemd` and unsupported-distro manual-required behavior.

Historical contract/proof runners and `output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/` are review input only.
