import { describe, expect, it } from 'vitest';
import {
  ActivityCaptureCapabilityStatus,
  ActivityCaptureCapabilityStatusSchema,
  ActivityDomainAttributionStatus,
  ActivityDomainAttributionStatusSchema,
  ActivityNetworkProtocol,
  ActivityNetworkProtocolSchema,
  ActivityNetworkTcpState,
  ActivityNetworkTcpStateSchema,
  ActivityObservationMode,
  ActivityObservationModeSchema,
  ActivityProcessAttributionStatus,
  ActivityProcessAttributionStatusSchema,
} from '../src/capture';

describe('activity capture contracts', () => {
  it('parses observation modes and capability statuses', () => {
    expect(ActivityObservationModeSchema.parse(ActivityObservationMode.Snapshot)).toBe('snapshot');
    expect(ActivityObservationModeSchema.parse(ActivityObservationMode.ActiveWindow)).toBe('active-window');
    expect(ActivityCaptureCapabilityStatusSchema.parse(ActivityCaptureCapabilityStatus.Available)).toBe('available');
    expect(ActivityCaptureCapabilityStatusSchema.parse(ActivityCaptureCapabilityStatus.NoActiveWindow)).toBe(
      'no-active-window'
    );
    expect(ActivityCaptureCapabilityStatusSchema.parse(ActivityCaptureCapabilityStatus.NoNetworkObservations)).toBe(
      'no-network-observations'
    );
    expect(ActivityObservationModeSchema.parse(ActivityObservationMode.NetworkSnapshot)).toBe('network-snapshot');
  });

  it('parses network/domain attribution values', () => {
    expect(ActivityNetworkProtocolSchema.parse(ActivityNetworkProtocol.Tcp)).toBe('tcp');
    expect(ActivityNetworkTcpStateSchema.parse(ActivityNetworkTcpState.Established)).toBe('established');
    expect(ActivityDomainAttributionStatusSchema.parse(ActivityDomainAttributionStatus.IpOnly)).toBe('ip-only');
    expect(ActivityProcessAttributionStatusSchema.parse(ActivityProcessAttributionStatus.ProcessAttributed)).toBe(
      'process-attributed'
    );
  });

  it('rejects unknown degraded status values', () => {
    const parsed = ActivityCaptureCapabilityStatusSchema.safeParse('maybe-working');

    expect(parsed.success).toBe(false);
  });

  it('rejects unknown network protocol values', () => {
    const parsed = ActivityNetworkProtocolSchema.safeParse('icmp');

    expect(parsed.success).toBe(false);
  });
});
