import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

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
type ChildRuntimeDeviceAuthorizationStateValue = LiteralValue<
  typeof ChildRuntimeDeviceAuthorizationStateLiteral
>;
type ChildRuntimeProvisioningReadinessStateValue = LiteralValue<
  typeof ChildRuntimeProvisioningReadinessStateLiteral
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
  deviceAuthorization: ChildRuntimeDeviceAuthorizationStateValue;
  provisioningReadiness: ChildRuntimeProvisioningReadinessStateValue;
  entitlementAccess: ChildRuntimeEntitlementAccessStateValue;
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

export const ChildRuntimePreflightDecisionSchema = withParser(
  Schema.Struct({
    runtimeStartState: ChildRuntimeStartStateSchema,
    manualReviewState: ChildRuntimeManualReviewStateSchema,
    deviceAuthorization: ChildRuntimeDeviceAuthorizationStateSchema,
    provisioningReadiness: ChildRuntimeProvisioningReadinessStateSchema,
    entitlementAccess: ChildRuntimeEntitlementAccessStateSchema,
    remoteUpload: ChildRuntimeStorageRemoteUploadStateSchema,
  }).pipe(
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
    decision.entitlementAccess === ChildRuntimeEntitlementAccessState.Allowed;
  return (
    (gatesAllowStart && decision.runtimeStartState === ChildRuntimeStartState.Allowed) ||
    (!gatesAllowStart && decision.runtimeStartState === ChildRuntimeStartState.Blocked)
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
