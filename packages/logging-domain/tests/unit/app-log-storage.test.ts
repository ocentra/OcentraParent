import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { createAppLogStorage } from '../../src/app-log/createAppLogStorage';
import { listAppLogSessionFiles } from '../../src/app-log/appNdjsonWriter';
import { TestLogScope } from '../../src/test-log/types';

describe('app-log storage retention', () => {
  const tempDirs: string[] = [];

  afterEach(() => {
    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('deletes older local sessions when retention is configured', async () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-app-log-'));
    tempDirs.push(tempDir);

    const firstStorage = createAppLogStorage({
      scope: TestLogScope.ParentCodex,
      rootDir: tempDir,
      sessionId: 'old-session',
      keepNewestSessions: 1,
    });

    firstStorage.storeLog({
      timestamp: 100,
      level: 'info',
      source: 'portal',
      context: 'retention',
      message: 'old log',
      data: null,
      file: null,
      filePath: null,
      line: null,
      column: null,
      correlationId: null,
      environment: 'test',
    });
    await firstStorage.flush();
    await firstStorage.dispose();

    const firstFile = listAppLogSessionFiles(TestLogScope.ParentCodex, tempDir)[0];
    if (firstFile == null) {
      throw new Error('Expected first session file');
    }
    fs.utimesSync(firstFile, new Date(1_000), new Date(1_000));

    const secondStorage = createAppLogStorage({
      scope: TestLogScope.ParentCodex,
      rootDir: tempDir,
      sessionId: 'new-session',
      keepNewestSessions: 1,
    });

    secondStorage.storeLog({
      timestamp: 200,
      level: 'warn',
      source: 'portal',
      context: 'retention',
      message: 'new log',
      data: null,
      file: null,
      filePath: null,
      line: null,
      column: null,
      correlationId: null,
      environment: 'test',
    });
    await secondStorage.flush();
    await secondStorage.dispose();

    const files = listAppLogSessionFiles(TestLogScope.ParentCodex, tempDir);
    expect(files).toHaveLength(1);
    const newestFile = files[0];
    if (newestFile == null) {
      throw new Error('Expected retained session file');
    }
    expect(path.basename(newestFile)).toContain('new-session');
  });
});
