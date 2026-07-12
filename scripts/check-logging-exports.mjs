#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

const REQUIRED_LOGGING_EXPORTS = [
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
];

const REQUIRED_PRODUCTION_EXPORTS = ['./package-info'];

function parseRepoRoot(argv) {
  const flag = argv.find((value) => value.startsWith('--root='));
  return path.resolve(flag == null ? process.cwd() : flag.slice('--root='.length));
}

function ensure(condition, message) {
  if (!condition) {
    throw new Error(`logging exports failed: ${message}`);
  }
}

function main() {
  const repoRoot = parseRepoRoot(process.argv.slice(2));
  const packageJsonPath = path.join(repoRoot, 'packages', 'logging-domain', 'package.json');
  ensure(fs.existsSync(packageJsonPath), 'missing packages/logging-domain/package.json');

  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  const exportsMap = packageJson.exports ?? {};

  ensure(exportsMap['./contracts'] == null, 'unexpected packages/logging-domain export ./contracts');
  ensure(
    !fs.existsSync(path.join(repoRoot, 'packages', 'logging-domain', 'src', 'contracts.ts')),
    'unexpected packages/logging-domain/src/contracts.ts'
  );
  ensure(
    !fs.existsSync(path.join(repoRoot, 'packages', 'logging-domain', 'src', 'core', 'logRuntimeConstants.ts')),
    'unexpected packages/logging-domain/src/core/logRuntimeConstants.ts'
  );
  for (const exportKey of [...REQUIRED_LOGGING_EXPORTS, ...REQUIRED_PRODUCTION_EXPORTS]) {
    ensure(exportsMap[exportKey] != null, `missing packages/logging-domain export ${exportKey}`);
  }

  process.stdout.write('logging export checks passed.\n');
}

try {
  main();
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
}
