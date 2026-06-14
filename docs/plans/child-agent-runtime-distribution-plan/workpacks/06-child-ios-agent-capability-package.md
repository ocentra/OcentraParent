# Workpack 06 - Child iOS Capability Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `06-child-ios-agent-capability-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child iOS capability package, provisioning limits, and manual-required proof boundary.

## Owns

- iOS capability package shape
- provisioning and supervision limits
- launch, availability, and recovery honesty on iOS
- manual-required states for unsupported child service behavior

## Must prove

- the iOS artifact is honest about what it can and cannot do
- provisioning and supervision limits are explicitly represented
- capability-only state is used when full service behavior is unavailable
- no hidden control or daemon claim exceeds iOS limits
- parent-client parity is not implied from the iOS slice

## Failure conditions

- background or persistent-service claims exceed iOS proof
- provisioning limits are hidden
- manual-required states are omitted
- the slice claims more than a capability package
