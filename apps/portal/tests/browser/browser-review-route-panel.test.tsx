import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute, type ParentBrowserPanelSnapshot } from '../../generated/parent-ui-bridge';
import { BrowserReviewRoutePanel, shouldRenderBrowserReviewRoute } from '../../src/BrowserReviewRoutePanel';

function panel(title: string): ParentBrowserPanelSnapshot {
  return {
    eyebrow: 'Browser route',
    title,
    body: `${title} body`,
    summary: `1 ${title.toLowerCase()} row`,
    summaryDetails: [
      { label: 'Rows returned', value: '1' },
      { label: 'Status', value: 'reported' },
    ],
    rows: [
      {
        key: `${title}-row`,
        title: `${title} row`,
        details: [{ label: 'State', value: 'manual-required' }],
      },
    ],
    emptyMessage: `${title} empty`,
    productClaim: `${title} projection only`,
  };
}

describe('Browser review product route', () => {
  it('is owned by Browser and not the developer Proof Panels route', () => {
    expect(shouldRenderBrowserReviewRoute(ParentRoute.Browser)).toBe(true);
    expect(shouldRenderBrowserReviewRoute(ParentRoute.ProofPanels)).toBe(false);
  });

  it('renders the service-backed social dashboard as the default review surface', () => {
    const markup = renderToStaticMarkup(
      createElement(BrowserReviewRoutePanel, {
        actions: {
          reconnect() {},
          async refreshRouteSnapshot() {
            return null;
          },
          selectCommandResult() {},
          async sendCommand() {
            return null;
          },
        },
        commandEnabled: true,
        browserParentExplanationPanel: panel('Browser explanation'),
        socialAuditExplanationPanel: panel('Social explanations'),
        socialDashboardPanel: panel('Social dashboard'),
        socialAlertReportPanel: panel('Social alerts'),
        socialAlertReportParentSurfacePanel: panel('Parent surface'),
        socialParentNotificationDeliveryPanel: panel('Notification delivery'),
        browserActionIntentStreamStatusPanel: panel('Action stream'),
        browserSocialProviderReceiptStreamStatusPanel: panel('Receipt stream'),
        browserSocialProviderReceiptIngestionReadinessStatusPanel: panel('Receipt readiness'),
      })
    );

    expect(markup).toContain('aria-label="Social review"');
    expect(markup).toContain('class="browser-review-route-panel"');
    expect(markup).toContain('class="browser-review-route-tabs"');
    expect(markup).toContain('class="browser-review-route-content"');
    expect(markup).toContain('Browser &amp; social review');
    expect(markup).toContain('Social dashboard row');
    expect(markup).toContain('Alerts &amp; delivery');
    expect(markup).not.toContain(PortalDom.Classes.DeveloperRoutePanel);
    expect(markup).not.toContain('Proof panel');
  });
});
