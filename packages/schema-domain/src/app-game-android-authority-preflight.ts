import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  AppGameAndroidPhysicalDeviceProofSchema,
  type AppGameAndroidPhysicalDeviceProof,
} from './app-game-android-physical-device-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameAndroidAuthorityPreflightSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-authority-preflight')
);

export const AppGameAndroidAuthorityPreflightStateSchema = withParser(
  Schema.Literal('authority-ready', 'authority-not-enrolled')
);

export const AppGameAndroidAuthorityPreflightActionSchema = withParser(
  Schema.Literal('hide-package', 'suspend-package', 'uninstall-block', 'lock-task', 'managed-configuration')
);

export const AppGameAndroidAuthorityPreflightProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-physical-device-policy-ref',
    'android-device-owner-proof',
    'android-profile-owner-proof'
  )
);

export const AppGameAndroidAuthorityPreflightBlockerSchema = withParser(
  Schema.Literal(
    'android-device-owner-not-proved',
    'android-profile-owner-not-proved',
    'android-managed-profile-not-proved',
    'android-package-policy-not-dispatchable',
    'android-adapter-dispatch-blocked-before-authority'
  )
);

const AndroidAuthorityLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidAuthorityPreflightLabel');

const AppGameAndroidAuthorityPreflightActionRowBaseSchema = Schema.Struct({
  action: AppGameAndroidAuthorityPreflightActionSchema,
  authorityState: AppGameAndroidAuthorityPreflightStateSchema,
  requiredProofRefs: Schema.Array(AppGameAndroidAuthorityPreflightProofRefSchema),
  blockerRefs: Schema.Array(AppGameAndroidAuthorityPreflightBlockerSchema),
  canDispatchAdapter: Schema.Boolean,
  devicePolicyManagerCallClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
});

export const AppGameAndroidAuthorityPreflightActionRowSchema = withParser(
  AppGameAndroidAuthorityPreflightActionRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        androidAuthorityActionRowIsHonest(row) ||
        'Expected Android authority preflight action rows to require owner/profile-owner proof before adapter dispatch'
    )
  )
);

const AppGameAndroidAuthorityPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidAuthorityPreflightSchemaVersionSchema,
  preflightId: AndroidAuthorityLabelSchema,
  generatedAt: ParentTimestampSchema,
  sourceProofId: AndroidAuthorityLabelSchema,
  authorityState: AppGameAndroidAuthorityPreflightStateSchema,
  deviceOwnerProofAttached: Schema.Boolean,
  profileOwnerProofAttached: Schema.Boolean,
  adbDeviceVisible: Schema.Boolean,
  policyServiceVisible: Schema.Boolean,
  rows: Schema.Array(AppGameAndroidAuthorityPreflightActionRowSchema),
  dispatchableActionCount: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
  blockedActionCount: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
  rawDeviceSerialClaimed: Schema.Literal(false),
  rawPackageNamesClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  proofRefs: Schema.Array(AppGameAndroidAuthorityPreflightProofRefSchema),
  openBlockers: Schema.Array(AppGameAndroidAuthorityPreflightBlockerSchema),
  parentVisibleSummary: AndroidAuthorityLabelSchema,
});

type AppGameAndroidAuthorityPreflightReadModelCandidate = Infer<
  typeof AppGameAndroidAuthorityPreflightReadModelBaseSchema
>;
type AppGameAndroidAuthorityPreflightActionRowCandidate = Infer<
  typeof AppGameAndroidAuthorityPreflightActionRowBaseSchema
>;

export const AppGameAndroidAuthorityPreflightReadModelSchema = withParser(
  AppGameAndroidAuthorityPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        androidAuthorityPreflightReadModelIsHonest(readModel) ||
        'Expected Android authority preflight to keep package lifecycle controls blocked until Device Owner or Profile Owner proof is attached'
    )
  )
);

export type AppGameAndroidAuthorityPreflightActionRow = Infer<typeof AppGameAndroidAuthorityPreflightActionRowSchema>;
export type AppGameAndroidAuthorityPreflightReadModel = Infer<typeof AppGameAndroidAuthorityPreflightReadModelSchema>;

export const decodeAppGameAndroidAuthorityPreflightReadModel = Schema.decodeUnknownSync(
  AppGameAndroidAuthorityPreflightReadModelSchema
);

export function createAppGameAndroidAuthorityPreflightReadModel(input: {
  readonly androidProof: AppGameAndroidPhysicalDeviceProof;
  readonly generatedAt: AppGameAndroidAuthorityPreflightReadModel['generatedAt'];
}): AppGameAndroidAuthorityPreflightReadModel {
  const proof = AppGameAndroidPhysicalDeviceProofSchema.parse(input.androidProof);
  const deviceOwnerProofAttached = false;
  const profileOwnerProofAttached = false;
  const rows = androidAuthorityActions().map(androidAuthorityPreflightActionRow);

  return decodeAppGameAndroidAuthorityPreflightReadModel({
    schemaVersion: 'app-game-android-authority-preflight',
    preflightId: 'android-authority-preflight-ref',
    generatedAt: input.generatedAt,
    sourceProofId: proof.proofId,
    authorityState: 'authority-not-enrolled',
    deviceOwnerProofAttached,
    profileOwnerProofAttached,
    adbDeviceVisible: proof.connectionState === 'physical-device-connected',
    policyServiceVisible: proof.proofRefs.includes('android-physical-device-policy-ref'),
    rows,
    dispatchableActionCount: 0,
    blockedActionCount: rows.length,
    rawDeviceSerialClaimed: false,
    rawPackageNamesClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: androidAuthorityProofRefs(),
    openBlockers: androidAuthorityOpenBlockers(),
    parentVisibleSummary:
      'Android physical device and policy service are visible, but Device Owner/Profile Owner authority is not enrolled, so package policy actions remain blocked before adapter dispatch.',
  });
}

export function summarizeAppGameAndroidAuthorityPreflightReadModel(
  readModel: AppGameAndroidAuthorityPreflightReadModel
) {
  return {
    authorityState: readModel.authorityState,
    deviceOwnerProofAttached: readModel.deviceOwnerProofAttached,
    profileOwnerProofAttached: readModel.profileOwnerProofAttached,
    dispatchableActionCount: readModel.dispatchableActionCount,
    blockedActionCount: readModel.blockedActionCount,
    openBlockerCount: readModel.openBlockers.length,
  } as const;
}

function androidAuthorityActions() {
  return ['hide-package', 'suspend-package', 'uninstall-block', 'lock-task', 'managed-configuration'] as const;
}

function androidAuthorityPreflightActionRow(action: ReturnType<typeof androidAuthorityActions>[number]) {
  return AppGameAndroidAuthorityPreflightActionRowSchema.parse({
    action,
    authorityState: 'authority-not-enrolled',
    requiredProofRefs: ['android-device-owner-proof', 'android-profile-owner-proof'],
    blockerRefs: androidAuthorityOpenBlockers(),
    canDispatchAdapter: false,
    devicePolicyManagerCallClaimed: false,
    platformEnforcementClaimed: false,
  });
}

function androidAuthorityProofRefs() {
  return ['android-physical-adb-device-ref', 'android-physical-device-policy-ref'];
}

function androidAuthorityOpenBlockers() {
  return [
    'android-device-owner-not-proved',
    'android-profile-owner-not-proved',
    'android-managed-profile-not-proved',
    'android-package-policy-not-dispatchable',
    'android-adapter-dispatch-blocked-before-authority',
  ];
}

function androidAuthorityActionRowIsHonest(row: AppGameAndroidAuthorityPreflightActionRowCandidate): boolean {
  if (row.authorityState === 'authority-ready') {
    return row.canDispatchAdapter && row.blockerRefs.length === 0 && row.requiredProofRefs.length > 0;
  }

  return (
    !row.canDispatchAdapter &&
    row.blockerRefs.includes('android-device-owner-not-proved') &&
    row.blockerRefs.includes('android-profile-owner-not-proved') &&
    row.blockerRefs.includes('android-adapter-dispatch-blocked-before-authority') &&
    !row.devicePolicyManagerCallClaimed &&
    !row.platformEnforcementClaimed
  );
}

function androidAuthorityPreflightReadModelIsHonest(
  readModel: AppGameAndroidAuthorityPreflightReadModelCandidate
): boolean {
  return (
    readModel.dispatchableActionCount === readModel.rows.filter((row) => row.canDispatchAdapter).length &&
    readModel.blockedActionCount === readModel.rows.filter((row) => !row.canDispatchAdapter).length &&
    androidAuthorityStateMatchesProof(readModel) &&
    !readModel.rawDeviceSerialClaimed &&
    !readModel.rawPackageNamesClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.childDeviceDeliveryClaimed
  );
}

function androidAuthorityStateMatchesProof(readModel: AppGameAndroidAuthorityPreflightReadModelCandidate): boolean {
  if (readModel.authorityState === 'authority-ready') {
    return (
      (readModel.deviceOwnerProofAttached || readModel.profileOwnerProofAttached) &&
      readModel.openBlockers.length === 0 &&
      readModel.dispatchableActionCount === readModel.rows.length
    );
  }

  return (
    !readModel.deviceOwnerProofAttached &&
    !readModel.profileOwnerProofAttached &&
    readModel.openBlockers.includes('android-device-owner-not-proved') &&
    readModel.openBlockers.includes('android-profile-owner-not-proved') &&
    readModel.blockedActionCount === readModel.rows.length
  );
}

