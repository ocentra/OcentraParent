import { PortalDom, PortalText, PortalTextToken } from '@ocentra-parent/portal-domain/contracts';
import { renderControlDeck } from './portal-control-card';

const primary = PortalDom.Classes.ControlCardAccentPrimary;
const privacy = PortalDom.Classes.ControlCardAccentPrivacy;
const warn = PortalDom.Classes.ControlCardAccentWarn;

export function renderActivityGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.Activity),
    PortalText.Resolve(PortalTextToken.ActivityDescription),
    [
      {
        title: PortalText.Resolve(PortalTextToken.AppGameSessions),
        body: PortalText.Resolve(PortalTextToken.SchedulesBudgetsBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.DataCustodyTip),
        accent: primary,
      },
      {
        title: PortalText.Resolve(PortalTextToken.ScreenAnalysis),
        body: PortalText.Resolve(PortalTextToken.AiRuntimeBody),
        status: PortalText.Resolve(PortalTextToken.PolicyModeAdvisory),
        tipTitle: PortalText.Resolve(PortalTextToken.RiskToKnow),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserRiskBody),
        accent: warn,
      },
    ]
  );
}

export function renderMemoryGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.Memory),
    PortalText.Resolve(PortalTextToken.MemoryDescription),
    [
      {
        title: PortalText.Resolve(PortalTextToken.Memory),
        body: PortalText.Resolve(PortalTextToken.MemoryBody),
        status: PortalText.Resolve(PortalTextToken.LocalOnlyStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.DataCustodyTip),
        accent: privacy,
      },
    ]
  );
}

export function renderAiGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.AiRuntime),
    PortalText.Resolve(PortalTextToken.AiRuntimeDescription),
    [
      {
        title: PortalText.Resolve(PortalTextToken.AiRuntime),
        body: PortalText.Resolve(PortalTextToken.AiRuntimeBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.DataCustodyTip),
        accent: primary,
      },
      {
        title: PortalText.Resolve(PortalTextToken.DataCustodyTitle),
        body: PortalText.Resolve(PortalTextToken.DataCustodyBody),
        status: PortalText.Resolve(PortalTextToken.LocalOnlyStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.RiskToKnow),
        tipBody: PortalText.Resolve(PortalTextToken.DataCustodyTip),
        accent: privacy,
      },
    ]
  );
}

export function renderPrivacyDesignGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.DataCustodyTitle),
    PortalText.Resolve(PortalTextToken.DataCustodyBody),
    [
      {
        title: PortalText.Resolve(PortalTextToken.DataCustodyTitle),
        body: PortalText.Resolve(PortalTextToken.DataCustodyBody),
        status: PortalText.Resolve(PortalTextToken.LocalOnlyStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.RiskToKnow),
        tipBody: PortalText.Resolve(PortalTextToken.DataCustodyTip),
        accent: privacy,
      },
    ]
  );
}

export function renderNotificationsGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.Notifications),
    PortalText.Resolve(PortalTextToken.NotificationsBody),
    [
      {
        title: PortalText.Resolve(PortalTextToken.Notifications),
        body: PortalText.Resolve(PortalTextToken.NotificationsBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserSupportedTip),
        accent: primary,
      },
    ]
  );
}

export function renderDriveConnectionsGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.DriveConnectionsTitle),
    PortalText.Resolve(PortalTextToken.DriveConnectionsBody),
    [
      {
        title: PortalText.Resolve(PortalTextToken.DriveConnectionsTitle),
        body: PortalText.Resolve(PortalTextToken.DriveConnectionsBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.DriveConnectionsTip),
        accent: privacy,
      },
    ]
  );
}

export function renderDeviceGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.Devices),
    PortalText.Resolve(PortalTextToken.DevicesDescription),
    [
      {
        title: PortalText.Resolve(PortalTextToken.DeviceInventory),
        body: PortalText.Resolve(PortalTextToken.DeviceInventoryBody),
        status: PortalText.Resolve(PortalTextToken.ProductStatusLive),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.DataCustodyTip),
        accent: primary,
      },
      {
        title: PortalText.Resolve(PortalTextToken.Pairing),
        body: PortalText.Resolve(PortalTextToken.PairingBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.DriveConnectionsTip),
        accent: privacy,
      },
      {
        title: PortalText.Resolve(PortalTextToken.MobileApp),
        body: PortalText.Resolve(PortalTextToken.MobileAppBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.RiskToKnow),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserUnsupportedTip),
        accent: warn,
      },
    ]
  );
}
