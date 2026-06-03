import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import {
  ScreenAnalysisModeSchema,
  ScreenCapabilityStatusSchema,
  ScreenCaptureScopeSchema,
  ScreenCaptureTriggerSchema,
  ScreenEvidenceCustodyStateSchema,
  ScreenRedactionModeSchema,
} from './screen-evidence-states';
import {
  ScreenEvidenceCadenceSecondsSchema,
  ScreenEvidenceParentSettingRefSchema,
  ScreenEvidenceReasonSchema,
  ScreenEvidenceRetryCountSchema,
  ScreenEvidenceSchemaVersion,
  ScreenEvidenceSettingVersionSchema,
  ScreenEvidenceSnippetLimitSchema,
  ScreenEvidenceTtlSecondsSchema,
} from './screen-evidence-primitives';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);

export const ScreenAnalysisParentSettingSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
    screenAnalysisEnabled: Schema.Boolean,
    analysisMode: ScreenAnalysisModeSchema,
    cadenceCaptureEnabled: Schema.Boolean,
    cadenceSeconds: ScreenEvidenceCadenceSecondsSchema,
    strictModeEnabled: Schema.Boolean,
    triggerCaptureEnabled: Schema.Boolean,
    enabledTriggers: Schema.Array(ScreenCaptureTriggerSchema),
    allowedCaptureScope: ScreenCaptureScopeSchema,
    ocrTextEnabled: Schema.Boolean,
    ocrTextSnippetLimit: ScreenEvidenceSnippetLimitSchema,
    redactionMode: ScreenRedactionModeSchema,
    temporaryImageTtlSeconds: ScreenEvidenceTtlSecondsSchema,
    maxRetryCount: ScreenEvidenceRetryCountSchema,
    deleteAfterSuccess: RequiredTrue,
    deleteAfterExpiry: RequiredTrue,
    retainRawImage: RequiredFalse,
    policyUseEnabled: Schema.Boolean,
    changedByParentRef: ScreenEvidenceParentSettingRefSchema,
    changedAt: ActivityTimestampSchema,
    settingVersion: ScreenEvidenceSettingVersionSchema,
    reason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (value) =>
        value.screenAnalysisEnabled ||
        (!value.cadenceCaptureEnabled &&
          !value.strictModeEnabled &&
          !value.triggerCaptureEnabled &&
          !value.policyUseEnabled) ||
        'Expected disabled screen analysis settings to keep capture and policy use disabled'
    ),
    Schema.filter(
      (value) =>
        !value.policyUseEnabled ||
        (value.screenAnalysisEnabled && value.analysisMode !== 'observeOnly') ||
        'Expected screen policy use to require parent opt-in and non-observe analysis mode'
    ),
    Schema.filter(
      (value) =>
        !value.strictModeEnabled ||
        (value.screenAnalysisEnabled && value.cadenceCaptureEnabled && value.cadenceSeconds === 60) ||
        'Expected strict screen analysis mode to be parent enabled with one-minute cadence capture'
    ),
    Schema.filter(
      (value) =>
        !value.triggerCaptureEnabled ||
        (value.screenAnalysisEnabled && value.enabledTriggers.length > 0) ||
        'Expected trigger capture to require parent opt-in and at least one trigger'
    )
  )
);

export const ScreenCapabilitySnapshotSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
    observedAt: ActivityTimestampSchema,
    capabilityStatus: ScreenCapabilityStatusSchema,
    captureScope: ScreenCaptureScopeSchema,
    parentSettingRef: ScreenEvidenceParentSettingRefSchema,
    settingVersion: ScreenEvidenceSettingVersionSchema,
    unavailableReason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
    custodyState: ScreenEvidenceCustodyStateSchema,
  })
);

export type ScreenAnalysisParentSetting = Infer<typeof ScreenAnalysisParentSettingSchema>;
export type ScreenCapabilitySnapshot = Infer<typeof ScreenCapabilitySnapshotSchema>;
