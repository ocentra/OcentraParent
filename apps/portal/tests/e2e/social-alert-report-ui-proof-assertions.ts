import { expect, type Page } from '@playwright/test';

const portalShellReadyTimeoutMs = 90_000;

export async function assertSocialAlertReportRoute(page: Page): Promise<void> {
  await page.goto('/#/browser?agent.browser.social-alert-report.read-model.reported');
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(alertRegion.getByRole('heading', { name: 'Social alerts and reports' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: '0 social alert/report rows' })).toBeVisible();
  await expect(
    alertRegion.getByRole('heading', { name: 'No social alert/report read model has been reported yet.' })
  ).toBeVisible();
  await expect(alertRegion.getByText('unavailable', { exact: true }).first()).toBeVisible();
  await expect(alertRegion.getByText('not reported', { exact: true }).first()).toBeVisible();
  await expect(alertRegion.getByText('Rendered parent alert/report intent surface only').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:provider delivery complete|report delivered|notification ui delivered)/iu);
  expect(routeText ?? '').not.toMatch(/(?:final policy execution proved|enforcement active)/iu);
}

export async function requestSocialParentNotificationDeliveryReadModel(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await alertRegion.getByRole('button', { name: 'Social parent notification delivery readiness' }).click();
  await expect(alertRegion.getByRole('heading', { name: '3 parent notification readiness rows' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

export async function requestSocialAlertReportParentSurfaceReadModel(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await alertRegion.getByRole('button', { name: 'Social parent surface status' }).click();
  await expect(alertRegion.getByRole('heading', { name: '3 parent surface rows' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

export async function requestSocialAlertReportReadModel(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await alertRegion.getByRole('button', { name: 'Social alerts and reports' }).click();
  await expect(alertRegion.getByRole('heading', { name: '4 social alert/report rows' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

export async function assertSocialParentNotificationDeliveryRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(
    alertRegion.getByRole('button', { name: 'Social parent notification delivery readiness' })
  ).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Parent report status ready' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Parent notification manual proof required' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Parent notification delivery unavailable' })).toBeVisible();
  await expect(alertRegion.getByText('parent-report-status-ready').first()).toBeVisible();
  await expect(alertRegion.getByText('parent-owned-report-ready').first()).toBeVisible();
  await expect(alertRegion.getByText('external-report-delivery-runtime-unavailable').first()).toBeVisible();
  await expect(alertRegion.getByText('manual-parent-notification-ui-runtime-proof-required').first()).toBeVisible();
  await expect(alertRegion.getByText('Parent report readiness projection only')).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:parent notification ui delivered|provider delivery complete)/iu);
  expect(routeText ?? '').not.toMatch(/(?:external runtime report delivered|final policy execution proved)/iu);
  expect(routeText ?? '').not.toMatch(/(?:enforcement active|provider receipt ingested)/iu);
}

export async function assertSocialAlertReportParentSurfaceRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion.getByRole('button', { name: 'Social parent surface status' })).toBeVisible();
  await expect(
    alertRegion.getByRole('heading', { name: 'Parent surface manual action required' }).first()
  ).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Parent surface unavailable' })).toBeVisible();
  await expect(alertRegion.getByText('manual-action-required').first()).toBeVisible();
  await expect(alertRegion.getByText('unavailable-visible').first()).toBeVisible();
  await expect(alertRegion.getByText('preference-setup-required').first()).toBeVisible();
  await expect(alertRegion.getByText('preference-disabled-visible').first()).toBeVisible();
  await expect(alertRegion.getByText('manual-parent-surface-runtime-proof-required').first()).toBeVisible();
  await expect(alertRegion.getByText('Parent-surface status projection only')).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:notification ui delivered|provider delivery complete)/iu);
  expect(routeText ?? '').not.toMatch(/(?:receipt ingested|final policy execution proved|enforcement active)/iu);
}

export async function assertServiceBackedSocialAlertReportRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion.getByRole('heading', { name: 'High-risk social alert intent' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Manual alert/report proof required' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Provider status manual required' }).first()).toBeVisible();
  await expect(alertRegion.getByText('local-outbox-only').first()).toBeVisible();
  await expect(alertRegion.getByText('manual-required').first()).toBeVisible();
  await expect(alertRegion.getByText('provider-adapter-required').first()).toBeVisible();
  await expect(
    alertRegion.getByText('provider-adapter-required-social-alert-report-high-risk-service').first()
  ).toBeVisible();
  await expect(
    alertRegion.getByText('provider-credentials-required-social-alert-report-high-risk-service').first()
  ).toBeVisible();
  await expect(
    alertRegion.getByText('provider-smoke-proof-required-social-alert-report-high-risk-service').first()
  ).toBeVisible();
  await expect(alertRegion.getByText('not-observed').first()).toBeVisible();
  await expect(alertRegion.getByText('social-high-risk-signal').first()).toBeVisible();
  await expect(alertRegion.getByText('social-manual-review-required').first()).toBeVisible();
  await expect(alertRegion.getByText('evidence-social-route-gate').first()).toBeVisible();
  await expect(alertRegion.getByText('evidence-social-manual-gap').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:provider delivery complete|provider observed|receipt ingested)/iu);
  expect(routeText ?? '').not.toMatch(/(?:report delivered|notification ui delivered|enforcement active)/iu);
}

export async function assertBrowserActionIntentStatusRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion.getByRole('heading', { name: 'Browser action-intent stream status' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(alertRegion.getByText('0 action candidates').first()).toBeVisible();
  await expect(alertRegion.getByText('not-claimed').first()).toBeVisible();
  await expect(alertRegion.getByText('not-observed').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:adapter dispatch complete|browser mutation complete)/iu);
  expect(routeText ?? '').not.toMatch(/(?:child intervention executed|final policy execution proved)/iu);
  expect(routeText ?? '').not.toMatch(/(?:unmanaged exact url supported|enforcement active)/iu);
}

export async function assertBrowserReceiptStatusRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion.getByRole('heading', { name: 'Social provider receipt stream status' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(alertRegion.getByRole('heading', { name: 'Social provider receipt ingestion readiness' })).toBeVisible();
  await expect(alertRegion.getByText('0 provider receipts observed').first()).toBeVisible();
  await expect(alertRegion.getByText('not-observed').first()).toBeVisible();
  await expect(alertRegion.getByText('unavailable').first()).toBeVisible();
  await expect(alertRegion.getByText('not-claimed').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(
    /(?:provider delivery complete|provider receipt ingested|webhook runtime ready)/iu
  );
  expect(routeText ?? '').not.toMatch(/(?:browser mutation complete|child intervention executed|enforcement active)/iu);
}
