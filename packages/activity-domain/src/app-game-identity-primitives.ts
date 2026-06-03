import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyAppGameIdentityText = Schema.String.pipe(Schema.minLength(1));

export const AppGameIdentityIdSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameIdentityId'));
export const AppGameDisplayLabelSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameDisplayLabel'));
export const AppGameParentLabelSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameParentLabel'));
export const AppGameExecutablePathRefSchema = NonEmptyAppGameIdentityText.pipe(
  Schema.brand('AppGameExecutablePathRef')
);
export const AppGamePackageIdSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGamePackageId'));
export const AppGameBundleIdSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameBundleId'));
export const AppGameAppUserModelIdSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameAppUserModelId'));
export const AppGameDesktopEntryIdSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameDesktopEntryId'));
export const AppGameApplicationTokenRefSchema = NonEmptyAppGameIdentityText.pipe(
  Schema.brand('AppGameApplicationTokenRef')
);
export const AppGamePublisherSignatureRefSchema = NonEmptyAppGameIdentityText.pipe(
  Schema.brand('AppGamePublisherSignatureRef')
);
export const AppGameFileHashRefSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameFileHashRef'));
export const AppGameLauncherAppIdSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameLauncherAppId'));
export const AppGameLauncherManifestIdSchema = NonEmptyAppGameIdentityText.pipe(
  Schema.brand('AppGameLauncherManifestId')
);
export const AppGameStoreIdSchema = NonEmptyAppGameIdentityText.pipe(Schema.brand('AppGameStoreId'));

export const AppGameProductKindSchema = withParser(
  Schema.Literal('nativeApp', 'nativeGame', 'launcher', 'unknownExecutable')
);

export const AppGameIdentityConfidenceSchema = withParser(
  Schema.Literal('weak', 'candidate', 'deterministic', 'parentLabeled', 'aiAssisted')
);

export const AppGameIdentityDeterministicRefKindSchema = withParser(
  Schema.Literal(
    'packageId',
    'bundleId',
    'appUserModelId',
    'desktopEntryId',
    'applicationTokenRef',
    'executablePathRef',
    'publisherSignatureRef',
    'fileHashRef',
    'launcherAppId',
    'launcherManifestId',
    'storeId',
    'catalogRef',
    'childGameEvidenceClaimId'
  )
);

export const AppGameProductKind = {
  NativeApp: AppGameProductKindSchema.parse('nativeApp'),
  NativeGame: AppGameProductKindSchema.parse('nativeGame'),
  Launcher: AppGameProductKindSchema.parse('launcher'),
  UnknownExecutable: AppGameProductKindSchema.parse('unknownExecutable'),
} as const;

export const AppGameIdentityConfidence = {
  Weak: AppGameIdentityConfidenceSchema.parse('weak'),
  Candidate: AppGameIdentityConfidenceSchema.parse('candidate'),
  Deterministic: AppGameIdentityConfidenceSchema.parse('deterministic'),
  ParentLabeled: AppGameIdentityConfidenceSchema.parse('parentLabeled'),
  AiAssisted: AppGameIdentityConfidenceSchema.parse('aiAssisted'),
} as const;

export const AppGameIdentityDeterministicRefKind = {
  PackageId: AppGameIdentityDeterministicRefKindSchema.parse('packageId'),
  BundleId: AppGameIdentityDeterministicRefKindSchema.parse('bundleId'),
  AppUserModelId: AppGameIdentityDeterministicRefKindSchema.parse('appUserModelId'),
  DesktopEntryId: AppGameIdentityDeterministicRefKindSchema.parse('desktopEntryId'),
  ApplicationTokenRef: AppGameIdentityDeterministicRefKindSchema.parse('applicationTokenRef'),
  ExecutablePathRef: AppGameIdentityDeterministicRefKindSchema.parse('executablePathRef'),
  PublisherSignatureRef: AppGameIdentityDeterministicRefKindSchema.parse('publisherSignatureRef'),
  FileHashRef: AppGameIdentityDeterministicRefKindSchema.parse('fileHashRef'),
  LauncherAppId: AppGameIdentityDeterministicRefKindSchema.parse('launcherAppId'),
  LauncherManifestId: AppGameIdentityDeterministicRefKindSchema.parse('launcherManifestId'),
  StoreId: AppGameIdentityDeterministicRefKindSchema.parse('storeId'),
  CatalogRef: AppGameIdentityDeterministicRefKindSchema.parse('catalogRef'),
  ChildGameEvidenceClaimId: AppGameIdentityDeterministicRefKindSchema.parse('childGameEvidenceClaimId'),
} as const;

export type AppGameProductKind = Infer<typeof AppGameProductKindSchema>;
export type AppGameIdentityConfidence = Infer<typeof AppGameIdentityConfidenceSchema>;
export type AppGameIdentityDeterministicRefKind = Infer<typeof AppGameIdentityDeterministicRefKindSchema>;
