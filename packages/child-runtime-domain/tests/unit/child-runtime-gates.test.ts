import { describe, expect, it } from 'vitest';

import {
  ChildRuntimeDeviceAuthorizationState,
  ChildRuntimeEnforcementDecisionSchema,
  ChildRuntimeEnforcementExecutionState,
  ChildRuntimeEntitlementAccessState,
  ChildRuntimeManualReviewState,
  ChildRuntimePreflightDecisionSchema,
  ChildRuntimeProvisioningReadinessState,
  ChildRuntimeRemoteAccessAuthorizationState,
  ChildRuntimeRemoteAccessDecisionSchema,
  ChildRuntimeStartState,
  ChildRuntimeStorageRemoteUploadState,
} from '../../src/child-runtime-gates';

describe('child runtime gate contracts', () => {
  it('accepts a coherent child runtime preflight allow decision', () => {
    const decision = ChildRuntimePreflightDecisionSchema.parse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      manualReviewState: ChildRuntimeManualReviewState.NotRequired,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.Ready,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Allowed,
    });

    expect(decision.runtimeStartState).toBe(ChildRuntimeStartState.Allowed);
  });

  it('rejects preflight allow decisions when entitlement blocks local child runtime', () => {
    const result = ChildRuntimePreflightDecisionSchema.safeParse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.Ready,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Blocked,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
    });

    expect(result.success).toBe(false);
  });

  it('accepts a coherent remote access allow decision', () => {
    const decision = ChildRuntimeRemoteAccessDecisionSchema.parse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      remoteAccessAuthorization: ChildRuntimeRemoteAccessAuthorizationState.Allowed,
    });

    expect(decision.remoteAccessAuthorization).toBe(
      ChildRuntimeRemoteAccessAuthorizationState.Allowed
    );
  });

  it('rejects remote access allow decisions when authorization rejects the request', () => {
    const result = ChildRuntimeRemoteAccessDecisionSchema.safeParse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      remoteAccessAuthorization: ChildRuntimeRemoteAccessAuthorizationState.Rejected,
    });

    expect(result.success).toBe(false);
  });

  it('accepts a coherent child runtime enforcement execute decision', () => {
    const decision = ChildRuntimeEnforcementDecisionSchema.parse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      enforcementExecution: ChildRuntimeEnforcementExecutionState.Execute,
    });

    expect(decision.enforcementExecution).toBe(ChildRuntimeEnforcementExecutionState.Execute);
  });

  it('rejects enforcement allow decisions when execution is blocked', () => {
    const result = ChildRuntimeEnforcementDecisionSchema.safeParse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      enforcementExecution: ChildRuntimeEnforcementExecutionState.DoNotExecute,
    });

    expect(result.success).toBe(false);
  });
});
