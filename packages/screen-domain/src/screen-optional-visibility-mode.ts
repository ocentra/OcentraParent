import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { ScreenEvidenceCustodyStateSchema } from './screen-evidence-states';
import {
  ScreenEvidenceParentSettingRefSchema,
  ScreenEvidenceReasonSchema,
  ScreenEvidenceSettingVersionSchema,
} from './screen-evidence-primitives';
import {
  ScreenLiveViewModeSchema,
  ScreenLiveViewTransportModeSchema,
  ScreenOptionalVisibilityApprovalRefSchema,
  ScreenOptionalVisibilityAuditRefSchema,
  ScreenOptionalVisibilityDisclosureStateSchema,
  ScreenOptionalVisibilityExportRefSchema,
  ScreenOptionalVisibilityModeSchemaVersion,
  ScreenOptionalVisibilityPlatformProofRefSchema,
  ScreenOptionalVisibilityPlatformProofStateSchema,
  ScreenOptionalVisibilityRetentionBehaviorSchema,
  ScreenOptionalVisibilitySettingIdSchema,
  ScreenOptionalVisibilitySourceLabelSchema,
  ScreenRawScreenshotRetentionModeSchema,
} from './screen-optional-visibility-mode-values';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const OptionalApprovalRefSchema = Schema.Union(ScreenOptionalVisibilityApprovalRefSchema, Schema.Null);
const OptionalAuditRefSchema = Schema.Union(ScreenOptionalVisibilityAuditRefSchema, Schema.Null);
const OptionalPlatformProofRefSchema = Schema.Union(ScreenOptionalVisibilityPlatformProofRefSchema, Schema.Null);
const OptionalExportRefSchema = Schema.Union(ScreenOptionalVisibilityExportRefSchema, Schema.Null);
const OptionalTtlSecondsSchema = Schema.Union(Schema.Number.pipe(Schema.int(), Schema.between(60, 86400)), Schema.Null);

const ScreenRawScreenshotRetentionOptInSettingBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenOptionalVisibilityModeSchemaVersion),
  settingId: ScreenOptionalVisibilitySettingIdSchema,
  parentSettingRef: ScreenEvidenceParentSettingRefSchema,
  settingVersion: ScreenEvidenceSettingVersionSchema,
  changedAt: ActivityTimestampSchema,
  mode: ScreenRawScreenshotRetentionModeSchema,
  explicitParentApproval: Schema.Boolean,
  approvalRef: OptionalApprovalRefSchema,
  disclosureState: ScreenOptionalVisibilityDisclosureStateSchema,
  auditRef: OptionalAuditRefSchema,
  ttlSeconds: OptionalTtlSecondsSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  exportRef: OptionalExportRefSchema,
  sourceLabel: ScreenOptionalVisibilitySourceLabelSchema,
  retentionBehavior: ScreenOptionalVisibilityRetentionBehaviorSchema,
  deleteAfterTtl: Schema.Boolean,
  deleteOnParentDisable: RequiredTrue,
  deleteProofRequired: Schema.Boolean,
  rawScreenshotRemoteUploadEnabled: RequiredFalse,
  reason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
});

type ScreenRawScreenshotRetentionOptInSettingBase = Infer<typeof ScreenRawScreenshotRetentionOptInSettingBaseSchema>;

export const ScreenRawScreenshotRetentionOptInSettingSchema = withParser(
  ScreenRawScreenshotRetentionOptInSettingBaseSchema.pipe(
    Schema.filter(
      (value: ScreenRawScreenshotRetentionOptInSettingBase) =>
        rawRetentionSettingIsConsistent(value) ||
        'Expected optional raw screenshot retention to be explicit, audited, custody-labeled, TTL-bound, and deletable'
    )
  )
);

const ScreenLiveViewOptInSettingBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenOptionalVisibilityModeSchemaVersion),
  settingId: ScreenOptionalVisibilitySettingIdSchema,
  parentSettingRef: ScreenEvidenceParentSettingRefSchema,
  settingVersion: ScreenEvidenceSettingVersionSchema,
  changedAt: ActivityTimestampSchema,
  liveViewMode: ScreenLiveViewModeSchema,
  transportMode: ScreenLiveViewTransportModeSchema,
  explicitParentApproval: Schema.Boolean,
  approvalRef: OptionalApprovalRefSchema,
  disclosureState: ScreenOptionalVisibilityDisclosureStateSchema,
  viewerAuditRef: OptionalAuditRefSchema,
  platformProofState: ScreenOptionalVisibilityPlatformProofStateSchema,
  platformProofRef: OptionalPlatformProofRefSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  sourceLabel: ScreenOptionalVisibilitySourceLabelSchema,
  frameRetentionBehavior: ScreenOptionalVisibilityRetentionBehaviorSchema,
  cacheRawFrames: RequiredFalse,
  sessionRecordingAllowed: RequiredFalse,
  remoteInputControlAllowed: RequiredFalse,
  stopOrRevokeAuditRequired: RequiredTrue,
  reason: Schema.Union(ScreenEvidenceReasonSchema, Schema.Null),
});

type ScreenLiveViewOptInSettingBase = Infer<typeof ScreenLiveViewOptInSettingBaseSchema>;

export const ScreenLiveViewOptInSettingSchema = withParser(
  ScreenLiveViewOptInSettingBaseSchema.pipe(
    Schema.filter(
      (value: ScreenLiveViewOptInSettingBase) =>
        liveViewSettingIsConsistent(value) ||
        'Expected optional live view to be explicit, view-only, audited, platform-proved, and no-frame-retention'
    )
  )
);

function rawRetentionSettingIsConsistent(value: ScreenRawScreenshotRetentionOptInSettingBase): boolean {
  if (value.mode === 'disabled') {
    return disabledRawRetentionIsConsistent(value);
  }
  if (value.mode === 'localShortTtl') {
    return approvedRawRetentionBaseIsConsistent(value) && value.custodyState === 'child-device-temp-queue';
  }

  return (
    approvedRawRetentionBaseIsConsistent(value) &&
    value.custodyState === 'parent-owned-export' &&
    value.exportRef !== null &&
    value.retentionBehavior === 'parentOwnedExportDeleteOnRevoke'
  );
}

function disabledRawRetentionIsConsistent(value: ScreenRawScreenshotRetentionOptInSettingBase): boolean {
  return (
    !value.explicitParentApproval &&
    value.approvalRef === null &&
    value.disclosureState === 'notRequired' &&
    value.auditRef === null &&
    value.ttlSeconds === null &&
    value.custodyState === 'unavailable' &&
    value.exportRef === null &&
    value.sourceLabel === 'unavailable' &&
    value.retentionBehavior === 'noRawRetention' &&
    !value.deleteAfterTtl &&
    !value.deleteProofRequired
  );
}

function approvedRawRetentionBaseIsConsistent(value: ScreenRawScreenshotRetentionOptInSettingBase): boolean {
  return (
    value.explicitParentApproval &&
    value.approvalRef !== null &&
    value.disclosureState === 'requiredShown' &&
    value.auditRef !== null &&
    value.ttlSeconds !== null &&
    value.sourceLabel === 'rawScreenshotRetention' &&
    value.deleteAfterTtl &&
    value.deleteProofRequired &&
    value.rawScreenshotRemoteUploadEnabled === false &&
    (value.retentionBehavior === 'deleteAfterTtl' || value.retentionBehavior === 'parentOwnedExportDeleteOnRevoke')
  );
}

function liveViewSettingIsConsistent(value: ScreenLiveViewOptInSettingBase): boolean {
  if (value.liveViewMode === 'disabled') {
    return disabledLiveViewIsConsistent(value);
  }
  if (!approvedLiveViewBaseIsConsistent(value)) {
    return false;
  }
  if (value.liveViewMode === 'lanOnlyView') {
    return value.transportMode === 'lanMutualAuth' && value.custodyState === 'live-lan-child-agent';
  }

  return value.transportMode === 'relayEndToEndEncrypted' && value.custodyState === 'ocentra-hosted-non-activity';
}

function disabledLiveViewIsConsistent(value: ScreenLiveViewOptInSettingBase): boolean {
  return (
    !value.explicitParentApproval &&
    value.approvalRef === null &&
    value.disclosureState === 'notRequired' &&
    value.viewerAuditRef === null &&
    value.platformProofState === 'notRequired' &&
    value.platformProofRef === null &&
    value.transportMode === 'none' &&
    value.custodyState === 'unavailable' &&
    value.sourceLabel === 'unavailable' &&
    value.frameRetentionBehavior === 'noFrameRetention'
  );
}

function approvedLiveViewBaseIsConsistent(value: ScreenLiveViewOptInSettingBase): boolean {
  return (
    value.explicitParentApproval &&
    value.approvalRef !== null &&
    value.disclosureState === 'requiredShown' &&
    value.viewerAuditRef !== null &&
    value.platformProofState === 'operatorVerified' &&
    value.platformProofRef !== null &&
    (value.sourceLabel === 'liveView' || value.sourceLabel === 'relay') &&
    value.frameRetentionBehavior === 'noFrameRetention' &&
    value.cacheRawFrames === false &&
    value.sessionRecordingAllowed === false &&
    value.remoteInputControlAllowed === false
  );
}

export type ScreenRawScreenshotRetentionOptInSetting = Infer<typeof ScreenRawScreenshotRetentionOptInSettingSchema>;
export type ScreenLiveViewOptInSetting = Infer<typeof ScreenLiveViewOptInSettingSchema>;
