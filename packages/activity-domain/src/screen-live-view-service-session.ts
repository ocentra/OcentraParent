import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import { ScreenEvidenceCustodyStateSchema } from './screen-evidence-states';
import { ScreenLiveViewPermissionEvidenceKindSchema } from './screen-live-view-platform-permission';
import {
  ScreenLiveViewModeSchema,
  ScreenLiveViewTransportModeSchema,
  ScreenOptionalVisibilityAuditRefSchema,
  ScreenOptionalVisibilityPlatformProofRefSchema,
  ScreenOptionalVisibilityRetentionBehaviorSchema,
  ScreenOptionalVisibilitySourceLabelSchema,
} from './screen-optional-visibility-mode-values';

export const ScreenLiveViewServiceSessionSchemaVersion = 1;

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const OptionalLiveViewProofRefSchema = Schema.Union(ScreenOptionalVisibilityPlatformProofRefSchema, Schema.Null);
const OptionalLiveViewAuditRefSchema = Schema.Union(ScreenOptionalVisibilityAuditRefSchema, Schema.Null);
const NonEmptyLiveViewText = Schema.String.pipe(Schema.minLength(1));

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
  platformPermissionProofRef: OptionalLiveViewProofRefSchema,
  viewerAuditRef: OptionalLiveViewAuditRefSchema,
  liveTransportProofRef: OptionalLiveViewProofRefSchema,
  serviceSessionState: ScreenLiveViewServiceSessionStateSchema,
  parentUiPersistenceState: ScreenLiveViewParentUiPersistenceStateSchema,
  relayCacheState: ScreenLiveViewRelayCacheStateSchema,
  rawFrameDeletedAfterTransport: RequiredTrue,
  cacheRawFrames: RequiredFalse,
  sessionRecordingAllowed: RequiredFalse,
  remoteInputControlAllowed: RequiredFalse,
  productLiveViewReady: Schema.Boolean,
  reason: NonEmptyLiveViewText,
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
    value.transportMode === 'none' &&
    value.permissionEvidenceKind === 'missing' &&
    value.sourceLabel === 'unavailable' &&
    value.custodyState === 'unavailable' &&
    value.frameRetentionBehavior === 'noFrameRetention' &&
    value.platformPermissionProofRef === null &&
    value.viewerAuditRef === null &&
    value.liveTransportProofRef === null &&
    value.serviceSessionState === 'disabled' &&
    value.parentUiPersistenceState === 'notRequired' &&
    value.relayCacheState === 'notUsed' &&
    !value.productLiveViewReady
  );
}

function enabledLiveViewServiceSessionEvidenceIsPresent(value: ScreenLiveViewServiceSessionGateInput): boolean {
  return (
    value.permissionEvidenceKind === 'live-view-permission' &&
    value.platformPermissionProofRef !== null &&
    value.viewerAuditRef !== null &&
    value.liveTransportProofRef !== null &&
    value.serviceSessionState === 'serviceRuntimeReady' &&
    value.parentUiPersistenceState === 'proved' &&
    value.frameRetentionBehavior === 'noFrameRetention' &&
    value.productLiveViewReady
  );
}

export type ScreenLiveViewServiceSessionState = Infer<typeof ScreenLiveViewServiceSessionStateSchema>;
export type ScreenLiveViewParentUiPersistenceState = Infer<typeof ScreenLiveViewParentUiPersistenceStateSchema>;
export type ScreenLiveViewRelayCacheState = Infer<typeof ScreenLiveViewRelayCacheStateSchema>;
export type ScreenLiveViewServiceSessionGate = Infer<typeof ScreenLiveViewServiceSessionGateSchema>;
