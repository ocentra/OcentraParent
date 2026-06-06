import { Schema, withParser, type Infer } from '@ocentra-parent/schema-domain/effect';
import { TrackingPolicySchemaVersion } from './tracking-location-policy';
import { TrackingPolicyAuditRefSchema, TrackingPolicyReasonCodeSchema } from './tracking-location-policy-primitives';

const PresenceProofTextSchema = Schema.String.pipe(Schema.minLength(1));

interface DesktopPresenceHintClaimShape {
  readonly source: Infer<typeof TrackingDesktopPresenceSourceSchema>;
  readonly state: Infer<typeof TrackingDesktopPresenceStateSchema>;
  readonly canClaimPreciseLocation: boolean;
  readonly physicalPresenceClaimed: boolean;
  readonly liveDeviceClaimed: boolean;
  readonly manualCheckInSeparateFromAutomaticPresence: boolean;
}

export const TrackingDesktopPresenceSourceSchema = withParser(
  Schema.Literal(
    'windows-os-location',
    'macos-os-location',
    'linux-manual-check-in',
    'lan-pairing',
    'home-wifi',
    'ip-coarse',
    'manual-check-in',
    'stale-cache',
    'offline',
    'missing-device'
  )
);

export const TrackingDesktopPresenceStateSchema = withParser(
  Schema.Literal('manual-required', 'hint-only', 'stale', 'offline', 'missing-device')
);

export const TrackingDesktopPresenceHintRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    source: TrackingDesktopPresenceSourceSchema,
    state: TrackingDesktopPresenceStateSchema,
    label: PresenceProofTextSchema,
    canClaimPreciseLocation: Schema.Literal(false),
    physicalPresenceClaimed: Schema.Literal(false),
    liveDeviceClaimed: Schema.Boolean,
    manualCheckInSeparateFromAutomaticPresence: Schema.Boolean,
    requiresManualProofForPreciseLocation: Schema.Boolean,
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  }).pipe(
    Schema.filter(
      (row) =>
        desktopPresenceHintClaimIsSafe(row) ||
        'Desktop presence hints must not become GPS, physical presence, or live-device proof'
    )
  )
);

export type TrackingDesktopPresenceHintRow = Infer<typeof TrackingDesktopPresenceHintRowSchema>;

export function buildTrackingDesktopPresenceHintRows(): readonly TrackingDesktopPresenceHintRow[] {
  return [
    row('windows-os-location', 'manual-required', 'Windows OS location proof required', true, false, [
      'desktop-precise-location-manual-required',
    ]),
    row('macos-os-location', 'manual-required', 'macOS OS location proof required', true, false, [
      'desktop-precise-location-manual-required',
    ]),
    row('linux-manual-check-in', 'manual-required', 'Linux manual check-in proof required', false, true, [
      'desktop-manual-check-in-separate',
    ]),
    row('lan-pairing', 'hint-only', 'LAN pairing is a connection hint only', false, false, [
      'lan-pairing-not-physical-presence',
    ]),
    row('home-wifi', 'hint-only', 'Home Wi-Fi is a presence hint only', false, false, ['wifi-not-precise-location']),
    row('ip-coarse', 'hint-only', 'IP region is a coarse hint only', false, false, ['ip-not-precise-location']),
    row('manual-check-in', 'manual-required', 'Manual check-in is child-reported status only', false, true, [
      'manual-check-in-not-automatic-location',
    ]),
    row('stale-cache', 'stale', 'Cached desktop status is stale', false, false, ['stale-not-live-location']),
    row('offline', 'offline', 'Desktop agent is offline', false, false, ['offline-not-live-location']),
    row('missing-device', 'missing-device', 'Missing-device mode uses last-known only', false, false, [
      'missing-device-last-known-only',
    ]),
  ];
}

export function summarizeTrackingDesktopPresenceHintProof(rows: readonly TrackingDesktopPresenceHintRow[]) {
  const parsedRows = rows.map((entry) => TrackingDesktopPresenceHintRowSchema.parse(entry));

  return {
    sourceCount: parsedRows.length,
    states: [...new Set(parsedRows.map((entry) => entry.state))],
    preciseLocationClaimCount: parsedRows.filter((entry) => entry.canClaimPreciseLocation).length,
    physicalPresenceClaimCount: parsedRows.filter((entry) => entry.physicalPresenceClaimed).length,
    liveDeviceClaimCount: parsedRows.filter((entry) => entry.liveDeviceClaimed).length,
    manualRequiredPreciseLocationRows: parsedRows
      .filter((entry) => entry.requiresManualProofForPreciseLocation)
      .map((entry) => entry.source),
    manualCheckInSeparatedRows: parsedRows
      .filter((entry) => entry.manualCheckInSeparateFromAutomaticPresence)
      .map((entry) => entry.source),
    hintOnlyRows: parsedRows.filter((entry) => entry.state === 'hint-only').map((entry) => entry.source),
    staleOfflineMissingRows: parsedRows
      .filter((entry) => ['stale', 'offline', 'missing-device'].includes(entry.state))
      .map((entry) => entry.source),
  };
}

function desktopPresenceHintClaimIsSafe(row: DesktopPresenceHintClaimShape): boolean {
  if (row.canClaimPreciseLocation || row.physicalPresenceClaimed) {
    return false;
  }
  if (['stale', 'offline', 'missing-device'].includes(row.state) && row.liveDeviceClaimed) {
    return false;
  }
  if (['manual-check-in', 'linux-manual-check-in'].includes(row.source)) {
    return row.manualCheckInSeparateFromAutomaticPresence;
  }
  return true;
}

function row(
  source: Infer<typeof TrackingDesktopPresenceSourceSchema>,
  state: Infer<typeof TrackingDesktopPresenceStateSchema>,
  label: string,
  requiresManualProofForPreciseLocation: boolean,
  manualCheckInSeparateFromAutomaticPresence: boolean,
  reasonCodes: readonly string[]
): TrackingDesktopPresenceHintRow {
  return TrackingDesktopPresenceHintRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    source,
    state,
    label,
    canClaimPreciseLocation: false,
    physicalPresenceClaimed: false,
    liveDeviceClaimed: state === 'manual-required' || state === 'hint-only',
    manualCheckInSeparateFromAutomaticPresence,
    requiresManualProofForPreciseLocation,
    reasonCodes,
    auditRefs: ['tracking-desktop-presence-hint-proof'],
  });
}
