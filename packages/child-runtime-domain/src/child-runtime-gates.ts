import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  SetupChildInstallState,
  SetupChildInstallStateSchema,
  SetupChildServiceState,
  SetupChildServiceStateSchema,
  SetupReadinessOverallState,
  SetupReadinessOverallStateSchema,
} from '@ocentra-parent/setup-domain/readiness';

export const ChildRuntimeStartStateLiteral = {
  Allowed: 'allowed',
  Blocked: 'blocked',
} as const;

export const ChildRuntimeManualReviewStateLiteral = {
  NotRequired: 'not-required',
  Required: 'required',
} as const;

export const ChildRuntimeDeviceAuthorizationStateLiteral = {
  Authorized: 'authorized',
  Rejected: 'rejected',
} as const;

export const ChildRuntimeProvisioningReadinessStateLiteral = {
  Ready: 'ready',
  NotReady: 'not-ready',
} as const;

export const ChildRuntimeProvisioningDecisionBlockerReasonLiteral = {
  ChildInstallNotInstalled: 'child-install-not-installed',
  ChildServiceNotStarted: 'child-service-not-started',
  ChildAppOffline: 'child-app-offline',
  ChildAppReinstallRequired: 'child-app-reinstall-required',
} as const;

export const ChildRuntimeEntitlementAccessStateLiteral = {
  Allowed: 'allowed',
  Blocked: 'blocked',
} as const;

export const ChildRuntimeStorageRemoteUploadStateLiteral = {
  Allowed: 'allowed',
  Blocked: 'blocked',
} as const;

export const ChildRuntimeRemoteAccessAuthorizationStateLiteral = {
  Allowed: 'allowed',
  Rejected: 'rejected',
} as const;

export const ChildRuntimeEnforcementExecutionStateLiteral = {
  Execute: 'execute',
  DoNotExecute: 'do-not-execute',
} as const;

type LiteralValue<T extends Record<string, string>> = T[keyof T];
type ChildRuntimeStartStateValue = LiteralValue<typeof ChildRuntimeStartStateLiteral>;
type ChildRuntimeManualReviewStateValue = LiteralValue<typeof ChildRuntimeManualReviewStateLiteral>;
type ChildRuntimeDeviceAuthorizationStateValue = LiteralValue<
  typeof ChildRuntimeDeviceAuthorizationStateLiteral
>;
type ChildRuntimeProvisioningReadinessStateValue = LiteralValue<
  typeof ChildRuntimeProvisioningReadinessStateLiteral
>;
type ChildRuntimeProvisioningDecisionBlockerReasonValue = LiteralValue<
  typeof ChildRuntimeProvisioningDecisionBlockerReasonLiteral
>;
type ChildRuntimeEntitlementAccessStateValue = LiteralValue<
  typeof ChildRuntimeEntitlementAccessStateLiteral
>;
type ChildRuntimeRemoteAccessAuthorizationStateValue = LiteralValue<
  typeof ChildRuntimeRemoteAccessAuthorizationStateLiteral
>;
type ChildRuntimeEnforcementExecutionStateValue = LiteralValue<
  typeof ChildRuntimeEnforcementExecutionStateLiteral
>;

type ChildRuntimePreflightGateFields = {
  runtimeStartState: ChildRuntimeStartStateValue;
  manualReviewState: ChildRuntimeManualReviewStateValue;
  deviceAuthorization: ChildRuntimeDeviceAuthorizationStateValue;
  provisioningReadiness: ChildRuntimeProvisioningReadinessStateValue;
  entitlementAccess: ChildRuntimeEntitlementAccessStateValue;
  provisioningDecision: ChildRuntimeProvisioningDecisionFields;
};

type ChildRuntimeProvisioningDecisionFields = {
  childInstallState: Infer<typeof SetupChildInstallStateSchema>;
  childServiceState: Infer<typeof SetupChildServiceStateSchema>;
  overallState: Infer<typeof SetupReadinessOverallStateSchema>;
  blockerReason: ChildRuntimeProvisioningDecisionBlockerReasonValue | null;
};

type ChildRuntimeRemoteAccessGateFields = {
  runtimeStartState: ChildRuntimeStartStateValue;
  remoteAccessAuthorization: ChildRuntimeRemoteAccessAuthorizationStateValue;
};

type ChildRuntimeEnforcementGateFields = {
  runtimeStartState: ChildRuntimeStartStateValue;
  enforcementExecution: ChildRuntimeEnforcementExecutionStateValue;
};

export const ChildRuntimeStartStateSchema = withParser(
  Schema.Literal(ChildRuntimeStartStateLiteral.Allowed, ChildRuntimeStartStateLiteral.Blocked)
);

export const ChildRuntimeManualReviewStateSchema = withParser(
  Schema.Literal(
    ChildRuntimeManualReviewStateLiteral.NotRequired,
    ChildRuntimeManualReviewStateLiteral.Required
  )
);

export const ChildRuntimeDeviceAuthorizationStateSchema = withParser(
  Schema.Literal(
    ChildRuntimeDeviceAuthorizationStateLiteral.Authorized,
    ChildRuntimeDeviceAuthorizationStateLiteral.Rejected
  )
);

export const ChildRuntimeProvisioningReadinessStateSchema = withParser(
  Schema.Literal(
    ChildRuntimeProvisioningReadinessStateLiteral.Ready,
    ChildRuntimeProvisioningReadinessStateLiteral.NotReady
  )
);

export const ChildRuntimeProvisioningDecisionBlockerReasonSchema = withParser(
  Schema.Literal(
    ChildRuntimeProvisioningDecisionBlockerReasonLiteral.ChildInstallNotInstalled,
    ChildRuntimeProvisioningDecisionBlockerReasonLiteral.ChildServiceNotStarted,
    ChildRuntimeProvisioningDecisionBlockerReasonLiteral.ChildAppOffline,
    ChildRuntimeProvisioningDecisionBlockerReasonLiteral.ChildAppReinstallRequired
  )
);

export const ChildRuntimeEntitlementAccessStateSchema = withParser(
  Schema.Literal(
    ChildRuntimeEntitlementAccessStateLiteral.Allowed,
    ChildRuntimeEntitlementAccessStateLiteral.Blocked
  )
);

export const ChildRuntimeStorageRemoteUploadStateSchema = withParser(
  Schema.Literal(
    ChildRuntimeStorageRemoteUploadStateLiteral.Allowed,
    ChildRuntimeStorageRemoteUploadStateLiteral.Blocked
  )
);

export const ChildRuntimeRemoteAccessAuthorizationStateSchema = withParser(
  Schema.Literal(
    ChildRuntimeRemoteAccessAuthorizationStateLiteral.Allowed,
    ChildRuntimeRemoteAccessAuthorizationStateLiteral.Rejected
  )
);

export const ChildRuntimeEnforcementExecutionStateSchema = withParser(
  Schema.Literal(
    ChildRuntimeEnforcementExecutionStateLiteral.Execute,
    ChildRuntimeEnforcementExecutionStateLiteral.DoNotExecute
  )
);

export const ChildRuntimeProvisioningDecisionSchema = withParser(
  Schema.Struct({
    childInstallState: SetupChildInstallStateSchema,
    childServiceState: SetupChildServiceStateSchema,
    overallState: SetupReadinessOverallStateSchema,
    blockerReason: Schema.Union(ChildRuntimeProvisioningDecisionBlockerReasonSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (decision) =>
        childRuntimeProvisioningDecisionShapeMatchesCoherence(decision) ||
        'Expected child runtime provisioningDecision to stay coherent with setup readiness states'
    )
  )
);

export const ChildRuntimePreflightDecisionSchema = withParser(
  Schema.Struct({
    runtimeStartState: ChildRuntimeStartStateSchema,
    manualReviewState: ChildRuntimeManualReviewStateSchema,
    deviceAuthorization: ChildRuntimeDeviceAuthorizationStateSchema,
    provisioningReadiness: ChildRuntimeProvisioningReadinessStateSchema,
    entitlementAccess: ChildRuntimeEntitlementAccessStateSchema,
    remoteUpload: ChildRuntimeStorageRemoteUploadStateSchema,
    provisioningDecision: ChildRuntimeProvisioningDecisionSchema,
  }).pipe(
    Schema.filter(
      (decision) =>
        childRuntimePreflightManualReviewStateMatchesGates(decision) ||
        'Expected child runtime manualReviewState to match provisioning readiness gates'
    ),
    Schema.filter(
      (decision) =>
        childRuntimeProvisioningDecisionMatchesGates(decision) ||
        'Expected child runtime provisioningDecision to match readiness gates'
    ),
    Schema.filter(
      (decision) =>
        childRuntimePreflightStartStateMatchesGates(decision) ||
        'Expected child runtime runtimeStartState to match device, provisioning, and entitlement gates'
    )
  )
);

export const ChildRuntimeRemoteAccessDecisionSchema = withParser(
  Schema.Struct({
    runtimeStartState: ChildRuntimeStartStateSchema,
    remoteAccessAuthorization: ChildRuntimeRemoteAccessAuthorizationStateSchema,
  }).pipe(
    Schema.filter(
      (decision) =>
        childRuntimeRemoteAccessStartStateMatchesAuthorization(decision) ||
        'Expected remote access runtimeStartState to match remote access authorization'
    )
  )
);

export const ChildRuntimeEnforcementDecisionSchema = withParser(
  Schema.Struct({
    runtimeStartState: ChildRuntimeStartStateSchema,
    enforcementExecution: ChildRuntimeEnforcementExecutionStateSchema,
  }).pipe(
    Schema.filter(
      (decision) =>
        childRuntimeEnforcementStartStateMatchesExecution(decision) ||
        'Expected enforcement runtimeStartState to match enforcement execution gate'
    )
  )
);

export type ChildRuntimeStartState = Infer<typeof ChildRuntimeStartStateSchema>;
export type ChildRuntimeManualReviewState = Infer<typeof ChildRuntimeManualReviewStateSchema>;
export type ChildRuntimeDeviceAuthorizationState = Infer<
  typeof ChildRuntimeDeviceAuthorizationStateSchema
>;
export type ChildRuntimeProvisioningReadinessState = Infer<
  typeof ChildRuntimeProvisioningReadinessStateSchema
>;
export type ChildRuntimeProvisioningDecisionBlockerReason = Infer<
  typeof ChildRuntimeProvisioningDecisionBlockerReasonSchema
>;
export type ChildRuntimeEntitlementAccessState = Infer<
  typeof ChildRuntimeEntitlementAccessStateSchema
>;
export type ChildRuntimeStorageRemoteUploadState = Infer<
  typeof ChildRuntimeStorageRemoteUploadStateSchema
>;
export type ChildRuntimeRemoteAccessAuthorizationState = Infer<
  typeof ChildRuntimeRemoteAccessAuthorizationStateSchema
>;
export type ChildRuntimeEnforcementExecutionState = Infer<
  typeof ChildRuntimeEnforcementExecutionStateSchema
>;
export type ChildRuntimePreflightDecision = Infer<typeof ChildRuntimePreflightDecisionSchema>;
export type ChildRuntimeProvisioningDecision = Infer<typeof ChildRuntimeProvisioningDecisionSchema>;
export type ChildRuntimeRemoteAccessDecision = Infer<
  typeof ChildRuntimeRemoteAccessDecisionSchema
>;
export type ChildRuntimeEnforcementDecision = Infer<typeof ChildRuntimeEnforcementDecisionSchema>;

export const ChildRuntimeStartState = {
  Allowed: ChildRuntimeStartStateSchema.parse(ChildRuntimeStartStateLiteral.Allowed),
  Blocked: ChildRuntimeStartStateSchema.parse(ChildRuntimeStartStateLiteral.Blocked),
} as const;

export const ChildRuntimeManualReviewState = {
  NotRequired: ChildRuntimeManualReviewStateSchema.parse(
    ChildRuntimeManualReviewStateLiteral.NotRequired
  ),
  Required: ChildRuntimeManualReviewStateSchema.parse(
    ChildRuntimeManualReviewStateLiteral.Required
  ),
} as const;

export const ChildRuntimeDeviceAuthorizationState = {
  Authorized: ChildRuntimeDeviceAuthorizationStateSchema.parse(
    ChildRuntimeDeviceAuthorizationStateLiteral.Authorized
  ),
  Rejected: ChildRuntimeDeviceAuthorizationStateSchema.parse(
    ChildRuntimeDeviceAuthorizationStateLiteral.Rejected
  ),
} as const;

export const ChildRuntimeProvisioningReadinessState = {
  Ready: ChildRuntimeProvisioningReadinessStateSchema.parse(
    ChildRuntimeProvisioningReadinessStateLiteral.Ready
  ),
  NotReady: ChildRuntimeProvisioningReadinessStateSchema.parse(
    ChildRuntimeProvisioningReadinessStateLiteral.NotReady
  ),
} as const;

export const ChildRuntimeProvisioningDecisionBlockerReason = {
  ChildInstallNotInstalled: ChildRuntimeProvisioningDecisionBlockerReasonSchema.parse(
    ChildRuntimeProvisioningDecisionBlockerReasonLiteral.ChildInstallNotInstalled
  ),
  ChildServiceNotStarted: ChildRuntimeProvisioningDecisionBlockerReasonSchema.parse(
    ChildRuntimeProvisioningDecisionBlockerReasonLiteral.ChildServiceNotStarted
  ),
  ChildAppOffline: ChildRuntimeProvisioningDecisionBlockerReasonSchema.parse(
    ChildRuntimeProvisioningDecisionBlockerReasonLiteral.ChildAppOffline
  ),
  ChildAppReinstallRequired: ChildRuntimeProvisioningDecisionBlockerReasonSchema.parse(
    ChildRuntimeProvisioningDecisionBlockerReasonLiteral.ChildAppReinstallRequired
  ),
} as const;

export const ChildRuntimeEntitlementAccessState = {
  Allowed: ChildRuntimeEntitlementAccessStateSchema.parse(
    ChildRuntimeEntitlementAccessStateLiteral.Allowed
  ),
  Blocked: ChildRuntimeEntitlementAccessStateSchema.parse(
    ChildRuntimeEntitlementAccessStateLiteral.Blocked
  ),
} as const;

export const ChildRuntimeStorageRemoteUploadState = {
  Allowed: ChildRuntimeStorageRemoteUploadStateSchema.parse(
    ChildRuntimeStorageRemoteUploadStateLiteral.Allowed
  ),
  Blocked: ChildRuntimeStorageRemoteUploadStateSchema.parse(
    ChildRuntimeStorageRemoteUploadStateLiteral.Blocked
  ),
} as const;

export const ChildRuntimeRemoteAccessAuthorizationState = {
  Allowed: ChildRuntimeRemoteAccessAuthorizationStateSchema.parse(
    ChildRuntimeRemoteAccessAuthorizationStateLiteral.Allowed
  ),
  Rejected: ChildRuntimeRemoteAccessAuthorizationStateSchema.parse(
    ChildRuntimeRemoteAccessAuthorizationStateLiteral.Rejected
  ),
} as const;

export const ChildRuntimeEnforcementExecutionState = {
  Execute: ChildRuntimeEnforcementExecutionStateSchema.parse(
    ChildRuntimeEnforcementExecutionStateLiteral.Execute
  ),
  DoNotExecute: ChildRuntimeEnforcementExecutionStateSchema.parse(
    ChildRuntimeEnforcementExecutionStateLiteral.DoNotExecute
  ),
} as const;

function childRuntimePreflightStartStateMatchesGates(
  decision: ChildRuntimePreflightGateFields
): boolean {
  const gatesAllowStart =
    decision.deviceAuthorization === ChildRuntimeDeviceAuthorizationState.Authorized &&
    decision.provisioningReadiness === ChildRuntimeProvisioningReadinessState.Ready &&
    decision.entitlementAccess === ChildRuntimeEntitlementAccessState.Allowed &&
    decision.provisioningDecision.overallState === SetupReadinessOverallState.Ready;
  return (
    (gatesAllowStart && decision.runtimeStartState === ChildRuntimeStartState.Allowed) ||
    (!gatesAllowStart && decision.runtimeStartState === ChildRuntimeStartState.Blocked)
  );
}

function childRuntimePreflightManualReviewStateMatchesGates(
  decision: ChildRuntimePreflightGateFields
): boolean {
  return (
    (decision.provisioningDecision.overallState === SetupReadinessOverallState.Ready &&
      decision.manualReviewState === ChildRuntimeManualReviewState.NotRequired) ||
    (decision.provisioningDecision.overallState !== SetupReadinessOverallState.Ready &&
      decision.manualReviewState === ChildRuntimeManualReviewState.Required)
  );
}

function childRuntimeProvisioningDecisionMatchesGates(
  decision: ChildRuntimePreflightGateFields
): boolean {
  return (
    (decision.provisioningDecision.overallState === SetupReadinessOverallState.Ready &&
      decision.provisioningReadiness === ChildRuntimeProvisioningReadinessState.Ready &&
      decision.provisioningDecision.blockerReason === null) ||
    (decision.provisioningDecision.overallState === SetupReadinessOverallState.Degraded &&
      decision.provisioningReadiness === ChildRuntimeProvisioningReadinessState.NotReady &&
      decision.provisioningDecision.blockerReason ===
        ChildRuntimeProvisioningDecisionBlockerReason.ChildAppOffline) ||
    (decision.provisioningDecision.overallState === SetupReadinessOverallState.Blocked &&
      decision.provisioningReadiness === ChildRuntimeProvisioningReadinessState.NotReady &&
      ((decision.provisioningDecision.blockerReason ===
        ChildRuntimeProvisioningDecisionBlockerReason.ChildInstallNotInstalled &&
        decision.provisioningDecision.childInstallState === SetupChildInstallState.NotInstalled &&
        decision.provisioningDecision.childServiceState === SetupChildServiceState.NotStarted) ||
        (decision.provisioningDecision.blockerReason ===
          ChildRuntimeProvisioningDecisionBlockerReason.ChildServiceNotStarted &&
          decision.provisioningDecision.childInstallState === SetupChildInstallState.Installed &&
          decision.provisioningDecision.childServiceState === SetupChildServiceState.NotStarted) ||
        (decision.provisioningDecision.blockerReason ===
          ChildRuntimeProvisioningDecisionBlockerReason.ChildAppReinstallRequired &&
          decision.provisioningDecision.childInstallState ===
            SetupChildInstallState.ReinstallRequired &&
          decision.provisioningDecision.childServiceState === SetupChildServiceState.NotStarted)))
  );
}

function childRuntimeProvisioningDecisionShapeMatchesCoherence(
  decision: ChildRuntimeProvisioningDecisionFields
): boolean {
  return (
    (decision.overallState === SetupReadinessOverallState.Ready &&
      decision.childInstallState === SetupChildInstallState.Installed &&
      decision.childServiceState === SetupChildServiceState.ServiceStarted &&
      decision.blockerReason === null) ||
    (decision.overallState === SetupReadinessOverallState.Degraded &&
      decision.childInstallState === SetupChildInstallState.Installed &&
      decision.childServiceState === SetupChildServiceState.Offline &&
      decision.blockerReason ===
        ChildRuntimeProvisioningDecisionBlockerReason.ChildAppOffline) ||
    (decision.overallState === SetupReadinessOverallState.Blocked &&
      ((decision.childInstallState === SetupChildInstallState.NotInstalled &&
        decision.childServiceState === SetupChildServiceState.NotStarted &&
        decision.blockerReason ===
          ChildRuntimeProvisioningDecisionBlockerReason.ChildInstallNotInstalled) ||
        (decision.childInstallState === SetupChildInstallState.Installed &&
          decision.childServiceState === SetupChildServiceState.NotStarted &&
          decision.blockerReason ===
            ChildRuntimeProvisioningDecisionBlockerReason.ChildServiceNotStarted) ||
        (decision.childInstallState === SetupChildInstallState.ReinstallRequired &&
          decision.childServiceState === SetupChildServiceState.NotStarted &&
          decision.blockerReason ===
            ChildRuntimeProvisioningDecisionBlockerReason.ChildAppReinstallRequired)))
  );
}

function childRuntimeRemoteAccessStartStateMatchesAuthorization(
  decision: ChildRuntimeRemoteAccessGateFields
): boolean {
  return (
    (decision.remoteAccessAuthorization === ChildRuntimeRemoteAccessAuthorizationState.Allowed &&
      decision.runtimeStartState === ChildRuntimeStartState.Allowed) ||
    (decision.remoteAccessAuthorization === ChildRuntimeRemoteAccessAuthorizationState.Rejected &&
      decision.runtimeStartState === ChildRuntimeStartState.Blocked)
  );
}

function childRuntimeEnforcementStartStateMatchesExecution(
  decision: ChildRuntimeEnforcementGateFields
): boolean {
  return (
    (decision.enforcementExecution === ChildRuntimeEnforcementExecutionState.Execute &&
      decision.runtimeStartState === ChildRuntimeStartState.Allowed) ||
    (decision.enforcementExecution === ChildRuntimeEnforcementExecutionState.DoNotExecute &&
      decision.runtimeStartState === ChildRuntimeStartState.Blocked)
  );
}
