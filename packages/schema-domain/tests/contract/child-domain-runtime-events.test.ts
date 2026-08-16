import { describe, expect, it } from 'vitest';

import {
  ChildDomainRuntimeEventTypeLiteral,
  ChildDomainRuntimeEventTypeSchema,
  ChildRuntimeDomainLiteral,
  ChildRuntimeDomainSchema,
} from '@ocentra-parent/schema-domain/child-domain-runtime-events';

describe('child domain runtime event contracts', () => {
  it('parses known child runtime domains', () => {
    expect(ChildRuntimeDomainSchema.parse(ChildRuntimeDomainLiteral.App)).toBe(ChildRuntimeDomainLiteral.App);
    expect(ChildRuntimeDomainSchema.parse(ChildRuntimeDomainLiteral.Browser)).toBe(ChildRuntimeDomainLiteral.Browser);
  });

  it('parses known child domain runtime event types', () => {
    expect(ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.AppAiAnalysisRequested)).toBe(
      ChildDomainRuntimeEventTypeLiteral.AppAiAnalysisRequested
    );
    expect(ChildDomainRuntimeEventTypeSchema.parse(ChildDomainRuntimeEventTypeLiteral.ScreenAiAnalysisRequested)).toBe(
      ChildDomainRuntimeEventTypeLiteral.ScreenAiAnalysisRequested
    );
  });

  it('rejects unowned child domain runtime event types', () => {
    expect(() => ChildDomainRuntimeEventTypeSchema.parse('child-domain.unowned.event')).toThrow();
  });
});
