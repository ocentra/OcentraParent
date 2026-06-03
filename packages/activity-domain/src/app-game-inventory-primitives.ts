import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyAppGameInventoryText = Schema.String.pipe(Schema.minLength(1));

export const AppGameInventorySourceRefSchema = NonEmptyAppGameInventoryText.pipe(
  Schema.brand('AppGameInventorySourceRef')
);

export const AppGameInventorySourceKindSchema = withParser(
  Schema.Literal(
    'osInstalledRecord',
    'shortcut',
    'storePackage',
    'launcherManifest',
    'parentCatalog',
    'managedDevice',
    'portableApp',
    'unknownSource'
  )
);

export const AppGameInventoryDetectionStateSchema = withParser(
  Schema.Literal('installed', 'detectable', 'missing', 'permissionLimited', 'stale', 'unavailable', 'adapterError')
);

export const AppGameInventoryCustodyStateSchema = withParser(
  Schema.Literal('localAgent', 'launcherManifest', 'parentCatalog', 'managedDevice', 'storePackage', 'unknown')
);

export const AppGameInventoryCategoryKindSchema = withParser(
  Schema.Literal(
    'game',
    'launcher',
    'education',
    'creative',
    'communication',
    'productivity',
    'browser',
    'system',
    'riskCandidate',
    'unknown'
  )
);

export const AppGameInventorySourceKind = {
  OsInstalledRecord: AppGameInventorySourceKindSchema.parse('osInstalledRecord'),
  Shortcut: AppGameInventorySourceKindSchema.parse('shortcut'),
  StorePackage: AppGameInventorySourceKindSchema.parse('storePackage'),
  LauncherManifest: AppGameInventorySourceKindSchema.parse('launcherManifest'),
  ParentCatalog: AppGameInventorySourceKindSchema.parse('parentCatalog'),
  ManagedDevice: AppGameInventorySourceKindSchema.parse('managedDevice'),
  PortableApp: AppGameInventorySourceKindSchema.parse('portableApp'),
  UnknownSource: AppGameInventorySourceKindSchema.parse('unknownSource'),
} as const;

export const AppGameInventoryDetectionState = {
  Installed: AppGameInventoryDetectionStateSchema.parse('installed'),
  Detectable: AppGameInventoryDetectionStateSchema.parse('detectable'),
  Missing: AppGameInventoryDetectionStateSchema.parse('missing'),
  PermissionLimited: AppGameInventoryDetectionStateSchema.parse('permissionLimited'),
  Stale: AppGameInventoryDetectionStateSchema.parse('stale'),
  Unavailable: AppGameInventoryDetectionStateSchema.parse('unavailable'),
  AdapterError: AppGameInventoryDetectionStateSchema.parse('adapterError'),
} as const;

export const AppGameInventoryCustodyState = {
  LocalAgent: AppGameInventoryCustodyStateSchema.parse('localAgent'),
  LauncherManifest: AppGameInventoryCustodyStateSchema.parse('launcherManifest'),
  ParentCatalog: AppGameInventoryCustodyStateSchema.parse('parentCatalog'),
  ManagedDevice: AppGameInventoryCustodyStateSchema.parse('managedDevice'),
  StorePackage: AppGameInventoryCustodyStateSchema.parse('storePackage'),
  Unknown: AppGameInventoryCustodyStateSchema.parse('unknown'),
} as const;

export const AppGameInventoryCategoryKind = {
  Game: AppGameInventoryCategoryKindSchema.parse('game'),
  Launcher: AppGameInventoryCategoryKindSchema.parse('launcher'),
  Education: AppGameInventoryCategoryKindSchema.parse('education'),
  Creative: AppGameInventoryCategoryKindSchema.parse('creative'),
  Communication: AppGameInventoryCategoryKindSchema.parse('communication'),
  Productivity: AppGameInventoryCategoryKindSchema.parse('productivity'),
  Browser: AppGameInventoryCategoryKindSchema.parse('browser'),
  System: AppGameInventoryCategoryKindSchema.parse('system'),
  RiskCandidate: AppGameInventoryCategoryKindSchema.parse('riskCandidate'),
  Unknown: AppGameInventoryCategoryKindSchema.parse('unknown'),
} as const;

export type AppGameInventorySourceKind = Infer<typeof AppGameInventorySourceKindSchema>;
export type AppGameInventoryDetectionState = Infer<typeof AppGameInventoryDetectionStateSchema>;
export type AppGameInventoryCustodyState = Infer<typeof AppGameInventoryCustodyStateSchema>;
export type AppGameInventoryCategoryKind = Infer<typeof AppGameInventoryCategoryKindSchema>;
