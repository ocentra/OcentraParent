import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, type Page } from '@playwright/test';
import { PortalTrackingRetentionSettingsWriteDefaults } from '@ocentra-parent/portal-domain/contracts';
import { hostedTrackingScreenshotPaths } from './tracking-hosted-ui-proof-accessibility-screenshots';

const repoRoot = path.resolve(process.cwd(), '..', '..');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'tracking-plan-hosted-ui-proof',
  'accessibility-summary.json'
);

type ParentPortalShellSummary = {
  readonly routes: readonly {
    readonly route: string;
    readonly screenshot: string;
    readonly assertions: readonly string[];
  }[];
};

type HostedTrackingLayoutBox = {
  readonly proofId: string;
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
  readonly width: number;
  readonly height: number;
};

export async function collectAccessibilitySummary(page: Page): Promise<{
  readonly hasNamedRegion: boolean;
  readonly headings: readonly string[];
  readonly paragraphs: readonly string[];
  readonly labels: readonly string[];
  readonly values: readonly string[];
  readonly buttons: readonly { readonly text: string; readonly disabled: boolean }[];
  readonly layoutBoxes: readonly HostedTrackingLayoutBox[];
  readonly unlabeledButtons: number;
}> {
  return page.evaluate(() => {
    const region = document.querySelector('[aria-label="Tracking status proof"]');
    const text = (element: Element): string => element.textContent?.trim() ?? '';
    const buttons = Array.from(region?.querySelectorAll('button') ?? []).map((element) => ({
      text: text(element),
      disabled: element.hasAttribute('disabled'),
    }));
    const requiredProofIds = [
      'family-dashboard-rollup',
      'missing-device-ui',
      'notification-parent-surface-history-ui',
      'parent-action-readiness-ui',
      'report-policy-consumer-ui',
      'report-export-ui',
      'service-backed-evidence-drawer',
      'service-backed-citation-detail',
      'retention-settings-ui',
      'child-check-in',
      'child-runtime-ui',
    ];
    const layoutBoxes = requiredProofIds.flatMap((proofId) => {
      const element = region?.querySelector<HTMLElement>(`[data-ocentra-tracking-proof="${proofId}"]`);
      if (element === null || element === undefined) {
        return [];
      }
      const rect = element.getBoundingClientRect();
      return [
        {
          proofId,
          left: Math.round(rect.left),
          top: Math.round(rect.top),
          right: Math.round(rect.right),
          bottom: Math.round(rect.bottom),
          width: Math.round(rect.width),
          height: Math.round(rect.height),
        },
      ];
    });
    return {
      hasNamedRegion: region !== null,
      headings: Array.from(region?.querySelectorAll('h2') ?? []).map(text),
      paragraphs: Array.from(region?.querySelectorAll('p') ?? []).map(text),
      labels: Array.from(region?.querySelectorAll('dt') ?? []).map(text),
      values: Array.from(region?.querySelectorAll('dd') ?? []).map(text),
      buttons,
      layoutBoxes,
      unlabeledButtons: buttons.filter((button) => button.text.length === 0).length,
    };
  });
}

export async function writeAccessibilitySummary(
  summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>,
  parentPortalShellSummary: ParentPortalShellSummary
): Promise<void> {
  assertAccessibilitySummary(summary);
  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/proof-panels',
        assertions: hostedTrackingAssertions(),
        summary,
        screenshots: hostedTrackingScreenshotPaths(repoRoot),
        parentPortalShellSummary,
      },
      null,
      2
    )}\n`
  );
}

function assertAccessibilitySummary(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  assertAccessibilityBasics(summary);
  assertAccessibilityHeadingsAndLabels(summary);
  assertAccessibilityValues(summary);
}

function assertAccessibilityHeadingsAndLabels(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  assertContainsAll(summary.headings, [
    'Tracking status proof',
    'Service read model',
    'Service data coverage',
    'Family dashboard tracking rollup',
    'Report policy consumer UI',
    'Report export read-model UI',
    'Notification history intent UI',
    'Parent action readiness UI',
    'Missing-device state UI',
    'Retention settings read-model UI',
    'Evidence drawer proof',
    'Child check-in request',
    'Child runtime UI proof',
    'Unsupported/manual tracking platform proof',
  ]);
  assertContainsAll(summary.paragraphs, [
    'Your parent is asking you to check in. Are you safe?',
    'Child sees a clear tracking request, safe response, help response, and location-share consent copy.',
  ]);
  assertContainsAll(summary.labels, [
    'Evidence references',
    'Row count',
    'Child copy',
    'Child delivery',
    'Readiness kind',
    'Product claim',
  ]);
}

function assertAccessibilityValues(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  assertAccessibilityReportAndNotificationValues(summary.values);
  assertAccessibilityParentActionAndMissingDeviceValues(summary.values);
  assertAccessibilityRetentionAndEvidenceValues(summary.values);
  assertAccessibilityChildAndPlatformValues(summary.values);
}

function assertAccessibilityReportAndNotificationValues(actualValues: readonly string[]): void {
  assertContainsAll(actualValues, [
    "I'm safe",
    'Family active summary',
    'Child attention summary',
    'Retention audit summary',
    'Parent report summary consumer',
    'Policy evidence drill-in consumer',
    'Retention audit export consumer',
    'consumer-ready',
    'tracking-journal-row-report-summary',
    'tracking-read-model-row-policy-drill-in',
    'tracking-report-policy-evidence-decision',
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
    'Hosted report/policy consumer rendering only; AI execution, product policy mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, production, and product readiness remain unclaimed.',
    'Redacted report packet',
    'Retention audit export packet',
    'Family dashboard summary packet',
    'Policy drill-in export packet',
    'report-export-read-model-ready',
    'tracking-report-export-evidence-redacted-report',
    'tracking-report-export-evidence-policy-drill-in',
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/28-report-export-read-model-proof.json',
    'Hosted report/export packet rendering only; raw location payload export, service mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.',
    'Notification history ready',
    'Manual notification action required',
    'Notification provider unavailable',
    'history-intent-ready',
    'manual-action-required',
    'provider-unavailable',
    'tracking-provider-attempt-home-arrival',
    'receipt-ingestion-required-home-arrival',
    'quiet-hours-requirement-left-school',
    'provider-adapter-unavailable | manual-parent-history-review-required',
    'output/tracking-plan-proof/26-alert-severity-and-notification-model/26-notification-parent-surface-history-proof.json',
    'Hosted notification history rendering only; preference mutation, quiet-hours runtime, provider delivery, receipt ingestion, child-device delivery, physical-device proof, authority, production storage, adapter dispatch, and product readiness remain unclaimed.',
  ]);
}

function assertAccessibilityParentActionAndMissingDeviceValues(actualValues: readonly string[]): void {
  assertContainsAll(actualValues, [
    'Expected-place parent alert ready',
    'Expected-place child check-in ready',
    'Parent acknowledgement recorded',
    'Critical escalation review ready',
    'alert-policy-ready',
    'acknowledgement-recorded',
    'escalation-review-ready',
    'expected-place-evidence-school-arrival',
    'tracking-parent-action-evidence-5',
    'output/tracking-plan-proof/16-expected-place-schedule-engine/29-expected-place-alert-policy-proof.json',
    'output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/30-parent-acknowledgement-action-readiness-proof.json',
    'Hosted parent action readiness rendering only; live service mutation, alert delivery, provider delivery, receipt ingestion, child-device runtime, physical-device proof, authority, production workers, adapter dispatch, and product readiness remain unclaimed.',
    'Last-known only state',
    'Powered-off offline state',
    'Contact requested state',
    'Manual platform proof state',
    'location-evidence-last-known-stale',
    'device-status-powered-off',
    'device-status-contact-action-queued',
    'device-status-platform-proof-required',
    'powered-off-current-location-proof-forbidden | hosted-read-only-missing-device-proof',
    'os-lost-mode-api-proof-required | physical-device-proof-required',
    'output/tracking-plan-proof/29-missing-device-mode/proof.json',
    'Hosted missing-device rendering only; current location runtime, powered-off tracking, remote sync, provider delivery, physical-device proof, OS lost-mode APIs, authority, production workers, and product readiness remain unclaimed.',
  ]);
}

function assertAccessibilityRetentionAndEvidenceValues(actualValues: readonly string[]): void {
  assertContainsAll(actualValues, [
    'Retention window setting',
    'Delete-after-alert setting',
    'Parent export setting',
    'Remote sync disabled setting',
    'Remote AI disabled setting',
    'settings-read-model-ready',
    'tracking-family-dashboard-evidence-active-summary',
    'tracking-hosted-expected-place-event',
    'location-evidence-hosted-1 | location-evidence-hosted-2',
    'tracking-retention-settings-evidence-window',
    'tracking-retention-settings-evidence-remote-ai-disabled',
    'Retention local service write result',
    PortalTrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
    PortalTrackingRetentionSettingsWriteDefaults.CommandId,
    PortalTrackingRetentionSettingsWriteDefaults.WriterIntentRef,
    PortalTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
    PortalTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef,
    'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json',
    PortalTrackingRetentionSettingsWriteDefaults.MutationProofRef,
    'Portal command/result rendering proves local service mutation execution, local durable settings persistence, and local state revision only; product-ready writable settings, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
    'read-only evidence drawer',
    'Display-only evidence drill-in; policy evaluation, action dispatch, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
    'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-evidence-drawer-hosted-ui-proof.json',
  ]);
}

function assertAccessibilityChildAndPlatformValues(actualValues: readonly string[]): void {
  assertContainsAll(actualValues, [
    'Need help',
    'Share current location',
    'Call parent',
    'Child-device delivery not proved',
    'Tracking request disclosed',
    'Safe response visible',
    'Help response visible',
    'Location share asks consent',
    'Hosted proof only, not child-agent delivery',
    'Android background location manual required',
    'Web child agent location unavailable',
    'Authority hard-control proof required',
    'platform-unsupported',
    'real-device-required',
    'authority-required',
    'No product claim',
  ]);
}

function assertAccessibilityBasics(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  expect(summary.hasNamedRegion).toBe(true);
  expect(summary.unlabeledButtons).toBe(0);
  assertHostedTrackingLayoutBoxes(summary.layoutBoxes);
}

function assertContainsAll(actualValues: readonly string[], expectedValues: readonly string[]): void {
  for (const expectedValue of expectedValues) {
    expect(actualValues).toContain(expectedValue);
  }
}

function assertHostedTrackingLayoutBoxes(layoutBoxes: readonly HostedTrackingLayoutBox[]): void {
  expect(layoutBoxes.map((box) => box.proofId).sort()).toEqual(
    [
      'child-check-in',
      'child-runtime-ui',
      'family-dashboard-rollup',
      'missing-device-ui',
      'notification-parent-surface-history-ui',
      'parent-action-readiness-ui',
      'report-policy-consumer-ui',
      'report-export-ui',
      'retention-settings-ui',
      'service-backed-citation-detail',
      'service-backed-evidence-drawer',
    ].sort()
  );
  for (const layoutBox of layoutBoxes) {
    expect(layoutBox.width).toBeGreaterThan(0);
    expect(layoutBox.height).toBeGreaterThan(0);
  }
  for (const [index, layoutBox] of layoutBoxes.entries()) {
    for (const otherBox of layoutBoxes.slice(index + 1)) {
      expect(layoutBoxesOverlap(layoutBox, otherBox)).toBe(false);
    }
  }
}

function layoutBoxesOverlap(first: HostedTrackingLayoutBox, second: HostedTrackingLayoutBox): boolean {
  return (
    first.left < second.right && first.right > second.left && first.top < second.bottom && first.bottom > second.top
  );
}

function hostedTrackingAssertions(): readonly string[] {
  return [
    'named-region',
    'visible-heading',
    'enabled-refresh-button',
    'service-backed-row-citation-visible',
    'service-data-coverage-visible',
    'family-dashboard-rollup-visible',
    'family-dashboard-rollup-screenshot',
    'report-policy-consumer-visible',
    'report-policy-consumer-screenshot',
    'report-export-read-model-visible',
    'report-export-read-model-screenshot',
    'notification-parent-surface-history-visible',
    'notification-parent-surface-history-screenshot',
    'parent-action-readiness-visible',
    'parent-action-readiness-screenshot',
    'missing-device-visible',
    'missing-device-screenshot',
    'service-backed-evidence-drawer-visible',
    'service-backed-evidence-drawer-screenshot',
    'service-backed-citation-detail-visible',
    'service-backed-citation-detail-screenshot',
    'retention-settings-read-model-visible',
    'retention-settings-local-write-clicked',
    'retention-settings-local-write-result-visible',
    'retention-settings-screenshot',
    'manual-required-visible',
    'physical-device-required-visible',
    'no-product-claim-visible',
    'child-check-in-copy-visible',
    'child-check-in-actions-visible',
    'child-device-delivery-not-claimed',
    'child-runtime-disclosure-visible',
    'child-runtime-safe-help-response-visible',
    'child-runtime-location-share-consent-visible',
    'child-runtime-hosted-only-boundary-visible',
    'unsupported-manual-platform-render-state-visible',
    'unsupported-manual-platform-screenshot',
    'no-unlabeled-buttons',
    'no-proof-card-overlap',
    'desktop-screenshot',
    'child-check-in-screenshot',
    'child-runtime-ui-screenshot',
    'parent-overview-shell-screenshot',
    'parent-devices-shell-screenshot',
    'mobile-screenshot',
  ];
}
