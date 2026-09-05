# Workpack 07 - Child Managed Service Respawn

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `07-child-managed-service-respawn`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: own health-aware managed respawn, bounded restart, deliberate-stop, and teardown behavior for supported child platforms.

Status: source incomplete; static manager declarations exist, but the production lifecycle boundary does not.

## Live source truth

Windows WinSW, macOS launchd, and Linux systemd declarations contain child binary/service values and restart settings. Android owns a foreground composition service. iOS remains capability-only and unsupported for managed-service respawn.

No production component joins manager callbacks, child startup/readiness/health, restart attempts, bounded backoff/loop guard, deliberate stop, reboot recovery, disable/removal, and teardown into one inspectable lifecycle state. Static `restart`, `KeepAlive`, or `Restart=always` values are not that implementation.

## Required production source outcome

- consume canonical platform package/runtime source from WP02-WP06 and trusted health/startup from WP10;
- expose platform-specific start, steady, degraded, restarting, deliberately stopped, disabled, removed, and manual-required states;
- bound retries/backoff and distinguish crash recovery from operator stop or removal;
- preserve iOS unsupported truth and Android platform limits.

Implementation dependencies: Child WP02-WP06 and WP10 reviewed implementation. Normal READY/DONE remains strict.

## Expected test-source gap

- kill, crash, reboot, service-manager restart, deliberate stop, disable, removal, and teardown by supported platform;
- bounded retry/backoff and restart-loop rejection;
- health transition and stale/unreachable service behavior;
- Android foreground-service lifecycle/manual-required cases;
- explicit iOS unsupported result.

## Owns

- restart survival across supported platforms
- recovery after kill, stop, reboot, or service-manager restart
- respawn truth by platform
- explicit unsupported or manual-required states

## Must prove

- managed respawn is only claimed where the platform can prove it
- restart survival is explicit and tested
- failure or unsupported states are visible, not hidden
- teardown or stop-path proof exists for each supported platform slice

## Failure conditions

- respawn is generalized across platforms without proof
- unsupported platforms are shown as supported
- manual-required states are hidden
- the slice reuses parent-client proof instead of child proof
- static manager configuration is counted as runtime respawn implementation
- deliberate stop or removal is mistaken for a crash that should restart
