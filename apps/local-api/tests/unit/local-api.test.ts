import { describe, expect, it } from 'vitest';

import {
  LocalApiManifest,
  LocalApiManifestSchema,
  LocalApiPathSchema,
  LocalApiRouteId,
  LocalApiRouteIdSchema,
  LocalApiRouteSchema,
  localApiRouteById,
  localApiRouteForCommand,
} from '../../src/local-api';
import { AgentCommandName } from '@ocentra-parent/agent-protocol-domain/contracts';

describe('local-api contract manifest', () => {
  it('declares the Rust agent-service routes without owning runtime behavior', () => {
    const manifest = LocalApiManifestSchema.parse(LocalApiManifest);

    expect(manifest.routes.map((route) => route.path)).toEqual([
      '/health',
      '/api/dev/log-snapshot',
      '/api/browser/intervention/page',
      '/api/dev/ws',
    ]);
    expect(manifest.routes.every((route) => route.owner === 'agent-service')).toBe(true);
  });

  it('rejects malformed paths and routes before UI code can use them', () => {
    expect(LocalApiPathSchema.safeParse('api/dev/ws').success).toBe(false);
    expect(
      LocalApiRouteSchema.safeParse({
        routeId: 'agent-service.bad',
        path: '/api/bad',
        method: 'POST',
        transport: 'http',
        owner: 'agent-service',
        operation: 'health-read',
        command: null,
      }).success
    ).toBe(false);
  });

  it('maps WebSocket commands to the canonical agent command route', () => {
    const route = localApiRouteForCommand(AgentCommandName.ActivityTrackingRetentionSettingsWrite);

    expect(route.routeId).toBe(LocalApiRouteId.DevWebSocket);
    expect(route.deliveryMode).toBe('request-response');
  });

  it('returns null for missing typed route ids instead of falling through to a wrong route', () => {
    const missingRouteId = LocalApiRouteIdSchema.parse('agent-service.missing');

    expect(localApiRouteById(missingRouteId)).toBeNull();
  });
});
