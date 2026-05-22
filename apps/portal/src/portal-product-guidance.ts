import { PortalDom, PortalText, PortalTextToken } from '@ocentra-parent/portal-domain/contracts';
import { renderControlDeck } from './portal-control-card';

const primary = PortalDom.Classes.ControlCardAccentPrimary;
const privacy = PortalDom.Classes.ControlCardAccentPrivacy;
const warn = PortalDom.Classes.ControlCardAccentWarn;

export function renderOverviewGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.FamilyRulesTitle),
    PortalText.Resolve(PortalTextToken.FamilyRulesBody),
    [
      {
        title: PortalText.Resolve(PortalTextToken.BrowserBlockTitle),
        body: PortalText.Resolve(PortalTextToken.BrowserBlockBody),
        status: PortalText.Resolve(PortalTextToken.PolicyModeAdvisory),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserBlockTip),
        accent: primary,
      },
      {
        title: PortalText.Resolve(PortalTextToken.DataCustodyTitle),
        body: PortalText.Resolve(PortalTextToken.DataCustodyBody),
        status: PortalText.Resolve(PortalTextToken.LocalOnlyStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.DataCustodyTip),
        accent: privacy,
      },
      {
        title: PortalText.Resolve(PortalTextToken.BrowserUnsupportedTitle),
        body: PortalText.Resolve(PortalTextToken.BrowserRiskBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.RiskToKnow),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserUnsupportedTip),
        accent: warn,
      },
    ]
  );
}

export function renderBrowserGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.BrowserControls),
    PortalText.Resolve(PortalTextToken.BrowserDescription),
    [
      {
        title: PortalText.Resolve(PortalTextToken.BrowserSupportedTitle),
        body: PortalText.Resolve(PortalTextToken.BrowserSupportedBody),
        status: PortalText.Resolve(PortalTextToken.ProductStatusPreviewOnly),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserSupportedTip),
        accent: primary,
      },
      {
        title: PortalText.Resolve(PortalTextToken.BrowserUnsupportedTitle),
        body: PortalText.Resolve(PortalTextToken.BrowserUnsupportedBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.RiskToKnow),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserUnsupportedTip),
        accent: warn,
      },
      {
        title: PortalText.Resolve(PortalTextToken.BrowserBlockTitle),
        body: PortalText.Resolve(PortalTextToken.BrowserBlockBody),
        status: PortalText.Resolve(PortalTextToken.PolicyModeAdvisory),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserBlockTip),
        accent: privacy,
      },
    ]
  );
}

export function renderPolicyGuidance(container: HTMLElement): void {
  renderControlDeck(
    container,
    PortalText.Resolve(PortalTextToken.ParentControls),
    PortalText.Resolve(PortalTextToken.RuleBuilderBody),
    [
      {
        title: PortalText.Resolve(PortalTextToken.RuleBuilder),
        body: PortalText.Resolve(PortalTextToken.RuleBuilderBody),
        status: PortalText.Resolve(PortalTextToken.PolicyModeAdvisory),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserBlockTip),
        accent: primary,
      },
      {
        title: PortalText.Resolve(PortalTextToken.SchedulesBudgets),
        body: PortalText.Resolve(PortalTextToken.SchedulesBudgetsBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.HowItWorks),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserSupportedTip),
        accent: privacy,
      },
      {
        title: PortalText.Resolve(PortalTextToken.Approvals),
        body: PortalText.Resolve(PortalTextToken.ApprovalsBody),
        status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
        tipTitle: PortalText.Resolve(PortalTextToken.RiskToKnow),
        tipBody: PortalText.Resolve(PortalTextToken.BrowserUnsupportedTip),
        accent: warn,
      },
    ]
  );
}
