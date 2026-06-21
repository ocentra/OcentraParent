import { decodeDisplayText, decodeTextTokenId, type DisplayText } from './text-contracts';

export const PortalDevTrackingUnsupportedTextToken = {
  TrackingUnsupportedManualProofTitle: decodeTextTokenId('portal.dev.trackingUnsupportedManualProofTitle'),
  TrackingUnsupportedManualProofBody: decodeTextTokenId('portal.dev.trackingUnsupportedManualProofBody'),
  TrackingUnsupportedManualAndroidBackground: decodeTextTokenId(
    'portal.dev.trackingUnsupportedManualAndroidBackground'
  ),
  TrackingUnsupportedManualAndroidGeofence: decodeTextTokenId('portal.dev.trackingUnsupportedManualAndroidGeofence'),
  TrackingUnsupportedManualIosBackground: decodeTextTokenId('portal.dev.trackingUnsupportedManualIosBackground'),
  TrackingUnsupportedManualIosGeofence: decodeTextTokenId('portal.dev.trackingUnsupportedManualIosGeofence'),
  TrackingUnsupportedManualDesktopOs: decodeTextTokenId('portal.dev.trackingUnsupportedManualDesktopOs'),
  TrackingUnsupportedManualWebChildAgent: decodeTextTokenId('portal.dev.trackingUnsupportedManualWebChildAgent'),
  TrackingUnsupportedManualAuthorityHardControl: decodeTextTokenId(
    'portal.dev.trackingUnsupportedManualAuthorityHardControl'
  ),
  TrackingUnsupportedManualBoundary: decodeTextTokenId('portal.dev.trackingUnsupportedManualBoundary'),
  TrackingSupportManualRequired: decodeTextTokenId('portal.dev.trackingSupportManualRequired'),
  TrackingSupportPlatformUnsupported: decodeTextTokenId('portal.dev.trackingSupportPlatformUnsupported'),
  TrackingSupportRealDeviceRequired: decodeTextTokenId('portal.dev.trackingSupportRealDeviceRequired'),
  TrackingRenderedManualRequired: decodeTextTokenId('portal.dev.trackingRenderedManualRequired'),
  TrackingRenderedUnavailable: decodeTextTokenId('portal.dev.trackingRenderedUnavailable'),
  TrackingRenderedAuthorityRequired: decodeTextTokenId('portal.dev.trackingRenderedAuthorityRequired'),
  TrackingStateTemporaryLive: decodeTextTokenId('portal.dev.trackingStateTemporaryLive'),
  TrackingStateMissingDevice: decodeTextTokenId('portal.dev.trackingStateMissingDevice'),
  TrackingStateRetentionDeleted: decodeTextTokenId('portal.dev.trackingStateRetentionDeleted'),
  TrackingRetentionHistoryHidden: decodeTextTokenId('portal.dev.trackingRetentionHistoryHidden'),
  TrackingDeletedEvidenceNotRendered: decodeTextTokenId('portal.dev.trackingDeletedEvidenceNotRendered'),
  TrackingEvidenceContracts: decodeTextTokenId('portal.dev.trackingEvidenceContracts'),
  TrackingEvidenceUiFixture: decodeTextTokenId('portal.dev.trackingEvidenceUiFixture'),
  TrackingEvidencePhysicalMissing: decodeTextTokenId('portal.dev.trackingEvidencePhysicalMissing'),
} as const;

export type PortalDevTrackingUnsupportedTextTokenValue =
  (typeof PortalDevTrackingUnsupportedTextToken)[keyof typeof PortalDevTrackingUnsupportedTextToken];

export const PortalDevTrackingUnsupportedText: Record<PortalDevTrackingUnsupportedTextTokenValue, DisplayText> = {
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualProofTitle]: decodeDisplayText(
    'Unsupported/manual tracking platform proof'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualProofBody]: decodeDisplayText(
    'Unsupported platform and manual-required adapter rows render as degraded states without invented capability.'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualAndroidBackground]: decodeDisplayText(
    'Android background location manual required'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualAndroidGeofence]: decodeDisplayText(
    'Android geofence transition manual required'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualIosBackground]: decodeDisplayText(
    'iOS background location manual required'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualIosGeofence]: decodeDisplayText(
    'iOS geofence transition manual required'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualDesktopOs]: decodeDisplayText(
    'Windows desktop OS location manual required'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualWebChildAgent]: decodeDisplayText(
    'Web child agent location unavailable'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualAuthorityHardControl]: decodeDisplayText(
    'Authority hard-control proof required'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingUnsupportedManualBoundary]: decodeDisplayText(
    'Hosted render-state proof only; physical-device, authority, provider delivery, and product readiness remain unclaimed.'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingSupportManualRequired]: decodeDisplayText('manual-required'),
  [PortalDevTrackingUnsupportedTextToken.TrackingSupportPlatformUnsupported]: decodeDisplayText('platform-unsupported'),
  [PortalDevTrackingUnsupportedTextToken.TrackingSupportRealDeviceRequired]: decodeDisplayText('real-device-required'),
  [PortalDevTrackingUnsupportedTextToken.TrackingRenderedManualRequired]: decodeDisplayText('manual-required'),
  [PortalDevTrackingUnsupportedTextToken.TrackingRenderedUnavailable]: decodeDisplayText('unavailable'),
  [PortalDevTrackingUnsupportedTextToken.TrackingRenderedAuthorityRequired]: decodeDisplayText('authority-required'),
  [PortalDevTrackingUnsupportedTextToken.TrackingStateTemporaryLive]: decodeDisplayText('Temporary live'),
  [PortalDevTrackingUnsupportedTextToken.TrackingStateMissingDevice]: decodeDisplayText('Missing device'),
  [PortalDevTrackingUnsupportedTextToken.TrackingStateRetentionDeleted]: decodeDisplayText('Retention deleted'),
  [PortalDevTrackingUnsupportedTextToken.TrackingRetentionHistoryHidden]: decodeDisplayText('Deleted history hidden'),
  [PortalDevTrackingUnsupportedTextToken.TrackingDeletedEvidenceNotRendered]: decodeDisplayText(
    'Deleted evidence not rendered'
  ),
  [PortalDevTrackingUnsupportedTextToken.TrackingEvidenceContracts]: decodeDisplayText('Contract/runtime proof'),
  [PortalDevTrackingUnsupportedTextToken.TrackingEvidenceUiFixture]: decodeDisplayText('UI fixture proof'),
  [PortalDevTrackingUnsupportedTextToken.TrackingEvidencePhysicalMissing]:
    decodeDisplayText('Physical artifact missing'),
};
