import { expect, it } from 'vitest';
import { asFileKey, asNdjsonSummaryContent, asOutputDir, asTestName } from '../../src/test-log/ndjsonBrands';

it('ndjsonBrands: exposes branded helpers for the NDJSON seam', () => {
  expect(asOutputDir('output')).toBe('output');
  expect(asFileKey('file-key')).toBe('file-key');
  expect(asTestName('Test Name')).toBe('Test Name');
  expect(asNdjsonSummaryContent('summary')).toBe('summary');
});
