#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

const GUIDANCE_FRAGMENTS = [
  'Use npm run agent:run -- <command> for validation where possible.',
  'Use npm run agent:query or npm run codex:evidence for failure evidence.',
  'Do not paste full raw logs into context unless explicitly needed.',
];

function parseRepoRoot(argv) {
  const flag = argv.find((value) => value.startsWith('--root='));
  return path.resolve(flag == null ? process.cwd() : flag.slice('--root='.length));
}

function ensure(condition, message) {
  if (!condition) {
    throw new Error(`local evidence wrapper failed: ${message}`);
  }
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function readText(filePath) {
  return fs.readFileSync(filePath, 'utf8');
}

function main() {
  const repoRoot = parseRepoRoot(process.argv.slice(2));
  const packageJson = readJson(path.join(repoRoot, 'package.json'));
  const scripts = packageJson.scripts ?? {};

  for (const scriptName of ['agent:run', 'agent:query', 'codex:evidence']) {
    ensure(typeof scripts[scriptName] === 'string', `missing package.json script ${scriptName}`);
  }

  for (const relativePath of [
    'scripts/dev/agent-run.mjs',
    'scripts/dev/agent-query.mjs',
    'scripts/dev/codex-evidence.mjs',
    'scripts/dev/lib/agent-diagnostic-parsers.mjs',
    'scripts/dev/lib/agent-artifacts.mjs',
  ]) {
    ensure(fs.existsSync(path.join(repoRoot, relativePath)), `missing ${relativePath}`);
  }

  const guidanceFiles = [
    path.join(repoRoot, 'docs', 'plans', 'logging-domain-parity', '03-local-validation-evidence.md'),
    path.join(repoRoot, 'docs', 'plans', 'logging-domain-parity', '04-validation-and-enforcement.md'),
  ].filter((filePath) => fs.existsSync(filePath));

  ensure(guidanceFiles.length > 0, 'missing logging-domain guidance docs for wrapper usage');
  const guidanceText = guidanceFiles.map((filePath) => readText(filePath)).join('\n');

  for (const fragment of GUIDANCE_FRAGMENTS) {
    ensure(guidanceText.includes(fragment), `missing guidance text: ${fragment}`);
  }

  process.stdout.write('local evidence wrapper checks passed.\n');
}

try {
  main();
} catch (error) {
  process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
  process.exitCode = 1;
}
