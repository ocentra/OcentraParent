import { describe, expect, it } from 'vitest';

import {
  ChildRuntimeDeviceAuthorizationState,
  ChildRuntimeEnforcementDecisionSchema,
  ChildRuntimeEnforcementExecutionState,
  ChildRuntimeEntitlementAccessState,
  ChildRuntimeManualReviewState,
  ChildRuntimePreflightDecisionSchema,
  ChildRuntimeProvisioningDecisionBlockerReason,
  ChildRuntimeProvisioningDecisionSchema,
  ChildRuntimeProvisioningReadinessState,
  ChildRuntimeRemoteAccessAuthorizationState,
  ChildRuntimeRemoteAccessDecisionSchema,
  ChildRuntimeStartState,
  ChildRuntimeStorageRemoteUploadState,
} from '@ocentra-parent/schema-domain/child-runtime-gates';
import {
  SetupChildInstallState,
  SetupChildServiceState,
  SetupReadinessOverallState,
} from '@ocentra-parent/schema-domain/setup-readiness';

describe('child runtime gate contracts', () => {
  it('accepts a coherent ready preflight decision with provisioningDecision', () => {
    const provisioningDecision = ChildRuntimeProvisioningDecisionSchema.parse({
      childInstallState: SetupChildInstallState.Installed,
      childServiceState: SetupChildServiceState.ServiceStarted,
      overallState: SetupReadinessOverallState.Ready,
      blockerReason: null,
    });

    const decision = ChildRuntimePreflightDecisionSchema.parse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      manualReviewState: ChildRuntimeManualReviewState.NotRequired,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.Ready,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Allowed,
      provisioningDecision,
    });

    expect(decision.runtimeStartState).toBe(ChildRuntimeStartState.Allowed);
    expect(decision.provisioningDecision.overallState).toBe(SetupReadinessOverallState.Ready);
  });

  it('rejects preflight allow decisions when entitlement blocks local child runtime', () => {
    const result = ChildRuntimePreflightDecisionSchema.safeParse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.Ready,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Blocked,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
      provisioningDecision: {
        childInstallState: SetupChildInstallState.Installed,
        childServiceState: SetupChildServiceState.ServiceStarted,
        overallState: SetupReadinessOverallState.Ready,
        blockerReason: null,
      },
    });

    expect(result.success).toBe(false);
  });

  it('accepts installed + not-started blocked/manual review shape', () => {
    const decision = ChildRuntimePreflightDecisionSchema.parse({
      runtimeStartState: ChildRuntimeStartState.Blocked,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.NotReady,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
      provisioningDecision: {
        childInstallState: SetupChildInstallState.Installed,
        childServiceState: SetupChildServiceState.NotStarted,
        overallState: SetupReadinessOverallState.Blocked,
        blockerReason: ChildRuntimeProvisioningDecisionBlockerReason.ChildServiceNotStarted,
      },
    });

    expect(decision.provisioningDecision.childServiceState).toBe(
      SetupChildServiceState.NotStarted
    );
  });

  it('accepts installed + offline degraded/manual review shape', () => {
    const decision = ChildRuntimePreflightDecisionSchema.parse({
      runtimeStartState: ChildRuntimeStartState.Blocked,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.NotReady,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
      provisioningDecision: {
        childInstallState: SetupChildInstallState.Installed,
        childServiceState: SetupChildServiceState.Offline,
        overallState: SetupReadinessOverallState.Degraded,
        blockerReason: ChildRuntimeProvisioningDecisionBlockerReason.ChildAppOffline,
      },
    });

    expect(decision.provisioningDecision.overallState).toBe(SetupReadinessOverallState.Degraded);
  });

  it('accepts reinstall-required blocked/manual review shape', () => {
    const decision = ChildRuntimePreflightDecisionSchema.parse({
      runtimeStartState: ChildRuntimeStartState.Blocked,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.NotReady,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
      provisioningDecision: {
        childInstallState: SetupChildInstallState.ReinstallRequired,
        childServiceState: SetupChildServiceState.NotStarted,
        overallState: SetupReadinessOverallState.Blocked,
        blockerReason: ChildRuntimeProvisioningDecisionBlockerReason.ChildAppReinstallRequired,
      },
    });

    expect(decision.provisioningDecision.childInstallState).toBe(
      SetupChildInstallState.ReinstallRequired
    );
  });

  it('rejects mismatch when runtimeStartState is allowed but provisioning says blocked or degraded', () => {
    const blockedResult = ChildRuntimePreflightDecisionSchema.safeParse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.NotReady,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
      provisioningDecision: {
        childInstallState: SetupChildInstallState.Installed,
        childServiceState: SetupChildServiceState.NotStarted,
        overallState: SetupReadinessOverallState.Blocked,
        blockerReason: ChildRuntimeProvisioningDecisionBlockerReason.ChildServiceNotStarted,
      },
    });
    const degradedResult = ChildRuntimePreflightDecisionSchema.safeParse({
      runtimeStartState: ChildRuntimeStartState.Allowed,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.NotReady,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
      provisioningDecision: {
        childInstallState: SetupChildInstallState.Installed,
        childServiceState: SetupChildServiceState.Offline,
        overallState: SetupReadinessOverallState.Degraded,
        blockerReason: ChildRuntimeProvisioningDecisionBlockerReason.ChildAppOffline,
      },
    });

    expect(blockedResult.success).toBe(false);
    expect(degradedResult.success).toBe(false);
  });

  it('rejects mismatch when provisioningReadiness is ready but provisioningDecision says blocked or degraded', () => {
    const blockedResult = ChildRuntimePreflightDecisionSchema.safeParse({
      runtimeStartState: ChildRuntimeStartState.Blocked,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.Ready,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
      provisioningDecision: {
        childInstallState: SetupChildInstallState.Installed,
        childServiceState: SetupChildServiceState.NotStarted,
        overallState: SetupReadinessOverallState.Blocked,
        blockerReason: ChildRuntimeProvisioningDecisionBlockerReason.ChildServiceNotStarted,
      },
    });
    const degradedResult = ChildRuntimePreflightDecisionSchema.safeParse({
      runtimeStartState: ChildRuntimeStartState.Blocked,
      manualReviewState: ChildRuntimeManualReviewState.Required,
      deviceAuthorization: ChildRuntimeDeviceAuthorizationState.Authorized,
      provisioningReadiness: ChildRuntimeProvisioningReadinessState.Ready,
      entitlementAccess: ChildRuntimeEntitlementAccessState.Allowed,
      remoteUpload: ChildRuntimeStorageRemoteUploadState.Blocked,
      provisioningDecision: {
        childInstallState: SetupChildInstallState.Installed,
        childServiceState: SetupChildServiceState.Offline,
        overallState: SetupReadinessOverallState.Degraded,
        blockerReason: ChildRuntimeProvisioningDecisionBlockerReason.ChildAppOffline,
      },
    });

    expect(blockedResult.success).toBe(false);
    expect(degradedResult.success).toBe(false);
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
