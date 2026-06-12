import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentDeviceIdSchema, ParentPlatformSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  LanPairingControllerLeaseIdSchema,
  LanPairingParentAuthoritySchema,
  LanPairingRouteIdSchema,
  LanPairingTimestampSchema,
} from './lan-pairing-values';

export const DeviceRuntimeRoleSchema = withParser(
  Schema.Literal('parent-controller', 'parent-observer', 'child-agent', 'portal', 'ai-provider')
);

export const DeviceRuntimeRoleStateSchema = withParser(
  Schema.Literal('implemented', 'scaffold', 'manual-required', 'unavailable')
);

export const DeviceRuntimeSurfaceSchema = withParser(
  Schema.Literal('parent-desktop', 'parent-mobile', 'child-desktop', 'child-android', 'child-ios')
);

export const DeviceRuntimeRouteStateSchema = withParser(
  Schema.Literal('localhost', 'local-network', 'manual-required', 'unavailable')
);

export const DeviceRuntimeAiProviderStateSchema = withParser(Schema.Literal('available', 'degraded', 'unavailable'));

export const DeviceRuntimeLocalAiClaimSchema = withParser(
  Schema.Literal('none', 'shared-physical-device-singleton', 'unavailable')
);

export const DeviceRuntimeRoleEntrySchema = withParser(
  Schema.Struct({
    role: DeviceRuntimeRoleSchema,
    state: DeviceRuntimeRoleStateSchema,
  })
);

type DeviceRuntimeRoleEntryValue = Infer<typeof DeviceRuntimeRoleEntrySchema>;

const DeviceRoleRuntimeReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal('v0.9'),
  physicalDeviceId: ParentDeviceIdSchema,
  surface: DeviceRuntimeSurfaceSchema,
  platform: ParentPlatformSchema,
  roles: Schema.Array(DeviceRuntimeRoleEntrySchema),
  primaryRole: DeviceRuntimeRoleSchema,
  controllerLeaseId: Schema.Union(LanPairingControllerLeaseIdSchema, Schema.Null),
  parentAuthority: Schema.Union(LanPairingParentAuthoritySchema, Schema.Null),
  selectedRouteId: Schema.Union(LanPairingRouteIdSchema, Schema.Null),
  routeState: DeviceRuntimeRouteStateSchema,
  lanAiProviderState: DeviceRuntimeAiProviderStateSchema,
  localAiRuntimeClaim: DeviceRuntimeLocalAiClaimSchema,
  updatedAt: LanPairingTimestampSchema,
});

type DeviceRoleRuntimeReadModelCandidate = Infer<typeof DeviceRoleRuntimeReadModelBaseSchema>;

export const DeviceRoleRuntimeReadModelSchema = withParser(
  DeviceRoleRuntimeReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        deviceRoleRuntimeReadModelIsConsistent(readModel) ||
        'Expected device role read model to use unique roles, valid controller authority, and one shared local AI runtime claim'
    )
  )
);

function deviceRoleRuntimeReadModelIsConsistent(readModel: DeviceRoleRuntimeReadModelCandidate): boolean {
  if (readModel.roles.length === 0 || !roleEntriesAreUnique(readModel.roles)) {
    return false;
  }

  if (!readModel.roles.some((entry) => entry.role === readModel.primaryRole)) {
    return false;
  }

  if (!controllerAuthorityIsConsistent(readModel)) {
    return false;
  }

  if (!aiProviderClaimIsConsistent(readModel)) {
    return false;
  }

  return readModel.selectedRouteId !== null || readModel.routeState !== 'local-network';
}

function roleEntriesAreUnique(entries: ReadonlyArray<DeviceRuntimeRoleEntryValue>): boolean {
  return new Set(entries.map((entry) => entry.role)).size === entries.length;
}

function controllerAuthorityIsConsistent(readModel: DeviceRoleRuntimeReadModelCandidate): boolean {
  const hasController = readModel.roles.some((entry) => entry.role === 'parent-controller');
  if (!hasController) {
    return readModel.parentAuthority !== 'active-controller';
  }

  return readModel.controllerLeaseId !== null && readModel.parentAuthority === 'active-controller';
}

function aiProviderClaimIsConsistent(readModel: DeviceRoleRuntimeReadModelCandidate): boolean {
  const hasAiProvider = readModel.roles.some((entry) => entry.role === 'ai-provider');
  if (!hasAiProvider) {
    return readModel.localAiRuntimeClaim !== 'shared-physical-device-singleton';
  }

  return readModel.localAiRuntimeClaim === 'shared-physical-device-singleton';
}

export type DeviceRuntimeRole = Infer<typeof DeviceRuntimeRoleSchema>;
export type DeviceRuntimeRoleState = Infer<typeof DeviceRuntimeRoleStateSchema>;
export type DeviceRuntimeSurface = Infer<typeof DeviceRuntimeSurfaceSchema>;
export type DeviceRuntimeRouteState = Infer<typeof DeviceRuntimeRouteStateSchema>;
export type DeviceRuntimeAiProviderState = Infer<typeof DeviceRuntimeAiProviderStateSchema>;
export type DeviceRuntimeLocalAiClaim = Infer<typeof DeviceRuntimeLocalAiClaimSchema>;
export type DeviceRuntimeRoleEntry = Infer<typeof DeviceRuntimeRoleEntrySchema>;
export type DeviceRoleRuntimeReadModel = Infer<typeof DeviceRoleRuntimeReadModelSchema>;
