import { expect, type Locator } from '@playwright/test';

export async function assertPolicyPreviewBoundaryDetails(policyPreview: Locator): Promise<void> {
  await expect(detailValue(policyPreview, 'Privacy mode')).toHaveText('Local only');
  await expect(detailValue(policyPreview, 'Adapter boundary')).toHaveText('Local adapter not connected');
  await expect(detailValue(policyPreview, 'Execution state')).toHaveText('Off');
  await expect(detailValue(policyPreview, 'Provider source')).toHaveText('Unavailable');
  await expect(detailValue(policyPreview, 'Parent rule context references')).not.toHaveText('');
  await expect(detailValue(policyPreview, 'Parent rule context ref IDs')).not.toHaveText('');
}

function detailValue(container: Locator, label: string): Locator {
  return container.locator('dt').filter({ hasText: label }).locator('xpath=following-sibling::dd[1]');
}
