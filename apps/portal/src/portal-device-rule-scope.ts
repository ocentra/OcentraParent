import {
  PortalDom,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
  type PortalDetailValue,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';
import type { PortalRuntimeState } from './portal-state';

type BadgeText = PortalDetailValue | PortalDisplayText;

type RuleCard = {
  readonly title: PortalDisplayText;
  readonly body: PortalDisplayText;
  readonly status: PortalDisplayText;
  readonly tipBody: PortalDisplayText;
};

export function renderDeviceRuleScope(container: HTMLElement, state: PortalRuntimeState): void {
  const panel = document.createElement(PortalDom.Tags.Section);
  panel.className = [PortalDom.Classes.Summary, PortalDom.Classes.DeviceRuleScopePanel].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = PortalText.Resolve(PortalTextToken.DeviceRuleScope);

  const note = document.createElement(PortalDom.Tags.Paragraph);
  note.className = PortalDom.Classes.ProductPanelNote;
  note.textContent = PortalText.Resolve(PortalTextToken.DeviceRuleScopeBody);

  const selector = document.createElement(PortalDom.Tags.Division);
  selector.className = PortalDom.Classes.AppStatusBar;
  selector.append(
    badge(PortalText.Resolve(PortalTextToken.ChildDevice), selectedDevice(state)),
    badge(PortalText.Resolve(PortalTextToken.FamilyDefault), PortalText.Resolve(PortalTextToken.PolicyModeAdvisory))
  );

  const grid = document.createElement(PortalDom.Tags.Division);
  grid.className = PortalDom.Classes.CapabilityGrid;
  for (const card of ruleCards()) {
    grid.append(ruleCard(card));
  }

  panel.append(title, note, selector, grid);
  container.append(panel);
}

function selectedDevice(state: PortalRuntimeState): PortalDetailValue {
  if (state.latestSnapshot === null) {
    return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.NotReported));
  }
  return decodePortalDetailValue(state.latestSnapshot.agent.deviceId);
}

function ruleCards(): readonly RuleCard[] {
  return [
    {
      title: PortalText.Resolve(PortalTextToken.ManagedWeb),
      body: PortalText.Resolve(PortalTextToken.BrowserBlockBody),
      status: PortalText.Resolve(PortalTextToken.PolicyModeAdvisory),
      tipBody: PortalText.Resolve(PortalTextToken.DeviceRuleScopeTip),
    },
    {
      title: PortalText.Resolve(PortalTextToken.AppGameSessions),
      body: PortalText.Resolve(PortalTextToken.SchedulesBudgetsBody),
      status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
      tipBody: PortalText.Resolve(PortalTextToken.DeviceRuleScopeTip),
    },
    {
      title: PortalText.Resolve(PortalTextToken.SchedulesBudgets),
      body: PortalText.Resolve(PortalTextToken.SchedulesBudgetsBody),
      status: PortalText.Resolve(PortalTextToken.DeviceRuleOverride),
      tipBody: PortalText.Resolve(PortalTextToken.DeviceRuleScopeTip),
    },
    {
      title: PortalText.Resolve(PortalTextToken.Approvals),
      body: PortalText.Resolve(PortalTextToken.ApprovalsBody),
      status: PortalText.Resolve(PortalTextToken.NotConfiguredStatus),
      tipBody: PortalText.Resolve(PortalTextToken.DeviceRuleScopeTip),
    },
  ];
}

function badge(labelText: PortalDisplayText, valueText: BadgeText): HTMLElement {
  const item = document.createElement(PortalDom.Tags.Span);
  item.className = PortalDom.Classes.ProductBadge;
  item.textContent = labelText;

  const value = document.createElement(PortalDom.Tags.Strong);
  value.textContent = valueText;
  item.append(value);
  return item;
}

function ruleCard(card: RuleCard): HTMLElement {
  const item = document.createElement(PortalDom.Tags.Section);
  item.className = [PortalDom.Classes.ControlCard, PortalDom.Classes.ControlCardAccentPrimary].join(
    PortalDom.Classes.ClassNameSeparator
  );

  const header = document.createElement(PortalDom.Tags.Division);
  header.className = PortalDom.Classes.ControlCardHeader;

  const glyph = document.createElement(PortalDom.Tags.Span);
  glyph.className = PortalDom.Classes.ControlCardGlyph;

  const title = document.createElement(PortalDom.Tags.HeadingTwo);
  title.textContent = card.title;

  const status = document.createElement(PortalDom.Tags.Span);
  status.className = PortalDom.Classes.ControlCardStatus;
  status.textContent = card.status;

  const body = document.createElement(PortalDom.Tags.Paragraph);
  body.className = PortalDom.Classes.ControlCardBody;
  body.textContent = card.body;

  const tip = document.createElement(PortalDom.Tags.Division);
  tip.className = PortalDom.Classes.ControlCardTip;

  const tipTitle = document.createElement(PortalDom.Tags.Strong);
  tipTitle.className = PortalDom.Classes.ControlCardTipTitle;
  tipTitle.textContent = PortalText.Resolve(PortalTextToken.HowItWorks);

  const tipBody = document.createElement(PortalDom.Tags.Paragraph);
  tipBody.className = PortalDom.Classes.ControlCardTipBody;
  tipBody.textContent = card.tipBody;

  header.append(glyph, title, status);
  tip.append(tipTitle, tipBody);
  item.append(header, body, tip);
  return item;
}
