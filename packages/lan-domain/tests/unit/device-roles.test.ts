import { describe, expect, it } from 'vitest';
import { DeviceRoleRuntimeReadModelSchema } from '@ocentra-parent/schema-domain/device-roles';

const updatedAt = '2026-05-27T06:20:00.000Z';

describe('device role runtime contracts', () => {
  it('DeviceRoleRuntimeReadModelSchema: parses controller, child-agent, and AI provider roles on one device', () => {
    const readModel = DeviceRoleRuntimeReadModelSchema.parse({
      schemaVersion: 'v0.9',
      physicalDeviceId: 'physical-device-family-pc',
      surface: 'parent-desktop',
      platform: 'windows',
      roles: [
        { role: 'parent-controller', state: 'implemented' },
        { role: 'child-agent', state: 'implemented' },
        { role: 'ai-provider', state: 'implemented' },
      ],
      primaryRole: 'parent-controller',
      controllerLeaseId: 'controller-lease-family-pc',
      parentAuthority: 'active-controller',
      selectedRouteId: 'lan-route-family-pc',
      routeState: 'local-network',
      lanAiProviderState: 'available',
      localAiRuntimeClaim: 'shared-physical-device-singleton',
      updatedAt,
    });

    expect(readModel.roles.map((entry) => entry.role)).toEqual(['parent-controller', 'child-agent', 'ai-provider']);
    expect(readModel.localAiRuntimeClaim).toBe('shared-physical-device-singleton');
    expect(readModel.parentAuthority).toBe('active-controller');
  });

  it('DeviceRoleRuntimeReadModelSchema: represents parent mobile observer unavailable LAN AI state honestly', () => {
    const readModel = DeviceRoleRuntimeReadModelSchema.parse({
      schemaVersion: 'v0.9',
      physicalDeviceId: 'physical-device-parent-phone',
      surface: 'parent-mobile',
      platform: 'android',
      roles: [{ role: 'parent-observer', state: 'scaffold' }],
      primaryRole: 'parent-observer',
      controllerLeaseId: null,
      parentAuthority: 'observer',
      selectedRouteId: null,
      routeState: 'manual-required',
      lanAiProviderState: 'unavailable',
      localAiRuntimeClaim: 'none',
      updatedAt,
    });

    expect(readModel.surface).toBe('parent-mobile');
    expect(readModel.roles[0]).toEqual({ role: 'parent-observer', state: 'scaffold' });
    expect(readModel.lanAiProviderState).toBe('unavailable');
  });
});

describe('device role runtime contract consistency checks', () => {
  it('DeviceRoleRuntimeReadModelSchema: rejects duplicate role entries and duplicate runtime claims', () => {
    const duplicateRole = DeviceRoleRuntimeReadModelSchema.safeParse({
      schemaVersion: 'v0.9',
      physicalDeviceId: 'physical-device-duplicate',
      surface: 'child-desktop',
      platform: 'windows',
      roles: [
        { role: 'child-agent', state: 'implemented' },
        { role: 'child-agent', state: 'implemented' },
      ],
      primaryRole: 'child-agent',
      controllerLeaseId: null,
      parentAuthority: null,
      selectedRouteId: null,
      routeState: 'localhost',
      lanAiProviderState: 'unavailable',
      localAiRuntimeClaim: 'none',
      updatedAt,
    });
    const duplicateAiClaimWithoutProvider = DeviceRoleRuntimeReadModelSchema.safeParse({
      schemaVersion: 'v0.9',
      physicalDeviceId: 'physical-device-no-provider',
      surface: 'parent-desktop',
      platform: 'windows',
      roles: [{ role: 'parent-observer', state: 'implemented' }],
      primaryRole: 'parent-observer',
      controllerLeaseId: null,
      parentAuthority: 'observer',
      selectedRouteId: null,
      routeState: 'localhost',
      lanAiProviderState: 'degraded',
      localAiRuntimeClaim: 'shared-physical-device-singleton',
      updatedAt,
    });

    expect(duplicateRole.success).toBe(false);
    expect(duplicateAiClaimWithoutProvider.success).toBe(false);
  });
});
