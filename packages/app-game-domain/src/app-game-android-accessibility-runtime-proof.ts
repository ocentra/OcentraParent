import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const AccessibilityRuntimeText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidAccessibilityRuntimeProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-accessibility-runtime-proof')
);

export const AppGameAndroidAccessibilityRuntimeDeclarationStateSchema = withParser(
  Schema.Literal('accessibility-service-declared', 'accessibility-service-missing')
);

export const AppGameAndroidAccessibilityRuntimeStateSchema = withParser(
  Schema.Literal('accessibility-runtime-waiting-for-enablement', 'accessibility-runtime-bound')
);

export const AppGameAndroidAccessibilityRuntimeEventSampleStateSchema = withParser(
  Schema.Literal(
    'accessibility-event-sample-waiting-for-enablement',
    'accessibility-event-sample-observed',
    'accessibility-event-sample-empty'
  )
);

export const AppGameAndroidAccessibilityRuntimeProofRefSchema = withParser(
  Schema.Literal(
    'android-accessibility-service-manifest-ref',
    'android-accessibility-service-config-ref',
    'android-accessibility-service-ui-ref',
    'android-accessibility-settings-ref'
  )
);

export const AppGameAndroidAccessibilityRuntimeGapSchema = withParser(
  Schema.Literal(
    'android-accessibility-service-enable-proof-not-attached',
    'android-accessibility-event-sample-not-observed',
    'android-accessibility-overlay-runtime-not-proved',
    'android-child-device-delivery-not-proved',
    'android-platform-enforcement-not-proved'
  )
);

const AccessibilityRuntimeLabelSchema = AccessibilityRuntimeText.pipe(
  Schema.brand('AppGameAndroidAccessibilityRuntimeProofLabel')
);

const AppGameAndroidAccessibilityRuntimeProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidAccessibilityRuntimeProofSchemaVersionSchema,
  runtimeProofId: AccessibilityRuntimeLabelSchema,
  declarationState: AppGameAndroidAccessibilityRuntimeDeclarationStateSchema,
  runtimeState: AppGameAndroidAccessibilityRuntimeStateSchema,
  eventSampleState: AppGameAndroidAccessibilityRuntimeEventSampleStateSchema,
  manifestServiceDeclared: Schema.Boolean,
  serviceConfigDeclared: Schema.Boolean,
  uiRuntimeStateObserved: Schema.Boolean,
  settingsStateObserved: Schema.Boolean,
  canDispatchOverlayAdapter: Schema.Boolean,
  proofRefs: Schema.Array(AppGameAndroidAccessibilityRuntimeProofRefSchema),
  openGaps: Schema.Array(AppGameAndroidAccessibilityRuntimeGapSchema),
  rawAccessibilityEventRowsStored: Schema.Literal(false),
  rawAccessibilityServiceNamesStored: Schema.Literal(false),
  rawOverlayContentStored: Schema.Literal(false),
  overlayRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  parentVisibleSummary: AccessibilityRuntimeLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type AccessibilityRuntimeCandidate = Infer<typeof AppGameAndroidAccessibilityRuntimeProofBaseSchema>;

export const AppGameAndroidAccessibilityRuntimeProofSchema = withParser(
  AppGameAndroidAccessibilityRuntimeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        accessibilityRuntimeProofIsHonest(proof) ||
        'Expected Android Accessibility runtime proof to require manifest/config/UI evidence and keep overlay execution, adapter dispatch, enforcement, child delivery, and raw event rows unclaimed'
    )
  )
);

export type AppGameAndroidAccessibilityRuntimeProof = Infer<typeof AppGameAndroidAccessibilityRuntimeProofSchema>;

export const decodeAppGameAndroidAccessibilityRuntimeProof = Schema.decodeUnknownSync(
  AppGameAndroidAccessibilityRuntimeProofSchema
);

export function createAppGameAndroidAccessibilityRuntimeProof(input: {
  readonly declarationState: AppGameAndroidAccessibilityRuntimeProof['declarationState'];
  readonly runtimeState: AppGameAndroidAccessibilityRuntimeProof['runtimeState'];
  readonly eventSampleState: AppGameAndroidAccessibilityRuntimeProof['eventSampleState'];
  readonly manifestServiceDeclared: AppGameAndroidAccessibilityRuntimeProof['manifestServiceDeclared'];
  readonly serviceConfigDeclared: AppGameAndroidAccessibilityRuntimeProof['serviceConfigDeclared'];
  readonly uiRuntimeStateObserved: AppGameAndroidAccessibilityRuntimeProof['uiRuntimeStateObserved'];
  readonly settingsStateObserved: AppGameAndroidAccessibilityRuntimeProof['settingsStateObserved'];
  readonly checkedAt: AppGameAndroidAccessibilityRuntimeProof['checkedAt'];
}): AppGameAndroidAccessibilityRuntimeProof {
  return decodeAppGameAndroidAccessibilityRuntimeProof({
    schemaVersion: 'app-game-android-accessibility-runtime-proof',
    runtimeProofId: 'android-accessibility-runtime-proof-ref',
    declarationState: input.declarationState,
    runtimeState: input.runtimeState,
    eventSampleState: input.eventSampleState,
    manifestServiceDeclared: input.manifestServiceDeclared,
    serviceConfigDeclared: input.serviceConfigDeclared,
    uiRuntimeStateObserved: input.uiRuntimeStateObserved,
    settingsStateObserved: input.settingsStateObserved,
    canDispatchOverlayAdapter: false,
    proofRefs: accessibilityRuntimeProofRefs(input),
    openGaps: accessibilityRuntimeOpenGaps(input.runtimeState, input.eventSampleState),
    rawAccessibilityEventRowsStored: false,
    rawAccessibilityServiceNamesStored: false,
    rawOverlayContentStored: false,
    overlayRuntimeClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    parentVisibleSummary: accessibilityRuntimeSummary(input.runtimeState, input.eventSampleState),
    checkedAt: input.checkedAt,
  });
}

export function summarizeAppGameAndroidAccessibilityRuntimeProof(proof: AppGameAndroidAccessibilityRuntimeProof) {
  return {
    declarationState: proof.declarationState,
    runtimeState: proof.runtimeState,
    eventSampleState: proof.eventSampleState,
    manifestServiceDeclared: proof.manifestServiceDeclared,
    serviceConfigDeclared: proof.serviceConfigDeclared,
    uiRuntimeStateObserved: proof.uiRuntimeStateObserved,
    openGapCount: proof.openGaps.length,
  } as const;
}

function accessibilityRuntimeProofRefs(input: {
  readonly manifestServiceDeclared: boolean;
  readonly serviceConfigDeclared: boolean;
  readonly uiRuntimeStateObserved: boolean;
  readonly settingsStateObserved: boolean;
}) {
  const refs = [];
  if (input.manifestServiceDeclared) {
    refs.push('android-accessibility-service-manifest-ref');
  }
  if (input.serviceConfigDeclared) {
    refs.push('android-accessibility-service-config-ref');
  }
  if (input.uiRuntimeStateObserved) {
    refs.push('android-accessibility-service-ui-ref');
  }
  if (input.settingsStateObserved) {
    refs.push('android-accessibility-settings-ref');
  }
  return refs;
}

function accessibilityRuntimeOpenGaps(
  runtimeState: AccessibilityRuntimeCandidate['runtimeState'],
  eventSampleState: AccessibilityRuntimeCandidate['eventSampleState']
) {
  const gaps = [
    'android-accessibility-overlay-runtime-not-proved',
    'android-child-device-delivery-not-proved',
    'android-platform-enforcement-not-proved',
  ];
  if (runtimeState !== 'accessibility-runtime-bound') {
    gaps.unshift('android-accessibility-service-enable-proof-not-attached');
  }
  if (eventSampleState !== 'accessibility-event-sample-observed') {
    gaps.unshift('android-accessibility-event-sample-not-observed');
  }
  return gaps;
}

function accessibilityRuntimeSummary(
  runtimeState: AccessibilityRuntimeCandidate['runtimeState'],
  eventSampleState: AccessibilityRuntimeCandidate['eventSampleState']
) {
  if (runtimeState === 'accessibility-runtime-bound' && eventSampleState === 'accessibility-event-sample-observed') {
    return 'Android Accessibility service is declared and bound with count-only event sample state; overlay execution, child delivery, and platform enforcement remain unclaimed.';
  }

  return 'Android Accessibility service is declared in the package, but enablement or event sample proof is still missing before overlay runtime can be claimed.';
}

// eslint-disable-next-line complexity -- proof honesty predicates intentionally enumerate required evidence gates.
function accessibilityRuntimeProofIsHonest(proof: AccessibilityRuntimeCandidate): boolean {
  return (
    proof.declarationState === 'accessibility-service-declared' &&
    proof.manifestServiceDeclared &&
    proof.serviceConfigDeclared &&
    proof.uiRuntimeStateObserved &&
    proof.proofRefs.includes('android-accessibility-service-manifest-ref') &&
    proof.proofRefs.includes('android-accessibility-service-config-ref') &&
    proof.proofRefs.includes('android-accessibility-service-ui-ref') &&
    proof.openGaps.includes('android-accessibility-overlay-runtime-not-proved') &&
    proof.openGaps.includes('android-child-device-delivery-not-proved') &&
    proof.openGaps.includes('android-platform-enforcement-not-proved') &&
    !proof.canDispatchOverlayAdapter &&
    !proof.rawAccessibilityEventRowsStored &&
    !proof.rawAccessibilityServiceNamesStored &&
    !proof.rawOverlayContentStored &&
    !proof.overlayRuntimeClaimed &&
    !proof.adapterDispatchClaimed &&
    !proof.platformEnforcementClaimed &&
    !proof.childDeviceDeliveryClaimed
  );
}
