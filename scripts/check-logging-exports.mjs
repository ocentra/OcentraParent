#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

const REQUIRED_LOGGING_EXPORTS = [
  './contracts',
  './test-log/types',
  './test-log/bridgeConvert',
  './test-log/ndjsonPaths',
  './test-log/ndjsonWriter',
  './test-log/testLogDuckDb',
  './test-log/logsTree',
  './test-log/wipeNdjsonScope',
  './transport/bridgeLogPayload',
  './transport/bridgeTransport',
  './app-log/createAppLogStorage',
];

const REQUIRED_PRODUCTION_EXPORTS = [
  './notification-audit-history',
  './notification-audit-history-handoff',
  './support-bundle-redaction',
  './support-bundle-redaction-read-model',
  './support-backend-upload-status',
  './support-backend-upload-status-read-model',
  './support-backend-upload-execution-runtime',
  './support-backend-upload-execution-runtime-read-model',
  './support-backend-upload-custody-audit',
  './support-backend-upload-custody-audit-read-model',
  './support-backend-provider-runtime-readiness',
  './support-backend-provider-runtime-readiness-read-model',
  './support-case-resolution-status',
  './support-case-resolution-status-read-model',
  './provider-secret-custody-status',
  './provider-secret-custody-status-read-model',
  './provider-secret-execution-readiness',
  './provider-secret-execution-readiness-read-model',
  './provider-secret-rotation-revocation-status',
  './provider-secret-rotation-revocation-status-read-model',
  './privacy-legal-disclosure-status',
  './privacy-legal-disclosure-status-read-model',
  './status-backend-payload-custody',
  './status-backend-payload-custody-read-model',
  './status-backend-redaction-manifest',
  './status-backend-redaction-manifest-read-model',
  './delete-executor-proof',
  './delete-executor-read-model',
  './support-incident-workflow',
  './support-incident-workflow-read-model',
  './tamper-integrity-audit',
  './tamper-integrity-audit-read-model',
  './package-info',
];

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
