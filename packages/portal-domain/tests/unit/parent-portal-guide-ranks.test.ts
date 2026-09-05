import { describe, expect, it } from 'vitest';

import { PARENT_PORTAL_GUIDE_TOPICS } from '../../src/parent-portal-guides';

describe('parent portal guide ranks', () => {
  it('numbers the ordered Start guide topics once without gaps', () => {
    const renderedRanks = PARENT_PORTAL_GUIDE_TOPICS.map((topic) => topic.rank);
    const expectedRanks = PARENT_PORTAL_GUIDE_TOPICS.map((_, index) => index + 1);

    expect(renderedRanks).toEqual(expectedRanks);
  });
});
