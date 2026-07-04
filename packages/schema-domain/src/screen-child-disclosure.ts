import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './evidence-primitives';
import {
  ScreenEvidenceParentSettingRefSchema,
  ScreenEvidenceSchemaVersion,
  ScreenEvidenceSettingVersionSchema,
} from './screen-evidence-primitives';
import {
  ScreenCapabilityStatusSchema,
  ScreenCaptureScopeSchema,
  ScreenDeletionStateSchema,
  ScreenEvidenceCustodyStateSchema,
  ScreenQueueStatusSchema,
} from './screen-evidence-states';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const EvidenceRefsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected screen child disclosure evidence refs')
);
const OptionalQueueStatusSchema = Schema.Union(ScreenQueueStatusSchema, Schema.Null);
const OptionalDeletionStateSchema = Schema.Union(ScreenDeletionStateSchema, Schema.Null);
const OptionalTextTokenSchema = Schema.Union(
  Schema.Literal(
    'screen.child.disabled.detail',
    'screen.child.paused.detail',
    'screen.child.capture.detail',
    'screen.child.analysis.detail',
    'screen.child.summary.detail',
    'screen.child.permission.detail',
    'screen.child.protected.detail',
    'screen.child.unavailable.detail'
  ),
  Schema.Null
);

export const ScreenChildDisclosureSnapshotIdSchema = withParser(
  brandedNonEmptyStringSchema('ScreenChildDisclosureSnapshotId')
);

export const ScreenChildDisclosureStateSchema = withParser(
  Schema.Literal(
    'disabledByParent',
    'pausedByParent',
    'captureActive',
    'localAnalysisRunning',
    'deletedSummaryReady',
    'permissionRequired',
    'protectedSurface',
    'unavailable'
  )
);

export const ScreenChildDisclosureToneSchema = withParser(Schema.Literal('calm', 'informational'));

export const ScreenChildDisclosureSurfaceSchema = withParser(
  Schema.Literal('child-agent-status-chip', 'child-agent-capture-banner', 'platform-permission-prompt', 'modeled-only')
);

export const ScreenChildDisclosureTextTokenSchema = withParser(
  Schema.Literal(
    'screen.child.disabled.title',
    'screen.child.paused.title',
    'screen.child.capture.title',
    'screen.child.analysis.title',
    'screen.child.summary.title',
    'screen.child.permission.title',
    'screen.child.protected.title',
    'screen.child.unavailable.title'
  )
);

const ScreenChildDisclosureSnapshotBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
  snapshotId: ScreenChildDisclosureSnapshotIdSchema,
  createdAt: ActivityTimestampSchema,
  sourceEvidenceRefs: EvidenceRefsSchema,
  parentSettingRef: ScreenEvidenceParentSettingRefSchema,
  settingVersion: ScreenEvidenceSettingVersionSchema,
  screenAnalysisEnabled: Schema.Boolean,
  cadenceCaptureEnabled: Schema.Boolean,
  triggerCaptureEnabled: Schema.Boolean,
  captureActive: Schema.Boolean,
  capabilityStatus: ScreenCapabilityStatusSchema,
  captureScope: ScreenCaptureScopeSchema,
  queueStatus: OptionalQueueStatusSchema,
  deletionState: OptionalDeletionStateSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  state: ScreenChildDisclosureStateSchema,
  tone: ScreenChildDisclosureToneSchema,
  surface: ScreenChildDisclosureSurfaceSchema,
  primaryTextToken: ScreenChildDisclosureTextTokenSchema,
  secondaryTextToken: OptionalTextTokenSchema,
  visibleToChildRequired: RequiredTrue,
  rawScreenshotShownToChild: RequiredFalse,
  hiddenCaptureClaimed: RequiredFalse,
  remoteViewerClaimed: RequiredFalse,
  policyAuthorityClaimed: RequiredFalse,
  renderedChildAgentDeliveryClaimed: Schema.Boolean,
});

export const ScreenChildDisclosureSnapshotSchema = withParser(
  ScreenChildDisclosureSnapshotBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenChildDisclosureSnapshotIsConsistent(value) ||
        'Expected screen child disclosure to be visible, calm, parent-setting-aware, and non-surveillance'
    )
  )
);

export type ScreenChildDisclosureSnapshot = Infer<typeof ScreenChildDisclosureSnapshotSchema>;
export type ScreenChildDisclosureState = Infer<typeof ScreenChildDisclosureStateSchema>;
export type ScreenChildDisclosureTextToken = Infer<typeof ScreenChildDisclosureTextTokenSchema>;
type ScreenChildDisclosureSnapshotInput = Infer<typeof ScreenChildDisclosureSnapshotBaseSchema>;

export const ScreenChildDisclosureTextToken = {
  Disabled: ScreenChildDisclosureTextTokenSchema.parse('screen.child.disabled.title'),
  Paused: ScreenChildDisclosureTextTokenSchema.parse('screen.child.paused.title'),
  Capture: ScreenChildDisclosureTextTokenSchema.parse('screen.child.capture.title'),
  Analysis: ScreenChildDisclosureTextTokenSchema.parse('screen.child.analysis.title'),
  Summary: ScreenChildDisclosureTextTokenSchema.parse('screen.child.summary.title'),
  Permission: ScreenChildDisclosureTextTokenSchema.parse('screen.child.permission.title'),
  Protected: ScreenChildDisclosureTextTokenSchema.parse('screen.child.protected.title'),
  Unavailable: ScreenChildDisclosureTextTokenSchema.parse('screen.child.unavailable.title'),
} as const;

const ScreenChildDisclosurePrimaryTokenByState = {
  disabledByParent: ScreenChildDisclosureTextToken.Disabled,
  pausedByParent: ScreenChildDisclosureTextToken.Paused,
  captureActive: ScreenChildDisclosureTextToken.Capture,
  localAnalysisRunning: ScreenChildDisclosureTextToken.Analysis,
  deletedSummaryReady: ScreenChildDisclosureTextToken.Summary,
  permissionRequired: ScreenChildDisclosureTextToken.Permission,
  protectedSurface: ScreenChildDisclosureTextToken.Protected,
  unavailable: ScreenChildDisclosureTextToken.Unavailable,
} as const satisfies Record<ScreenChildDisclosureState, ScreenChildDisclosureTextToken>;

const ScreenChildDisclosureStateValidators = {
  disabledByParent: disabledStateDoesNotCapture,
  pausedByParent: pausedStateKeepsCaptureInactive,
  captureActive: activeCaptureStateIsVisibleAndReady,
  localAnalysisRunning: captureRemainsInactive,
  deletedSummaryReady: deletedSummaryStateHasDeletedCustody,
  permissionRequired: permissionRequiredStateHasLimitedCapability,
  protectedSurface: protectedSurfaceStateHasProtectedQueue,
  unavailable: captureRemainsInactive,
} as const satisfies Record<ScreenChildDisclosureState, (value: ScreenChildDisclosureSnapshotInput) => boolean>;

export function screenChildDisclosureProofSnapshots(): ReadonlyArray<ScreenChildDisclosureSnapshot> {
  return [
    ScreenChildDisclosureSnapshotSchema.parse({
      ...baseDisclosureSnapshot('screen-child-disclosure-disabled', 'disabledByParent'),
      screenAnalysisEnabled: false,
      cadenceCaptureEnabled: false,
      triggerCaptureEnabled: false,
      capabilityStatus: 'disabledByParent',
      captureScope: 'unsupported',
      custodyState: 'unavailable',
    }),
    ScreenChildDisclosureSnapshotSchema.parse({
      ...baseDisclosureSnapshot('screen-child-disclosure-paused', 'pausedByParent'),
      capabilityStatus: 'ready',
      captureScope: 'activeWindow',
    }),
    ScreenChildDisclosureSnapshotSchema.parse({
      ...baseDisclosureSnapshot('screen-child-disclosure-capture-active', 'captureActive'),
      captureActive: true,
      capabilityStatus: 'ready',
      captureScope: 'activeWindow',
      queueStatus: 'queued',
      surface: 'child-agent-capture-banner',
    }),
    ScreenChildDisclosureSnapshotSchema.parse({
      ...baseDisclosureSnapshot('screen-child-disclosure-protected', 'protectedSurface'),
      capabilityStatus: 'protectedSurface',
      captureScope: 'activeWindow',
      custodyState: 'unavailable',
      queueStatus: 'protectedSurface',
      surface: 'child-agent-status-chip',
    }),
    ScreenChildDisclosureSnapshotSchema.parse({
      ...baseDisclosureSnapshot('screen-child-disclosure-summary-ready', 'deletedSummaryReady'),
      capabilityStatus: 'ready',
      captureScope: 'activeWindow',
      queueStatus: 'deleted',
      deletionState: 'deleted',
      custodyState: 'child-device-query-store',
    }),
  ];
}

function screenChildDisclosureSnapshotIsConsistent(value: ScreenChildDisclosureSnapshotInput) {
  if (!stateMatchesPrimaryToken(value.state, value.primaryTextToken)) {
    return false;
  }
  if (value.screenAnalysisEnabled === false) {
    return disabledStateDoesNotCapture(value);
  }
  return ScreenChildDisclosureStateValidators[value.state](value);
}

function disabledStateDoesNotCapture(value: ScreenChildDisclosureSnapshotInput) {
  return (
    value.state === 'disabledByParent' &&
    !value.cadenceCaptureEnabled &&
    !value.triggerCaptureEnabled &&
    !value.captureActive &&
    value.capabilityStatus === 'disabledByParent'
  );
}

function activeCaptureStateIsVisibleAndReady(value: ScreenChildDisclosureSnapshotInput) {
  return (
    value.state === 'captureActive' &&
    value.screenAnalysisEnabled &&
    value.capabilityStatus === 'ready' &&
    value.captureScope !== 'unsupported' &&
    value.surface === 'child-agent-capture-banner'
  );
}

function captureRemainsInactive(value: ScreenChildDisclosureSnapshotInput) {
  return value.captureActive === false;
}

function pausedStateKeepsCaptureInactive(value: ScreenChildDisclosureSnapshotInput) {
  return value.state === 'pausedByParent' && value.screenAnalysisEnabled && captureRemainsInactive(value);
}

function permissionRequiredStateHasLimitedCapability(value: ScreenChildDisclosureSnapshotInput) {
  return (
    value.capabilityStatus === 'permissionRequired' || value.capabilityStatus === 'permissionLimited'
  );
}

function protectedSurfaceStateHasProtectedQueue(value: ScreenChildDisclosureSnapshotInput) {
  return value.capabilityStatus === 'protectedSurface' && value.queueStatus === 'protectedSurface';
}

function deletedSummaryStateHasDeletedCustody(value: ScreenChildDisclosureSnapshotInput) {
  return (
    (value.deletionState === 'deleted' || value.deletionState === 'expiredDeleted') &&
    (value.custodyState === 'child-device-query-store' || value.custodyState === 'child-device-journal') &&
    !value.captureActive
  );
}

function stateMatchesPrimaryToken(state: ScreenChildDisclosureState, token: ScreenChildDisclosureTextToken) {
  return stateToPrimaryToken(state) === token;
}

function stateToPrimaryToken(state: ScreenChildDisclosureState): ScreenChildDisclosureTextToken {
  return ScreenChildDisclosurePrimaryTokenByState[state];
}

function baseDisclosureSnapshot(snapshotId: string, state: ScreenChildDisclosureState) {
  return {
    schemaVersion: ScreenEvidenceSchemaVersion,
    snapshotId,
    createdAt: '2026-06-06T21:45:00Z',
    sourceEvidenceRefs: ['screen-child-disclosure-proof-evidence'],
    parentSettingRef: 'screen-child-disclosure-parent-setting',
    settingVersion: 1,
    screenAnalysisEnabled: true,
    cadenceCaptureEnabled: false,
    triggerCaptureEnabled: false,
    captureActive: false,
    capabilityStatus: 'ready',
    captureScope: 'activeWindow',
    queueStatus: null,
    deletionState: null,
    custodyState: 'child-device-query-store',
    state,
    tone: 'calm',
    surface: 'modeled-only',
    primaryTextToken: stateToPrimaryToken(state),
    secondaryTextToken: null,
    visibleToChildRequired: true,
    rawScreenshotShownToChild: false,
    hiddenCaptureClaimed: false,
    remoteViewerClaimed: false,
    policyAuthorityClaimed: false,
    renderedChildAgentDeliveryClaimed: false,
  };
}
