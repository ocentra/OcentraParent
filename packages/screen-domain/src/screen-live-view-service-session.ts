import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { ScreenEvidenceCustodyStateSchema } from './screen-evidence-states';
import { ScreenLiveViewPermissionEvidenceKindSchema } from './screen-live-view-platform-permission';
import {
  screenLiveViewDisabledCoreFieldsAreConsistent,
  screenLiveViewDisablesFrameStorageAndRemoteControl,
  screenLiveViewHasViewerAuditAndTransportProof,
  ScreenLiveViewOptionalAuditRefSchema,
  ScreenLiveViewOptionalProofRefSchema,
  ScreenLiveViewRequiredFalseSchema,
  ScreenLiveViewRequiredTrueSchema,
} from './screen-live-view-readiness-core';
import {
  ScreenLiveViewModeSchema,
  ScreenLiveViewTransportModeSchema,
  ScreenOptionalVisibilityRetentionBehaviorSchema,
  ScreenOptionalVisibilitySourceLabelSchema,
} from './screen-optional-visibility-mode-values';

export const ScreenLiveViewServiceSessionSchemaVersion = 1;

export const ScreenLiveViewServiceSessionStateSchema = withParser(
  Schema.Literal('disabled', 'loopbackTransportOnly', 'serviceRuntimeReady')
);

export const ScreenLiveViewParentUiPersistenceStateSchema = withParser(
  Schema.Literal('notRequired', 'missing', 'proved')
);

export const ScreenLiveViewRelayCacheStateSchema = withParser(Schema.Literal('notUsed', 'missing', 'proved'));

const ScreenLiveViewServiceSessionGateBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLiveViewServiceSessionSchemaVersion),
  checkedAt: ActivityTimestampSchema,
  liveViewMode: ScreenLiveViewModeSchema,
  transportMode: ScreenLiveViewTransportModeSchema,
  permissionEvidenceKind: ScreenLiveViewPermissionEvidenceKindSchema,
  sourceLabel: ScreenOptionalVisibilitySourceLabelSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  frameRetentionBehavior: ScreenOptionalVisibilityRetentionBehaviorSchema,
  platformPermissionProofRef: ScreenLiveViewOptionalProofRefSchema,
  viewerAuditRef: ScreenLiveViewOptionalAuditRefSchema,
  liveTransportProofRef: ScreenLiveViewOptionalProofRefSchema,
  serviceSessionState: ScreenLiveViewServiceSessionStateSchema,
  parentUiPersistenceState: ScreenLiveViewParentUiPersistenceStateSchema,
  relayCacheState: ScreenLiveViewRelayCacheStateSchema,
  rawFrameDeletedAfterTransport: ScreenLiveViewRequiredTrueSchema,
  cacheRawFrames: ScreenLiveViewRequiredFalseSchema,
  sessionRecordingAllowed: ScreenLiveViewRequiredFalseSchema,
  remoteInputControlAllowed: ScreenLiveViewRequiredFalseSchema,
  productLiveViewReady: Schema.Boolean,
  reason: NonEmptyStringSchema,
});

type ScreenLiveViewServiceSessionGateInput = Infer<typeof ScreenLiveViewServiceSessionGateBaseSchema>;

export const ScreenLiveViewServiceSessionGateSchema = withParser(
  ScreenLiveViewServiceSessionGateBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenLiveViewServiceSessionGateIsConsistent(value) ||
        'Expected live view service session readiness to require service runtime, platform live-view permission, viewer audit, transport proof, parent UI persistence, no frame retention, no recording, and no remote input before product readiness'
    )
  )
);

export function screenLiveViewServiceSessionGateIsConsistent(value: ScreenLiveViewServiceSessionGateInput): boolean {
  if (value.liveViewMode === 'disabled') {
    return disabledLiveViewServiceSessionGateIsConsistent(value);
  }

  if (!enabledLiveViewServiceSessionEvidenceIsPresent(value)) {
    return !value.productLiveViewReady;
  }

  if (value.liveViewMode === 'lanOnlyView') {
    return value.transportMode === 'lanMutualAuth' && value.relayCacheState === 'notUsed';
  }

  return value.transportMode === 'relayEndToEndEncrypted' && value.relayCacheState === 'proved';
}

function disabledLiveViewServiceSessionGateIsConsistent(value: ScreenLiveViewServiceSessionGateInput): boolean {
  return (
    screenLiveViewDisabledCoreFieldsAreConsistent(value) &&
    value.permissionEvidenceKind === 'missing' &&
    value.platformPermissionProofRef === null &&
    value.serviceSessionState === 'disabled' &&
    value.parentUiPersistenceState === 'notRequired' &&
    value.relayCacheState === 'notUsed'
  );
}

function enabledLiveViewServiceSessionEvidenceIsPresent(value: ScreenLiveViewServiceSessionGateInput): boolean {
  return (
    value.permissionEvidenceKind === 'live-view-permission' &&
    value.platformPermissionProofRef !== null &&
    screenLiveViewHasViewerAuditAndTransportProof(value) &&
    value.serviceSessionState === 'serviceRuntimeReady' &&
    value.parentUiPersistenceState === 'proved' &&
    value.rawFrameDeletedAfterTransport &&
    screenLiveViewDisablesFrameStorageAndRemoteControl(value) &&
    value.productLiveViewReady
  );
}

export type ScreenLiveViewServiceSessionState = Infer<typeof ScreenLiveViewServiceSessionStateSchema>;
export type ScreenLiveViewParentUiPersistenceState = Infer<typeof ScreenLiveViewParentUiPersistenceStateSchema>;
export type ScreenLiveViewRelayCacheState = Infer<typeof ScreenLiveViewRelayCacheStateSchema>;
export type ScreenLiveViewServiceSessionGate = Infer<typeof ScreenLiveViewServiceSessionGateSchema>;

