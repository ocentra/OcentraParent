import { describe, expect, it } from 'vitest';
import { LanPairingApiPath, LanPairingEndpointId, LanPairingHeader } from '../src/constants/lan-pairing';

describe('LAN pairing endpoint constants', () => {
  it('LanPairingApiPath: exposes slash-prefixed LAN endpoint paths', () => {
    expect(LanPairingApiPath.Discovery).toBe('/api/lan-pairing/discovery');
    expect(LanPairingApiPath.Challenge).toBe('/api/lan-pairing/challenge');
    expect(LanPairingApiPath.Proof).toBe('/api/lan-pairing/proof');
    expect(LanPairingApiPath.Control).toBe('/api/lan-pairing/control');
    expect(LanPairingApiPath.Registry).toBe('/api/lan-pairing/registry');
  });

  it('LanPairingEndpointId: keeps LAN route ids distinct from local health routes', () => {
    expect(LanPairingEndpointId.Discovery).toBe('lan-pairing.discovery');
    expect(LanPairingEndpointId.Control).toBe('lan-pairing.control');
  });

  it('LanPairingHeader: exposes typed headers for LAN proof and intent routing', () => {
    expect(LanPairingHeader.PairingProof).toBe('X-Ocentra-LAN-Pairing-Proof');
    expect(LanPairingHeader.IntentId).toBe('X-Ocentra-LAN-Intent-Id');
    expect(LanPairingHeader.TargetDeviceId).toBe('X-Ocentra-LAN-Target-Device-Id');
  });
});
