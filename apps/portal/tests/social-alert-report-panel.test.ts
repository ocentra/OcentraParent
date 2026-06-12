import { describe, expect, it } from 'vitest';
import { createSocialAlertReportPanelIntent } from '@ocentra-parent/portal-domain/contracts';

describe('portal social alert/report panel export', () => {
  it('projects missing service state as unavailable without delivery claims', () => {
    const intent = createSocialAlertReportPanelIntent(undefined);

    expect(intent.rows).toEqual([]);
    expect(intent.productClaim).toContain('provider delivery');
    expect(intent.productClaim).toContain('enforcement');
  });
});
