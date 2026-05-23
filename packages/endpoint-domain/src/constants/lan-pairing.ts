import { decodeApiPath, decodeEndpointId, decodeHeaderName } from '../types/brands';

export const LanPairingEndpointId = {
  Discovery: decodeEndpointId('lan-pairing.discovery'),
  Challenge: decodeEndpointId('lan-pairing.challenge'),
  Proof: decodeEndpointId('lan-pairing.proof'),
  Control: decodeEndpointId('lan-pairing.control'),
  Registry: decodeEndpointId('lan-pairing.registry'),
} as const;

export const LanPairingApiPath = {
  Discovery: decodeApiPath('/api/lan-pairing/discovery'),
  Challenge: decodeApiPath('/api/lan-pairing/challenge'),
  Proof: decodeApiPath('/api/lan-pairing/proof'),
  Control: decodeApiPath('/api/lan-pairing/control'),
  Registry: decodeApiPath('/api/lan-pairing/registry'),
} as const;

export const LanPairingHeader = {
  Origin: decodeHeaderName('Origin'),
  PairingProof: decodeHeaderName('X-Ocentra-LAN-Pairing-Proof'),
  PairingId: decodeHeaderName('X-Ocentra-LAN-Pairing-Id'),
  IntentId: decodeHeaderName('X-Ocentra-LAN-Intent-Id'),
  TargetDeviceId: decodeHeaderName('X-Ocentra-LAN-Target-Device-Id'),
} as const;
