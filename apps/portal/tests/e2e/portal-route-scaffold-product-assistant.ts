import { expect, type Page } from '@playwright/test';

export async function assertAssistantRouteSurface(page: Page): Promise<void> {
  await expect(page.getByRole('button', { name: 'Close parent assistant' })).toBeVisible();
}
