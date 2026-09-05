import { mkdir } from 'node:fs/promises';
import path from 'node:path';
import { expect, type Locator, type Page } from '@playwright/test';
import { hostedTrackingScreenshotPaths } from './tracking-hosted-ui-proof-accessibility-screenshots';

const portalShellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const screenshotPaths = hostedTrackingScreenshotPaths(repoRoot);

const unavailableBody = 'Tracking is not connected to the local service. No child location or activity is being shown.';
const unavailableCardBody =
  'Retry status to load the Rust-owned tracking read model, including device, custody, freshness, and evidence rows.';

const forbiddenProofFixtureLabels = [
  'tracking-hosted-expected-place-event',
  'ui-fixture',
  'Family dashboard tracking rollup',
  'Notification history intent UI',
  'Parent action readiness UI',
  'Child check-in request',
  'Rust child-agent tracking read model is not wired yet.',
] as const;

const forbiddenProductFixtureLabels = ['tracking-hosted-expected-place-event', 'ui-fixture', 'Proof tier'] as const;

export async function assertHostedPolicyTrackingRoute(page: Page): Promise<Locator> {
  await page.goto('/#/proof-panels');
  await page.getByRole('button', { name: 'Tracking status', exact: true }).click();
  const region = trackingRegion(page, 'proof', 'unavailable');
  await assertUnavailableTrackingRegion(region);
  await expect(page.getByRole('heading', { name: 'PROOF PANELS parent controls', exact: true })).toBeVisible();
  return region;
}

export async function assertHostedProductTrackingRoute(
  page: Page,
  layout: 'desktop' | 'mobile' = 'desktop'
): Promise<Locator> {
  await page.goto('/#/policy-tracking');
  const region = trackingRegion(page, 'product');
  await assertProductTrackingBoundary(region);
  const sidePane = page.locator('.parent-portal-study-side-pane');
  if (layout === 'desktop') {
    await expect(sidePane).toBeVisible();
  } else {
    await expect(sidePane).toHaveCount(0);
  }
  await expect(page.locator('.parent-portal-visually-hidden')).toHaveCount(0);
  await expect(page.locator('.parent-portal-study-main-board')).toHaveCount(0);
  return region;
}

export async function reconnectUnavailableTrackingRegion(region: Locator): Promise<void> {
  const retryStatus = region.getByRole('button', { name: 'Retry status', exact: true });
  await expect(retryStatus).toBeEnabled();
  await retryStatus.click();
  await assertUnavailableTrackingRegion(region);
}

export async function reconnectProductTrackingBoundary(region: Locator): Promise<void> {
  const retryStatus = region.getByRole('button', { name: 'Retry status', exact: true });
  await expect(retryStatus).toBeEnabled();
  await retryStatus.click();
  await assertProductTrackingBoundary(region);
}

export async function captureHostedTrackingScreenshots(page: Page): Promise<void> {
  await mkdir(path.dirname(absoluteScreenshot(screenshotPaths.proofUnavailable)), { recursive: true });
  await page.setViewportSize({ width: 1280, height: 960 });
  await assertHostedPolicyTrackingRoute(page);
  await page.screenshot({ fullPage: true, path: absoluteScreenshot(screenshotPaths.proofUnavailable) });

  await assertHostedProductTrackingRoute(page);
  await page.screenshot({ fullPage: true, path: absoluteScreenshot(screenshotPaths.productUnavailable) });

  await page.setViewportSize({ width: 390, height: 844 });
  await assertHostedProductTrackingRoute(page, 'mobile');
  await page.screenshot({ fullPage: true, path: absoluteScreenshot(screenshotPaths.productUnavailableMobile) });
}

export async function captureParentPortalShellScreenshots(page: Page): Promise<{
  readonly routes: readonly {
    readonly route: string;
    readonly screenshot: string;
    readonly assertions: readonly string[];
  }[];
}> {
  await page.setViewportSize({ width: 1280, height: 960 });
  await assertAndCaptureParentPortalShellRoute(page, {
    route: '#/overview',
    screenshot: screenshotPaths.parentOverview,
    expectedSvgText: ['Current device state', 'WHAT PARENTS CONTROL', 'DATA CUSTODY'],
  });
  await assertAndCaptureParentPortalShellRoute(page, {
    route: '#/devices',
    screenshot: screenshotPaths.parentDevices,
    expectedSvgText: ['SELECTED DEVICE CONTEXT', 'SELECTED DEVICE', 'SOURCE', 'CONTROL'],
  });
  return {
    routes: [
      {
        route: '#/overview',
        screenshot: screenshotPaths.parentOverview,
        assertions: ['parent-overview-shell-visible', 'tracking-route-panel-not-mounted-on-overview'],
      },
      {
        route: '#/devices',
        screenshot: screenshotPaths.parentDevices,
        assertions: ['parent-devices-shell-visible', 'tracking-route-panel-not-mounted-on-devices'],
      },
    ],
  };
}

async function assertUnavailableTrackingRegion(region: Locator): Promise<void> {
  await expect(region).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(region).toHaveAttribute('data-ocentra-tracking-route-state', 'unavailable');
  await expect(region.getByRole('heading', { name: 'Tracking status unavailable' })).toBeVisible();
  await expect(region.getByRole('heading', { name: 'Service read model' })).toBeVisible();
  await expect(region.getByRole('heading', { name: 'Location and devices' })).toBeVisible();
  await expect(region.getByRole('heading', { name: 'Tracking controls' })).toBeVisible();
  await expect(region.getByText(unavailableBody, { exact: true })).toBeVisible();
  await expect(region.getByText(unavailableCardBody, { exact: true })).toBeVisible();
  await expect(region.getByRole('button', { name: 'Retry status', exact: true })).toBeEnabled();
  for (const label of forbiddenProofFixtureLabels) {
    await expect(region.getByText(label, { exact: false })).toHaveCount(0);
  }
  await expect(region.locator('[data-ocentra-tracking-proof]')).toHaveCount(0);
}

async function assertUnavailableTrackingReport(region: Locator): Promise<void> {
  await expect(region).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(region).toHaveAttribute('data-ocentra-tracking-surface', 'product');
  await expect(region).not.toHaveAttribute('data-ocentra-tracking-route-state');
  await expect(region.getByRole('heading', { name: 'Tracking status', exact: true })).toBeVisible();
  await expect(
    region.getByText(
      'Current child tracking history, service coverage, custody, and honest connection gaps from the local Rust service.',
      { exact: true }
    )
  ).toBeVisible();
  await expect(region.getByRole('button', { name: 'Retry status', exact: true })).toBeEnabled();
  await assertTrackingReportCard(region, 'Tracking live summary', {
    labels: ['Status', 'Rows returned', 'Last observed', 'Event ID', 'Capability', 'Custody', 'Evidence refs'],
    values: ['Unavailable', '0', 'Not reported', 'Not reported', 'Unavailable', 'Unavailable', 'Not reported'],
  });
  await assertTrackingReportCard(region, 'Tracking service data coverage', {
    labels: ['Status', 'Rows returned', 'Deleted evidence', 'Activity kinds', 'Devices', 'Capability'],
    values: ['Unavailable', '0', '0', 'Unavailable', 'Unavailable', 'Unavailable'],
  });
  await assertTrackingReportCard(region, 'Current child status', {
    labels: ['Status', 'Device', 'Child or place', 'Last observed', 'Reason'],
    values: ['Unavailable', 'Not reported', 'Not reported', 'Not reported', 'No active tracking row is available.'],
  });
  await assertTrackingReportCard(region, 'Last known location', {
    labels: ['Status', 'Last observed', 'Location label', 'Accuracy', 'Map coordinates', 'Permission', 'Reason'],
    values: [
      'Unavailable',
      'Not reported',
      'Not reported',
      'Not supplied',
      'Not supplied',
      'Not supplied',
      'No active location-observed row is supplied by the service read model.',
    ],
  });
  await assertTrackingReportCard(region, 'Child tracking surface', {
    labels: [
      'Status',
      'Recorded check-ins',
      'Authenticated delivery',
      'Location consent',
      'Safe or help response',
      'Reason',
    ],
    values: [
      'Unavailable',
      'Not reported',
      'Not supplied',
      'Not supplied',
      'Not supplied',
      'The current service read model contains history, not an authenticated child-runtime status or action channel.',
    ],
  });
  await assertTrackingReportCard(region, 'Tracking controls', {
    labels: [
      'Status',
      'Exception editor',
      'Child check-in',
      'Temporary live tracking',
      'Missing-device action',
      'Notification preferences',
    ],
    values: [
      'Unavailable',
      'No owner-authorized mutation input',
      'No authenticated delivery input',
      'No durable live-session input',
      'No owner-authorized decision input',
      'No writable preference input',
    ],
  });
  await expect(region.getByRole('heading', { name: 'Tracking boundary', exact: true })).toBeVisible();
  for (const label of forbiddenProductFixtureLabels) {
    await expect(region.getByText(label, { exact: false })).toHaveCount(0);
  }
  await expect(region.locator('[data-ocentra-tracking-proof]')).toHaveCount(0);
}

async function assertProductTrackingBoundary(region: Locator): Promise<void> {
  await expect(region).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  const routeState = await region.getAttribute('data-ocentra-tracking-route-state');
  if (routeState === 'unavailable') {
    await assertUnavailableTrackingRegion(region);
    return;
  }
  expect(routeState).toBeNull();
  await assertUnavailableTrackingReport(region);
}

async function assertTrackingReportCard(
  region: Locator,
  title: string,
  expected: { readonly labels: readonly string[]; readonly values: readonly string[] }
): Promise<void> {
  const card = region.locator('article').filter({ hasText: title });
  await expect(card.locator('dt')).toHaveText(expected.labels);
  await expect(card.locator('dd')).toHaveText(expected.values);
}

async function assertAndCaptureParentPortalShellRoute(
  page: Page,
  route: {
    readonly route: string;
    readonly screenshot: string;
    readonly expectedSvgText: readonly string[];
  }
): Promise<void> {
  await page.goto(route.route);
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  for (const expectedText of route.expectedSvgText) {
    await expect(surface.locator('text').filter({ hasText: expectedText }).first()).toBeVisible();
  }
  await expect(page.locator('[data-ocentra-tracking-surface]')).toHaveCount(0);
  await page.screenshot({ fullPage: true, path: absoluteScreenshot(route.screenshot) });
}

function trackingRegion(page: Page, surface: 'product' | 'proof', state?: 'unavailable'): Locator {
  const stateSelector = state === undefined ? '' : `[data-ocentra-tracking-route-state="${state}"]`;
  return page.locator(`section[data-ocentra-tracking-surface="${surface}"]${stateSelector}`);
}

function absoluteScreenshot(relativePath: string): string {
  return path.join(repoRoot, relativePath);
}
