import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyAppGameCategoryText = Schema.String.pipe(Schema.minLength(1));

export const AppGameCategoryCandidateIdSchema = NonEmptyAppGameCategoryText.pipe(
  Schema.brand('AppGameCategoryCandidateId')
);
export const AppGameCategorySourceRefSchema = NonEmptyAppGameCategoryText.pipe(
  Schema.brand('AppGameCategorySourceRef')
);
export const AppGameCategoryReasonCodeSchema = NonEmptyAppGameCategoryText.pipe(
  Schema.brand('AppGameCategoryReasonCode')
);

export const AppGameCategoryFamilySchema = withParser(
  Schema.Literal('nativeApp', 'nativeGame', 'riskCandidate', 'gameContext')
);

export const AppGameNativeAppCategorySchema = withParser(
  Schema.Literal(
    'school',
    'productivity',
    'social',
    'messaging',
    'video',
    'music',
    'aiChatbot',
    'vpnProxy',
    'remoteDesktop',
    'downloadTorrent',
    'developer',
    'installerUpdater',
    'system',
    'unknown',
    'futureMappedClass'
  )
);

export const AppGameNativeGameCategorySchema = withParser(
  Schema.Literal(
    'educational',
    'casual',
    'puzzle',
    'strategy',
    'shooter',
    'horror',
    'casinoLike',
    'sports',
    'racing',
    'simulation',
    'sandbox',
    'rpg',
    'mmo',
    'launcherOnly',
    'unknownGameCandidate'
  )
);

export const AppGameRiskSignalKindSchema = withParser(
  Schema.Literal(
    'vpnProxy',
    'remoteDesktop',
    'downloadTorrent',
    'installerUpdater',
    'aiChatbot',
    'socialVideoMessaging',
    'unknownRisk',
    'ageMaturity',
    'purchaseCapability',
    'onlineInteraction'
  )
);

export const AppGameContextSignalKindSchema = withParser(
  Schema.Literal('rating', 'ageLabel', 'multiplayer', 'userGeneratedContent', 'chatOrVoice', 'purchaseCapability')
);

export const AppGameCategorySourceKindSchema = withParser(
  Schema.Literal(
    'catalog',
    'storeMetadata',
    'launcherManifest',
    'parentLabel',
    'localAi',
    'processMetadata',
    'executableName',
    'managedDevice',
    'manualReview'
  )
);

export const AppGameCategoryCandidateStateSchema = withParser(
  Schema.Literal(
    'catalogCandidate',
    'parentDisplayOverride',
    'aiCandidate',
    'nameHeuristicCandidate',
    'manualReviewCandidate',
    'unknownCandidate'
  )
);

export const AppGameCategoryPolicyCandidateActionSchema = withParser(
  Schema.Literal('none', 'observe', 'warn', 'askParent', 'manualReview')
);

export const AppGameCategoryEnforcementStateSchema = withParser(Schema.Literal('notEnforcement'));

export const AppGameCategoryFamily = {
  NativeApp: AppGameCategoryFamilySchema.parse('nativeApp'),
  NativeGame: AppGameCategoryFamilySchema.parse('nativeGame'),
  RiskCandidate: AppGameCategoryFamilySchema.parse('riskCandidate'),
  GameContext: AppGameCategoryFamilySchema.parse('gameContext'),
} as const;

export const AppGameNativeAppCategory = {
  School: AppGameNativeAppCategorySchema.parse('school'),
  Productivity: AppGameNativeAppCategorySchema.parse('productivity'),
  Social: AppGameNativeAppCategorySchema.parse('social'),
  Messaging: AppGameNativeAppCategorySchema.parse('messaging'),
  Video: AppGameNativeAppCategorySchema.parse('video'),
  Music: AppGameNativeAppCategorySchema.parse('music'),
  AiChatbot: AppGameNativeAppCategorySchema.parse('aiChatbot'),
  VpnProxy: AppGameNativeAppCategorySchema.parse('vpnProxy'),
  RemoteDesktop: AppGameNativeAppCategorySchema.parse('remoteDesktop'),
  DownloadTorrent: AppGameNativeAppCategorySchema.parse('downloadTorrent'),
  Developer: AppGameNativeAppCategorySchema.parse('developer'),
  InstallerUpdater: AppGameNativeAppCategorySchema.parse('installerUpdater'),
  System: AppGameNativeAppCategorySchema.parse('system'),
  Unknown: AppGameNativeAppCategorySchema.parse('unknown'),
  FutureMappedClass: AppGameNativeAppCategorySchema.parse('futureMappedClass'),
} as const;

export const AppGameNativeGameCategory = {
  Educational: AppGameNativeGameCategorySchema.parse('educational'),
  Casual: AppGameNativeGameCategorySchema.parse('casual'),
  Puzzle: AppGameNativeGameCategorySchema.parse('puzzle'),
  Strategy: AppGameNativeGameCategorySchema.parse('strategy'),
  Shooter: AppGameNativeGameCategorySchema.parse('shooter'),
  Horror: AppGameNativeGameCategorySchema.parse('horror'),
  CasinoLike: AppGameNativeGameCategorySchema.parse('casinoLike'),
  Sports: AppGameNativeGameCategorySchema.parse('sports'),
  Racing: AppGameNativeGameCategorySchema.parse('racing'),
  Simulation: AppGameNativeGameCategorySchema.parse('simulation'),
  Sandbox: AppGameNativeGameCategorySchema.parse('sandbox'),
  Rpg: AppGameNativeGameCategorySchema.parse('rpg'),
  Mmo: AppGameNativeGameCategorySchema.parse('mmo'),
  LauncherOnly: AppGameNativeGameCategorySchema.parse('launcherOnly'),
  UnknownGameCandidate: AppGameNativeGameCategorySchema.parse('unknownGameCandidate'),
} as const;

export const AppGameRiskSignalKind = {
  VpnProxy: AppGameRiskSignalKindSchema.parse('vpnProxy'),
  RemoteDesktop: AppGameRiskSignalKindSchema.parse('remoteDesktop'),
  DownloadTorrent: AppGameRiskSignalKindSchema.parse('downloadTorrent'),
  InstallerUpdater: AppGameRiskSignalKindSchema.parse('installerUpdater'),
  AiChatbot: AppGameRiskSignalKindSchema.parse('aiChatbot'),
  SocialVideoMessaging: AppGameRiskSignalKindSchema.parse('socialVideoMessaging'),
  UnknownRisk: AppGameRiskSignalKindSchema.parse('unknownRisk'),
  AgeMaturity: AppGameRiskSignalKindSchema.parse('ageMaturity'),
  PurchaseCapability: AppGameRiskSignalKindSchema.parse('purchaseCapability'),
  OnlineInteraction: AppGameRiskSignalKindSchema.parse('onlineInteraction'),
} as const;

export const AppGameContextSignalKind = {
  Rating: AppGameContextSignalKindSchema.parse('rating'),
  AgeLabel: AppGameContextSignalKindSchema.parse('ageLabel'),
  Multiplayer: AppGameContextSignalKindSchema.parse('multiplayer'),
  UserGeneratedContent: AppGameContextSignalKindSchema.parse('userGeneratedContent'),
  ChatOrVoice: AppGameContextSignalKindSchema.parse('chatOrVoice'),
  PurchaseCapability: AppGameContextSignalKindSchema.parse('purchaseCapability'),
} as const;

export const AppGameCategorySourceKind = {
  Catalog: AppGameCategorySourceKindSchema.parse('catalog'),
  StoreMetadata: AppGameCategorySourceKindSchema.parse('storeMetadata'),
  LauncherManifest: AppGameCategorySourceKindSchema.parse('launcherManifest'),
  ParentLabel: AppGameCategorySourceKindSchema.parse('parentLabel'),
  LocalAi: AppGameCategorySourceKindSchema.parse('localAi'),
  ProcessMetadata: AppGameCategorySourceKindSchema.parse('processMetadata'),
  ExecutableName: AppGameCategorySourceKindSchema.parse('executableName'),
  ManagedDevice: AppGameCategorySourceKindSchema.parse('managedDevice'),
  ManualReview: AppGameCategorySourceKindSchema.parse('manualReview'),
} as const;

export const AppGameCategoryCandidateState = {
  CatalogCandidate: AppGameCategoryCandidateStateSchema.parse('catalogCandidate'),
  ParentDisplayOverride: AppGameCategoryCandidateStateSchema.parse('parentDisplayOverride'),
  AiCandidate: AppGameCategoryCandidateStateSchema.parse('aiCandidate'),
  NameHeuristicCandidate: AppGameCategoryCandidateStateSchema.parse('nameHeuristicCandidate'),
  ManualReviewCandidate: AppGameCategoryCandidateStateSchema.parse('manualReviewCandidate'),
  UnknownCandidate: AppGameCategoryCandidateStateSchema.parse('unknownCandidate'),
} as const;

export const AppGameCategoryPolicyCandidateAction = {
  None: AppGameCategoryPolicyCandidateActionSchema.parse('none'),
  Observe: AppGameCategoryPolicyCandidateActionSchema.parse('observe'),
  Warn: AppGameCategoryPolicyCandidateActionSchema.parse('warn'),
  AskParent: AppGameCategoryPolicyCandidateActionSchema.parse('askParent'),
  ManualReview: AppGameCategoryPolicyCandidateActionSchema.parse('manualReview'),
} as const;

export const AppGameCategoryEnforcementState = {
  NotEnforcement: AppGameCategoryEnforcementStateSchema.parse('notEnforcement'),
} as const;

export type AppGameCategoryCandidateId = Infer<typeof AppGameCategoryCandidateIdSchema>;
export type AppGameCategorySourceRef = Infer<typeof AppGameCategorySourceRefSchema>;
export type AppGameCategoryReasonCode = Infer<typeof AppGameCategoryReasonCodeSchema>;
export type AppGameCategoryFamily = Infer<typeof AppGameCategoryFamilySchema>;
export type AppGameNativeAppCategory = Infer<typeof AppGameNativeAppCategorySchema>;
export type AppGameNativeGameCategory = Infer<typeof AppGameNativeGameCategorySchema>;
export type AppGameRiskSignalKind = Infer<typeof AppGameRiskSignalKindSchema>;
export type AppGameContextSignalKind = Infer<typeof AppGameContextSignalKindSchema>;
export type AppGameCategorySourceKind = Infer<typeof AppGameCategorySourceKindSchema>;
export type AppGameCategoryCandidateState = Infer<typeof AppGameCategoryCandidateStateSchema>;
export type AppGameCategoryPolicyCandidateAction = Infer<typeof AppGameCategoryPolicyCandidateActionSchema>;
export type AppGameCategoryEnforcementState = Infer<typeof AppGameCategoryEnforcementStateSchema>;
