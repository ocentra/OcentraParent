import { describe, expect, it } from 'vitest';
import {
  literalRecordFromValues,
  literalSchema,
  literalValues,
  parsedLiteralRecord,
} from '../../src/policy-literal-contracts';

describe('policy-literal-contracts', () => {
  it('derives literal adapters from value tables only', () => {
    const values = ['alpha', 'beta', 'gamma'] as const;
    const record = literalRecordFromValues(values);

    expect(literalValues(record)).toEqual(values);
    expect(literalSchema(record).parse('beta')).toBe('beta');
    expect(parsedLiteralRecord(record, (value) => value.toUpperCase()).Alpha).toBe('ALPHA');
  });
});
