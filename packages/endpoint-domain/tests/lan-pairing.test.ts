import { describe, expect, it } from 'vitest';
import {
  LanPairingHeader,
  LanPairingSupportedRuntimeApiPath,
  LanPairingSupportedRuntimeEndpointId,
  PlannedLanPairingEndpointId,
  PlannedLanPairingHttpApiPath,
  PlannedLanPairingHttpEndpointSupport,
} from '../src/constants/lan-pairing';

describe('LAN pairing endpoint constants', () => {
  it('LanPairingSupportedRuntimeApiPath: advertises only the served WebSocket runtime path', () => {
    expect(LanPairingSupportedRuntimeEndpointId.WebSocket).toBe('agent.dev.ws');
    expect(LanPairingSupportedRuntimeApiPath.WebSocket).toBe('/api/dev/ws');
    expect(Object.values(LanPairingSupportedRuntimeApiPath)).not.toContain(PlannedLanPairingHttpApiPath.Proof);
  });

  it('PlannedLanPairingHttpApiPath: marks LAN HTTP endpoints as planned unsupported routes', () => {
    expect(PlannedLanPairingHttpApiPath.Discovery).toBe('/api/lan-pairing/discovery');
    expect(PlannedLanPairingHttpApiPath.Challenge).toBe('/api/lan-pairing/challenge');
    expect(PlannedLanPairingHttpApiPath.Proof).toBe('/api/lan-pairing/proof');
    expect(PlannedLanPairingHttpApiPath.Control).toBe('/api/lan-pairing/control');
    expect(PlannedLanPairingHttpApiPath.Registry).toBe('/api/lan-pairing/registry');
    expect(PlannedLanPairingEndpointId.Discovery).toBe('lan-pairing.discovery');
    expect(PlannedLanPairingEndpointId.Control).toBe('lan-pairing.control');
    expect(Object.values(PlannedLanPairingHttpEndpointSupport)).toEqual([
      'planned-unsupported',
      'planned-unsupported',
      'planned-unsupported',
      'planned-unsupported',
      'planned-unsupported',
    ]);
  });

  it('LanPairingHeader: exposes typed headers for LAN proof and intent routing', () => {
    expect(LanPairingHeader.PairingProof).toBe('X-Ocentra-LAN-Pairing-Proof');
    expect(LanPairingHeader.IntentId).toBe('X-Ocentra-LAN-Intent-Id');
    expect(LanPairingHeader.TargetDeviceId).toBe('X-Ocentra-LAN-Target-Device-Id');
  });
});
