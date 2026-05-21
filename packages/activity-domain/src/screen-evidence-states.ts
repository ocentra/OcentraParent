import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const ScreenAnalysisModeSchema = withParser(
  Schema.Literal('observeOnly', 'policyDryRun', 'enforcementEligible')
);

export const ScreenCaptureTriggerSchema = withParser(
  Schema.Literal(
    'foregroundAppChange',
    'managedUrlChange',
    'appGameForegroundStart',
    'unusualNetworkChange',
    'policyAmbiguity',
    'manualParentTestCapture'
  )
);

export const ScreenCaptureReasonSchema = withParser(
  Schema.Literal(
    'cadence',
    'foregroundAppChange',
    'managedUrlChange',
    'appGameForegroundStart',
    'unusualNetworkChange',
    'policyAmbiguity',
    'manualParentTestCapture',
    'retry'
  )
);

export const ScreenCaptureScopeSchema = withParser(
  Schema.Literal('fullScreen', 'activeDisplay', 'activeWindow', 'managedBrowserWindow', 'unsupported')
);

export const ScreenCapabilityStatusSchema = withParser(
  Schema.Literal(
    'disabledByParent',
    'unsupportedPlatform',
    'unsupportedScope',
    'permissionRequired',
    'permissionLimited',
    'protectedSurface',
    'screenLocked',
    'sessionUnavailable',
    'modelUnavailable',
    'queueUnavailable',
    'degraded',
    'adapterError',
    'ready'
  )
);

export const ScreenQueueStatusSchema = withParser(
  Schema.Literal(
    'queued',
    'processing',
    'analyzed',
    'deleting',
    'deleted',
    'expired',
    'failed',
    'invalid',
    'unavailable',
    'permissionLimited',
    'protectedSurface',
    'canceled'
  )
);

export const ScreenDeletionStateSchema = withParser(
  Schema.Literal('deletionRequired', 'deleted', 'deletePending', 'deleteFailed', 'expiredDeleted', 'unavailableNoImage')
);

export const ScreenEvidenceCustodyStateSchema = withParser(
  Schema.Literal(
    'live-local-child-agent',
    'live-lan-child-agent',
    'child-device-temp-queue',
    'child-device-journal',
    'child-device-query-store',
    'parent-device-cache',
    'parent-owned-export',
    'ocentra-hosted-non-activity',
    'unavailable'
  )
);

export const ScreenImageFormatSchema = withParser(Schema.Literal('png', 'jpeg', 'webp', 'bmp', 'unknown'));
export const ScreenLocalModelProviderKindSchema = withParser(
  Schema.Literal('localOcr', 'localVision', 'localMultimodal', 'unavailable')
);
export const ScreenVisibleCategorySchema = withParser(
  Schema.Literal(
    'school',
    'video',
    'chat',
    'game',
    'adultContent',
    'violence',
    'bypassTool',
    'shopping',
    'productivity',
    'unknown'
  )
);
export const ScreenRiskSignalSchema = withParser(
  Schema.Literal(
    'possibleBypassTool',
    'credentialPrompt',
    'unsafeVisibleContent',
    'selfHarmSignal',
    'explicitContentSignal',
    'unknown'
  )
);
export const ScreenRedactionModeSchema = withParser(
  Schema.Literal('disabled', 'localSensitiveText', 'localSensitiveRegions')
);
export const ScreenRedactionNoteSchema = withParser(
  Schema.Literal('credentialLikeTextRedacted', 'protectedRegionSkipped', 'ocrDisabled', 'noTextExtracted')
);
export const ScreenUncertaintyReasonSchema = withParser(
  Schema.Literal(
    'lowConfidence',
    'ambiguousImage',
    'unsupportedLanguage',
    'protectedSurface',
    'modelUnavailable',
    'insufficientPixels',
    'unknown'
  )
);

export type ScreenAnalysisMode = Infer<typeof ScreenAnalysisModeSchema>;
export type ScreenCaptureTrigger = Infer<typeof ScreenCaptureTriggerSchema>;
export type ScreenCaptureReason = Infer<typeof ScreenCaptureReasonSchema>;
export type ScreenCaptureScope = Infer<typeof ScreenCaptureScopeSchema>;
export type ScreenCapabilityStatus = Infer<typeof ScreenCapabilityStatusSchema>;
export type ScreenQueueStatus = Infer<typeof ScreenQueueStatusSchema>;
export type ScreenDeletionState = Infer<typeof ScreenDeletionStateSchema>;
export type ScreenEvidenceCustodyState = Infer<typeof ScreenEvidenceCustodyStateSchema>;
export type ScreenVisibleCategory = Infer<typeof ScreenVisibleCategorySchema>;
export type ScreenRiskSignal = Infer<typeof ScreenRiskSignalSchema>;
