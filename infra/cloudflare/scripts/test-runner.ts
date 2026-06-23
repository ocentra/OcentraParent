#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const typeArg = process.argv.find((arg) => arg.startsWith('--type='));
const selectedType = typeArg?.slice('--type='.length);

const testTargetsByType = new Map<string, ReadonlyArray<string>>([
  ['unit', ['tests/unit']],
  ['integration', ['tests/integration']],
  ['e2e', ['tests/e2e']],
  ['contract', ['tests/contract']],
  ['security', ['tests/security']],
  ['property', ['tests/property']],
  ['fuzz', ['tests/fuzz']],
]);

const selectedTargets = selectedType === undefined ? ['tests'] : (testTargetsByType.get(selectedType) ?? null);

if (selectedTargets === null) {
  console.error(`Unknown cloudflare test type: ${selectedType}`);
  process.exit(1);
}

function collectTestFiles(targetPath: string, files: string[]): void {
  const stat = fs.statSync(targetPath);
  if (stat.isDirectory()) {
    for (const entry of fs.readdirSync(targetPath, { withFileTypes: true })) {
      collectTestFiles(path.join(targetPath, entry.name), files);
    }
    return;
  }

  if (targetPath.endsWith('.test.ts')) {
    files.push(targetPath);
  }
}

const selectedFiles: string[] = [];
for (const target of selectedTargets) {
  collectTestFiles(path.resolve(process.cwd(), target), selectedFiles);
}

selectedFiles.sort();

if (selectedFiles.length === 0) {
  console.error(`No cloudflare test files found for type: ${selectedType ?? 'all'}`);
  process.exit(1);
}

const result = spawnSync(process.execPath, ['--import', 'tsx', '--test', ...selectedFiles], {
  cwd: process.cwd(),
  stdio: 'inherit',
});

process.exit(result.status ?? 1);
