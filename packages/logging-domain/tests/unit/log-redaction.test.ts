import { describe, expect, it } from 'vitest';
import { redactStructuredLogValue, redactUnstructuredLogText } from '../../src/core/log-redaction';

const Redacted = '[REDACTED]';
const RedactedPath = '[REDACTED_PATH]';
const RedactedUrl = '[REDACTED_URL]';
const Circular = '[CIRCULAR]';
const Unsupported = '[UNSUPPORTED_LOG_VALUE]';

describe('structured logging redaction', () => {
  it('redacts sensitive, path, and URL fields at every structured nesting level', () => {
    const result = redactStructuredLogValue({
      event: 'query-complete',
      authorization: 'Bearer parent-secret',
      profilePath: 'C:\\Users\\parent\\profile.json',
      targetUrl: 'https://parent.example/private',
      details: {
        sessionId: 'session-secret',
        count: 2,
        rows: [{ apiKey: 'api-secret', label: 'visible-row' }],
      },
    });

    expect(result).toEqual({
      event: 'query-complete',
      authorization: Redacted,
      profilePath: Redacted,
      targetUrl: Redacted,
      details: {
        sessionId: Redacted,
        count: 2,
        rows: [{ apiKey: Redacted, label: 'visible-row' }],
      },
    });
  });
});

describe('unsupported structured logging values', () => {
  it('marks circular and unsupported structured values without invoking accessors', () => {
    const circular: Record<string, unknown> = { label: 'root' };
    circular['self'] = circular;
    const sparse: unknown[] = ['first'];
    sparse.length = 3;
    sparse[2] = 'third';
    const accessor: Record<string, unknown> = {};
    Object.defineProperty(accessor, 'value', {
      enumerable: true,
      get: () => {
        throw new Error('the sanitizer must not invoke getters');
      },
    });

    const result = redactStructuredLogValue({
      circular,
      sparse,
      accessor,
      undefinedValue: undefined,
      bigintValue: 1n,
      functionValue: () => 'private',
      symbolValue: Symbol('private'),
      nonFinite: Number.POSITIVE_INFINITY,
      dateValue: new Date('2026-09-02T00:00:00.000Z'),
      siteObject: new URL('https://parent.example/private'),
    });

    expect(result).toEqual({
      circular: { label: 'root', self: Circular },
      sparse: ['first', Unsupported, 'third'],
      accessor: { value: Unsupported },
      undefinedValue: Unsupported,
      bigintValue: Unsupported,
      functionValue: Unsupported,
      symbolValue: Unsupported,
      nonFinite: Unsupported,
      dateValue: Unsupported,
      siteObject: Unsupported,
    });
    expect(JSON.parse(JSON.stringify(result))).toEqual(result);
  });

  it('contains reflection failures and preserves ordinary non-sensitive data exactly', () => {
    const throwingProxy = new Proxy(
      {},
      {
        ownKeys: () => {
          throw new Error('reflection unavailable');
        },
      }
    );
    const safeValue = {
      state: 'ready',
      enabled: true,
      count: 3,
      ratio: 0.5,
      nullable: null,
      tags: ['local', 'proof'],
      nested: { owner: 'logging-domain' },
    };

    expect(redactStructuredLogValue(throwingProxy)).toBe(Unsupported);
    expect(redactStructuredLogValue(safeValue)).toEqual(safeValue);
  });
});

describe('unstructured logging redaction', () => {
  it('redacts authorization values, URLs, paths, and sensitive assignments in unstructured text', () => {
    const value =
      'Bearer parent-secret https://parent.example/private C:\\Users\\parent\\proof.log /home/parent/proof.log label=visible password=parent-secret';

    expect(redactUnstructuredLogText(value)).toBe(
      `Bearer ${Redacted} ${RedactedUrl} ${RedactedPath} ${RedactedPath} label=visible password=${Redacted}`
    );
  });
});
