import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import { ScreenEvidenceParentSettingRefSchema } from './screen-evidence-primitives';
import {
  ScreenLiveViewParentUiPersistenceStateSchema,
  ScreenLiveViewServiceSessionGateSchema,
} from './screen-live-view-service-session';
import { ScreenLiveViewOptInSettingSchema } from './screen-optional-visibility-mode';
import { ScreenOptionalVisibilityAuditRefSchema } from './screen-optional-visibility-mode-values';

export const ScreenLiveViewParentUiPersistenceSchemaVersion = 1;

const NonEmptyLiveViewText = Schema.String.pipe(Schema.minLength(1));
const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);

export const ScreenLiveViewParentUiPersistenceStatusSchema = withParser(
  Schema.Literal('disabledRendered', 'persistedParentOptIn')
);

const ScreenLiveViewParentUiPersistenceProofBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenLiveViewParentUiPersistenceSchemaVersion),
  checkedAt: ActivityTimestampSchema,
  status: ScreenLiveViewParentUiPersistenceStatusSchema,
  parentSettingRef: ScreenEvidenceParentSettingRefSchema,
  liveViewSetting: ScreenLiveViewOptInSettingSchema,
  serviceSessionGate: ScreenLiveViewServiceSessionGateSchema,
  parentUiPersistenceState: ScreenLiveViewParentUiPersistenceStateSchema,
  settingsRouteRendered: RequiredTrue,
  persistedInParentSettingsStore: RequiredTrue,
  viewerAuditRef: ScreenOptionalVisibilityAuditRefSchema,
  portalProofRef: NonEmptyLiveViewText,
  serviceSettingsProofRef: NonEmptyLiveViewText,
  rawFramesRetained: RequiredFalse,
  remoteInputAllowed: RequiredFalse,
  productLiveViewReady: RequiredFalse,
  reason: NonEmptyLiveViewText,
});

type ScreenLiveViewParentUiPersistenceProofInput = Infer<typeof ScreenLiveViewParentUiPersistenceProofBaseSchema>;

export const ScreenLiveViewParentUiPersistenceProofSchema = withParser(
  ScreenLiveViewParentUiPersistenceProofBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenLiveViewParentUiPersistenceProofIsConsistent(value) ||
        'Expected parent UI persistence proof to persist an explicit live-view opt-in, render the Settings route, carry viewer audit, and keep product live view false until runtime/prompt/transport gates are complete'
    )
  )
);

export function screenLiveViewParentUiPersistenceProofIsConsistent(
  value: ScreenLiveViewParentUiPersistenceProofInput
): boolean {
  if (!parentUiProofKeepsLiveViewNonProduct(value)) {
    return false;
  }
  if (value.status === 'disabledRendered') {
    return disabledUiPersistenceIsConsistent(value);
  }

  return persistedOptInUiPersistenceIsConsistent(value);
}

function parentUiProofKeepsLiveViewNonProduct(value: ScreenLiveViewParentUiPersistenceProofInput): boolean {
  return (
    !value.rawFramesRetained &&
    !value.remoteInputAllowed &&
    !value.productLiveViewReady &&
    !value.serviceSessionGate.productLiveViewReady
  );
}

function disabledUiPersistenceIsConsistent(value: ScreenLiveViewParentUiPersistenceProofInput): boolean {
  return (
    value.liveViewSetting.liveViewMode === 'disabled' &&
    value.parentUiPersistenceState === 'notRequired' &&
    value.serviceSessionGate.parentUiPersistenceState === 'notRequired'
  );
}

function persistedOptInUiPersistenceIsConsistent(value: ScreenLiveViewParentUiPersistenceProofInput): boolean {
  return (
    value.liveViewSetting.liveViewMode !== 'disabled' &&
    value.liveViewSetting.explicitParentApproval &&
    value.liveViewSetting.viewerAuditRef === value.viewerAuditRef &&
    value.liveViewSetting.cacheRawFrames === false &&
    value.liveViewSetting.sessionRecordingAllowed === false &&
    value.liveViewSetting.remoteInputControlAllowed === false &&
    value.parentUiPersistenceState === 'proved' &&
    value.serviceSessionGate.parentUiPersistenceState === 'proved'
  );
}

export type ScreenLiveViewParentUiPersistenceStatus = Infer<typeof ScreenLiveViewParentUiPersistenceStatusSchema>;
export type ScreenLiveViewParentUiPersistenceProof = Infer<typeof ScreenLiveViewParentUiPersistenceProofSchema>;
