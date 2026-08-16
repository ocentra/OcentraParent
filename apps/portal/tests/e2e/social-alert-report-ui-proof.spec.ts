import { expect, test } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';
import {
  requestSocialAlertReportParentSurfaceReadModel,
  requestSocialAlertReportReadModel,
  requestSocialParentNotificationDeliveryReadModel,
  assertBrowserActionIntentStatusRows,
  assertBrowserReceiptStatusRows,
  assertServiceBackedSocialAlertReportRows,
  assertSocialAlertReportRoute,
  assertSocialAlertReportParentSurfaceRows,
  assertSocialParentNotificationDeliveryRows,
} from './social-alert-report-ui-proof-assertions';
import { captureSocialAlertReportScreenshots } from './social-alert-report-ui-proof-capture';
import { collectAccessibilitySummary, writeAccessibilitySummary } from './social-alert-report-ui-proof-accessibility';

test.setTimeout(120_000);

if (process.env['SOCIAL_ALERT_REPORT_UI_PROOF'] === '1') {
  test('browser route renders service-backed social alert and report intent rows', async ({ page }) => {
    const browserFailures = collectBrowserFailures(page);

    await assertSocialAlertReportRoute(page);
    await requestSocialAlertReportReadModel(page);
    await assertServiceBackedSocialAlertReportRows(page);
    await requestSocialParentNotificationDeliveryReadModel(page);
    await assertSocialParentNotificationDeliveryRows(page);
    await requestSocialAlertReportParentSurfaceReadModel(page);
    await assertSocialAlertReportParentSurfaceRows(page);
    await assertBrowserActionIntentStatusRows(page);
    await assertBrowserReceiptStatusRows(page);
    await captureSocialAlertReportScreenshots(page);
    await writeAccessibilitySummary(await collectAccessibilitySummary(page));

    expect(browserFailures).toEqual([]);
  });
}
