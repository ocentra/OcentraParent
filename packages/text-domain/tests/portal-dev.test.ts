import { describe, expect, it } from 'vitest';
import { decodeDisplayText } from '../src/contracts';
import { PortalDevText, PortalDevTextToken } from '../src/portal-dev';

describe('portal dev text', () => {
  it('PortalDevText: exposes schema-backed display text tokens', () => {
    const title = PortalDevText[PortalDevTextToken.AppTitle];
    expect(decodeDisplayText(title)).toBe(title);
    expect(title).toBe('Ocentra Parent');
    expect(PortalDevText[PortalDevTextToken.Subtitle]).toBe('Family safety for local child devices');
    expect(PortalDevText[PortalDevTextToken.ParentPortal]).toBe('Start here');
    expect(PortalDevText[PortalDevTextToken.Activity]).toBe('Activity');
    expect(PortalDevText[PortalDevTextToken.SettingsRules]).toBe('Settings');
    expect(PortalDevText[PortalDevTextToken.FrameTuner]).toBe('App layout');
    expect(PortalDevText[PortalDevTextToken.FrameTunerDescription]).toBe(
      'Layout and content editor for parent portal app surfaces.'
    );
    expect(PortalDevText[PortalDevTextToken.HeaderBrandLeft]).toBe("O'centra");
    expect(PortalDevText[PortalDevTextToken.HeaderBrandRight]).toBe('Parent');
    expect(PortalDevText[PortalDevTextToken.HeaderTagline]).toBe('Your House Your Rule');
    expect(PortalDevText[PortalDevTextToken.FooterVersion]).toBe('[ alpha v0.1.1 ]');
    expect(PortalDevText[PortalDevTextToken.AuthTitle]).toBe('Protect the family console');
    expect(PortalDevText[PortalDevTextToken.AuthUnavailable]).toBe(
      'Parent identity is not connected on this device yet.'
    );
    expect(PortalDevText[PortalDevTextToken.DeviceRuleScope]).toBe('Device rule scope');
    expect(PortalDevText[PortalDevTextToken.ManagedWeb]).toBe('Managed web');
    expect(PortalDevText[PortalDevTextToken.ProductSurfacePending]).toBe(
      'No family setting is configured for this area yet.'
    );
    expect(PortalDevText[PortalDevTextToken.LiveActivity]).toBe('Live activity');
    expect(PortalDevText[PortalDevTextToken.RecentActivity]).toBe('Recent activity');
    expect(PortalDevText[PortalDevTextToken.BrowserIntervention]).toBe('Browser protection');
    expect(PortalDevText[PortalDevTextToken.PolicyPreview]).toBe('Policy decision');
    expect(PortalDevText[PortalDevTextToken.PolicyPreviewNoEnforcement]).toBe('Protection mode: advisory.');
    expect(PortalDevText[PortalDevTextToken.GetPolicyPreviewReadModel]).toBe('Refresh policy decision');
    expect(PortalDevText[PortalDevTextToken.GetBrowserInterventionReadModel]).toBe('Refresh browser protection');
    expect(PortalDevText[PortalDevTextToken.GetActivityReportDaily]).toBe('Build daily activity report');
    expect(PortalDevText[PortalDevTextToken.GetActivityReportHistory]).toBe('Refresh activity report history');
    expect(PortalDevText[PortalDevTextToken.GetActivityScreenReadModel]).toBe('Refresh activity screen');
    expect(PortalDevText[PortalDevTextToken.GetActivityAppUseReadModel]).toBe('Refresh activity app use');
    expect(PortalDevText[PortalDevTextToken.GetActivityBrowserReadModel]).toBe('Refresh activity browser');
    expect(PortalDevText[PortalDevTextToken.GetActivityGamesReadModel]).toBe('Refresh activity games');
    expect(PortalDevText[PortalDevTextToken.GetActivityNetworkReadModel]).toBe('Refresh activity network');
    expect(PortalDevText[PortalDevTextToken.TrackingChildCheckInProofTitle]).toBe('Child check-in request');
    expect(PortalDevText[PortalDevTextToken.TrackingChildCheckInProofBody]).toBe(
      'Your parent is asking you to check in. Are you safe?'
    );
    expect(PortalDevText[PortalDevTextToken.TrackingChildCheckInSafeAction]).toBe("I'm safe");
    expect(PortalDevText[PortalDevTextToken.TrackingChildCheckInHelpAction]).toBe('Need help');
    expect(PortalDevText[PortalDevTextToken.TrackingChildCheckInShareLocationAction]).toBe('Share current location');
    expect(PortalDevText[PortalDevTextToken.TrackingChildCheckInCallParentAction]).toBe('Call parent');
    expect(PortalDevText[PortalDevTextToken.TrackingChildCheckInDeliveryBoundary]).toBe(
      'Child-device delivery not proved'
    );
    expect(PortalDevText[PortalDevTextToken.TrackingChildCheckInCopyBoundary]).toBe('Calm copy, no accusation');
    expect(PortalDevText[PortalDevTextToken.CommandResult]).toBe('Command result');
    expect(PortalDevText[PortalDevTextToken.CopyResult]).toBe('Copy result');
  });
});
