import { expect, test } from '@playwright/test';

test.describe('hosted parent web portal distribution', () => {
  test('distinguishes preview, staging, and production routes without rendering the host-bridge shell', async ({
    page,
  }) => {
    await page.goto('/preview');
    await expect(page.getByTestId('hosted-portal-distribution')).toBeVisible();
    await expect(page.getByTestId('hosted-environment-badge')).toContainText('Preview verification route');
    await expect(page.getByTestId('hosted-primary-action')).toContainText('Open parent portal preview');
    await expect(page.getByRole('heading', { exact: true, name: 'Device controls' })).toHaveCount(0);

    await page.goto('/staging');
    await expect(page.getByTestId('hosted-environment-badge')).toContainText('Staging verification route');
    await expect(page.getByTestId('hosted-primary-action')).toContainText('Open parent portal staging');

    await page.goto('/production');
    await expect(page.getByTestId('hosted-environment-badge')).toContainText('Production release route');
    await expect(page.getByTestId('hosted-primary-action')).toContainText('Open parent portal production');
  });

  test('rejects the wrong hosted route without exposing a child or setup surface', async ({ page }) => {
    await page.goto('/child-runtime');

    await expect(page.getByTestId('hosted-route-blocker')).toContainText('Unsupported hosted parent portal route');
    await expect(page.getByTestId('hosted-route-card')).toContainText('/preview, /staging, /production');
    await expect(page.getByRole('heading', { exact: true, name: 'Device controls' })).toHaveCount(0);
    await expect(page.getByText('Current device state')).toHaveCount(0);
  });

  test('keeps parent-only controls hidden when auth is missing', async ({ page }) => {
    await page.goto('/production?auth=missing');

    await expect(page.getByTestId('hosted-auth-required')).toContainText('Parent sign-in is required');
    await expect(page.getByTestId('hosted-primary-action')).toContainText('Parent release action blocked');
    await expect(page.getByTestId('hosted-primary-action')).toHaveAttribute('aria-disabled', 'true');
  });

  test('marks stale cache honestly and blocks fresh install claims', async ({ page }) => {
    await page.goto('/staging?cache=stale&cacheAgeMinutes=180');

    await expect(page.getByTestId('hosted-cache-stale')).toContainText('Cached shell age is 180 minutes');
    await expect(page.getByTestId('hosted-cache-stale')).toContainText('Fresh install or release claims are blocked');
    await expect(page.getByTestId('hosted-primary-action')).toContainText('Parent release action blocked');
  });

  test('blocks preview routes from presenting themselves as production releases', async ({ page }) => {
    await page.goto('/preview?release=production');

    await expect(page.getByTestId('hosted-production-claim-blocked')).toContainText('Production release claim blocked');
    await expect(page.getByTestId('hosted-primary-action')).toContainText('Parent release action blocked');
  });
});
