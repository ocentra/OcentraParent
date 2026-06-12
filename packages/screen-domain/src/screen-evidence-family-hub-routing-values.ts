import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema } from '@ocentra-parent/evidence-domain/primitives';
import { ScreenEvidenceModelRuntimeRefSchema } from './screen-evidence-primitives';

const NonEmptyFamilyHubText = Schema.String.pipe(Schema.minLength(1));

export const ScreenFamilyAiHubRouteSchemaVersion = 1;

export const ScreenFamilyAiHubRouteIdSchema = withParser(
  NonEmptyFamilyHubText.pipe(Schema.brand('ScreenFamilyAiHubRouteId'))
);
export const ScreenFamilyAiHubIdSchema = withParser(NonEmptyFamilyHubText.pipe(Schema.brand('ScreenFamilyAiHubId')));
export const ScreenFamilyAiHubRouteRefSchema = withParser(
  NonEmptyFamilyHubText.pipe(Schema.brand('ScreenFamilyAiHubRouteRef'))
);

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

export const ScreenFamilyAiHubOptionalTextSchema = Schema.Union(NonEmptyFamilyHubText, Schema.Null);
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
