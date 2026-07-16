import { describe, expect, it } from 'vitest';
import { RunType, TestLogScope, TestLogSchemaVersion } from '../../src/test-log/types';
import {
  bridgeEntryToStoredLog,
  bridgePayloadToStoredLog,
  createBridgeEntryFromStoredLog,
  storedLogToBridgeEntry,
  storedLogToBridgePayload,
} from '../../src/test-log/bridgeConvert';
import {
  bridgeEntryToGeneratedStoredLog,
  createGeneratedBridgeEntryFromStoredLog,
  storedGeneratedLogToBridgeEntry,
  storedGeneratedLogToBridgePayload,
} from '../../src/bridge-log-runtime';

function createBridgeEntry() {
  return {
    testName: 'bridge convert',
    runId: 'run-1',
    runType: RunType.Single,
    consumer: null,
    log: {
      log_timestamp: 1_718_000_000_000,
      level: 'info' as const,
      source: 'portal',
      context: 'convert',
      message: 'bridge mapping',
      data: '{"ok":true}',
      file: 'bridge.ts',
      file_path: 'packages/logging-domain/src/transport/bridgeTransport.ts',
      line: 10,
      column: 2,
      correlation_id: 'cid-1',
      tags: ['smoke'],
      stack: null,
      suite_type: 'unit' as const,
      origin: 'portal' as const,
      environment: 'test',
    },
  };
}

function createStoredLog() {
  return {
    schemaVersion: TestLogSchemaVersion,
    type: 'log' as const,
    scope: TestLogScope.ParentTest,
    runId: 'run-1',
    runType: RunType.Single,
    suiteType: 'unit' as const,
    testName: 'bridge convert',
    timestamp: 1_718_000_000_000,
    level: 'info' as const,
    source: 'portal',
    context: 'convert',
    message: 'bridge mapping',
    data: '{"ok":true}',
    file: 'bridge.ts',
    filePath: 'packages/logging-domain/src/transport/bridgeTransport.ts',
    line: 10,
    column: 2,
    correlationId: 'cid-1',
    tags: ['smoke'],
    stack: null,
    origin: 'portal' as const,
    environment: 'test',
  };
}

describe('bridge convert', () => {
  it('maps bridge payloads into stored logs with explicit defaults', () => {
    const entry = createBridgeEntry();
    expect(
      bridgePayloadToStoredLog(entry.log, {
        testName: entry.testName,
        runId: entry.runId,
      })
    ).toEqual({
      ...createStoredLog(),
      scope: TestLogScope.ParentTest,
      runType: RunType.Single,
    });
  });

  it('stays in parity with the generated helper for both directions', () => {
    const entry = createBridgeEntry();
    const stored = createStoredLog();

    expect(bridgeEntryToStoredLog(entry)).toEqual(bridgeEntryToGeneratedStoredLog(entry));
    expect(storedLogToBridgePayload(stored)).toEqual(storedGeneratedLogToBridgePayload(stored));
    expect(storedLogToBridgeEntry(stored)).toEqual(storedGeneratedLogToBridgeEntry(stored));
  });

  it('applies bridge entry overrides through the generated mapping rules', () => {
    const stored = createStoredLog();
    expect(
      createBridgeEntryFromStoredLog(stored, {
        testName: 'override test',
        runId: 'run-2',
        runType: 'suite',
        consumer: 'worker-test',
      })
    ).toEqual(
      createGeneratedBridgeEntryFromStoredLog(stored, {
        testName: 'override test',
        runId: 'run-2',
        runType: 'suite',
        consumer: 'worker-test',
      })
    );
  });
});
