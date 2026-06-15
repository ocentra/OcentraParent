import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { ScreenEvidenceCustodyStateSchema } from './screen-evidence-states';
import {
  ScreenLiveViewModeSchema,
  ScreenLiveViewTransportModeSchema,
  ScreenOptionalVisibilityAuditRefSchema,
  ScreenOptionalVisibilityPlatformProofRefSchema,
  ScreenOptionalVisibilityPlatformProofStateSchema,
  ScreenOptionalVisibilityRetentionBehaviorSchema,
  ScreenOptionalVisibilitySourceLabelSchema,
} from './screen-optional-visibility-mode-values';

export const ScreenLiveViewPermissionGateSchemaVersion = 1;
export const ScreenLiveViewProductionReadinessEvidenceSchemaVersion = 1;

const NonEmptyLiveViewText = Schema.String.pipe(Schema.minLength(1));
const RequiredFalse = Schema.Literal(false);
const OptionalLiveViewProofRefSchema = Schema.Union(ScreenOptionalVisibilityPlatformProofRefSchema, Schema.Null);
const OptionalLiveViewAuditRefSchema = Schema.Union(ScreenOptionalVisibilityAuditRefSchema, Schema.Null);

export const ScreenLiveViewPlatformKindSchema = withParser(
  Schema.Literal('windows', 'macos', 'linux-x11', 'linux-wayland', 'android-mediaprojection', 'ios-replaykit')
);

export const ScreenLiveViewPermissionEvidenceKindSchema = withParser(
  Schema.Literal('live-view-permission', 'screen-capture-only', 'missing')
);

export const ScreenLiveViewPromptArtifactKindSchema = withParser(
  Schema.Literal('platform-permission-prompt-screenshot', 'platform-permission-recording', 'os-permission-audit-log')
);

const ScreenLiveViewPlatformPermissionGateBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLiveViewPermissionGateSchemaVersion),
  checkedAt: ActivityTimestampSchema,
  platform: ScreenLiveViewPlatformKindSchema,
  liveViewMode: ScreenLiveViewModeSchema,
  transportMode: ScreenLiveViewTransportModeSchema,
  permissionEvidenceKind: ScreenLiveViewPermissionEvidenceKindSchema,
  platformProofState: ScreenOptionalVisibilityPlatformProofStateSchema,
  platformProofRef: OptionalLiveViewProofRefSchema,
  viewerAuditRef: OptionalLiveViewAuditRefSchema,
  sourceLabel: ScreenOptionalVisibilitySourceLabelSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  frameRetentionBehavior: ScreenOptionalVisibilityRetentionBehaviorSchema,
  liveTransportProofRef: OptionalLiveViewProofRefSchema,
  explicitViewerDisclosure: Schema.Boolean,
  cacheRawFrames: RequiredFalse,
  sessionRecordingAllowed: RequiredFalse,
  remoteInputControlAllowed: RequiredFalse,
  productLiveViewReady: Schema.Boolean,
  reason: NonEmptyLiveViewText,
});

type ScreenLiveViewPlatformPermissionGateInput = Infer<typeof ScreenLiveViewPlatformPermissionGateBaseSchema>;

const ScreenLiveViewPromptArtifactSchema = Schema.Struct({
  platform: ScreenLiveViewPlatformKindSchema,
  artifactKind: ScreenLiveViewPromptArtifactKindSchema,
  artifactRef: ScreenOptionalVisibilityPlatformProofRefSchema,
  artifactDigest: NonEmptyLiveViewText,
  capturedAt: ActivityTimestampSchema,
  operatorAuditRef: ScreenOptionalVisibilityAuditRefSchema,
  permissionEvidenceKind: Schema.Literal('live-view-permission'),
  rawFrameIncluded: RequiredFalse,
  containsUserPrivateContent: RequiredFalse,
});

const ScreenLiveViewProductionReadinessEvidenceBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLiveViewProductionReadinessEvidenceSchemaVersion),
  checkedAt: ActivityTimestampSchema,
  permissionGate: ScreenLiveViewPlatformPermissionGateBaseSchema,
  promptArtifact: ScreenLiveViewPromptArtifactSchema,
  liveTransportProofRef: ScreenOptionalVisibilityPlatformProofRefSchema,
  physicalDeviceParityProofRef: ScreenOptionalVisibilityPlatformProofRefSchema,
  privacyLegalApprovalRef: ScreenOptionalVisibilityAuditRefSchema,
  productionWorkerStartProofRef: ScreenOptionalVisibilityPlatformProofRefSchema,
  relayCacheExecutionProofRef: OptionalLiveViewProofRefSchema,
  productLiveViewReady: Schema.Boolean,
});

type ScreenLiveViewProductionReadinessEvidenceInput = Infer<typeof ScreenLiveViewProductionReadinessEvidenceBaseSchema>;

export const ScreenLiveViewPlatformPermissionGateSchema = withParser(
  ScreenLiveViewPlatformPermissionGateBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenLiveViewPermissionGateIsConsistent(value) ||
        'Expected live view permission gate to require live-view permission evidence, transport proof, viewer audit, no frame retention, and no remote input before readiness'
    )
  )
);

export const ScreenLiveViewProductionReadinessEvidenceSchema = withParser(
  ScreenLiveViewProductionReadinessEvidenceBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenLiveViewProductionReadinessEvidenceIsConsistent(value) ||
        'Expected live view production readiness to include a ready permission gate, matching platform prompt artifact, transport proof, physical parity, privacy/legal approval, production worker start proof, and relay/cache proof when relay-backed'
    )
  )
);

export function screenLiveViewPermissionGateIsConsistent(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  if (value.liveViewMode === 'disabled') {
    return disabledLiveViewPermissionGateIsConsistent(value);
  }

  return enabledLiveViewPermissionGateIsConsistent(value);
}

export function screenLiveViewProductionReadinessEvidenceIsConsistent(
  value: ScreenLiveViewProductionReadinessEvidenceInput
): boolean {
  if (!value.productLiveViewReady) {
    return false;
  }

  if (!screenLiveViewPermissionGateIsConsistent(value.permissionGate)) {
    return false;
  }

  if (!value.permissionGate.productLiveViewReady) {
    return false;
  }

  if (value.promptArtifact.platform !== value.permissionGate.platform) {
    return false;
  }

  if (value.promptArtifact.artifactRef !== value.permissionGate.platformProofRef) {
    return false;
  }

  if (value.promptArtifact.operatorAuditRef !== value.permissionGate.viewerAuditRef) {
    return false;
  }

  if (value.liveTransportProofRef !== value.permissionGate.liveTransportProofRef) {
    return false;
  }

  if (value.permissionGate.liveViewMode === 'relayBackedView') {
    return value.relayCacheExecutionProofRef !== null;
  }

  return value.relayCacheExecutionProofRef === null;
}

function disabledLiveViewPermissionGateIsConsistent(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  return (
    value.transportMode === 'none' &&
    value.permissionEvidenceKind === 'missing' &&
    value.platformProofState === 'notRequired' &&
    value.platformProofRef === null &&
    value.viewerAuditRef === null &&
    value.sourceLabel === 'unavailable' &&
    value.custodyState === 'unavailable' &&
    value.frameRetentionBehavior === 'noFrameRetention' &&
    value.liveTransportProofRef === null &&
    !value.explicitViewerDisclosure &&
    !value.productLiveViewReady
  );
}

function enabledLiveViewPermissionGateIsConsistent(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  if (!enabledLiveViewProofInputsAreReady(value)) {
    return !value.productLiveViewReady;
  }

  if (value.liveViewMode === 'lanOnlyView') {
    return lanLiveViewPermissionGateIsConsistent(value);
  }

  return relayLiveViewPermissionGateIsConsistent(value);
}

function enabledLiveViewProofInputsAreReady(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  return (
    value.permissionEvidenceKind === 'live-view-permission' &&
    value.platformProofState === 'operatorVerified' &&
    value.platformProofRef !== null &&
    value.viewerAuditRef !== null &&
    value.frameRetentionBehavior === 'noFrameRetention' &&
    value.liveTransportProofRef !== null &&
    value.explicitViewerDisclosure
  );
}

function lanLiveViewPermissionGateIsConsistent(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  return (
    value.transportMode === 'lanMutualAuth' &&
    value.sourceLabel === 'liveView' &&
    value.custodyState === 'live-lan-child-agent' &&
    value.productLiveViewReady
  );
}

function relayLiveViewPermissionGateIsConsistent(value: ScreenLiveViewPlatformPermissionGateInput): boolean {
  return (
    value.transportMode === 'relayEndToEndEncrypted' &&
    value.sourceLabel === 'relay' &&
    value.custodyState === 'ocentra-hosted-non-activity' &&
    value.productLiveViewReady
  );
}

export type ScreenLiveViewPlatformKind = Infer<typeof ScreenLiveViewPlatformKindSchema>;
export type ScreenLiveViewPermissionEvidenceKind = Infer<typeof ScreenLiveViewPermissionEvidenceKindSchema>;
export type ScreenLiveViewPromptArtifactKind = Infer<typeof ScreenLiveViewPromptArtifactKindSchema>;
export type ScreenLiveViewPlatformPermissionGate = Infer<typeof ScreenLiveViewPlatformPermissionGateSchema>;
export type ScreenLiveViewProductionReadinessEvidence = Infer<typeof ScreenLiveViewProductionReadinessEvidenceSchema>;
