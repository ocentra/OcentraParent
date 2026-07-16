import { describe, expect, it } from 'vitest';
import { parseStackTrace } from '../../src/core/stackTraceParser';
import {
  moduleNameFromGeneratedPath,
  parseGeneratedStackTrace,
  resolveGeneratedLoggerContext,
  resolveGeneratedLoggerSource,
} from '../../src/stack-trace-runtime';

describe('stack trace parser', () => {
  it('parses normal stack lines with function names and file urls', () => {
    const stack = `Error
    at LoggerTestFixture.emitHelloWorldLogs (file:///C:/repo/packages/logging-domain/tests/unit/logger.test.ts:21:9)`;

    expect(parseStackTrace(stack)).toEqual([
      {
        functionName: 'LoggerTestFixture.emitHelloWorldLogs',
        file: 'logger.test.ts',
        filePath: 'C:/repo/packages/logging-domain/tests/unit/logger.test.ts',
        line: 21,
        column: 9,
      },
    ]);
  });

  it('parses minimal and malformed stack lines without throwing', () => {
    const stack = `Error
    at file:///C:/repo/packages/logging-domain/src/core/logger.ts:88:17
not a frame`;

    expect(parseStackTrace(stack)).toEqual([
      {
        functionName: null,
        file: 'logger.ts',
        filePath: 'C:/repo/packages/logging-domain/src/core/logger.ts',
        line: 88,
        column: 17,
      },
    ]);
  });

  it('stays in parity with the generated helper and location rules', () => {
    const stack = `Error
    at emitHelloWorldLogs (packages/logging-domain/tests/unit/logger.test.ts:21:9)
    at file:///C:/repo/packages/logging-domain/src/core/logger.ts:88:17`;

    const parsed = parseStackTrace(stack);
    expect(parsed).toEqual(parseGeneratedStackTrace(stack));
    expect(moduleNameFromGeneratedPath('packages/logging-domain/tests/unit/logger.test.ts')).toBe('LoggerTest');
    expect(resolveGeneratedLoggerContext('LoggerTest', parsed[0] ?? null, 'module')).toBe(
      'LoggerTest.emitHelloWorldLogs'
    );
    expect(resolveGeneratedLoggerSource('LoggerTest', parsed[0] ?? null)).toBe('LoggerTest');
  });
});
