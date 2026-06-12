import { AppGamePlatformExtensionRoutingMatrixSchema } from './app-game-platform-extension-routing';
import { AndroidPlatformExtensionRows } from './app-game-platform-extension-routing-android-data';
import { PlatformExtensionGeneratedAt } from './app-game-platform-extension-routing-data-support';
import { IosPlatformExtensionRows } from './app-game-platform-extension-routing-ios-data';
import { LinuxPlatformExtensionRows } from './app-game-platform-extension-routing-linux-data';
import { MacosPlatformExtensionRows } from './app-game-platform-extension-routing-macos-data';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';

export const AppGamePlatformExtensionRoutingMatrix = AppGamePlatformExtensionRoutingMatrixSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  matrixId: 'app-game-platform-extension-proof-routing',
  generatedAt: PlatformExtensionGeneratedAt,
  rows: [
    ...MacosPlatformExtensionRows,
    ...IosPlatformExtensionRows,
    ...AndroidPlatformExtensionRows,
    ...LinuxPlatformExtensionRows,
  ],
});
