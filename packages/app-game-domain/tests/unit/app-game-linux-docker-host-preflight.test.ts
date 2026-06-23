import { describe, expect, it } from 'vitest';
import {
  AppGameLinuxDockerHostPreflightReadModelSchema,
  createAppGameLinuxDockerHostPreflightReadModel,
  summarizeAppGameLinuxDockerHostPreflightReadModel,
} from '@ocentra-parent/schema-domain/app-game-linux-docker-host-preflight';

describe('app-game Linux Docker host preflight', () => {
  it('records visible Docker host inventory as redacted readiness without enforcement claims', () => {
    const readModel = createAppGameLinuxDockerHostPreflightReadModel({
      generatedAt: '2026-06-08T19:15:00.000Z',
      dockerCliObserved: true,
      dockerDaemonObserved: true,
      contextCount: 1,
      imageCount: 2,
      containerCount: 1,
    });
    const summary = summarizeAppGameLinuxDockerHostPreflightReadModel(readModel);

    expect(summary).toMatchObject({
      preflightState: 'docker-daemon-visible',
      dockerCliObserved: true,
      dockerDaemonObserved: true,
      contextCount: 1,
      imageCount: 2,
      containerCount: 1,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
    });
    expect(readModel.proofRefs).toEqual([
      'linux-docker-host-preflight-ref',
      'linux-docker-cli-ref',
      'linux-docker-context-ref',
      'linux-docker-daemon-ref',
      'linux-docker-image-inventory-ref',
      'linux-docker-container-inventory-ref',
    ]);
    expect(readModel.openGaps).toEqual([
      'linux-container-policy-not-proved',
      'linux-platform-enforcement-not-proved',
      'linux-child-device-delivery-not-proved',
    ]);
  });

  it('keeps daemon and inventory gaps open when only the Docker CLI is visible', () => {
    const readModel = createAppGameLinuxDockerHostPreflightReadModel({
      generatedAt: '2026-06-08T19:15:00.000Z',
      dockerCliObserved: true,
      dockerDaemonObserved: false,
      contextCount: 1,
      imageCount: 0,
      containerCount: 0,
    });

    expect(readModel.preflightState).toBe('docker-cli-visible-daemon-unavailable');
    expect(readModel.openGaps).toEqual(
      expect.arrayContaining([
        'linux-docker-daemon-not-proved',
        'linux-docker-image-inventory-not-proved',
        'linux-docker-container-inventory-not-proved',
      ])
    );
    expect(readModel.proofRefs).not.toContain('linux-docker-daemon-ref');
  });

  it('rejects raw inventory custody or claim upgrades', () => {
    const readModel = createAppGameLinuxDockerHostPreflightReadModel({
      generatedAt: '2026-06-08T19:15:00.000Z',
      dockerCliObserved: true,
      dockerDaemonObserved: true,
      contextCount: 1,
      imageCount: 1,
      containerCount: 1,
    });

    expect(
      AppGameLinuxDockerHostPreflightReadModelSchema.safeParse({ ...readModel, imageNamesRedacted: false }).success
    ).toBe(false);
    expect(
      AppGameLinuxDockerHostPreflightReadModelSchema.safeParse({ ...readModel, adapterDispatchClaimed: true }).success
    ).toBe(false);
    expect(
      AppGameLinuxDockerHostPreflightReadModelSchema.safeParse({ ...readModel, platformEnforcementClaimed: true })
        .success
    ).toBe(false);
  });
});
