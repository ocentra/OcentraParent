import { expect, test } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.setTimeout(180_000);

const portalShellReadyTimeoutMs = 90_000;

test('assistant chat bubble controls support keyboard collapse and copy', async ({ context, page }) => {
  const browserFailures = collectBrowserFailures(page);
  await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: 'http://127.0.0.1:4490' });
  await page.goto('/#/assistant');

  await expect(page.getByRole('button', { exact: true, name: 'Close parent assistant' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  const assistantMessage = page.getByRole('article', { name: /^MIA: .+/u }).first();
  await expect(assistantMessage).toBeVisible();
  const assistantMessageLabel = await assistantMessage.getAttribute('aria-label');
  expect(assistantMessageLabel).toMatch(/^MIA: .+/u);
  const assistantMessageText = assistantMessageLabel?.replace(/^MIA: /u, '') ?? '';

  const collapseButton = page.getByRole('button', { exact: true, name: 'Collapse MIA message' }).first();
  await collapseButton.focus();
  await expect(collapseButton).toBeFocused();
  await page.keyboard.press('Enter');

  const expandButton = page.getByRole('button', { exact: true, name: 'Expand MIA message' }).first();
  await expect(expandButton).toBeVisible();
  await expect(expandButton).toHaveAttribute('aria-expanded', 'false');
  await expandButton.focus();
  await page.keyboard.press('Space');

  await expect(collapseButton).toBeVisible();
  await expect(collapseButton).toHaveAttribute('aria-expanded', 'true');

  const copyButton = page.getByRole('button', { exact: true, name: 'Copy MIA message' }).first();
  await copyButton.focus();
  await expect(copyButton).toBeFocused();
  await page.keyboard.press('Enter');

  await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(assistantMessageText);
  expect(browserFailures).toEqual([]);
});
