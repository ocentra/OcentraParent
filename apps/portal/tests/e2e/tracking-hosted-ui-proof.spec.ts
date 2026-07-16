import { expect, test } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';
import {
  assertHostedPolicyTrackingRoute,
  captureHostedTrackingScreenshots,
  captureParentPortalShellScreenshots,
} from './tracking-hosted-ui-proof-assertions';
import { collectAccessibilitySummary, writeAccessibilitySummary } from './tracking-hosted-ui-proof-accessibility';

test.setTimeout(120_000);

test('proof panels route renders hosted policy tracking proof without product claims', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await assertHostedPolicyTrackingRoute(page);
  await captureHostedTrackingScreenshots(page);
  const accessibilitySummary = await collectAccessibilitySummary(page);
  const parentPortalShellSummary = await captureParentPortalShellScreenshots(page);
  await writeAccessibilitySummary(accessibilitySummary, parentPortalShellSummary);

  expect(browserFailures).toEqual([]);
});
