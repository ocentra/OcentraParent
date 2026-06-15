import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const EnabledSampleText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidAccessibilityEnabledSampleProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-accessibility-enabled-sample-proof')
);

export const AppGameAndroidAccessibilityEnabledSampleRuntimeStateSchema = withParser(
  Schema.Literal('accessibility-runtime-bound')
);

export const AppGameAndroidAccessibilityEnabledSampleEventStateSchema = withParser(
  Schema.Literal('accessibility-event-sample-observed')
);

export const AppGameAndroidAccessibilityEnabledSampleProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-accessibility-service-settings-enable-ref',
    'android-accessibility-service-ui-ref',
    'android-accessibility-window-state-count-ref'
  )
);

export const AppGameAndroidAccessibilityEnabledSampleGapSchema = withParser(
  Schema.Literal(
    'android-accessibility-overlay-runtime-not-proved',
    'android-device-owner-authority-not-proved',
    'android-play-policy-not-proved',
    'android-child-device-delivery-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const EnabledSampleLabelSchema = EnabledSampleText.pipe(
  Schema.brand('AppGameAndroidAccessibilityEnabledSampleProofLabel')
);

const AppGameAndroidAccessibilityEnabledSampleProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidAccessibilityEnabledSampleProofSchemaVersionSchema,
  packageId: EnabledSampleLabelSchema,
  runtimeState: AppGameAndroidAccessibilityEnabledSampleRuntimeStateSchema,
  eventSampleState: AppGameAndroidAccessibilityEnabledSampleEventStateSchema,
  serviceEnabledBySettings: Schema.Literal(true),
  uiStateObserved: Schema.Literal(true),
  eventSampleCount: Schema.Number.pipe(Schema.int(), Schema.greaterThan(0)),
  proofRefs: Schema.Array(AppGameAndroidAccessibilityEnabledSampleProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidAccessibilityEnabledSampleGapSchema),
  rawAccessibilityEventRowsStored: Schema.Literal(false),
  rawAccessibilityServiceNamesStored: Schema.Literal(false),
  rawOverlayContentStored: Schema.Literal(false),
  overlayRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  parentVisibleSummary: EnabledSampleLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type EnabledSampleCandidate = Infer<typeof AppGameAndroidAccessibilityEnabledSampleProofBaseSchema>;

export const AppGameAndroidAccessibilityEnabledSampleProofSchema = withParser(
  AppGameAndroidAccessibilityEnabledSampleProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        enabledSampleProofIsHonest(proof) ||
        'Expected Android Accessibility enabled sample proof to require settings enablement, bound service, observed count-only window-state events, and no raw-event, overlay, dispatch, delivery, or enforcement claims'
    )
  )
);

export type AppGameAndroidAccessibilityEnabledSampleProof = Infer<
  typeof AppGameAndroidAccessibilityEnabledSampleProofSchema
>;

export const decodeAppGameAndroidAccessibilityEnabledSampleProof = Schema.decodeUnknownSync(
  AppGameAndroidAccessibilityEnabledSampleProofSchema
);

export function createAppGameAndroidAccessibilityEnabledSampleProof(input: {
  readonly eventSampleCount: AppGameAndroidAccessibilityEnabledSampleProof['eventSampleCount'];
  readonly checkedAt: AppGameAndroidAccessibilityEnabledSampleProof['checkedAt'];
}): AppGameAndroidAccessibilityEnabledSampleProof {
  return decodeAppGameAndroidAccessibilityEnabledSampleProof({
    schemaVersion: 'app-game-android-accessibility-enabled-sample-proof',
    packageId: 'ca.ocentra.parent.agent',
    runtimeState: 'accessibility-runtime-bound',
    eventSampleState: 'accessibility-event-sample-observed',
    serviceEnabledBySettings: true,
    uiStateObserved: true,
    eventSampleCount: input.eventSampleCount,
    proofRefs: [
      'android-physical-adb-device-ref',
      'android-accessibility-service-settings-enable-ref',
      'android-accessibility-service-ui-ref',
      'android-accessibility-window-state-count-ref',
    ],
    openGaps: [
      'android-accessibility-overlay-runtime-not-proved',
      'android-device-owner-authority-not-proved',
      'android-play-policy-not-proved',
      'android-child-device-delivery-not-proved',
      'android-platform-enforcement-not-proved',
    ],
    rawAccessibilityEventRowsStored: false,
    rawAccessibilityServiceNamesStored: false,
    rawOverlayContentStored: false,
    overlayRuntimeClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    childDeviceDeliveryClaimed: false,
    parentVisibleSummary:
      'Android Accessibility service is settings-enabled and reports count-only window-state events; overlays, delivery, dispatch, and enforcement remain unclaimed.',
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidAccessibilityEnabledSampleProof(
  proof: AppGameAndroidAccessibilityEnabledSampleProof
) {
  return {
    runtimeState: proof.runtimeState,
    eventSampleState: proof.eventSampleState,
    eventSampleCount: proof.eventSampleCount,
    openGapCount: proof.openGaps.length,
  } as const;
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function enabledSampleProofIsHonest(proof: EnabledSampleCandidate): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.runtimeState === 'accessibility-runtime-bound' &&
    proof.eventSampleState === 'accessibility-event-sample-observed' &&
    proof.eventSampleCount > 0 &&
    proof.proofRefs.includes('android-physical-adb-device-ref') &&
    proof.proofRefs.includes('android-accessibility-service-settings-enable-ref') &&
    proof.proofRefs.includes('android-accessibility-service-ui-ref') &&
    proof.proofRefs.includes('android-accessibility-window-state-count-ref') &&
    proof.openGaps.includes('android-accessibility-overlay-runtime-not-proved') &&
    proof.openGaps.includes('android-device-owner-authority-not-proved') &&
    proof.openGaps.includes('android-play-policy-not-proved') &&
    proof.openGaps.includes('android-child-device-delivery-not-proved') &&
    proof.openGaps.includes('android-platform-enforcement-not-proved')
  );
}
