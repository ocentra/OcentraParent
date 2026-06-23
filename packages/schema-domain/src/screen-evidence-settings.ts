import { type Infer, Schema, withParser } from './effect';
import { ActivityTimestampSchema } from './evidence-primitives';
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
  ScreenEvidenceRemoteApprovalRefSchema,
  ScreenEvidenceRetryCountSchema,
  ScreenEvidenceSchemaVersion,
  ScreenEvidenceSettingVersionSchema,
  ScreenEvidenceSnippetLimitSchema,
  ScreenEvidenceTtlSecondsSchema,
} from './screen-evidence-primitives';
import { ScreenOcrTextRetentionModeSchema } from './screen-ocr-contracts';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const ScreenDisabledMode = Schema.Literal('disabled');
const ScreenRawScreenshotRetentionModeSchema = withParser(Schema.Literal('disabled', 'parentApprovedLocalShortTtl'));
const ScreenRemoteSummaryModeSchema = withParser(Schema.Literal('disabled', 'parentApprovedRedactedSummary'));
const ApprovedRawRetentionMaxTtlSeconds = 120;

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
    ocrTextRetentionMode: ScreenOcrTextRetentionModeSchema,
    credentialSuppressionEnabled: RequiredTrue,
    piiRedactionEnabled: Schema.Boolean,
    temporaryImageTtlSeconds: ScreenEvidenceTtlSecondsSchema,
    maxRetryCount: ScreenEvidenceRetryCountSchema,
    deleteAfterSuccess: RequiredTrue,
    deleteAfterExpiry: RequiredTrue,
    retainRawImage: Schema.Boolean,
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
    ),
    Schema.filter(
      (value) =>
        value.ocrTextEnabled ||
        (value.ocrTextSnippetLimit === 0 &&
          value.redactionMode === 'disabled' &&
          value.ocrTextRetentionMode === 'disabled' &&
          !value.piiRedactionEnabled) ||
        'Expected disabled OCR text settings to retain no snippets or PII redaction mode'
    ),
    Schema.filter(
      (value) =>
        !value.ocrTextEnabled ||
        (value.ocrTextSnippetLimit > 0 &&
          value.redactionMode !== 'disabled' &&
          value.ocrTextRetentionMode !== 'disabled') ||
        'Expected enabled OCR text settings to select explicit snippet retention and redaction behavior'
    ),
    Schema.filter(
      (value) =>
        !value.retainRawImage ||
        (value.screenAnalysisEnabled &&
          value.temporaryImageTtlSeconds <= ApprovedRawRetentionMaxTtlSeconds &&
          value.deleteAfterSuccess &&
          value.deleteAfterExpiry) ||
        'Expected raw screenshot retention to require parent-enabled local short TTL custody with deletion after success and expiry'
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

export const ScreenEvidenceRemoteBoundarySettingSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
    parentSettingRef: ScreenEvidenceParentSettingRefSchema,
    settingVersion: ScreenEvidenceSettingVersionSchema,
    rawScreenshotRetentionMode: ScreenRawScreenshotRetentionModeSchema,
    liveViewMode: ScreenDisabledMode,
    rawScreenshotRemoteUploadEnabled: RequiredFalse,
    remoteSummaryMode: ScreenRemoteSummaryModeSchema,
    remoteSummaryRedactedOnly: RequiredTrue,
    parentApprovedRemoteSummary: Schema.Boolean,
    remoteSummaryApprovalRef: Schema.Union(ScreenEvidenceRemoteApprovalRefSchema, Schema.Null),
    remoteSummaryDestinationCustodyState: ScreenEvidenceCustodyStateSchema,
    changedByParentRef: ScreenEvidenceParentSettingRefSchema,
    changedAt: ActivityTimestampSchema,
    reason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (value) =>
        value.remoteSummaryMode !== 'parentApprovedRedactedSummary' ||
        (value.parentApprovedRemoteSummary &&
          value.remoteSummaryApprovalRef !== null &&
          value.remoteSummaryDestinationCustodyState === 'parent-owned-export') ||
        'Expected remote screen summaries to require parent approval, audit ref, and parent-owned export custody'
    ),
    Schema.filter(
      (value) =>
        value.remoteSummaryMode !== 'disabled' ||
        (!value.parentApprovedRemoteSummary &&
          value.remoteSummaryApprovalRef === null &&
          value.remoteSummaryDestinationCustodyState === 'unavailable') ||
        'Expected disabled remote screen summaries to have no approval ref or destination custody'
    ),
    Schema.filter(
      (value) =>
        value.rawScreenshotRetentionMode !== 'parentApprovedLocalShortTtl' ||
        (!value.rawScreenshotRemoteUploadEnabled && value.liveViewMode === 'disabled') ||
        'Expected local raw screenshot retention mode to keep raw remote upload and live view disabled'
    )
  )
);

export type ScreenAnalysisParentSetting = Infer<typeof ScreenAnalysisParentSettingSchema>;
export type ScreenCapabilitySnapshot = Infer<typeof ScreenCapabilitySnapshotSchema>;
export type ScreenEvidenceRemoteBoundarySetting = Infer<typeof ScreenEvidenceRemoteBoundarySettingSchema>;
