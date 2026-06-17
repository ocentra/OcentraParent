import { expect, test } from '@playwright/test';

const portalShellReadyTimeoutMs = 90_000;

test('start route renders the first-run setup proof surface with explicit blockers and no fake ready state', async ({
  page,
}) => {
  await page.goto('/#/start');

  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  const setupRegion = page.getByLabel('First-run family setup');
  await expect(setupRegion).toBeVisible();
  await expect(setupRegion.getByRole('heading', { name: 'State machine summary' })).toBeVisible();
  await expect(setupRegion.getByRole('heading', { name: 'Invite, role, and support visibility' })).toBeVisible();
  await expect(setupRegion.getByRole('heading', { name: 'Trust and session distinction' })).toBeVisible();
  await expect(setupRegion.getByText('welcome -> welcome-screen | readiness-report-absent')).toBeVisible();
  await expect(setupRegion.getByText('data-custody-status-screen')).toBeVisible();
  await expect(setupRegion.getByText('manual-required -> manual-required-screen | recovery-required | blocked')).toBeVisible();
  await expect(setupRegion.getByText('setup-blocked -> setup-blocked-screen | setup-complete withheld')).toBeVisible();
  await expect(setupRegion.getByText('setup-complete -> setup-complete-screen | ready')).toBeVisible();
  await expect(
    setupRegion.getByText('pending invite -> co-parent role stays distinct from parent-owner and child-device trust')
  ).toBeVisible();
  await expect(
    setupRegion.getByText('pending invite -> observer stays read-only and cannot inherit owner controls')
  ).toBeVisible();
  await expect(setupRegion.getByText('support-admin remains a separate audited support state | never parent-owner')).toBeVisible();
  await expect(setupRegion.getByText('Pairing:action-required:accepted | Pairing:complete:trusted')).toBeVisible();
  await expect(setupRegion.getByText('Account:action-required:recovery-required | manual-required-screen')).toBeVisible();
  await expect(setupRegion.getByText('Child service:action-required:revoked')).toBeVisible();
  await expect(setupRegion.getByText('Parent app:action-required:stale')).toBeVisible();
  await expect(setupRegion.getByText('Network reachability:action-required:direct-entry-required')).toBeVisible();
  await expect(setupRegion.getByText('physical-household-lan')).toBeVisible();
  await expect(setupRegion.getByText('parent-owned-storage')).toBeVisible();
  await expect(setupRegion.getByText('account-identity-family-plan')).toBeVisible();
  await expect(setupRegion.getByText('parent-desktop-runtime-package-plan')).toBeVisible();
  await expect(
    setupRegion.getByText('setup-complete requires overall readiness = ready after data-custody')
  ).toBeVisible();
});
