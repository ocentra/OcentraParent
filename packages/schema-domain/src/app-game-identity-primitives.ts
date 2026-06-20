import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from './effect';

export const AppGameIdentityIdSchema = brandedNonEmptyStringSchema('AppGameIdentityId');
export const AppGameDisplayLabelSchema = brandedNonEmptyStringSchema('AppGameDisplayLabel');
export const AppGameParentLabelSchema = brandedNonEmptyStringSchema('AppGameParentLabel');
export const AppGameExecutablePathRefSchema = brandedNonEmptyStringSchema('AppGameExecutablePathRef');
export const AppGamePackageIdSchema = brandedNonEmptyStringSchema('AppGamePackageId');
export const AppGameBundleIdSchema = brandedNonEmptyStringSchema('AppGameBundleId');
export const AppGameAppUserModelIdSchema = brandedNonEmptyStringSchema('AppGameAppUserModelId');
export const AppGameDesktopEntryIdSchema = brandedNonEmptyStringSchema('AppGameDesktopEntryId');
export const AppGameApplicationTokenRefSchema = brandedNonEmptyStringSchema('AppGameApplicationTokenRef');
export const AppGamePublisherSignatureRefSchema = brandedNonEmptyStringSchema('AppGamePublisherSignatureRef');
export const AppGameFileHashRefSchema = brandedNonEmptyStringSchema('AppGameFileHashRef');
export const AppGameLauncherAppIdSchema = brandedNonEmptyStringSchema('AppGameLauncherAppId');
export const AppGameLauncherManifestIdSchema = brandedNonEmptyStringSchema('AppGameLauncherManifestId');
export const AppGameStoreIdSchema = brandedNonEmptyStringSchema('AppGameStoreId');

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
