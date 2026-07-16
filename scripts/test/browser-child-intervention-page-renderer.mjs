import { renderBrowserChildInterventionPage as renderDomainBrowserChildInterventionPage } from '@ocentra-parent/portal-domain/browser-child-intervention-page';

const defaultBlockMarker = 'OCENTRA_MANAGED_BROWSER_BLOCKED';

export function renderBrowserChildInterventionPage({
  rule,
  requestedUrl,
  bridge,
  backdrop,
  blockMarker = defaultBlockMarker,
}) {
  return renderDomainBrowserChildInterventionPage({
    action: rule.action,
    backdrop,
    blockMarker,
    bridge,
    deliveryState: rule.deliveryState,
    outcome: rule.outcome,
    parentRequestEnabled: rule.action !== 'checking-hold',
    reason: rule.label,
    requestedUrl,
    ruleId: rule.id,
    ruleLabel: rule.label,
    ruleMarker: rule.marker,
    targetType: rule.targetType,
  });
}
