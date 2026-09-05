import { expect, test } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';
import {
  assertHostedPolicyTrackingRoute,
  assertHostedProductTrackingRoute,
  captureHostedTrackingScreenshots,
  captureParentPortalShellScreenshots,
  reconnectProductTrackingBoundary,
  reconnectUnavailableTrackingRegion,
} from './tracking-hosted-ui-proof-assertions';
import { collectAccessibilitySummary, writeAccessibilitySummary } from './tracking-hosted-ui-proof-accessibility';

test.setTimeout(120_000);

const expectedUnavailableTransportFailure = 'Failed to load resource: net::ERR_CONNECTION_REFUSED';

test('proof panels route renders the current fail-closed tracking boundary without fixtures', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  const trackingRegion = await assertHostedPolicyTrackingRoute(page);
  const accessibilitySummary = await collectAccessibilitySummary(page);
  await reconnectUnavailableTrackingRegion(trackingRegion);
  await captureHostedTrackingScreenshots(page);
  const parentPortalShellSummary = await captureParentPortalShellScreenshots(page);
  await writeAccessibilitySummary(accessibilitySummary, parentPortalShellSummary);

  expectNoUnexpectedBrowserFailures(browserFailures);
});

test('policy tracking route renders the current Rust unavailable report without fabricating rows', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  const trackingRegion = await assertHostedProductTrackingRoute(page);
  await reconnectProductTrackingBoundary(trackingRegion);
  expectNoUnexpectedBrowserFailures(browserFailures);
});

function expectNoUnexpectedBrowserFailures(browserFailures: readonly string[]): void {
  expect(browserFailures.filter((failure) => failure !== expectedUnavailableTransportFailure)).toEqual([]);
}
