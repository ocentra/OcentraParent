import { expect, test } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.setTimeout(120_000);

test('remote access has a dedicated destination and remains fail closed without an authenticated session', async ({
  page,
}) => {
  const browserFailures = collectBrowserFailures(page);

  await page.goto('/#/remote-access');

  await expect(page.getByRole('heading', { name: 'Remote access unavailable' })).toBeVisible({ timeout: 90_000 });
  await expect(page.getByRole('button', { exact: true, name: 'Open Start Here' })).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Open devices' })).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Review remote screen policy' })).toBeVisible();
  await expect(page.getByRole('heading', { exact: true, name: 'Not reported' })).toHaveCount(2);
  await expect(page.getByRole('heading', { exact: true, name: 'Manual required' })).toBeVisible();
  await expect(page.locator('[data-ocentra-remote-access-state] article')).toHaveCount(3);
  await expect(page.getByText('REMOTE ACCESS NOT AVAILABLE')).toHaveCount(0);
  await expect(page.getByText('REMOTE TARGET NOT REPORTED')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Request view' })).toHaveCount(0);
  expect(browserFailures).toEqual([]);
});

test('remote screen policy does not render fixture modes or actions as current product state', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await page.goto('/#/policy-remote-screen');

  await expect(page.getByRole('heading', { name: 'Remote screen controls unavailable' })).toBeVisible({
    timeout: 90_000,
  });
  await expect(page.getByRole('button', { exact: true, name: 'Open screen analysis' })).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Open devices' })).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Open Start Here' })).toBeVisible();
  await expect(page.getByText('REMOTE SCREEN POLICY NOT AVAILABLE')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Request view' })).toHaveCount(0);
  await expect(page.getByText('Ask first', { exact: true })).toHaveCount(0);
  await expect(page.getByText('Live view', { exact: true })).toHaveCount(0);
  expect(browserFailures).toEqual([]);
});
