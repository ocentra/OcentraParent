# Workpack 09 - Child Signing Store Device Owner Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `09-child-signing-store-device-owner-matrix`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the signing, store, and device-owner matrix for child artifacts by platform.

## Owns

- signing state by artifact
- store distribution state by platform
- device-owner or managed-profile truth where applicable
- manual-required state when a platform cannot prove the claim

## Must prove

- each child artifact names its signing and store assumptions explicitly
- device-owner claims are platform-specific and evidence-backed
- unsupported store or signing states are manual-required, not hidden
- the matrix covers Windows, macOS, Linux, Android, and iOS rows honestly

## Failure conditions

- a generic signing/store claim replaces the platform matrix
- device-owner support is implied without platform proof
- manual-required rows are omitted
- child and parent artifact claims are conflated
