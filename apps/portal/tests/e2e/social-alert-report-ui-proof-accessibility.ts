import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, type Page } from '@playwright/test';

const repoRoot = path.resolve(process.cwd(), '..', '..');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'social-alert-report-intent-ui-proof',
  'accessibility-summary.json'
);

export async function collectAccessibilitySummary(page: Page): Promise<{
  readonly hasNamedRegion: boolean;
  readonly headings: readonly string[];
  readonly labels: readonly string[];
  readonly values: readonly string[];
  readonly buttons: readonly { readonly text: string; readonly disabled: boolean }[];
  readonly unlabeledButtons: number;
}> {
  return page.evaluate(() => {
    const region = document.querySelector('[aria-label="Social alerts and reports"]');
    const text = (element: Element): string => element.textContent?.trim() ?? '';
    const buttons = Array.from(region?.querySelectorAll('button') ?? []).map((element) => ({
      text: text(element),
      disabled: element.hasAttribute('disabled'),
    }));
    return {
      hasNamedRegion: region !== null,
      headings: Array.from(region?.querySelectorAll('h2') ?? []).map(text),
      labels: Array.from(region?.querySelectorAll('dt') ?? []).map(text),
      values: Array.from(region?.querySelectorAll('dd') ?? []).map(text),
      buttons,
      unlabeledButtons: buttons.filter((button) => button.text.length === 0).length,
    };
  });
}

export async function writeAccessibilitySummary(
  summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>
): Promise<void> {
  assertAccessibilitySummary(summary);

  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/browser',
        assertions: [
          'named-social-alert-report-region',
          'zero-row-before-command-visible',
          'service-backed-four-row-summary-visible',
          'high-risk-local-outbox-row-visible',
          'manual-required-row-visible',
          'provider-status-manual-required-row-visible',
          'parent-notification-delivery-readiness-command-visible',
          'parent-report-status-ready-row-visible',
          'parent-notification-manual-required-row-visible',
          'parent-notification-unavailable-row-visible',
          'parent-notification-delivery-non-claims-visible',
          'parent-surface-status-command-visible',
          'parent-surface-manual-row-visible',
          'parent-surface-unavailable-row-visible',
          'parent-surface-non-claims-visible',
          'action-intent-stream-status-visible',
          'action-intent-zero-candidates-visible',
          'receipt-stream-status-visible',
          'receipt-ingestion-readiness-visible',
          'receipt-zero-provider-receipts-visible',
          'non-claim-copy-visible',
          'provider-report-notification-final-policy-enforcement-claims-not-visible',
          'action-intent-adapter-child-intervention-browser-mutation-enforcement-claims-not-visible',
          'receipt-ingestion-runtime-provider-delivery-enforcement-claims-not-visible',
          'no-unlabeled-buttons',
          'desktop-screenshot',
          'mobile-screenshot',
        ],
        summary,
        screenshots: {
          desktop: path
            .relative(
              repoRoot,
              path.join(
                repoRoot,
                'output',
                'browser-plan-proof',
                'social-alert-report-intent-ui-proof',
                '06-ui-snapshots',
                'social-alert-report-browser-route.png'
              )
            )
            .replace(/\\/gu, '/'),
          mobile: path
            .relative(
              repoRoot,
              path.join(
                repoRoot,
                'output',
                'browser-plan-proof',
                'social-alert-report-intent-ui-proof',
                '06-ui-snapshots',
                'social-alert-report-browser-route-mobile.png'
              )
            )
            .replace(/\\/gu, '/'),
        },
      },
      null,
      2
    )}\n`
  );
}

function assertAccessibilitySummary(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  expect(summary.hasNamedRegion).toBe(true);
  expect(summary.unlabeledButtons).toBe(0);
  for (const heading of expectedAccessibilityHeadings()) {
    expect(summary.headings).toContain(heading);
  }
  for (const label of ['Rows returned', 'Generated at', 'Capability', 'Product claim']) {
    expect(summary.labels).toContain(label);
  }
  for (const value of expectedAccessibilityValues()) {
    expect(summary.values).toContain(value);
  }
}

function expectedAccessibilityHeadings(): readonly string[] {
  return [
    'Social alerts and reports',
    '4 social alert/report rows',
    'High-risk social alert intent',
    'Manual alert/report proof required',
    'Provider status manual required',
    '3 parent notification readiness rows',
    'Parent report status ready',
    'Parent notification manual proof required',
    'Parent notification delivery unavailable',
    '3 parent surface rows',
    'Parent surface manual action required',
    'Parent surface unavailable',
    'Browser action-intent stream status',
    'Social provider receipt stream status',
    'Social provider receipt ingestion readiness',
  ];
}

function expectedAccessibilityValues(): readonly string[] {
  return [
    '4',
    '3',
    'parent-report-status-ready',
    'parent-owned-report-ready',
    'manual-action-required',
    'unavailable-visible',
    'preference-setup-required',
    '0 action candidates',
    '0 provider receipts observed',
    'local-outbox-only',
    'manual-required',
    'not-observed',
  ];
}
