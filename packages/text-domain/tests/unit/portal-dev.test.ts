import { describe, expect, it } from 'vitest';
import { decodeDisplayText } from '../../src/contracts';
import { PortalDevText, PortalDevTextToken } from '../../src/portal-dev';

const expectPortalDevTextEntries = (entries: ReadonlyArray<readonly [PortalDevTextToken, string]>) => {
  for (const [token, expected] of entries) {
    expect(PortalDevText[token]).toBe(expected);
  }
};

describe('portal dev text', () => {
  it('PortalDevText: exposes schema-backed display text tokens', () => {
    const title = PortalDevText[PortalDevTextToken.AppTitle];
    expect(decodeDisplayText(title)).toBe(title);
    expectPortalDevTextEntries([
      [PortalDevTextToken.AppTitle, 'Ocentra Parent'],
      [PortalDevTextToken.Subtitle, 'Family safety for local child devices'],
      [PortalDevTextToken.ParentPortal, 'Start here'],
      [PortalDevTextToken.Activity, 'Activity'],
      [PortalDevTextToken.SettingsRules, 'Settings'],
      [PortalDevTextToken.FrameTuner, 'App layout'],
      [PortalDevTextToken.FrameTunerDescription, 'Layout and content editor for parent portal app surfaces.'],
      [PortalDevTextToken.HeaderBrandLeft, "O'centra"],
      [PortalDevTextToken.HeaderBrandRight, 'Parent'],
      [PortalDevTextToken.HeaderTagline, 'Your House Your Rule'],
      [PortalDevTextToken.FooterVersion, '[ alpha v0.1.1 ]'],
      [PortalDevTextToken.AuthTitle, 'Protect the family console'],
      [PortalDevTextToken.AuthUnavailable, 'Parent identity is not connected on this device yet.'],
      [PortalDevTextToken.DeviceRuleScope, 'Device rule scope'],
      [PortalDevTextToken.ManagedWeb, 'Managed web'],
      [PortalDevTextToken.ProductSurfacePending, 'No family setting is configured for this area yet.'],
      [PortalDevTextToken.LiveActivity, 'Live activity'],
      [PortalDevTextToken.RecentActivity, 'Recent activity'],
      [PortalDevTextToken.BrowserIntervention, 'Browser protection'],
      [PortalDevTextToken.PolicyPreview, 'Policy decision'],
      [PortalDevTextToken.PolicyPreviewNoEnforcement, 'Protection mode: advisory.'],
      [PortalDevTextToken.AppGameNotificationParentSurface, 'App/game notification surface'],
      [
        PortalDevTextToken.AppGameNotificationParentSurfaceBody,
        'Redacted app/game alert rows show setup and drill-in refs only.',
      ],
      [
        PortalDevTextToken.AppGameNotificationParentSurfaceNoData,
        'No app/game notification parent-surface intent has been reported yet.',
      ],
      [
        PortalDevTextToken.AppGameNotificationParentSurfaceNoRuntimeClaim,
        'Portal renders intent rows only; provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed.',
      ],
      [PortalDevTextToken.AppGamePolicyReadiness, 'App/game policy readiness'],
      [
        PortalDevTextToken.AppGamePolicyReadinessNoProductClaim,
        'Readiness rendering only; policy execution and adapter dispatch are not proved.',
      ],
      [PortalDevTextToken.GetActivityAppGamePolicyReadinessReadModel, 'Refresh policy readiness'],
      [PortalDevTextToken.GetPolicyPreviewReadModel, 'Refresh policy decision'],
      [PortalDevTextToken.GetBrowserInterventionReadModel, 'Refresh browser protection'],
      [PortalDevTextToken.GetActivityReportDaily, 'Build daily activity report'],
      [PortalDevTextToken.GetActivityReportHistory, 'Refresh activity report history'],
      [PortalDevTextToken.GetActivityScreenReadModel, 'Refresh activity screen'],
      [PortalDevTextToken.GetActivityAppUseReadModel, 'Refresh activity app use'],
      [PortalDevTextToken.GetActivityBrowserReadModel, 'Refresh activity browser'],
      [PortalDevTextToken.GetActivityGamesReadModel, 'Refresh activity games'],
      [PortalDevTextToken.GetActivityNetworkReadModel, 'Refresh activity network'],
      [PortalDevTextToken.TrackingServiceDataCoverage, 'Service data coverage'],
      [PortalDevTextToken.TrackingChildCheckInProofTitle, 'Child check-in request'],
      [PortalDevTextToken.TrackingChildCheckInProofBody, 'Your parent is asking you to check in. Are you safe?'],
      [PortalDevTextToken.TrackingChildCheckInSafeAction, "I'm safe"],
      [PortalDevTextToken.TrackingChildCheckInHelpAction, 'Need help'],
      [PortalDevTextToken.TrackingChildCheckInShareLocationAction, 'Share current location'],
      [PortalDevTextToken.TrackingChildCheckInCallParentAction, 'Call parent'],
      [PortalDevTextToken.TrackingChildCheckInDeliveryBoundary, 'Child-device delivery not proved'],
      [PortalDevTextToken.TrackingChildCheckInCopyBoundary, 'Calm copy, no accusation'],
      [PortalDevTextToken.CommandResult, 'Command result'],
      [PortalDevTextToken.CopyResult, 'Copy result'],
    ]);
  });
});
