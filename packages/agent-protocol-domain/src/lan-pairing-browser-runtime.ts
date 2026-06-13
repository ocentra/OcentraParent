import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LanBrowserAddDeviceRuntimeRequestSchema,
  LanBrowserDiscoveryScanRequestSchema,
} from '@ocentra-parent/lan-domain/lan-pairing-browser-runtime';

export const AgentLanBrowserRuntimeCommandNameLiteral = {
  BrowserDiscoveryScan: 'agent.lan-pairing.browser-discovery.scan',
  AddDeviceRequest: 'agent.lan-pairing.add-device.request',
} as const;

export const AgentLanBrowserRuntimeEventNameLiteral = {
  BrowserDiscoveryReported: 'agent.lan-pairing.browser-discovery.reported',
  AddDeviceReported: 'agent.lan-pairing.add-device.reported',
} as const;

export const AgentLanBrowserRuntimeCommandSchema = withParser(
  Schema.Literal(
    AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan,
    AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest
  )
);

export const AgentLanBrowserRuntimeEventSchema = withParser(
  Schema.Literal(
    AgentLanBrowserRuntimeEventNameLiteral.BrowserDiscoveryReported,
    AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported
  )
);

export const AgentLanBrowserDiscoveryScanRequestSchema = LanBrowserDiscoveryScanRequestSchema;
export const AgentLanBrowserAddDeviceRequestSchema = LanBrowserAddDeviceRuntimeRequestSchema;

export type AgentLanBrowserRuntimeCommand = Infer<typeof AgentLanBrowserRuntimeCommandSchema>;
export type AgentLanBrowserRuntimeEvent = Infer<typeof AgentLanBrowserRuntimeEventSchema>;
export type AgentLanBrowserDiscoveryScanRequest = Infer<typeof AgentLanBrowserDiscoveryScanRequestSchema>;
export type AgentLanBrowserAddDeviceRequest = Infer<typeof AgentLanBrowserAddDeviceRequestSchema>;
