import { decodeApiPath, decodeEndpointId, decodeHeaderName } from '../types/brands';

export const LanPairingSupportedRuntimeEndpointId = {
  WebSocket: decodeEndpointId('agent.dev.ws'),
} as const;

export const LanPairingSupportedRuntimeApiPath = {
  WebSocket: decodeApiPath('/api/dev/ws'),
} as const;

export const PlannedLanPairingEndpointId = {
  Discovery: decodeEndpointId('lan-pairing.discovery'),
  Challenge: decodeEndpointId('lan-pairing.challenge'),
  Proof: decodeEndpointId('lan-pairing.proof'),
  Control: decodeEndpointId('lan-pairing.control'),
  Registry: decodeEndpointId('lan-pairing.registry'),
} as const;

export const PlannedLanPairingHttpApiPath = {
  Discovery: decodeApiPath('/api/lan-pairing/discovery'),
  Challenge: decodeApiPath('/api/lan-pairing/challenge'),
  Proof: decodeApiPath('/api/lan-pairing/proof'),
  Control: decodeApiPath('/api/lan-pairing/control'),
  Registry: decodeApiPath('/api/lan-pairing/registry'),
} as const;

export const PlannedLanPairingHttpEndpointSupport = {
  Discovery: 'planned-unsupported',
  Challenge: 'planned-unsupported',
  Proof: 'planned-unsupported',
  Control: 'planned-unsupported',
  Registry: 'planned-unsupported',
} as const;

export const LanPairingHeader = {
  Origin: decodeHeaderName('Origin'),
  PairingProof: decodeHeaderName('X-Ocentra-LAN-Pairing-Proof'),
  PairingId: decodeHeaderName('X-Ocentra-LAN-Pairing-Id'),
  IntentId: decodeHeaderName('X-Ocentra-LAN-Intent-Id'),
  TargetDeviceId: decodeHeaderName('X-Ocentra-LAN-Target-Device-Id'),
} as const;
