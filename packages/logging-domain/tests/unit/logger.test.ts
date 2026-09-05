import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import type { AddressInfo } from 'node:net';
import { afterEach, describe, expect, it } from 'vitest';
import type { GeneratedStackTrace as StackTrace } from '../../src/generated-logging-contracts';
import { RunType, TestLogOrigin, TestLogScope } from '../../src/test-log/types';
import { Logger } from '../../src/core/logger';
import { createLocalArtifactBridgeQueueStorage } from '../../src/core/localArtifactBridgeQueueStorage';
import { getStackTrace } from '../../src/core/stackTrace';
import { createBridgeServer } from '../../src/transport/bridgeServer';
import { getTestLogScopeDir, listNdjsonFiles } from '../../src/test-log/ndjsonPaths';
import { readTestLogEntriesFromFile } from '../../src/test-log/ndjsonWriter';
import { closeLocalArtifactMutationProvider } from '../../src/local-artifact-mutation-provider';

class LoggerTestFixture {
  private readonly log = Logger.instance;

  constructor() {
    this.log.register(import.meta.url);
  }

  private logInfo = (message: string, stackTrace: StackTrace, data?: unknown, enabled = false) => {
    this.log.logInfo(message, stackTrace, data, enabled);
  };

  emitHelloWorldLogs(): void {
    this.logInfo('logger fixture info log', getStackTrace(), { hello: 'world' }, true);
  }
}

function makeTempDir(): string {
  return fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-logger-'));
}

async function listen(server: ReturnType<typeof createBridgeServer>): Promise<AddressInfo> {
  await new Promise<void>((resolve) => {
    server.listen(0, '127.0.0.1', () => resolve());
  });
  return server.address() as AddressInfo;
}

async function closeServer(server: ReturnType<typeof createBridgeServer>): Promise<void> {
  await new Promise<void>((resolve, reject) => {
    server.close((error) => {
      if (error != null) {
        reject(error);
        return;
      }
      resolve();
    });
  });
}

describe.skipIf(process.platform !== 'win32')('logger source and context registration', () => {
  const tempDirs: string[] = [];
  const servers: Array<ReturnType<typeof createBridgeServer>> = [];
  const originalLogLevel = process.env.OCENTRA_PARENT_LOG_LEVEL;

  afterEach(async () => {
    Logger.instance.reset();
    if (originalLogLevel == null) {
      delete process.env.OCENTRA_PARENT_LOG_LEVEL;
    } else {
      process.env.OCENTRA_PARENT_LOG_LEVEL = originalLogLevel;
    }

    await Promise.all(servers.splice(0, servers.length).map(closeServer));

    for (const tempDir of tempDirs.splice(0, tempDirs.length)) {
      await closeLocalArtifactMutationProvider(tempDir);
      fs.rmSync(tempDir, { force: true, recursive: true });
    }
  });

  it('stores registered source, context, and file path metadata through the bridge', async () => {
    process.env.OCENTRA_PARENT_LOG_LEVEL = 'debug';
    const tempDir = makeTempDir();
    tempDirs.push(tempDir);

    const server = createBridgeServer({ rootDir: tempDir });
    servers.push(server);
    const address = await listen(server);

    Logger.instance.configure({
      bridgeEndpoint: `http://127.0.0.1:${address.port}`,
      runId: 'logger-unit-run',
      testName: 'logger.test.ts',
      scope: TestLogScope.ParentTest,
      runType: RunType.Single,
      origin: TestLogOrigin.Test,
      environment: 'test',
      skipHealthCheck: true,
      bridgeQueueStorage: createLocalArtifactBridgeQueueStorage(tempDir),
    });

    const fixture = new LoggerTestFixture();
    fixture.emitHelloWorldLogs();
    await Logger.instance.flush();
    await closeLocalArtifactMutationProvider(tempDir);

    const files = listNdjsonFiles(getTestLogScopeDir(TestLogScope.ParentTest, tempDir));
    expect(files).toHaveLength(1);

    const rows = readTestLogEntriesFromFile(files[0] ?? '', tempDir);

    expect(rows).toHaveLength(1);
    expect(rows[0]?.source).toBe('LoggerTestFixture');
    expect(rows[0]?.context).toContain('emitHelloWorldLogs');
    expect(rows[0]?.filePath).toContain('tests/unit/logger.test.ts');
  });
});
