import { describe, expect, it } from 'vitest';
import {
  NonEmptyStringSchema,
  Schema,
  brandedNonEmptyStringSchema,
  parseUnknown,
  safeParseUnknown,
  withParser,
} from '../../src/effect';

describe('schema-domain effect helpers', () => {
  it('parseUnknown: returns decoded values for valid input', () => {
    const schema = Schema.Struct({ name: Schema.String });

    expect(parseUnknown(schema, { name: 'Ocentra Parent' })).toEqual({ name: 'Ocentra Parent' });
  });

  it('safeParseUnknown: returns decode errors for invalid input', () => {
    const schema = Schema.Struct({ name: Schema.String });
    const parsed = safeParseUnknown(schema, { name: 42 });

    expect(parsed.success).toBe(false);
    if (!parsed.success) {
      expect(parsed.error.issues.length).toBeGreaterThan(0);
    }
  });

  it('withParser: attaches parse helpers without changing schema behavior', () => {
    const schema = withParser(Schema.Struct({ enabled: Schema.Boolean }));

    expect(schema.parse({ enabled: true })).toEqual({ enabled: true });
    expect(schema.safeParse({ enabled: 'yes' }).success).toBe(false);
  });

  it('withParser: preserves schema usability as a nested field', () => {
    const statusSchema = withParser(Schema.Literal('ready', 'empty'));
    const schema = Schema.Struct({ status: statusSchema });

    expect(parseUnknown(schema, { status: 'ready' })).toEqual({ status: 'ready' });
    expect(safeParseUnknown(schema, { status: 'missing' }).success).toBe(false);
  });

  it('NonEmptyStringSchema: rejects empty shared text values', () => {
    expect(safeParseUnknown(NonEmptyStringSchema, 'contract-ref').success).toBe(true);
    expect(safeParseUnknown(NonEmptyStringSchema, '').success).toBe(false);
  });

  it('brandedNonEmptyStringSchema: creates branded shared text schemas', () => {
    const schema = brandedNonEmptyStringSchema('SharedDomainReference');

    expect(parseUnknown(schema, 'domain-ref')).toBe('domain-ref');
    expect(safeParseUnknown(schema, '').success).toBe(false);
  });
});
