/* thin adapter over Rust-owned generated enforcement contracts */

import * as Generated from './generated-enforcement';
import { withParser } from './effect';

const STRICT_ENFORCEMENT_PARSER_OPTIONS = { onExcessProperty: 'error' } as const;

export const EnforcementAdapterKindSchema = withParser(
  Generated.EnforcementAdapterKindSchema,
  STRICT_ENFORCEMENT_PARSER_OPTIONS
);
export const EnforcementModeSchema = withParser(Generated.EnforcementModeSchema, STRICT_ENFORCEMENT_PARSER_OPTIONS);
export const EnforcementCapabilityStateSchema = withParser(
  Generated.EnforcementCapabilityStateSchema,
  STRICT_ENFORCEMENT_PARSER_OPTIONS
);
export const EnforcementCapabilityStatusSchema = withParser(
  Generated.EnforcementCapabilityStatusSchema,
  STRICT_ENFORCEMENT_PARSER_OPTIONS
);
export const EnforcementUnavailableStatusSchema = withParser(
  Generated.EnforcementUnavailableStatusSchema,
  STRICT_ENFORCEMENT_PARSER_OPTIONS
);
export const EnforcementIntentSchema = withParser(Generated.EnforcementIntentSchema, STRICT_ENFORCEMENT_PARSER_OPTIONS);
export const EnforcementActionSchema = withParser(Generated.EnforcementActionSchema, STRICT_ENFORCEMENT_PARSER_OPTIONS);
export const EnforcementResultSchema = withParser(Generated.EnforcementResultSchema, STRICT_ENFORCEMENT_PARSER_OPTIONS);
export const EnforcementAuditEventSchema = withParser(
  Generated.EnforcementAuditEventSchema,
  STRICT_ENFORCEMENT_PARSER_OPTIONS
);
export const EnforcementTimerEventSchema = withParser(
  Generated.EnforcementTimerEventSchema,
  STRICT_ENFORCEMENT_PARSER_OPTIONS
);
export const EnforcementActiveTimerStateSchema = withParser(
  Generated.EnforcementActiveTimerStateSchema,
  STRICT_ENFORCEMENT_PARSER_OPTIONS
);

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
