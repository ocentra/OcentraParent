import { describe, expect, it } from 'vitest';
import {
  type ChildAndroidStorageProtocolReadModel,
  ChildAndroidStorageProtocolReadModelSchema,
} from '@ocentra-parent/schema-domain/child-android-storage-protocol-proof';

describe('child android storage protocol proof contracts', () => {
  acceptsPackageLocalStorageProtocolProof();
  rejectsMissingStorageSurfaces();
  rejectsExternalTransportClaims();
  rejectsEncryptedJournalUpgrade();
  rejectsHostedDefaultStorage();
  rejectsAppPrivateDurableStorageUpgrade();
});

function acceptsPackageLocalStorageProtocolProof(): void {
  it('ChildAndroidStorageProtocolReadModelSchema: accepts package-local storage protocol proof with explicit gaps', () => {
    const parsed = ChildAndroidStorageProtocolReadModelSchema.parse(validReadModel());

    expect(parsed.schemaVersion).toBe('child-android-storage-protocol-capability-proof');
    expect(parsed.protocolBridgeProof.commands).toEqual([
      'child.android.storage.snapshot.get',
      'child.android.storage.capability.proof.get',
      'child.android.storage.protocol.proof.get',
    ]);
    expect(surfaceState(parsed, 'app-private-files')).toEqual({
      parentCapabilityStatus: 'scaffold',
      proofState: 'package-local-scaffold',
      custody: 'child-device-local',
      defaultStorageMode: 'package-local-app-private',
      rawChildActivityStorage: 'not-collected',
    });
    expect(surfaceState(parsed, 'ocentra-hosted-child-activity-storage')).toEqual({
      parentCapabilityStatus: 'not-implemented',
      proofState: 'not-implemented',
      custody: 'ocentra-hosted',
      defaultStorageMode: 'not-default',
      rawChildActivityStorage: 'not-default',
    });
  });
}

function rejectsMissingStorageSurfaces(): void {
  it('ChildAndroidStorageProtocolReadModelSchema: rejects missing storage surfaces', () => {
    const model = validReadModel();

    expect(
      ChildAndroidStorageProtocolReadModelSchema.safeParse({
        ...model,
        storageSurfaces: model.storageSurfaces.filter((entry) => entry.surface !== 'sqlite-query-store'),
      }).success
    ).toBe(false);
  });
}

function rejectsExternalTransportClaims(): void {
  it('ChildAndroidStorageProtocolReadModelSchema: rejects external transport claims from package-local proof', () => {
    const model = validReadModel();

    expect(
      ChildAndroidStorageProtocolReadModelSchema.safeParse({
        ...model,
        protocolBridgeProof: {
          ...model.protocolBridgeProof,
          externalTransportState: 'package-local-scaffold',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsEncryptedJournalUpgrade(): void {
  it('ChildAndroidStorageProtocolReadModelSchema: rejects encrypted journal proof without device persistence artifacts', () => {
    const model = validReadModel();

    expect(
      ChildAndroidStorageProtocolReadModelSchema.safeParse({
        ...model,
        storageSurfaces: model.storageSurfaces.map((entry) =>
          entry.surface === 'encrypted-evidence-journal'
            ? { ...entry, parentCapabilityStatus: 'implemented', proofState: 'ci-mechanical-proof' }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsHostedDefaultStorage(): void {
  it('ChildAndroidStorageProtocolReadModelSchema: rejects hosted storage as default child activity storage', () => {
    const model = validReadModel();

    expect(
      ChildAndroidStorageProtocolReadModelSchema.safeParse({
        ...model,
        storageSurfaces: model.storageSurfaces.map((entry) =>
          entry.surface === 'ocentra-hosted-child-activity-storage'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                proofState: 'ci-mechanical-proof',
                defaultStorageMode: 'package-local-app-private',
                rawChildActivityStorage: 'temporary-local-only',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsAppPrivateDurableStorageUpgrade(): void {
  it('ChildAndroidStorageProtocolReadModelSchema: rejects app-private files upgraded to durable evidence storage', () => {
    const model = validReadModel();

    expect(
      ChildAndroidStorageProtocolReadModelSchema.safeParse({
        ...model,
        storageSurfaces: model.storageSurfaces.map((entry) =>
          entry.surface === 'app-private-files'
            ? {
                ...entry,
                parentCapabilityStatus: 'implemented',
                proofState: 'ci-mechanical-proof',
                rawChildActivityStorage: 'temporary-local-only',
              }
            : entry
        ),
      }).success
    ).toBe(false);
  });
}

function surfaceState(
  model: ChildAndroidStorageProtocolReadModel,
  surface: ChildAndroidStorageProtocolReadModel['storageSurfaces'][number]['surface']
) {
  const entry = model.storageSurfaces.find((proof) => proof.surface === surface);
  return {
    parentCapabilityStatus: entry?.parentCapabilityStatus,
    proofState: entry?.proofState,
    custody: entry?.custody,
    defaultStorageMode: entry?.defaultStorageMode,
    rawChildActivityStorage: entry?.rawChildActivityStorage,
  };
}

function validReadModel(): ChildAndroidStorageProtocolReadModel {
  return ChildAndroidStorageProtocolReadModelSchema.parse({
    schemaVersion: 'child-android-storage-protocol-capability-proof',
    protocolBridgeProof: {
      packageId: 'ca.ocentra.parent.agent',
      nativeBridgeClass: 'ca.ocentra.parent.agent.ChildAndroidStorageProtocolProof',
      bridgeState: 'package-local-scaffold',
      externalTransportState: 'not-implemented',
      commands: [
        'child.android.storage.snapshot.get',
        'child.android.storage.capability.proof.get',
        'child.android.storage.protocol.proof.get',
      ],
      events: [
        'child.android.storage.snapshot.reported',
        'child.android.storage.capability.proof.reported',
        'child.android.storage.protocol.proof.reported',
      ],
      runtimeOwner: 'android-native-wrapper',
      proofRequirement: 'Android storage protocol bridge constants compile into the debug package',
      claimBoundary: 'package-local storage proof is not LAN/WebSocket child-agent transport',
    },
    storageSurfaces: storageSurfaces(),
    claimBoundaries: {
      appPrivateFiles:
        'app-private files are only a package-local scaffold until emulator or physical-device persistence proof exists',
      encryptedEvidenceJournal:
        'encrypted evidence journal is not implemented and cannot be claimed from package constants',
      sqliteQueryStore: 'SQLite query store is not implemented for child Android storage in this slice',
      parentOwnedExport: 'parent-owned export is planned and cannot silently upload child activity by default',
      ocentraHostedStorage:
        'Ocentra-hosted child activity storage is not the default and is not implemented by this proof',
      protocolTransport: 'storage protocol snapshot is package-local and not external LAN/WebSocket transport',
      childAndroidStoragePersistence: 'device persistence requires emulator or physical-device artifacts',
    },
    updatedAt: '2026-05-31T00:00:00.000Z',
  });
}

function storageSurfaces() {
  return [
    storageSurface(
      'app-private-files',
      'local-storage',
      'scaffold',
      'package-local-scaffold',
      'android-app-private-storage',
      'child-device-local',
      'package-local-app-private',
      'not-collected',
      'Android app-private files are named as the local package storage target, without device persistence proof'
    ),
    storageSurface(
      'encrypted-evidence-journal',
      'local-storage',
      'not-implemented',
      'not-implemented',
      'child-agent-runtime',
      'child-device-local',
      'disabled',
      'temporary-local-only',
      'Encrypted evidence journal remains unimplemented until runtime persistence and deletion proof exists'
    ),
    storageSurface(
      'sqlite-query-store',
      'local-storage',
      'not-implemented',
      'not-implemented',
      'child-agent-runtime',
      'child-device-local',
      'disabled',
      'not-collected',
      'SQLite query store remains unimplemented on child Android'
    ),
    storageSurface(
      'parent-owned-export',
      'local-storage',
      'planned',
      'planned',
      'parent-owned-storage',
      'parent-owned-local',
      'parent-owned-export-only',
      'not-collected',
      'Parent-owned export remains a future explicit export path, not default storage'
    ),
    storageSurface(
      'ocentra-hosted-child-activity-storage',
      'local-storage',
      'not-implemented',
      'not-implemented',
      'ocentra-hosted-service',
      'ocentra-hosted',
      'not-default',
      'not-default',
      'Hosted child activity storage is not implemented and is never default storage'
    ),
    storageSurface(
      'protocol-storage-snapshot',
      'typed-protocol-bridge',
      'scaffold',
      'ci-mechanical-proof',
      'agent-protocol',
      'none',
      'disabled',
      'not-collected',
      'Storage snapshot command/event names compile into the native package bridge'
    ),
  ];
}

function storageSurface(
  surface: string,
  parentCapability: string,
  parentCapabilityStatus: string,
  proofState: string,
  runtimeOwner: string,
  custody: string,
  defaultStorageMode: string,
  rawChildActivityStorage: string,
  proofRequirement: string
) {
  return {
    surface,
    parentCapability,
    parentCapabilityStatus,
    proofState,
    runtimeOwner,
    custody,
    defaultStorageMode,
    rawChildActivityStorage,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}
