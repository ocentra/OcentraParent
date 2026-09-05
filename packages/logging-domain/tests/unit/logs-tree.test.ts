import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { getDefaultLogRoot } from '../../src/test-log/ndjsonPaths';
import { buildLogsTree, getDirPath, listFileKeysInScope, tryGet } from '../../src/test-log/logsTree';
import { closeLocalArtifactMutationProvider } from '../../src/local-artifact-mutation-provider';

describe.skipIf(process.platform !== 'win32')('logsTree', () => {
  const tempDirs: string[] = [];

  afterEach(async () => {
    for (const root of tempDirs.splice(0, tempDirs.length)) {
      await closeLocalArtifactMutationProvider(root);
      fs.rmSync(path.dirname(root), { force: true, recursive: true });
    }
  });

  it('lists file keys and resolves stored NDJSON paths from a temp root', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'logging-domain-logs-tree-'));
    const root = path.join(tempDir, 'output');
    tempDirs.push(root);
    const scopePath = path.join(root, 'test-logs', 'parent-test', 'single', 'unit');
    fs.mkdirSync(scopePath, { recursive: true });
    const filePath = path.join(scopePath, 'sample.ndjson');
    fs.writeFileSync(filePath, '{"message":"one"}\n', 'utf8');

    const previousLogDir = process.env.OCENTRA_PARENT_LOG_DIR;
    process.env.OCENTRA_PARENT_LOG_DIR = root;
    try {
      const tree = buildLogsTree(root);
      expect(listFileKeysInScope({ scope: 'parent-test', runType: 'single', suiteType: 'unit' }, root)).toEqual([
        'sample',
      ]);
      expect(tryGet(tree, { scope: 'parent-test', runType: 'single', suiteType: 'unit' }, 'sample')).toBe(filePath);
      expect(getDirPath({ scope: 'parent-test', runType: 'single', suiteType: 'unit' }, 'sample', root)).toBe(
        scopePath
      );
      expect(getDefaultLogRoot()).toBe(path.resolve(root));
    } finally {
      if (previousLogDir == null) {
        delete process.env.OCENTRA_PARENT_LOG_DIR;
      } else {
        process.env.OCENTRA_PARENT_LOG_DIR = previousLogDir;
      }
    }
  });
});
