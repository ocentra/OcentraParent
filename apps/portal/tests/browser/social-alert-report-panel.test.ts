import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute, type ParentBrowserPanelSnapshot } from '../../generated/parent-ui-bridge';
import { SocialAlertReportRoutePanel, shouldRenderSocialAlertReportRoute } from '../../src/SocialAlertReportRoutePanel';

function samplePanel(title: string, summary: string): ParentBrowserPanelSnapshot {
  return {
    eyebrow: 'Browser route',
    title,
    body: `${title} body`,
    summary,
    summaryDetails: [
      { label: 'Rows returned', value: '1' },
      { label: 'Status', value: 'ready' },
      { label: 'Product claim', value: `${title} projection only.` },
    ],
    rows: [
      {
        key: `${title}-row-1`,
        title: `${title} row`,
        details: [
          { label: 'Status', value: 'manual-required' },
          { label: 'Evidence references', value: 'evidence-ref-1' },
        ],
      },
    ],
    emptyMessage: `${title} empty`,
    productClaim: `${title} projection only.`,
  };
}

describe('portal social alert/report panel', () => {
  it('mounts only on the proof-panels route', () => {
    expect(shouldRenderSocialAlertReportRoute(ParentRoute.ProofPanels)).toBe(true);
    expect(shouldRenderSocialAlertReportRoute(ParentRoute.AppGameSessions)).toBe(false);
  });

  it('renders Rust-owned social alert, parent surface, notification, and receipt panels', () => {
    const actions = {
      reconnect() {},
      async refreshRouteSnapshot() {
        return null;
      },
      selectCommandResult() {
        return undefined;
      },
      async sendCommand() {
        return null;
      },
    };
    const markup = renderToStaticMarkup(
      createElement(SocialAlertReportRoutePanel, {
        actions,
        commandEnabled: true,
        socialAlertReportPanel: samplePanel('Social alerts and reports', '1 social alert/report rows'),
        socialAlertReportParentSurfacePanel: samplePanel('Social parent surface status', '1 parent surface rows'),
        socialParentNotificationDeliveryPanel: samplePanel(
          'Social parent notification delivery readiness',
          '1 parent notification readiness rows'
        ),
        browserActionIntentStreamStatusPanel: samplePanel('Browser action-intent stream status', '0 action candidates'),
        browserSocialProviderReceiptStreamStatusPanel: samplePanel(
          'Social provider receipt stream status',
          '0 receipt boundary rows'
        ),
        browserSocialProviderReceiptIngestionReadinessStatusPanel: samplePanel(
          'Social provider receipt ingestion readiness',
          '0 readiness rows'
        ),
      })
    );

    expect(markup).toContain('Social alerts and reports');
    expect(markup).toContain('1 social alert/report rows');
    expect(markup).toContain('Social parent surface status');
    expect(markup).toContain('Social parent notification delivery readiness');
    expect(markup).toContain('Browser action-intent stream status');
    expect(markup).toContain('Social provider receipt stream status');
    expect(markup).toContain('Social provider receipt ingestion readiness');
    expect(markup).toContain('manual-required');
    expect(markup).toContain('evidence-ref-1');
  });
});
