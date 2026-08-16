#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

function parseRepoRoot(argv) {
  const flag = argv.find((value) => value.startsWith('--root='));
  return path.resolve(flag == null ? process.cwd() : flag.slice('--root='.length));
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function readText(filePath) {
  return fs.readFileSync(filePath, 'utf8');
}

function ensure(condition, message) {
  if (!condition) {
    throw new Error(`logging-domain parity failed: ${message}`);
  }
}

function main() {
  const repoRoot = parseRepoRoot(process.argv.slice(2));
  const packageRoot = path.join(repoRoot, 'packages', 'logging-domain');
  const packageJsonPath = path.join(packageRoot, 'package.json');
  ensure(fs.existsSync(packageJsonPath), 'missing packages/logging-domain/package.json');

  const packageJson = readJson(packageJsonPath);
  const scripts = packageJson.scripts ?? {};
  for (const scriptName of ['bridge', 'db:ensure', 'db:rebuild', 'db:ingest', 'test:query']) {
    ensure(
      typeof scripts[scriptName] === 'string',
      `missing packages/logging-domain package.json script ${scriptName}`
    );
  }

  for (const relativePath of [
    'packages/logging-domain/src/test-log',
    'packages/logging-domain/src/transport',
    'packages/logging-domain/src/app-log',
    'packages/logging-domain/scripts/log-bridge.ts',
    'packages/logging-domain/scripts/rebuild-db-from-ndjson.ts',
    'packages/logging-domain/scripts/query-test-logs.ts',
  ]) {
    ensure(fs.existsSync(path.join(repoRoot, relativePath)), `missing ${relativePath}`);
  }

  const exportsMap = packageJson.exports ?? {};
  ensure(exportsMap['./contracts'] == null, 'unexpected packages/logging-domain export ./contracts');
  for (const exportKey of [
    './test-log/types',
    './test-log/bridgeConvert',
    './test-log/ndjsonPaths',
    './test-log/ndjsonBrands',
    './test-log/ndjsonWriter',
    './test-log/testLogDuckDb',
    './test-log/logsTree',
    './test-log/wipeNdjsonScope',
    './transport/bridgeTransport',
    './app-log/appNdjsonWriter',
    './app-log/types',
    './package-info',
  ]) {
    ensure(exportsMap[exportKey] != null, `missing packages/logging-domain export ${exportKey}`);
  }
  ensure(
    !fs.existsSync(path.join(packageRoot, 'src', 'contracts.ts')),
    'unexpected packages/logging-domain/src/contracts.ts'
  );
  ensure(
    !fs.existsSync(path.join(packageRoot, 'src', 'core', 'logRuntimeConstants.ts')),
    'unexpected packages/logging-domain/src/core/logRuntimeConstants.ts'
  );
  const schemaTypesText = readText(path.join(packageRoot, 'src', 'test-log', 'types.ts'));
  ensure(
    schemaTypesText.includes('fallback: TestLogScope = TestLogScope.ParentTest'),
    'generic test-log scope fallback must default to parent-test instead of parent-cloudflare'
  );

  const configText = readText(path.join(packageRoot, 'src', 'core', 'logConfig.ts'));
  ensure(
    !configText.includes('ParentCloudflare'),
    'generic logging config must not hardcode parent-cloudflare as the default scope'
  );

  process.stdout.write('logging-domain parity checks passed.\n');
}

try {
  main();
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
}
