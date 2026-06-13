import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const ScreenOptionalVisibilityModeSchemaVersion = 1;

export const ScreenOptionalVisibilitySettingIdSchema = brandedNonEmptyStringSchema('ScreenOptionalVisibilitySettingId');
export const ScreenOptionalVisibilityApprovalRefSchema = brandedNonEmptyStringSchema('ScreenOptionalVisibilityApprovalRef');
export const ScreenOptionalVisibilityAuditRefSchema = brandedNonEmptyStringSchema('ScreenOptionalVisibilityAuditRef');
export const ScreenOptionalVisibilityPlatformProofRefSchema = brandedNonEmptyStringSchema('ScreenOptionalVisibilityPlatformProofRef');
export const ScreenOptionalVisibilityExportRefSchema = brandedNonEmptyStringSchema('ScreenOptionalVisibilityExportRef');

export const ScreenRawScreenshotRetentionModeSchema = withParser(
  Schema.Literal('disabled', 'localShortTtl', 'parentOwnedExport')
);

export const ScreenLiveViewModeSchema = withParser(Schema.Literal('disabled', 'lanOnlyView', 'relayBackedView'));

export const ScreenLiveViewTransportModeSchema = withParser(
  Schema.Literal('none', 'lanMutualAuth', 'relayEndToEndEncrypted')
);

export const ScreenOptionalVisibilityDisclosureStateSchema = withParser(Schema.Literal('notRequired', 'requiredShown'));

export const ScreenOptionalVisibilityPlatformProofStateSchema = withParser(
  Schema.Literal('notRequired', 'missing', 'operatorVerified')
);

export const ScreenOptionalVisibilitySourceLabelSchema = withParser(
  Schema.Literal('localSummary', 'rawScreenshotRetention', 'liveView', 'relay', 'cache', 'unavailable')
);

export const ScreenOptionalVisibilityRetentionBehaviorSchema = withParser(
  Schema.Literal('noRawRetention', 'deleteAfterTtl', 'parentOwnedExportDeleteOnRevoke', 'noFrameRetention')
);

export type ScreenRawScreenshotRetentionMode = Infer<typeof ScreenRawScreenshotRetentionModeSchema>;
export type ScreenLiveViewMode = Infer<typeof ScreenLiveViewModeSchema>;
export type ScreenLiveViewTransportMode = Infer<typeof ScreenLiveViewTransportModeSchema>;
export type ScreenOptionalVisibilityDisclosureState = Infer<typeof ScreenOptionalVisibilityDisclosureStateSchema>;
export type ScreenOptionalVisibilityPlatformProofState = Infer<typeof ScreenOptionalVisibilityPlatformProofStateSchema>;
export type ScreenOptionalVisibilitySourceLabel = Infer<typeof ScreenOptionalVisibilitySourceLabelSchema>;

