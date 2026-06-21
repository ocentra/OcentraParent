import { describe, expect, it } from 'vitest';
import {
  screenChildDisclosureProofSnapshots,
} from '@ocentra-parent/schema-domain/screen-child-disclosure';
import {
  createScreenChildDisclosurePageModel,
  renderScreenChildDisclosurePage,
} from '../../src/screen-child-disclosure-page';

describe('screen child disclosure page', () => {
  it('renders child-visible states without raw screenshot or hidden-capture claims', () => {
    const model = createScreenChildDisclosurePageModel(screenChildDisclosureProofSnapshots());
    const html = renderScreenChildDisclosurePage(model);

    expect(model.rawScreenshotRendered).toBe(false);
    expect(model.hiddenCaptureClaimed).toBe(false);
    expect(model.renderedChildAgentDeliveryClaimed).toBe(false);
    expect(html).toContain('data-ocentra-screen-disclosure-state="disabledByParent"');
    expect(html).toContain('data-ocentra-screen-disclosure-state="captureActive"');
    expect(html).toContain('data-ocentra-screen-disclosure-state="protectedSurface"');
    expect(html).toContain('Screen activity is being checked');
    expect(html).toContain('Raw screenshot shown');
    expect(html).not.toContain('<img');
  });
});
