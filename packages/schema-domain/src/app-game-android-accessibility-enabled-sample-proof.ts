import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

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

const EnabledSampleLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidAccessibilityEnabledSampleProofLabel');

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

function enabledSampleProofIsHonest(proof: EnabledSampleCandidate): boolean {
  return (
    enabledSampleCoreStateIsHonest(proof) &&
    enabledSampleProofRefsArePresent(proof) &&
    enabledSampleOpenGapsArePresent(proof)
  );
}

function enabledSampleCoreStateIsHonest(proof: EnabledSampleCandidate): boolean {
  return (
    proof.packageId === 'ca.ocentra.parent.agent' &&
    proof.runtimeState === 'accessibility-runtime-bound' &&
    proof.eventSampleState === 'accessibility-event-sample-observed' &&
    proof.eventSampleCount > 0
  );
}

function enabledSampleProofRefsArePresent(proof: EnabledSampleCandidate): boolean {
  return includesAll(proof.proofRefs, [
    'android-physical-adb-device-ref',
    'android-accessibility-service-settings-enable-ref',
    'android-accessibility-service-ui-ref',
    'android-accessibility-window-state-count-ref',
  ] as const);
}

function enabledSampleOpenGapsArePresent(proof: EnabledSampleCandidate): boolean {
  return includesAll(proof.openGaps, [
    'android-accessibility-overlay-runtime-not-proved',
    'android-device-owner-authority-not-proved',
    'android-play-policy-not-proved',
    'android-child-device-delivery-not-proved',
    'android-platform-enforcement-not-proved',
  ] as const);
}

function includesAll<T extends string>(values: readonly T[], required: readonly T[]): boolean {
  return required.every((value) => values.includes(value));
}
