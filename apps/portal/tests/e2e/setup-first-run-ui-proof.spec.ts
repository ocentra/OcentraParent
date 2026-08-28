import { expect, test } from '@playwright/test';

const portalShellReadyTimeoutMs = 90_000;

test('start route renders an honest setup boundary panel without invented readiness flow', async ({ page }) => {
  await page.goto('/#/start');

  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  const setupRegion = page.getByLabel('Setup-first-run boundary status');
  await expect(setupRegion).toBeVisible();
  await expect(setupRegion.getByRole('heading', { name: 'Current boundary status' })).toBeVisible();
  await expect(setupRegion.getByRole('heading', { name: 'What is real now' })).toBeVisible();
  await expect(setupRegion.getByRole('heading', { name: 'What is missing' })).toBeVisible();
  await expect(setupRegion.getByRole('heading', { name: 'Where it belongs' })).toBeVisible();
  await expect(
    setupRegion.getByText(
      'The Start route exists, but live setup-first-run runtime state is not yet wired into the Rust parent snapshot.'
    )
  ).toBeVisible();
  await expect(setupRegion.getByText('unavailable', { exact: true })).toBeVisible();
  await expect(setupRegion.getByText('Start route is visible in the portal shell')).toBeVisible();
  await expect(setupRegion.getByText('Host bridge snapshot reaches TS presentation')).toBeVisible();
  await expect(setupRegion.getByText('Route-contract projection only')).toBeVisible();
  await expect(setupRegion.getByText('Account/provider state')).toBeVisible();
  await expect(setupRegion.getByText('Pairing/trust state')).toBeVisible();
  await expect(setupRegion.getByText('Data-custody/readiness state')).toBeVisible();
  await expect(setupRegion.getByText('withheld until a live Rust snapshot exists')).toBeVisible();
  await expect(setupRegion.getByText('parent runtime + setup read model')).toBeVisible();
  await expect(setupRegion.getByText('presentation only')).toBeVisible();
  await expect(setupRegion.getByText('claim only what the live Rust snapshot can prove')).toBeVisible();
  await expect(setupRegion.getByText('manual-required', { exact: true }).first()).toBeVisible();
  await expect(
    setupRegion.getByText('observation only; ownership and trust remain unavailable', { exact: true })
  ).toBeVisible();
  await expect(setupRegion.getByText('Action planning', { exact: true })).toBeVisible();
  await expect(setupRegion.getByText('not invoked', { exact: true }).first()).toBeVisible();
  await expect(setupRegion).not.toContainText('onboarding complete');
});
