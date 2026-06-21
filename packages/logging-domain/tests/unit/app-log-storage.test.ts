import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { TestLogScope } from '@ocentra-parent/schema-domain/test-log/types';
import { createAppLogStorage } from '../../src/app-log/createAppLogStorage';

describe('app-log storage', () => {
  const tempDirs: string[] = [];

  afterEach(() => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('stores, queries, and clears session logs under a temp root', async () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-app-log-'));
    tempDirs.push(tempDir);
    const storage = createAppLogStorage({
      scope: TestLogScope.ParentTest,
      rootDir: tempDir,
      sessionId: 'session-one',
      keepNewestSessions: 3,
    });

    storage.storeLog({
      timestamp: 1,
      level: 'info',
      source: 'portal',
      context: 'app-log',
      message: 'first app log',
      data: null,
      file: null,
      filePath: null,
      line: null,
      column: null,
      correlationId: null,
      environment: 'test',
    });
    await storage.flush();

    const query = await storage.queryLogs({ search: 'first app' });
    expect(query).toHaveLength(1);
    expect(query[0]?.message).toBe('first app log');

    const stats = await storage.getStats();
    expect(stats.totalLogs).toBe(1);
    expect(stats.sessions).toBe(1);

    const cleared = await storage.clearLogs();
    expect(cleared).toBe(1);
    expect(await storage.queryLogs()).toHaveLength(0);
  });
});
