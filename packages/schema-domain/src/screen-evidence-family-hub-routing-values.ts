import { Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { ActivityEvidenceIdSchema } from './evidence-primitives';
import { ScreenEvidenceModelRuntimeRefSchema } from './screen-evidence-primitives';

export const ScreenFamilyAiHubRouteSchemaVersion = 1;

export const ScreenFamilyAiHubRouteIdSchema = withParser(brandedNonEmptyStringSchema('ScreenFamilyAiHubRouteId'));
export const ScreenFamilyAiHubIdSchema = withParser(brandedNonEmptyStringSchema('ScreenFamilyAiHubId'));
export const ScreenFamilyAiHubRouteRefSchema = withParser(brandedNonEmptyStringSchema('ScreenFamilyAiHubRouteRef'));

export const ScreenFamilyAiHubRequestedTaskSchema = withParser(
  Schema.Literal('guidedVisionClassification', 'guidedMultimodalClassification', 'ocrTextFallback')
);
export const ScreenFamilyAiHubCapabilityStateSchema = withParser(
  Schema.Literal('available', 'disabledByParent', 'hubUnavailable', 'lanProofMissing', 'resourceExhausted')
);
export const ScreenFamilyAiHubDegradedStateSchema = withParser(
  Schema.Literal(
    'childLocalAlreadySelected',
    'parentDisabled',
    'hubUnavailable',
    'lanProofMissing',
    'resourceExhausted',
    'unsupportedTask',
    'custodyUnsafe',
    'manualRequired'
  )
);
export const ScreenFamilyAiHubExecutionStateSchema = withParser(
  Schema.Literal('selected', 'manualRequired', 'unavailable')
);
export const ScreenFamilyAiHubTransferModeSchema = withParser(
  Schema.Literal('summaryOnly', 'redactedCrop', 'noTransfer')
);
export const ScreenChildLocalAnalysisAttemptStateSchema = withParser(
  Schema.Literal('selected', 'degraded', 'manualRequired', 'unavailable')
);

export const ScreenFamilyAiHubOptionalTextSchema = Schema.Union(NonEmptyStringSchema, Schema.Null);
export const ScreenFamilyAiHubOptionalRuntimeRefSchema = Schema.Union(ScreenEvidenceModelRuntimeRefSchema, Schema.Null);
export const ScreenFamilyAiHubRequiredFalseSchema = Schema.Literal(false);
export const ScreenFamilyAiHubRequiredTrueSchema = Schema.Literal(true);
export const ScreenFamilyAiHubSupportedTasksSchema = Schema.Array(ScreenFamilyAiHubRequestedTaskSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one screen family AI hub supported task')
);
export const ScreenFamilyAiHubAuditEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one screen family AI hub audit evidence id')
);
export const ScreenFamilyAiHubDegradedStatesSchema = Schema.Array(ScreenFamilyAiHubDegradedStateSchema);
