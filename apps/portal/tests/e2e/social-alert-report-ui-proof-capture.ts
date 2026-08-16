import { mkdir } from 'node:fs/promises';
import path from 'node:path';
import { expect, type Page } from '@playwright/test';

const repoRoot = path.resolve(process.cwd(), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'browser-plan-proof', 'social-alert-report-intent-ui-proof');
const screenshotDir = path.join(proofRoot, '06-ui-snapshots');
const desktopScreenshotPath = path.join(screenshotDir, 'social-alert-report-browser-route.png');
const mobileScreenshotPath = path.join(screenshotDir, 'social-alert-report-browser-route-mobile.png');

export async function captureSocialAlertReportScreenshots(page: Page): Promise<void> {
  await mkdir(screenshotDir, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Social alerts and reports' })).toBeVisible();
  await page.screenshot({ fullPage: true, path: mobileScreenshotPath });
}
