#!/usr/bin/env node
import fs from 'node:fs';
import os from 'node:os';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const enforcerWrapper = path.join(repoRoot, 'scripts', 'enforcer', 'run-ocentra-enforcer.mjs');
const singleSourceContractsConfig = path.join('scripts', 'check-single-source-contracts.json');
const ignoredExpansionDirs = new Set([
  '.agents',
  '.codebase-memory',
  '.codeql-local',
  '.codex',
  '.codex-artifacts',
  '.codex-logs',
  '.codex-tmp',
  '.git',
  '.enforce',
  '.ledger',
  '.logs',
  '.tmp',
  '.wix',
  'coverage',
  'dist',
  'node_modules',
  'output',
  'target',
  'target-lan-verify',
  'target-parent-domain-logger-fixture',
  'target-parent-dev',
  'test-results',
  'tmp',
]);

export function main(rawArgs = process.argv.slice(2)) {
  const files = expandFiles(parseFiles(rawArgs));
  if (files === null) {
    if (hasDiffScope(rawArgs)) {
      runEnforcer(['architecture', 'check', '--scope', 'diff', ...rawArgs]);
      return;
    }
    runEnforcer(['check', 'architecture-policy', ...rawArgs]);
    return;
  }
  if (files.length === 0) return;

  const { generatedFiles, generatorFiles, sourceFiles } = classifyArchitectureFiles(files);
  const passthroughArgs = stripFiles(rawArgs);

  if (sourceFiles.length > 0) {
    runEnforcer([
      'check',
      'architecture-policy',
      '--check-config',
      singleSourceContractsConfig,
      ...passthroughArgs,
      ...filesFromArgs('source', sourceFiles),
    ]);
  }
  if (generatedFiles.length > 0) {
    runEnforcer(['check', 'generated-artifacts', ...passthroughArgs, ...filesFromArgs('generated', generatedFiles)]);
  }
  if (generatorFiles.length > 0) {
    runEnforcer(['check', 'reexports', ...passthroughArgs, ...filesFromArgs('generator', generatorFiles)]);
  }
}

function hasDiffScope(args) {
  return args.some((arg) => arg === '--base' || arg.startsWith('--base='));
}

function parseFiles(args) {
  const files = [];
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === '--files') {
      index += 1;
      while (index < args.length && !args[index].startsWith('--')) {
        files.push(...splitFiles(args[index]));
        index += 1;
      }
      index -= 1;
      continue;
    }
    if (arg.startsWith('--files=')) {
      files.push(...splitFiles(arg.slice('--files='.length)));
      continue;
    }
    if (arg === '--files-from') {
      files.push(...readFileManifest(args[++index]));
      continue;
    }
    if (arg.startsWith('--files-from=')) {
      files.push(...readFileManifest(arg.slice('--files-from='.length)));
    }
  }
  return files.length === 0 ? null : files;
}

function stripFiles(args) {
  const stripped = [];
  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === '--files') {
      index += 1;
      while (index < args.length && !args[index].startsWith('--')) {
        index += 1;
      }
      index -= 1;
      continue;
    }
    if (arg.startsWith('--files=')) {
      continue;
    }
    if (arg === '--files-from') {
      index += 1;
      continue;
    }
    if (arg.startsWith('--files-from=')) {
      continue;
    }
    stripped.push(arg);
  }
  return stripped;
}

function splitFiles(value) {
  return value
    .split(/[,\r\n]/u)
    .map((entry) => entry.trim())
    .filter(Boolean);
}

function readFileManifest(manifestPath) {
  const absolute = path.isAbsolute(manifestPath) ? manifestPath : path.resolve(repoRoot, manifestPath);
  const text = fs.readFileSync(absolute, 'utf8');
  const trimmed = text.trim();
  if (trimmed.startsWith('{') || trimmed.startsWith('[')) {
    const parsed = JSON.parse(trimmed);
    return Array.isArray(parsed) ? parsed : (parsed.files ?? []);
  }
  return splitFiles(text);
}

function filesFromArgs(kind, files) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'ocentra-architecture-scope-'));
  const manifestPath = path.join(dir, `${kind}.json`);
  fs.writeFileSync(manifestPath, JSON.stringify({ files }, null, 2), 'utf8');
  return ['--files-from', manifestPath];
}

function expandFiles(files) {
  if (files === null) return null;
  const expanded = [];
  for (const file of files) {
    if (pathSegments(file).some((segment) => ignoredExpansionDirs.has(segment))) continue;
    const absolute = path.resolve(repoRoot, file);
    if (fs.existsSync(absolute) && fs.statSync(absolute).isDirectory()) {
      expanded.push(...collectDirectoryFiles(file));
      continue;
    }
    expanded.push(file);
  }
  return expanded;
}

function pathSegments(file) {
  return file.replace(/\\/gu, '/').split('/').filter(Boolean);
}

function collectDirectoryFiles(start) {
  const absoluteStart = path.resolve(repoRoot, start);
  const collected = [];
  const stack = [absoluteStart];
  while (stack.length > 0) {
    const current = stack.pop();
    const entries = fs.readdirSync(current, { withFileTypes: true });
    for (const entry of entries) {
      const absolute = path.join(current, entry.name);
      if (entry.isDirectory()) {
        if (ignoredExpansionDirs.has(entry.name)) continue;
        stack.push(absolute);
        continue;
      }
      collected.push(path.relative(repoRoot, absolute));
    }
  }
  return collected;
}

function isGeneratedArtifact(file) {
  const normalized = file.replace(/\\/gu, '/').toLowerCase();
  return (
    normalized.includes('/generated/') ||
    normalized.includes('/dist/') ||
    normalized.startsWith('apps/portal/generated/') ||
    normalized.startsWith('packages/portal-domain/src/generated/') ||
    normalized.startsWith('packages/schema-domain/src/generated/')
  );
}

function isGeneratedProducer(file) {
  const normalized = file.replace(/\\/gu, '/').toLowerCase();
  return (
    /^crates\/schema\/src\/[^/]+_ts\.rs$/u.test(normalized) ||
    normalized === 'crates/ai-contracts/src/ai_contracts_ts.rs' ||
    /^crates\/schema\/src\/bin\/export_[^/]+\.rs$/u.test(normalized) ||
    normalized === 'crates/tracking-core/src/generated_bridge.rs'
  );
}

export function classifyArchitectureFiles(files) {
  const generatedFiles = files.filter(isGeneratedArtifact);
  const generatorFiles = files.filter((file) => !isGeneratedArtifact(file) && isGeneratedProducer(file));
  const sourceFiles = files.filter(
    (file) => !isGeneratedArtifact(file) && !isGeneratedProducer(file) && isSourceLike(file)
  );
  return { generatedFiles, generatorFiles, sourceFiles };
}

function isSourceLike(file) {
  const normalized = file.replace(/\\/gu, '/').toLowerCase();
  return (
    normalized.endsWith('.ts') ||
    normalized.endsWith('.tsx') ||
    normalized.endsWith('.js') ||
    normalized.endsWith('.mjs') ||
    normalized.endsWith('.cjs') ||
    normalized.endsWith('.rs') ||
    normalized.endsWith('.py') ||
    normalized.endsWith('.json')
  );
}

function runEnforcer(args) {
  const result = spawnSync(process.execPath, [enforcerWrapper, ...args], {
    cwd: repoRoot,
    env: process.env,
    stdio: 'inherit',
    windowsHide: true,
  });
  if (result.error) {
    throw result.error;
  }
  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main();
}
