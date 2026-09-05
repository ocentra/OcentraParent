import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, type Page } from '@playwright/test';
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

type HostedTrackingAccessibilitySummary = {
  readonly ariaLabel: string | null;
  readonly routeState: string | null;
  readonly surface: string | null;
  readonly headings: readonly string[];
  readonly paragraphs: readonly string[];
  readonly buttons: readonly { readonly text: string; readonly disabled: boolean }[];
  readonly fixtureMarkerCount: number;
  readonly proofCardCount: number;
  readonly unlabeledButtons: number;
};

export async function collectAccessibilitySummary(page: Page): Promise<HostedTrackingAccessibilitySummary> {
  return page.evaluate(() => {
    const region = document.querySelector('[data-ocentra-tracking-surface="proof"]');
    const queryAll = (selector: string): Element[] =>
      region === null ? [] : Array.from(region.querySelectorAll(selector));
    const text = (element: Element): string => element.textContent?.trim() ?? '';
    const attribute = (name: string): string | null => (region === null ? null : region.getAttribute(name));
    const content = (): string => (region === null ? '' : (region.textContent ?? ''));
    const buttons = queryAll('button').map((element) => ({
      text: text(element),
      disabled: element.hasAttribute('disabled'),
    }));
    const fixtureTokens = [
      'tracking-hosted-expected-place-event',
      'ui-fixture',
      'Family dashboard tracking rollup',
      'Child check-in request',
    ];
    return {
      ariaLabel: attribute('aria-label'),
      routeState: attribute('data-ocentra-tracking-route-state'),
      surface: attribute('data-ocentra-tracking-surface'),
      headings: queryAll('h2').map(text),
      paragraphs: queryAll('p').map(text),
      buttons,
      fixtureMarkerCount: fixtureTokens.filter((token) => content().includes(token)).length,
      proofCardCount: queryAll('[data-ocentra-tracking-proof]').length,
      unlabeledButtons: buttons.filter((button) => button.text.length === 0).length,
    };
  });
}

export async function writeAccessibilitySummary(
  summary: HostedTrackingAccessibilitySummary,
  parentPortalShellSummary: ParentPortalShellSummary
): Promise<void> {
  assertAccessibilitySummary(summary);
  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/proof-panels',
        assertions: [
          'named-tracking-region',
          'unavailable-state-visible',
          'reconnect-action-labeled',
          'no-fixture-markers',
          'no-proof-only-cards',
        ],
        summary,
        screenshots: hostedTrackingScreenshotPaths(repoRoot),
        parentPortalShellSummary,
      },
      null,
      2
    )}\n`
  );
}

function assertAccessibilitySummary(summary: HostedTrackingAccessibilitySummary): void {
  expect(summary.ariaLabel).toBe('Tracking status');
  expect(summary.routeState).toBe('unavailable');
  expect(summary.surface).toBe('proof');
  expect(summary.headings).toEqual([
    'Tracking status unavailable',
    'Service read model',
    'Location and devices',
    'Tracking controls',
  ]);
  expect(summary.paragraphs).toEqual([
    'Service read model',
    'Tracking is not connected to the local service. No child location or activity is being shown.',
    'Retry status to load the Rust-owned tracking read model, including device, custody, freshness, and evidence rows.',
    'No location, accuracy, device freshness, or child status is displayed without a valid service row.',
    'Check-in, exception, live-tracking, missing-device, and notification actions stay unavailable until the service supplies owner-authorized inputs.',
  ]);
  expect(summary.buttons).toEqual([{ text: 'Retry status', disabled: false }]);
  expect(summary.fixtureMarkerCount).toBe(0);
  expect(summary.proofCardCount).toBe(0);
  expect(summary.unlabeledButtons).toBe(0);
}
