/* thin adapter over Rust-owned generated enforcement contracts */

import * as Generated from './generated-enforcement';

export const EnforcementAdapterKindSchema = Generated.EnforcementAdapterKindSchema;
export const EnforcementModeSchema = Generated.EnforcementModeSchema;
export const EnforcementCapabilityStateSchema = Generated.EnforcementCapabilityStateSchema;
export const EnforcementCapabilityStatusSchema = Generated.EnforcementCapabilityStatusSchema;
export const EnforcementUnavailableStatusSchema = Generated.EnforcementUnavailableStatusSchema;
export const EnforcementIntentSchema = Generated.EnforcementIntentSchema;
export const EnforcementActionSchema = Generated.EnforcementActionSchema;
export const EnforcementResultSchema = Generated.EnforcementResultSchema;
export const EnforcementAuditEventSchema = Generated.EnforcementAuditEventSchema;
export const EnforcementTimerEventSchema = Generated.EnforcementTimerEventSchema;
export const EnforcementActiveTimerStateSchema = Generated.EnforcementActiveTimerStateSchema;

export type EnforcementCapabilityStatus = Generated.EnforcementCapabilityStatus;
export type EnforcementUnavailableStatus = Generated.EnforcementUnavailableStatus;
export type EnforcementIntent = Generated.EnforcementIntent;
export type EnforcementAction = Generated.EnforcementAction;
export type EnforcementResult = Generated.EnforcementResult;
export type EnforcementAuditEvent = Generated.EnforcementAuditEvent;
export type EnforcementTimerEvent = Generated.EnforcementTimerEvent;
export type EnforcementActiveTimerState = Generated.EnforcementActiveTimerState;

export const EnforcementIntentSource = Generated.EnforcementIntentSource;
export const EnforcementAdapterKind = Generated.EnforcementAdapterKind;
export const EnforcementMode = Generated.EnforcementMode;
export const EnforcementCapabilityState = Generated.EnforcementCapabilityState;
export const EnforcementUnavailableReason = Generated.EnforcementUnavailableReason;
export const EnforcementResultStatus = Generated.EnforcementResultStatus;
export const EnforcementRollbackState = Generated.EnforcementRollbackState;
export const EnforcementAdapterResultCode = Generated.EnforcementAdapterResultCode;
export const EnforcementTimerEventKind = Generated.EnforcementTimerEventKind;
export const EnforcementAuditEventKind = Generated.EnforcementAuditEventKind;

export const enforcementCapabilityStatusReasonIsConsistent = Generated.enforcementCapabilityStatusReasonIsConsistent;
export const enforcementUnavailableStatusIsConsistent = Generated.enforcementUnavailableStatusIsConsistent;
export const enforcementCapabilityStatusesMatch = Generated.enforcementCapabilityStatusesMatch;
export const enforcementUnavailableStatusesMatch = Generated.enforcementUnavailableStatusesMatch;
export const enforcementAuditEventBoundaryIsConsistent = Generated.enforcementAuditEventBoundaryIsConsistent;
export const enforcementTimerUnavailableReasonIsConsistent = Generated.enforcementTimerUnavailableReasonIsConsistent;
export const enforcementActiveTimerStateIsConsistent = Generated.enforcementActiveTimerStateIsConsistent;
