import { expect, test, type Locator, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.setTimeout(120_000);

test('proof panels render as one opaque contained route surface', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await page.goto('/#/proof-panels');
  const proofPanels = page.getByRole('region', { exact: true, name: 'Proof panels' });
  const panelPicker = proofPanels.getByRole('combobox', { exact: true, name: 'Proof panel' });
  const trackingStatusTab = proofPanels.getByRole('button', {
    exact: true,
    name: 'Tracking status',
  });
  await expect(proofPanels).toBeVisible({ timeout: 90_000 });
  await expect(trackingStatusTab).toBeVisible();
  await expect(trackingStatusTab).toHaveAttribute('aria-pressed', 'true');
  await expect(panelPicker).toBeHidden();
  await assertContainedOpaqueSurface(page, proofPanels);

  await page.setViewportSize({ width: 390, height: 844 });
  await expect(proofPanels).toBeVisible();
  await expect(panelPicker).toBeVisible();
  await expect(panelPicker).toHaveValue('tracking-status');
  await expect(trackingStatusTab).toBeHidden();
  await assertContainedOpaqueSurface(page, proofPanels);
  expect(browserFailures).toEqual([]);
});

async function assertContainedOpaqueSurface(page: Page, proofPanels: Locator): Promise<void> {
  const layout = await proofPanels.evaluate((element) => {
    const bounds = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    const centerElement = document.elementFromPoint(bounds.left + bounds.width / 2, bounds.top + bounds.height / 2);
    return {
      backgroundColor: style.backgroundColor,
      borderTopStyle: style.borderTopStyle,
      borderTopWidth: style.borderTopWidth,
      centerOwnedByPanel: centerElement !== null && element.contains(centerElement),
      left: bounds.left,
      right: bounds.right,
      width: bounds.width,
    };
  });
  const viewportWidth = page.viewportSize()?.width;
  if (viewportWidth === undefined) {
    throw new Error('Proof Panels layout test requires a configured browser viewport.');
  }

  expect(layout.backgroundColor).toBe('rgb(2, 12, 22)');
  expect(layout.borderTopStyle).toBe('solid');
  expect(layout.borderTopWidth).toBe('1px');
  expect(layout.centerOwnedByPanel).toBe(true);
  expect(layout.left).toBeGreaterThanOrEqual(0);
  expect(layout.right).toBeLessThanOrEqual(viewportWidth);
  expect(layout.width).toBeGreaterThan(0);
}
