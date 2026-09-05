import { describe, expect, it } from 'vitest';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import { shouldRenderNetworkEvidenceDrawerRoute } from '../../src/NetworkEvidenceDrawerRoutePanel';
import { expectLanSourceMatrixProjection, expectNoLanSourceMatrixProjection } from './lan-source-matrix-test-helpers';

describe('portal live activity network flow state', () => {
  registerNetworkEvidenceDrawerRouteTests();
  registerLanSourceMatrixProjectionTests();
});

function registerNetworkEvidenceDrawerRouteTests(): void {
  it('mounts the network evidence drawer on canonical network product routes only', () => {
    expect(shouldRenderNetworkEvidenceDrawerRoute(ParentRoute.Activity)).toBe(false);
    expect(shouldRenderNetworkEvidenceDrawerRoute(ParentRoute.NetworkActivity)).toBe(true);
    expect(shouldRenderNetworkEvidenceDrawerRoute(ParentRoute.Commands)).toBe(false);
    expect(shouldRenderNetworkEvidenceDrawerRoute(ParentRoute.Overview)).toBe(false);
  });
}

function registerLanSourceMatrixProjectionTests(): void {
  it('projects LAN discovery source-matrix rows, claims, and restart-persisted weak identity truth', () => {
    expectLanSourceMatrixProjection();
  });

  it('keeps LAN source-matrix projection absent when no add-device source matrix was reported', () => {
    expectNoLanSourceMatrixProjection();
  });
}
