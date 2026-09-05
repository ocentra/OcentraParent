import { describe, expect, it } from 'vitest';
import { SchemaParseError, schema } from '../../../../vendor/ocentra-parent-core-ui/shims/effect-builder';
import {
  DEFAULT_PARENT_PORTAL_CONTENT,
  parseParentPortalContent,
} from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgContent';

describe('vendor schema effect builder', () => {
  const contentSchema = schema
    .object({
      name: schema.string().min(2),
      mode: schema.enum(['ready', 'blocked']),
      values: schema.array(schema.number()).min(1),
      note: schema.string().optional(),
    })
    .strict();

  it('rejects strict unknown keys and missing required fields', () => {
    expect(() => contentSchema.parse({ name: 'ready', mode: 'ready', values: [1], extra: true })).toThrow(
      SchemaParseError
    );
    expect(() => contentSchema.parse({ name: 'ready', mode: 'ready' })).toThrow(/values.*required/);
  });

  it('rejects minimum, enum, array, and nested value violations', () => {
    expect(() => contentSchema.parse({ name: 'x', mode: 'ready', values: [1] })).toThrow(/name/);
    expect(() => contentSchema.parse({ name: 'ready', mode: 'unknown', values: [1] })).toThrow(/mode/);
    expect(() => contentSchema.parse({ name: 'ready', mode: 'ready', values: [] })).toThrow(/values/);
    expect(() => contentSchema.parse({ name: 'ready', mode: 'ready', values: ['one'] })).toThrow(/values\.0/);
  });

  it('accepts optional fields, partial objects, and defaults', () => {
    const parsed = contentSchema.parse({ name: 'ready', mode: 'ready', values: [1] });
    expect(parsed.note).toBeUndefined();

    const partialSchema = contentSchema.partial();
    expect(partialSchema.parse({ mode: 'blocked' })).toEqual({ mode: 'blocked' });

    const defaulted = schema.string().optional().default('fallback');
    const fallback: string = defaulted.parse(undefined);
    expect(fallback).toBe('fallback');
  });

  it('parses the real Portal content through its owning schema', () => {
    const parsed = parseParentPortalContent(DEFAULT_PARENT_PORTAL_CONTENT);

    expect(parsed.tabs[0]?.id).toBe('overall');
    expect(parsed.navItems.length).toBeGreaterThan(0);
    expect(parsed.modes.parentOverview.rowSource).toBe('api');
  });
});
