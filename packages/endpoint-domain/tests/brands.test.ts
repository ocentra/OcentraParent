import { describe, expect, it } from 'vitest';
import { decodeApiPath, decodeHeaderName, decodePathSegment } from '../src/types/brands';

describe('endpoint-domain brands', () => {
  it('decodeApiPath: accepts slash-prefixed paths', () => {
    expect(decodeApiPath('/api/v1/health')).toBe('/api/v1/health');
  });

  it('decodeApiPath: rejects non-path values', () => {
    expect(() => decodeApiPath('api/v1/health')).toThrow();
  });

  it('decodePathSegment: rejects segments containing slashes', () => {
    expect(() => decodePathSegment('devices/list')).toThrow();
  });

  it('decodeHeaderName: accepts non-empty header names', () => {
    expect(decodeHeaderName('Authorization')).toBe('Authorization');
  });
});
