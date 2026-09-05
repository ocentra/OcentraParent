import { expect, test } from '@playwright/test';

const portalShellReadyTimeoutMs = 90_000;

test('start route renders the Rust-owned setup boundary without invented readiness', async ({ page }) => {
  await page.goto('/#/start');

  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  const setupRegion = page.getByRole('region', { exact: true, name: 'Setup-first-run boundary status' });
  await expect(setupRegion).toBeVisible();
  await expect(setupRegion).toHaveAttribute('data-ocentra-setup-proof', 'first-run-route');
  await expect(
    setupRegion.getByRole('heading', { exact: true, name: 'Setup-first-run boundary status' })
  ).toBeVisible();
  await expect(setupRegion).toContainText(
    'The Start route exists, but live setup-first-run runtime state is not yet wired into the Rust parent snapshot.'
  );
  await expect(setupRegion).toContainText('Runtime state');
  await expect(setupRegion).toContainText('unavailable');

  await expect(page.getByRole('button', { exact: true, name: 'Open Set Up Ocentra Parent' })).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Open Devices And Pairing' })).toBeVisible();
  await expect(setupRegion).not.toContainText('onboarding complete');
});

test('start navigation resets the guide dashboard to the setup topic', async ({ page }) => {
  await page.goto('/#/policy');

  await expect(
    page.getByRole('heading', { exact: true, name: 'Start with a family rule, override only when needed' })
  ).toBeVisible({ timeout: portalShellReadyTimeoutMs });

  await page.getByRole('button', { exact: true, name: 'Open START HERE' }).click();
  await expect(page).toHaveURL(/#\/start$/);
  await expect(page.getByRole('button', { exact: true, name: 'Open Set Up Ocentra Parent' })).toHaveAttribute(
    'aria-pressed',
    'true'
  );
  await expect(page.getByRole('button', { exact: true, name: 'Open Rules' })).toHaveAttribute('aria-pressed', 'false');
});
