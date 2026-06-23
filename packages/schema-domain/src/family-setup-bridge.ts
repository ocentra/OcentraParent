import { HouseholdAuthorityInputSchema, ParentStepUpAssertionSchema } from './family-household-authority';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import { RecoveryOperationSchema as FamilyRecoveryOperationSchema } from './family-restore-lifecycle';
import { SetupInviteSchema } from './family-setup-invite';
import { type Infer, Schema, withParser } from './effect';
import {
  SetupAccountReadinessStateSchema,
  SetupChildAppReadinessStateSchema,
  SetupChildInstallStateSchema,
  SetupChildServiceStateSchema,
  SetupDataCustodySyncStateSchema,
  SetupNetworkReachabilityStateSchema,
  SetupParentAppReadinessStateSchema,
  SetupPermissionReadinessStateSchema,
  SetupPolicyBaselineStateSchema,
  SetupReadinessReportIdSchema,
  SetupRecoveryKindSchema,
  SetupRecoveryOperationIdSchema,
  SetupRecoveryStateSchema,
} from './setup-readiness';
import {
  SetupPairingApprovalChallengeSchema,
  SetupPairingApprovalResponseSchema,
  SetupPairingFailureReasonSchema,
  SetupPairingIntentIdSchema,
  SetupPairingStateSchema,
} from './setup-pairing-intent';

export const SetupFamilyReadinessInputSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readinessReportId: SetupReadinessReportIdSchema,
    family: FamilyReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    parentDevice: ParentDeviceReferenceSchema,
    childProfile: ChildProfileReferenceSchema,
    pairingIntentId: SetupPairingIntentIdSchema,
    setupInvite: SetupInviteSchema,
    householdAuthorityInput: HouseholdAuthorityInputSchema,
    recoveryOperation: Schema.Union(FamilyRecoveryOperationSchema, Schema.Null),
    parentStepUpAssertion: Schema.optionalWith(Schema.Union(ParentStepUpAssertionSchema, Schema.Null), {
      default: () => null,
    }),
    pairingApprovalChallenge: Schema.optionalWith(Schema.Union(SetupPairingApprovalChallengeSchema, Schema.Null), {
      default: () => null,
    }),
    pairingApprovalResponse: Schema.optionalWith(Schema.Union(SetupPairingApprovalResponseSchema, Schema.Null), {
      default: () => null,
    }),
    parentAppState: SetupParentAppReadinessStateSchema,
    childAppState: SetupChildAppReadinessStateSchema,
    childInstallState: Schema.optionalWith(Schema.Union(SetupChildInstallStateSchema, Schema.Null), {
      default: () => null,
    }),
    childServiceState: Schema.optionalWith(Schema.Union(SetupChildServiceStateSchema, Schema.Null), {
      default: () => null,
    }),
    permissionState: SetupPermissionReadinessStateSchema,
    policyBaselineState: SetupPolicyBaselineStateSchema,
    networkReachabilityState: SetupNetworkReachabilityStateSchema,
    custodySyncPending: Schema.Boolean,
    replayDetected: Schema.Boolean,
    staleCode: Schema.Boolean,
    childDeviceRevoked: Schema.Boolean,
    observedAt: ParentTimestampSchema,
  })
);

export const SetupFamilyRecoveryOperationInputSchema = withParser(
  Schema.Struct({
    recoveryOperationId: SetupRecoveryOperationIdSchema,
    setupRecoveryKind: SetupRecoveryKindSchema,
    parentAccount: ParentAccountReferenceSchema,
    parentDevice: ParentDeviceReferenceSchema,
    childProfile: ChildProfileReferenceSchema,
    childDevice: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
    sourcePairingState: SetupPairingStateSchema,
    familyRecoveryOperation: FamilyRecoveryOperationSchema,
  })
);

export const SetupPairingProjectionSchema = withParser(
  Schema.Struct({
    pairingState: SetupPairingStateSchema,
    failureReason: Schema.Union(SetupPairingFailureReasonSchema, Schema.Null),
    accountState: SetupAccountReadinessStateSchema,
    recoveryState: SetupRecoveryStateSchema,
  })
);

export const SetupRecoveryProjectionSchema = withParser(
  Schema.Struct({
    accountState: SetupAccountReadinessStateSchema,
    recoveryState: SetupRecoveryStateSchema,
    dataCustodySyncState: SetupDataCustodySyncStateSchema,
  })
);

export type SetupFamilyReadinessInput = Infer<typeof SetupFamilyReadinessInputSchema>;
export type SetupFamilyRecoveryOperationInput = Infer<typeof SetupFamilyRecoveryOperationInputSchema>;
export type SetupPairingProjection = Infer<typeof SetupPairingProjectionSchema>;
export type SetupRecoveryProjection = Infer<typeof SetupRecoveryProjectionSchema>;
