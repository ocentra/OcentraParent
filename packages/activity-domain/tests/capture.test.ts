import { describe, expect, it } from 'vitest';
import {
  ActivityCaptureCapabilityStatus,
  ActivityCaptureCapabilityStatusSchema,
  ActivityObservationMode,
  ActivityObservationModeSchema,
} from '../src/capture';

describe('activity capture contracts', () => {
  it('parses observation modes and capability statuses', () => {
    expect(ActivityObservationModeSchema.parse(ActivityObservationMode.Snapshot)).toBe('snapshot');
    expect(ActivityObservationModeSchema.parse(ActivityObservationMode.ActiveWindow)).toBe('active-window');
    expect(ActivityCaptureCapabilityStatusSchema.parse(ActivityCaptureCapabilityStatus.Available)).toBe('available');
    expect(ActivityCaptureCapabilityStatusSchema.parse(ActivityCaptureCapabilityStatus.NoActiveWindow)).toBe(
      'no-active-window'
    );
  });

  it('rejects unknown degraded status values', () => {
    const parsed = ActivityCaptureCapabilityStatusSchema.safeParse('maybe-working');

    expect(parsed.success).toBe(false);
  });
});
