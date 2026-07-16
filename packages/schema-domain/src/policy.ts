/* thin adapter over Rust-owned generated policy contracts */

import { type Infer } from './effect';
import * as policyContracts from './policy-contracts';
import {
  compareGeneratedPolicyActionStrictness,
  resolveGeneratedPolicyPreviewBudgetBoundaryState,
  selectGeneratedStricterPolicyAction,
} from './generated-policy-control-helpers';
import { GeneratedPolicyActionStrictnessRank } from './generated-policy';

export const PolicyTimestampSchema = policyContracts.PolicyTimestampSchema;
export const PolicyRuleIdSchema = policyContracts.PolicyRuleIdSchema;
export const PolicyScheduleIdSchema = policyContracts.PolicyScheduleIdSchema;
export const PolicyTargetIdSchema = policyContracts.PolicyTargetIdSchema;
export const PermissionRequestIdSchema = policyContracts.PermissionRequestIdSchema;
export const PolicyDecisionIdSchema = policyContracts.PolicyDecisionIdSchema;
export const PolicyReasonCodeSchema = policyContracts.PolicyReasonCodeSchema;
export const PolicyLocalTimeSchema = policyContracts.PolicyLocalTimeSchema;
export const PolicyTimeZoneSchema = policyContracts.PolicyTimeZoneSchema;
export const LocalAiResultReferenceIdSchema = policyContracts.LocalAiResultReferenceIdSchema;
export const PolicyScheduleExceptionIdSchema = policyContracts.PolicyScheduleExceptionIdSchema;
export const PolicyPreviewIdSchema = policyContracts.PolicyPreviewIdSchema;

export const PolicyActionLiteral = policyContracts.PolicyActionLiteral;
export const PolicyTargetTypeLiteral = policyContracts.PolicyTargetTypeLiteral;
export const PolicyScheduleDayLiteral = policyContracts.PolicyScheduleDayLiteral;
export const PolicyDecisionHandoffStateLiteral = policyContracts.PolicyDecisionHandoffStateLiteral;
export const PermissionRequestStateLiteral = policyContracts.PermissionRequestStateLiteral;
export const PolicyScheduleBoundaryStateLiteral = policyContracts.PolicyScheduleBoundaryStateLiteral;
export const PolicyScheduleDstTransitionLiteral = policyContracts.PolicyScheduleDstTransitionLiteral;
export const PolicyScheduleDstResolutionLiteral = policyContracts.PolicyScheduleDstResolutionLiteral;
export const PolicyScheduleClockSourceLiteral = policyContracts.PolicyScheduleClockSourceLiteral;
export const PolicyScheduleBudgetResetKindLiteral = policyContracts.PolicyScheduleBudgetResetKindLiteral;
export const PolicyScheduleBudgetCarryoverModeLiteral = policyContracts.PolicyScheduleBudgetCarryoverModeLiteral;
export const PolicyScheduleOfflineRecoveryLiteral = policyContracts.PolicyScheduleOfflineRecoveryLiteral;
export const PolicyScheduleOfflineRecoveryStateLiteral = policyContracts.PolicyScheduleOfflineRecoveryStateLiteral;
export const PolicyPreviewOriginLiteral = policyContracts.PolicyPreviewOriginLiteral;
export const PolicyPreviewConfirmationStateLiteral = policyContracts.PolicyPreviewConfirmationStateLiteral;
export const PolicyPreviewBudgetBoundaryStateLiteral = policyContracts.PolicyPreviewBudgetBoundaryStateLiteral;

export const PolicyActionSchema = policyContracts.PolicyActionSchema;
export const PolicyTargetTypeSchema = policyContracts.PolicyTargetTypeSchema;
export const PolicyScheduleDaySchema = policyContracts.PolicyScheduleDaySchema;
export const PolicyDecisionHandoffStateSchema = policyContracts.PolicyDecisionHandoffStateSchema;
export const PermissionRequestStateSchema = policyContracts.PermissionRequestStateSchema;
export const PolicyScheduleBoundaryStateSchema = policyContracts.PolicyScheduleBoundaryStateSchema;
export const PolicyScheduleDstTransitionSchema = policyContracts.PolicyScheduleDstTransitionSchema;
export const PolicyScheduleDstResolutionSchema = policyContracts.PolicyScheduleDstResolutionSchema;
export const PolicyScheduleClockSourceSchema = policyContracts.PolicyScheduleClockSourceSchema;
export const PolicyScheduleBudgetResetKindSchema = policyContracts.PolicyScheduleBudgetResetKindSchema;
export const PolicyScheduleBudgetCarryoverModeSchema = policyContracts.PolicyScheduleBudgetCarryoverModeSchema;
export const PolicyScheduleOfflineRecoverySchema = policyContracts.PolicyScheduleOfflineRecoverySchema;
export const PolicyScheduleOfflineRecoveryStateSchema = policyContracts.PolicyScheduleOfflineRecoveryStateSchema;
export const PolicyPreviewOriginSchema = policyContracts.PolicyPreviewOriginSchema;
export const PolicyPreviewConfirmationStateSchema = policyContracts.PolicyPreviewConfirmationStateSchema;
export const PolicyPreviewBudgetBoundaryStateSchema = policyContracts.PolicyPreviewBudgetBoundaryStateSchema;

export const PolicyTargetSchema = policyContracts.PolicyTargetSchema;
export const PolicyScheduleWindowSchema = policyContracts.PolicyScheduleWindowSchema;
export const PolicyScheduleBudgetResetSchema = policyContracts.PolicyScheduleBudgetResetSchema;
export const PolicyScheduleBudgetCarryoverSchema = policyContracts.PolicyScheduleBudgetCarryoverSchema;
export const PolicyScheduleTimeBudgetSchema = policyContracts.PolicyScheduleTimeBudgetSchema;
export const PolicyScheduleSchema = policyContracts.PolicyScheduleSchema;
export const PolicyScheduleDstBoundarySchema = policyContracts.PolicyScheduleDstBoundarySchema;
export const PolicyScheduleClockSkewSchema = policyContracts.PolicyScheduleClockSkewSchema;
export const PolicyScheduleExceptionSchema = policyContracts.PolicyScheduleExceptionSchema;
export const PolicyScheduleExpirySchema = policyContracts.PolicyScheduleExpirySchema;
export const PolicyScheduleOfflineRecoveryStatusSchema = policyContracts.PolicyScheduleOfflineRecoveryStatusSchema;
export const PolicyScheduleTimeBudgetStatusSchema = policyContracts.PolicyScheduleTimeBudgetStatusSchema;
export const PolicyScheduleBoundarySchema = policyContracts.PolicyScheduleBoundarySchema;
export const PolicyRuleSchema = policyContracts.PolicyRuleSchema;
export const FamilyPolicySetSchema = policyContracts.FamilyPolicySetSchema;
export const PermissionRequestSchema = policyContracts.PermissionRequestSchema;
export const PolicyDecisionSchema = policyContracts.PolicyDecisionSchema;
export const PolicyPreviewSchema = policyContracts.PolicyPreviewSchema;

export type PolicyRuleId = typeof PolicyRuleIdSchema.Type;
export type PolicyScheduleId = typeof PolicyScheduleIdSchema.Type;
export type PolicyTargetId = typeof PolicyTargetIdSchema.Type;
export type PermissionRequestId = typeof PermissionRequestIdSchema.Type;
export type PolicyDecisionId = typeof PolicyDecisionIdSchema.Type;
export type PolicyReasonCode = typeof PolicyReasonCodeSchema.Type;
export type LocalAiResultReferenceId = typeof LocalAiResultReferenceIdSchema.Type;
export type PolicyScheduleExceptionId = typeof PolicyScheduleExceptionIdSchema.Type;
export type PolicyPreviewId = typeof PolicyPreviewIdSchema.Type;
export type PolicyAction = Infer<typeof PolicyActionSchema>;
export type PolicyTargetType = Infer<typeof PolicyTargetTypeSchema>;
export type PolicyScheduleDay = Infer<typeof PolicyScheduleDaySchema>;
export type PolicyDecisionHandoffState = Infer<typeof PolicyDecisionHandoffStateSchema>;
export type PermissionRequestState = Infer<typeof PermissionRequestStateSchema>;
export type PolicyScheduleBoundaryState = Infer<typeof PolicyScheduleBoundaryStateSchema>;
export type PolicyScheduleDstTransition = Infer<typeof PolicyScheduleDstTransitionSchema>;
export type PolicyScheduleDstResolution = Infer<typeof PolicyScheduleDstResolutionSchema>;
export type PolicyScheduleClockSource = Infer<typeof PolicyScheduleClockSourceSchema>;
export type PolicyScheduleBudgetResetKind = Infer<typeof PolicyScheduleBudgetResetKindSchema>;
export type PolicyScheduleBudgetCarryoverMode = Infer<typeof PolicyScheduleBudgetCarryoverModeSchema>;
export type PolicyScheduleOfflineRecovery = Infer<typeof PolicyScheduleOfflineRecoverySchema>;
export type PolicyScheduleOfflineRecoveryState = Infer<typeof PolicyScheduleOfflineRecoveryStateSchema>;
export type PolicyTarget = Infer<typeof PolicyTargetSchema>;
export type PolicyScheduleWindow = Infer<typeof PolicyScheduleWindowSchema>;
export type PolicyScheduleBudgetReset = Infer<typeof PolicyScheduleBudgetResetSchema>;
export type PolicyScheduleBudgetCarryover = Infer<typeof PolicyScheduleBudgetCarryoverSchema>;
export type PolicyScheduleTimeBudget = Infer<typeof PolicyScheduleTimeBudgetSchema>;
export type PolicySchedule = Infer<typeof PolicyScheduleSchema>;
export type PolicyScheduleDstBoundary = Infer<typeof PolicyScheduleDstBoundarySchema>;
export type PolicyScheduleClockSkew = Infer<typeof PolicyScheduleClockSkewSchema>;
export type PolicyScheduleException = Infer<typeof PolicyScheduleExceptionSchema>;
export type PolicyScheduleExpiry = Infer<typeof PolicyScheduleExpirySchema>;
export type PolicyScheduleOfflineRecoveryStatus = Infer<typeof PolicyScheduleOfflineRecoveryStatusSchema>;
export type PolicyScheduleTimeBudgetStatus = Infer<typeof PolicyScheduleTimeBudgetStatusSchema>;
export type PolicyScheduleBoundary = Infer<typeof PolicyScheduleBoundarySchema>;
export type PolicyRule = Infer<typeof PolicyRuleSchema>;
export type FamilyPolicySet = Infer<typeof FamilyPolicySetSchema>;
export type PermissionRequest = Infer<typeof PermissionRequestSchema>;
export type PolicyDecision = Infer<typeof PolicyDecisionSchema>;
export type PolicyPreviewOrigin = Infer<typeof PolicyPreviewOriginSchema>;
export type PolicyPreviewConfirmationState = Infer<typeof PolicyPreviewConfirmationStateSchema>;
export type PolicyPreviewBudgetBoundaryState = Infer<typeof PolicyPreviewBudgetBoundaryStateSchema>;
export type PolicyPreview = Infer<typeof PolicyPreviewSchema>;

export const PolicyAction = policyContracts.PolicyAction;
export const PolicyTargetType = policyContracts.PolicyTargetType;
export const PolicyScheduleDay = policyContracts.PolicyScheduleDay;
export const PolicyDecisionHandoffState = policyContracts.PolicyDecisionHandoffState;
export const PermissionRequestState = policyContracts.PermissionRequestState;
export const PolicyScheduleBoundaryState = policyContracts.PolicyScheduleBoundaryState;
export const PolicyScheduleDstTransition = policyContracts.PolicyScheduleDstTransition;
export const PolicyScheduleDstResolution = policyContracts.PolicyScheduleDstResolution;
export const PolicyScheduleClockSource = policyContracts.PolicyScheduleClockSource;
export const PolicyScheduleBudgetResetKind = policyContracts.PolicyScheduleBudgetResetKind;
export const PolicyScheduleBudgetCarryoverMode = policyContracts.PolicyScheduleBudgetCarryoverMode;
export const PolicyScheduleOfflineRecovery = policyContracts.PolicyScheduleOfflineRecovery;
export const PolicyScheduleOfflineRecoveryState = policyContracts.PolicyScheduleOfflineRecoveryState;
export const PolicyPreviewOrigin = policyContracts.PolicyPreviewOrigin;
export const PolicyPreviewConfirmationState = policyContracts.PolicyPreviewConfirmationState;
export const PolicyPreviewBudgetBoundaryState = policyContracts.PolicyPreviewBudgetBoundaryState;

export const PolicyActionStrictnessRank = GeneratedPolicyActionStrictnessRank as Readonly<Record<PolicyAction, number>>;

export const parsePolicySchedule = policyContracts.parsePolicySchedule;
export const parseFamilyPolicySet = policyContracts.parseFamilyPolicySet;
export const parsePolicyScheduleBoundary = policyContracts.parsePolicyScheduleBoundary;
export const parsePolicyPreview = policyContracts.parsePolicyPreview;

export function resolvePolicyPreviewBudgetBoundaryState(
  boundary: PolicyScheduleBoundary | null
): PolicyPreviewBudgetBoundaryState {
  return resolveGeneratedPolicyPreviewBudgetBoundaryState(boundary);
}

export function comparePolicyActionStrictness(left: PolicyAction, right: PolicyAction): number {
  return compareGeneratedPolicyActionStrictness(left, right);
}

export function selectStricterPolicyAction(parentRuleAction: PolicyAction, localAiAction: PolicyAction): PolicyAction {
  return selectGeneratedStricterPolicyAction(parentRuleAction, localAiAction);
}
