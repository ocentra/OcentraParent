import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityTimestampSchema } from './primitives';
import {
  ScreenEvidenceParentSettingRefSchema,
  ScreenEvidenceReasonSchema,
  ScreenEvidenceSettingVersionSchema,
} from './screen-evidence-primitives';
import { ScreenCapabilityStatusSchema, ScreenEvidenceCustodyStateSchema } from './screen-evidence-states';
import {
  type ScreenChildDisclosureIndicator,
  type ScreenChildDisclosureState,
  ScreenChildDisclosureAuditRefSchema,
  ScreenChildDisclosureDeliverySchema,
  ScreenChildDisclosureIndicatorSchema,
  ScreenChildDisclosureProofIdSchema,
  ScreenChildDisclosureStateSchema,
  ScreenChildDisclosureStatusIdSchema,
  ScreenChildDisclosureSurfaceSchema,
  ScreenChildDisclosureTextTokenRefSchema,
  ScreenChildDisclosureUxSchemaVersion,
} from './screen-child-disclosure-ux-values';
import { ScreenOptionalVisibilityDisclosureStateSchema } from './screen-optional-visibility-mode-values';

export * from './screen-child-disclosure-ux-values';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const OptionalAuditRefSchema = Schema.Union(ScreenChildDisclosureAuditRefSchema, Schema.Null);
const OptionalReasonSchema = Schema.Union(ScreenEvidenceReasonSchema, Schema.Null);

export const ScreenChildDisclosureCopyRefsSchema = Schema.Struct({
  titleTokenRef: ScreenChildDisclosureTextTokenRefSchema,
  bodyTokenRef: ScreenChildDisclosureTextTokenRefSchema,
  statusTokenRef: ScreenChildDisclosureTextTokenRefSchema,
  actionTokenRef: ScreenChildDisclosureTextTokenRefSchema,
});

const ScreenChildDisclosureStatusBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenChildDisclosureUxSchemaVersion),
  statusId: ScreenChildDisclosureStatusIdSchema,
  state: ScreenChildDisclosureStateSchema,
  indicator: ScreenChildDisclosureIndicatorSchema,
  surface: ScreenChildDisclosureSurfaceSchema,
  delivery: ScreenChildDisclosureDeliverySchema,
  parentSettingRef: ScreenEvidenceParentSettingRefSchema,
  settingVersion: ScreenEvidenceSettingVersionSchema,
  updatedAt: ActivityTimestampSchema,
  capabilityStatus: ScreenCapabilityStatusSchema,
  disclosureState: ScreenOptionalVisibilityDisclosureStateSchema,
  custodyState: ScreenEvidenceCustodyStateSchema,
  childVisible: RequiredTrue,
  visibleStatusRequired: RequiredTrue,
  captureActive: Schema.Boolean,
  pausedByParent: Schema.Boolean,
  hiddenCaptureAllowed: RequiredFalse,
  rawScreenshotPathVisible: RequiredFalse,
  rawScreenshotRemoteUploadEnabled: RequiredFalse,
  localOnlyDisclosure: RequiredTrue,
  auditRef: OptionalAuditRefSchema,
  copyRefs: ScreenChildDisclosureCopyRefsSchema,
  reason: OptionalReasonSchema,
});

type ScreenChildDisclosureStatusBase = Infer<typeof ScreenChildDisclosureStatusBaseSchema>;

export const ScreenChildDisclosureStatusSchema = withParser(
  ScreenChildDisclosureStatusBaseSchema.pipe(
    Schema.filter(
      (value: ScreenChildDisclosureStatusBase) =>
        screenChildDisclosureStatusIsConsistent(value) ||
        'Expected child screen disclosure status to be visible, local-only, audited when enabled, and never hidden capture'
    )
  )
);

const ScreenChildDisclosureStatusArraySchema = Schema.Array(ScreenChildDisclosureStatusSchema).pipe(
  Schema.filter(
    (value) =>
      (value.length === 5 &&
        screenChildDisclosureStatesAreComplete(value) &&
        screenChildDisclosureStatusesAreSafe(value)) ||
      'Expected disabled, paused, ready, capture-active, and protected-surface child disclosure rows'
  )
);

export const ScreenChildDisclosureUxProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenChildDisclosureUxSchemaVersion),
    proofId: ScreenChildDisclosureProofIdSchema,
    generatedAt: ActivityTimestampSchema,
    proofTier: Schema.Literal('P2_CONTRACT_WITH_PORTAL_PROOF'),
    titleTokenRef: ScreenChildDisclosureTextTokenRefSchema,
    introTokenRef: ScreenChildDisclosureTextTokenRefSchema,
    statuses: ScreenChildDisclosureStatusArraySchema,
    hiddenCaptureAllowed: RequiredFalse,
    rawScreenshotPathVisible: RequiredFalse,
    rawScreenshotRemoteUploadEnabled: RequiredFalse,
    productionChildAppClaimed: RequiredFalse,
    servicePersistenceClaimed: RequiredFalse,
    osNotificationClaimed: RequiredFalse,
    localOnlyDisclosure: RequiredTrue,
  })
);

export type ScreenChildDisclosureCopyRefs = Infer<typeof ScreenChildDisclosureCopyRefsSchema>;
export type ScreenChildDisclosureStatus = Infer<typeof ScreenChildDisclosureStatusSchema>;
export type ScreenChildDisclosureUxProof = Infer<typeof ScreenChildDisclosureUxProofSchema>;

const GeneratedAt = '2026-06-05T07:26:00Z';
const ParentSettingRef = 'screen-child-disclosure-parent-setting';
const SettingVersion = 1;

export function screenChildDisclosureUxProof(): ScreenChildDisclosureUxProof {
  return ScreenChildDisclosureUxProofSchema.parse({
    schemaVersion: ScreenChildDisclosureUxSchemaVersion,
    proofId: 'screen-child-disclosure-ux-proof',
    generatedAt: GeneratedAt,
    proofTier: 'P2_CONTRACT_WITH_PORTAL_PROOF',
    titleTokenRef: 'screen.childDisclosure.title',
    introTokenRef: 'screen.childDisclosure.intro',
    statuses: ScreenChildDisclosureStatuses,
    hiddenCaptureAllowed: false,
    rawScreenshotPathVisible: false,
    rawScreenshotRemoteUploadEnabled: false,
    productionChildAppClaimed: false,
    servicePersistenceClaimed: false,
    osNotificationClaimed: false,
    localOnlyDisclosure: true,
  });
}

const ScreenChildDisclosureStatuses: readonly ScreenChildDisclosureStatus[] = [
  screenChildDisclosureStatus({
    statusId: 'screen-child-disclosure-disabled',
    state: 'disabled',
    indicator: 'off',
    capabilityStatus: 'disabledByParent',
    disclosureState: 'requiredShown',
    captureActive: false,
    pausedByParent: false,
    auditRef: 'screen-child-disclosure-audit-disabled',
    copyRefs: {
      titleTokenRef: 'screen.childDisclosure.disabled.title',
      bodyTokenRef: 'screen.childDisclosure.disabled.body',
      statusTokenRef: 'screen.childDisclosure.disabled.status',
      actionTokenRef: 'screen.childDisclosure.action.askParent',
    },
    reason: 'parent setting keeps local screen checks off',
  }),
  screenChildDisclosureStatus({
    statusId: 'screen-child-disclosure-paused',
    state: 'paused',
    indicator: 'paused',
    capabilityStatus: 'ready',
    disclosureState: 'requiredShown',
    captureActive: false,
    pausedByParent: true,
    auditRef: 'screen-child-disclosure-audit-paused',
    copyRefs: {
      titleTokenRef: 'screen.childDisclosure.paused.title',
      bodyTokenRef: 'screen.childDisclosure.paused.body',
      statusTokenRef: 'screen.childDisclosure.paused.status',
      actionTokenRef: 'screen.childDisclosure.action.askParent',
    },
    reason: 'parent paused local screen checks',
  }),
  screenChildDisclosureStatus({
    statusId: 'screen-child-disclosure-ready',
    state: 'ready',
    indicator: 'ready',
    capabilityStatus: 'ready',
    disclosureState: 'requiredShown',
    captureActive: false,
    pausedByParent: false,
    auditRef: 'screen-child-disclosure-audit-ready',
    copyRefs: {
      titleTokenRef: 'screen.childDisclosure.ready.title',
      bodyTokenRef: 'screen.childDisclosure.ready.body',
      statusTokenRef: 'screen.childDisclosure.ready.status',
      actionTokenRef: 'screen.childDisclosure.action.askParent',
    },
    reason: 'local screen checks are enabled and waiting for a parent-approved trigger',
  }),
  screenChildDisclosureStatus({
    statusId: 'screen-child-disclosure-active',
    state: 'captureActive',
    indicator: 'active',
    capabilityStatus: 'ready',
    disclosureState: 'requiredShown',
    captureActive: true,
    pausedByParent: false,
    auditRef: 'screen-child-disclosure-audit-active',
    copyRefs: {
      titleTokenRef: 'screen.childDisclosure.active.title',
      bodyTokenRef: 'screen.childDisclosure.active.body',
      statusTokenRef: 'screen.childDisclosure.active.status',
      actionTokenRef: 'screen.childDisclosure.action.askParent',
    },
    reason: 'a parent-approved local screen check is active',
  }),
  screenChildDisclosureStatus({
    statusId: 'screen-child-disclosure-protected-surface',
    state: 'protectedSurface',
    indicator: 'unavailable',
    capabilityStatus: 'protectedSurface',
    disclosureState: 'requiredShown',
    captureActive: false,
    pausedByParent: false,
    auditRef: 'screen-child-disclosure-audit-protected-surface',
    copyRefs: {
      titleTokenRef: 'screen.childDisclosure.protected.title',
      bodyTokenRef: 'screen.childDisclosure.protected.body',
      statusTokenRef: 'screen.childDisclosure.protected.status',
      actionTokenRef: 'screen.childDisclosure.action.askParent',
    },
    reason: 'protected surfaces are skipped and disclosed locally',
  }),
];

function screenChildDisclosureStatus(input: {
  readonly statusId: string;
  readonly state: ScreenChildDisclosureState;
  readonly indicator: ScreenChildDisclosureIndicator;
  readonly capabilityStatus: ScreenChildDisclosureStatus['capabilityStatus'];
  readonly disclosureState: ScreenChildDisclosureStatus['disclosureState'];
  readonly captureActive: boolean;
  readonly pausedByParent: boolean;
  readonly auditRef: string;
  readonly copyRefs: {
    readonly titleTokenRef: string;
    readonly bodyTokenRef: string;
    readonly statusTokenRef: string;
    readonly actionTokenRef: string;
  };
  readonly reason: string;
}): ScreenChildDisclosureStatus {
  return ScreenChildDisclosureStatusSchema.parse({
    schemaVersion: ScreenChildDisclosureUxSchemaVersion,
    surface: 'localStatus',
    delivery: 'childDeviceLocal',
    parentSettingRef: ParentSettingRef,
    settingVersion: SettingVersion,
    updatedAt: GeneratedAt,
    custodyState: 'live-local-child-agent',
    childVisible: true,
    visibleStatusRequired: true,
    hiddenCaptureAllowed: false,
    rawScreenshotPathVisible: false,
    rawScreenshotRemoteUploadEnabled: false,
    localOnlyDisclosure: true,
    ...input,
  });
}

function screenChildDisclosureStatusIsConsistent(value: ScreenChildDisclosureStatusBase): boolean {
  if (!baseDisclosureStatusIsSafe(value)) {
    return false;
  }
  if (value.state === 'disabled') {
    return disabledDisclosureStatusIsConsistent(value);
  }
  if (value.state === 'paused') {
    return pausedDisclosureStatusIsConsistent(value);
  }
  if (value.state === 'ready') {
    return readyDisclosureStatusIsConsistent(value);
  }
  if (value.state === 'captureActive') {
    return captureActiveDisclosureStatusIsConsistent(value);
  }
  return protectedSurfaceDisclosureStatusIsConsistent(value);
}

function baseDisclosureStatusIsSafe(value: ScreenChildDisclosureStatusBase): boolean {
  return (
    value.childVisible &&
    value.visibleStatusRequired &&
    !value.hiddenCaptureAllowed &&
    !value.rawScreenshotPathVisible &&
    !value.rawScreenshotRemoteUploadEnabled &&
    value.localOnlyDisclosure &&
    value.delivery === 'childDeviceLocal' &&
    value.custodyState === 'live-local-child-agent' &&
    value.disclosureState === 'requiredShown' &&
    value.auditRef !== null
  );
}

function disabledDisclosureStatusIsConsistent(value: ScreenChildDisclosureStatusBase): boolean {
  return (
    value.indicator === 'off' &&
    value.capabilityStatus === 'disabledByParent' &&
    !value.captureActive &&
    !value.pausedByParent
  );
}

function pausedDisclosureStatusIsConsistent(value: ScreenChildDisclosureStatusBase): boolean {
  return (
    value.indicator === 'paused' && value.capabilityStatus === 'ready' && !value.captureActive && value.pausedByParent
  );
}

function readyDisclosureStatusIsConsistent(value: ScreenChildDisclosureStatusBase): boolean {
  return (
    value.indicator === 'ready' && value.capabilityStatus === 'ready' && !value.captureActive && !value.pausedByParent
  );
}

function captureActiveDisclosureStatusIsConsistent(value: ScreenChildDisclosureStatusBase): boolean {
  return (
    value.indicator === 'active' && value.capabilityStatus === 'ready' && value.captureActive && !value.pausedByParent
  );
}

function protectedSurfaceDisclosureStatusIsConsistent(value: ScreenChildDisclosureStatusBase): boolean {
  return (
    value.indicator === 'unavailable' &&
    value.capabilityStatus === 'protectedSurface' &&
    !value.captureActive &&
    !value.pausedByParent
  );
}

function screenChildDisclosureStatesAreComplete(statuses: readonly ScreenChildDisclosureStatus[]): boolean {
  const states = new Set(statuses.map((status) => status.state));
  return (
    states.has('disabled') &&
    states.has('paused') &&
    states.has('ready') &&
    states.has('captureActive') &&
    states.has('protectedSurface')
  );
}

function screenChildDisclosureStatusesAreSafe(statuses: readonly ScreenChildDisclosureStatus[]): boolean {
  return statuses.every(
    (status) =>
      status.childVisible &&
      !status.hiddenCaptureAllowed &&
      !status.rawScreenshotPathVisible &&
      !status.rawScreenshotRemoteUploadEnabled &&
      status.localOnlyDisclosure
  );
}
