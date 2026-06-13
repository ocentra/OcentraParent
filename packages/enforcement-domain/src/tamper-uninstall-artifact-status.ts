import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentControlCapabilityNameSchema,
  ParentControlCapabilityStatusSchema,
  ParentControlPlatformSchema,
} from '@ocentra-parent/capability-domain/capabilities';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
const tamperUninstallArtifactText = <Brand extends string>(brand: Brand) =>
  NonEmptyStringSchema.pipe(Schema.brand(brand));

export const TamperUninstallArtifactStatusReadModelIdSchema = tamperUninstallArtifactText(
  'TamperUninstallArtifactStatusReadModelId'
);
export const TamperUninstallArtifactStatusEntryIdSchema = tamperUninstallArtifactText(
  'TamperUninstallArtifactStatusEntryId'
);
export const TamperUninstallArtifactStatusReferenceSchema = tamperUninstallArtifactText(
  'TamperUninstallArtifactStatusReference'
);
export const TamperUninstallArtifactStatusRequirementSchema = tamperUninstallArtifactText(
  'TamperUninstallArtifactStatusRequirement'
);
export const TamperUninstallArtifactStatusBoundarySchema = tamperUninstallArtifactText(
  'TamperUninstallArtifactStatusBoundary'
);

export const TamperUninstallArtifactSurfaceSchema = withParser(
  Schema.Literal(
    'windows-service-stop',
    'windows-package-uninstall',
    'linux-service-package',
    'macos-launchd-package',
    'android-package-removed',
    'android-device-owner-managed-profile',
    'ios-family-controls-device-activity',
    'admin-removal-flow'
  )
);
export const TamperUninstallArtifactStateSchema = withParser(
  Schema.Literal('manual-required', 'device-proof-required', 'not-collected', 'unsupported', 'documented-admin-removal')
);
export const TamperUninstallParentVisibleStatusSchema = withParser(
  Schema.Literal('artifact-needed', 'device-proof-needed', 'status-only', 'unsupported', 'admin-removal-documented')
);
export const TamperUninstallArtifactCustodySchema = withParser(
  Schema.Literal('not-collected', 'manual-review-required', 'package-local-status-only', 'documented-flow-only')
);

const RequiredSurfaces = [
  'windows-service-stop',
  'windows-package-uninstall',
  'linux-service-package',
  'macos-launchd-package',
  'android-package-removed',
  'android-device-owner-managed-profile',
  'ios-family-controls-device-activity',
  'admin-removal-flow',
] as const satisfies ReadonlyArray<TamperUninstallArtifactSurface>;

const TamperUninstallArtifactStatusEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  statusEntryId: TamperUninstallArtifactStatusEntryIdSchema,
  surface: TamperUninstallArtifactSurfaceSchema,
  platform: ParentControlPlatformSchema,
  capability: ParentControlCapabilityNameSchema,
  capabilityStatus: ParentControlCapabilityStatusSchema,
  artifactState: TamperUninstallArtifactStateSchema,
  parentVisibleStatus: TamperUninstallParentVisibleStatusSchema,
  custodyState: TamperUninstallArtifactCustodySchema,
  requiredArtifacts: Schema.Array(TamperUninstallArtifactStatusRequirementSchema),
  sourceProofRefs: Schema.Array(TamperUninstallArtifactStatusReferenceSchema),
  auditRefs: Schema.Array(TamperUninstallArtifactStatusReferenceSchema),
  integrityRefs: Schema.Array(TamperUninstallArtifactStatusReferenceSchema),
  adminRemovalFlowRefs: Schema.Array(TamperUninstallArtifactStatusReferenceSchema),
  boundary: TamperUninstallArtifactStatusBoundarySchema,
  uninstallDetectionClaimed: Schema.Boolean,
  tamperResistanceClaimed: Schema.Boolean,
  stealthPersistenceClaimed: Schema.Boolean,
  privilegeEscalationClaimed: Schema.Boolean,
  adminRemovalBlockingClaimed: Schema.Boolean,
  providerDeliveryClaimed: Schema.Boolean,
  rawChildDataIncluded: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type TamperUninstallArtifactStatusEntryCandidate = Infer<typeof TamperUninstallArtifactStatusEntryBaseSchema>;

export const TamperUninstallArtifactStatusEntrySchema = withParser(
  TamperUninstallArtifactStatusEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        tamperUninstallArtifactStatusEntryIsHonest(entry) ||
        'Expected tamper uninstall artifact status entries to require manual/device proof without uninstall, anti-tamper, provider-delivery, admin-removal-blocking, privilege, stealth, or raw child data claims'
    )
  )
);

export const TamperUninstallArtifactStatusReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: TamperUninstallArtifactStatusReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(TamperUninstallArtifactStatusReferenceSchema),
    entries: Schema.Array(TamperUninstallArtifactStatusEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.statusEntryId)).size === readModel.entries.length ||
        'Expected tamper uninstall artifact status entry ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        requiredValuesArePresent(
          readModel.entries.map((entry) => entry.surface),
          RequiredSurfaces
        ) || 'Expected tamper uninstall artifact status to cover every required platform/removal surface'
    )
  )
);

function tamperUninstallArtifactStatusEntryIsHonest(entry: TamperUninstallArtifactStatusEntryCandidate): boolean {
  return (
    !tamperUninstallArtifactStatusHasClaimUpgrade(entry) &&
    entry.requiredArtifacts.length > 0 &&
    entry.sourceProofRefs.length > 0 &&
    entry.auditRefs.length > 0 &&
    entry.integrityRefs.length > 0 &&
    tamperUninstallArtifactStatusMatchesState(entry)
  );
}

function tamperUninstallArtifactStatusHasClaimUpgrade(entry: TamperUninstallArtifactStatusEntryCandidate): boolean {
  return [
    entry.uninstallDetectionClaimed,
    entry.tamperResistanceClaimed,
    entry.stealthPersistenceClaimed,
    entry.privilegeEscalationClaimed,
    entry.adminRemovalBlockingClaimed,
    entry.providerDeliveryClaimed,
    entry.rawChildDataIncluded,
  ].some(Boolean);
}

function tamperUninstallArtifactStatusMatchesState(entry: TamperUninstallArtifactStatusEntryCandidate): boolean {
  if (entry.surface === 'admin-removal-flow') {
    return (
      entry.artifactState === 'documented-admin-removal' &&
      entry.parentVisibleStatus === 'admin-removal-documented' &&
      entry.custodyState === 'documented-flow-only' &&
      entry.adminRemovalFlowRefs.length > 0
    );
  }
  if (entry.platform === 'android' || entry.platform === 'ios') {
    return entry.artifactState === 'device-proof-required' && entry.parentVisibleStatus === 'device-proof-needed';
  }
  return entry.artifactState === 'manual-required' && entry.parentVisibleStatus === 'artifact-needed';
}

function requiredValuesArePresent<T extends string>(actualValues: ReadonlyArray<T>, requiredValues: ReadonlyArray<T>) {
  const actual = new Set(actualValues);
  return actual.size === actualValues.length && requiredValues.every((value) => actual.has(value));
}

export type TamperUninstallArtifactSurface = Infer<typeof TamperUninstallArtifactSurfaceSchema>;
export type TamperUninstallArtifactState = Infer<typeof TamperUninstallArtifactStateSchema>;
export type TamperUninstallParentVisibleStatus = Infer<typeof TamperUninstallParentVisibleStatusSchema>;
export type TamperUninstallArtifactCustody = Infer<typeof TamperUninstallArtifactCustodySchema>;
export type TamperUninstallArtifactStatusEntry = Infer<typeof TamperUninstallArtifactStatusEntrySchema>;
export type TamperUninstallArtifactStatusReadModel = Infer<typeof TamperUninstallArtifactStatusReadModelSchema>;

export const decodeTamperUninstallArtifactStatusEntry = Schema.decodeUnknownSync(
  TamperUninstallArtifactStatusEntrySchema
);
export const decodeTamperUninstallArtifactStatusReadModel = Schema.decodeUnknownSync(
  TamperUninstallArtifactStatusReadModelSchema
);

