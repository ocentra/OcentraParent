import { describe, expect, it } from 'vitest';
import {
  TrackingDesktopPresenceHintRowSchema,
  buildTrackingDesktopPresenceHintRows,
  summarizeTrackingDesktopPresenceHintProof,
} from '../../src/tracking-desktop-presence-hint-proof';

describe('tracking desktop presence hint proof', () => {
  it('keeps LAN, Wi-Fi, and IP rows as hint-only with no precise location claim', () => {
    const summary = summarizeTrackingDesktopPresenceHintProof(buildTrackingDesktopPresenceHintRows());

    expect(summary.hintOnlyRows).toEqual(['lan-pairing', 'home-wifi', 'ip-coarse']);
    expect(summary.preciseLocationClaimCount).toBe(0);
    expect(summary.physicalPresenceClaimCount).toBe(0);
  });

  it('separates manual check-in from automatic physical presence', () => {
    const summary = summarizeTrackingDesktopPresenceHintProof(buildTrackingDesktopPresenceHintRows());

    expect(summary.manualCheckInSeparatedRows).toEqual(['linux-manual-check-in', 'manual-check-in']);
    expect(summary.manualRequiredPreciseLocationRows).toEqual(['windows-os-location', 'macos-os-location']);
  });

  it('represents stale, offline, and missing-device rows without live-device claims', () => {
    const rows = buildTrackingDesktopPresenceHintRows();
    const staleOfflineMissingRows = rows.filter((row) => ['stale', 'offline', 'missing-device'].includes(row.state));

    expect(staleOfflineMissingRows.map((row) => row.source)).toEqual(['stale-cache', 'offline', 'missing-device']);
    expect(staleOfflineMissingRows.every((row) => row.liveDeviceClaimed === false)).toBe(true);
  });

  it('rejects GPS, physical presence, and stale-live upgrades', () => {
    const rows = buildTrackingDesktopPresenceHintRows();

    expect(
      TrackingDesktopPresenceHintRowSchema.safeParse({
        ...rows[3],
        canClaimPreciseLocation: true,
      }).success
    ).toBe(false);
    expect(
      TrackingDesktopPresenceHintRowSchema.safeParse({
        ...rows[4],
        physicalPresenceClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingDesktopPresenceHintRowSchema.safeParse({
        ...rows[7],
        liveDeviceClaimed: true,
      }).success
    ).toBe(false);
  });
});
