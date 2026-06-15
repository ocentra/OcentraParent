import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameAndroidPhysicalDeviceProofSchema,
  type AppGameAndroidPhysicalDeviceProof,
} from './app-game-android-physical-device-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const AndroidAccessibilityText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAndroidAccessibilityOverlayPreflightSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-accessibility-overlay-preflight')
);

export const AppGameAndroidAccessibilityOverlayPreflightStateSchema = withParser(
  Schema.Literal('accessibility-service-not-enabled', 'accessibility-service-enabled')
);

export const AppGameAndroidAccessibilityOverlayPreflightActionSchema = withParser(
  Schema.Literal('warning-overlay', 'block-overlay', 'request-overlay', 'usage-context-overlay')
);

export const AppGameAndroidAccessibilityOverlayPreflightProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-accessibility-settings-ref',
    'android-accessibility-enabled-services-ref',
    'android-accessibility-service-enable-proof'
  )
);

export const AppGameAndroidAccessibilityOverlayPreflightBlockerSchema = withParser(
  Schema.Literal(
    'android-accessibility-service-not-enabled',
    'android-accessibility-service-names-redacted',
    'android-overlay-runtime-not-proved',
    'android-adapter-dispatch-blocked-before-accessibility'
  )
);

const AndroidAccessibilityLabelSchema = AndroidAccessibilityText.pipe(
  Schema.brand('AppGameAndroidAccessibilityOverlayPreflightLabel')
);

const AndroidAccessibilityCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

export const AppGameAndroidAccessibilitySettingsSampleSchema = withParser(
  Schema.Struct({
    accessibilityEnabled: Schema.Boolean,
    enabledServiceCount: AndroidAccessibilityCountSchema,
    serviceNamesRedacted: Schema.Literal(true),
    settingsReadable: Schema.Boolean,
  })
);

const AndroidAccessibilityOverlayPreflightRowBaseSchema = Schema.Struct({
  action: AppGameAndroidAccessibilityOverlayPreflightActionSchema,
  preflightState: AppGameAndroidAccessibilityOverlayPreflightStateSchema,
  requiredProofRefs: Schema.Array(AppGameAndroidAccessibilityOverlayPreflightProofRefSchema),
  blockerRefs: Schema.Array(AppGameAndroidAccessibilityOverlayPreflightBlockerSchema),
  canDispatchAdapter: Schema.Boolean,
  accessibilityServiceClaimed: Schema.Literal(false),
  overlayRuntimeClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
});

const AndroidAccessibilityOverlayPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidAccessibilityOverlayPreflightSchemaVersionSchema,
  preflightId: AndroidAccessibilityLabelSchema,
  generatedAt: ParentTimestampSchema,
  sourceProofId: AndroidAccessibilityLabelSchema,
  preflightState: AppGameAndroidAccessibilityOverlayPreflightStateSchema,
  physicalDeviceConnected: Schema.Boolean,
  accessibilitySettingsReadable: Schema.Boolean,
  accessibilityEnabled: Schema.Boolean,
  enabledServiceCount: AndroidAccessibilityCountSchema,
  serviceNamesRedacted: Schema.Literal(true),
  rows: Schema.Array(AndroidAccessibilityOverlayPreflightRowBaseSchema),
  dispatchableActionCount: AndroidAccessibilityCountSchema,
  blockedActionCount: AndroidAccessibilityCountSchema,
  rawAccessibilityServiceNamesClaimed: Schema.Literal(false),
  rawOverlayContentClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  proofRefs: Schema.Array(AppGameAndroidAccessibilityOverlayPreflightProofRefSchema),
  openBlockers: Schema.Array(AppGameAndroidAccessibilityOverlayPreflightBlockerSchema),
  parentVisibleSummary: AndroidAccessibilityLabelSchema,
});

type AndroidAccessibilityOverlayPreflightRowCandidate = Infer<typeof AndroidAccessibilityOverlayPreflightRowBaseSchema>;
type AndroidAccessibilityOverlayPreflightReadModelCandidate = Infer<
  typeof AndroidAccessibilityOverlayPreflightReadModelBaseSchema
>;

export const AppGameAndroidAccessibilityOverlayPreflightRowSchema = withParser(
  AndroidAccessibilityOverlayPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        androidAccessibilityOverlayPreflightRowIsHonest(row) ||
        'Expected Android Accessibility overlay rows to remain blocked until an enabled service and overlay runtime proof exist'
    )
  )
);

export const AppGameAndroidAccessibilityOverlayPreflightReadModelSchema = withParser(
  AndroidAccessibilityOverlayPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        androidAccessibilityOverlayPreflightReadModelIsHonest(readModel) ||
        'Expected Android Accessibility overlay preflight to keep service names redacted and overlay execution unclaimed'
    )
  )
);

export type AppGameAndroidAccessibilitySettingsSample = Infer<typeof AppGameAndroidAccessibilitySettingsSampleSchema>;
export type AppGameAndroidAccessibilityOverlayPreflightRow = Infer<
  typeof AppGameAndroidAccessibilityOverlayPreflightRowSchema
>;
export type AppGameAndroidAccessibilityOverlayPreflightReadModel = Infer<
  typeof AppGameAndroidAccessibilityOverlayPreflightReadModelSchema
>;

export const decodeAppGameAndroidAccessibilityOverlayPreflightReadModel = Schema.decodeUnknownSync(
  AppGameAndroidAccessibilityOverlayPreflightReadModelSchema
);

export function createAppGameAndroidAccessibilityOverlayPreflightReadModel(input: {
  readonly androidProof: AppGameAndroidPhysicalDeviceProof;
  readonly accessibilitySettings: AppGameAndroidAccessibilitySettingsSample;
  readonly generatedAt: AppGameAndroidAccessibilityOverlayPreflightReadModel['generatedAt'];
}): AppGameAndroidAccessibilityOverlayPreflightReadModel {
  const proof = AppGameAndroidPhysicalDeviceProofSchema.parse(input.androidProof);
  const settings = AppGameAndroidAccessibilitySettingsSampleSchema.parse(input.accessibilitySettings);
  const serviceEnabled = settings.accessibilityEnabled && settings.enabledServiceCount > 0;
  const rows = androidAccessibilityActions().map((action) =>
    androidAccessibilityOverlayPreflightRow(action, serviceEnabled)
  );

  return decodeAppGameAndroidAccessibilityOverlayPreflightReadModel({
    schemaVersion: 'app-game-android-accessibility-overlay-preflight',
    preflightId: 'android-accessibility-overlay-preflight-ref',
    generatedAt: input.generatedAt,
    sourceProofId: proof.proofId,
    preflightState: serviceEnabled ? 'accessibility-service-enabled' : 'accessibility-service-not-enabled',
    physicalDeviceConnected: proof.connectionState === 'physical-device-connected',
    accessibilitySettingsReadable: settings.settingsReadable,
    accessibilityEnabled: settings.accessibilityEnabled,
    enabledServiceCount: settings.enabledServiceCount,
    serviceNamesRedacted: true,
    rows,
    dispatchableActionCount: rows.filter((row) => row.canDispatchAdapter).length,
    blockedActionCount: rows.filter((row) => !row.canDispatchAdapter).length,
    rawAccessibilityServiceNamesClaimed: false,
    rawOverlayContentClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: androidAccessibilityProofRefs(settings),
    openBlockers: androidAccessibilityOpenBlockers(serviceEnabled),
    parentVisibleSummary: serviceEnabled
      ? 'Android Accessibility settings show an enabled service count, but overlay runtime and child-device delivery proof are still required before any app/game overlay action can dispatch.'
      : 'Android physical device is reachable, but no enabled Accessibility service proof is attached, so warning/block/request overlay actions remain blocked before adapter dispatch.',
  });
}

export function summarizeAppGameAndroidAccessibilityOverlayPreflightReadModel(
  readModel: AppGameAndroidAccessibilityOverlayPreflightReadModel
) {
  return {
    preflightState: readModel.preflightState,
    accessibilitySettingsReadable: readModel.accessibilitySettingsReadable,
    accessibilityEnabled: readModel.accessibilityEnabled,
    enabledServiceCount: readModel.enabledServiceCount,
    dispatchableActionCount: readModel.dispatchableActionCount,
    blockedActionCount: readModel.blockedActionCount,
  } as const;
}

function androidAccessibilityActions() {
  return ['warning-overlay', 'block-overlay', 'request-overlay', 'usage-context-overlay'] as const;
}

function androidAccessibilityOverlayPreflightRow(
  action: ReturnType<typeof androidAccessibilityActions>[number],
  serviceEnabled: boolean
): AppGameAndroidAccessibilityOverlayPreflightRow {
  return AppGameAndroidAccessibilityOverlayPreflightRowSchema.parse({
    action,
    preflightState: serviceEnabled ? 'accessibility-service-enabled' : 'accessibility-service-not-enabled',
    requiredProofRefs: ['android-accessibility-service-enable-proof'],
    blockerRefs: androidAccessibilityOpenBlockers(serviceEnabled),
    canDispatchAdapter: false,
    accessibilityServiceClaimed: false,
    overlayRuntimeClaimed: false,
    platformEnforcementClaimed: false,
  });
}

function androidAccessibilityProofRefs(settings: AppGameAndroidAccessibilitySettingsSample) {
  const refs = ['android-physical-adb-device-ref', 'android-accessibility-settings-ref'];
  if (settings.settingsReadable) {
    refs.push('android-accessibility-enabled-services-ref');
  }
  return refs;
}

function androidAccessibilityOpenBlockers(serviceEnabled: boolean) {
  const blockers = ['android-overlay-runtime-not-proved', 'android-adapter-dispatch-blocked-before-accessibility'];
  if (!serviceEnabled) {
    blockers.unshift('android-accessibility-service-not-enabled');
  }
  blockers.push('android-accessibility-service-names-redacted');
  return blockers;
}

function androidAccessibilityOverlayPreflightRowIsHonest(
  row: AndroidAccessibilityOverlayPreflightRowCandidate
): boolean {
  return (
    !row.canDispatchAdapter &&
    row.blockerRefs.includes('android-overlay-runtime-not-proved') &&
    row.blockerRefs.includes('android-adapter-dispatch-blocked-before-accessibility') &&
    !row.accessibilityServiceClaimed &&
    !row.overlayRuntimeClaimed &&
    !row.platformEnforcementClaimed
  );
}

function androidAccessibilityOverlayPreflightReadModelIsHonest(
  readModel: AndroidAccessibilityOverlayPreflightReadModelCandidate
): boolean {
  return (
    readModel.dispatchableActionCount === 0 &&
    readModel.blockedActionCount === readModel.rows.length &&
    readModel.physicalDeviceConnected &&
    readModel.serviceNamesRedacted &&
    readModel.openBlockers.includes('android-accessibility-service-names-redacted') &&
    !readModel.rawAccessibilityServiceNamesClaimed &&
    !readModel.rawOverlayContentClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.childDeviceDeliveryClaimed
  );
}
