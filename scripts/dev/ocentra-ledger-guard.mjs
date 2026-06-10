#!/usr/bin/env node
import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join, resolve } from 'node:path';

const repoRoot = git(['rev-parse', '--show-toplevel']);
const wrapperPath = join(repoRoot, 'scripts', 'dev', 'ocentra-ledger.mjs');
const lane = process.env.LEDGER_LANE ?? process.env.OCENTRA_PARENT_LEDGER_LANE ?? inferLane(repoRoot);
const changedPaths = gitLines(['diff', '--name-only', 'HEAD']).filter((path) => !isGeneratedArtifactPath(path));

ensureIdentity(lane);

const args = [wrapperPath, 'guard', '--lane', lane, '--changed', changedPaths.join('\n')];

if (lane === 'primary') {
  args.push('--allow-primary-without-claims');
}

const result = spawnSync(process.execPath, args, {
  cwd: repoRoot,
  env: process.env,
  stdio: 'inherit',
  windowsHide: true,
});

process.exit(result.status ?? 1);

function ensureIdentity(defaultLane) {
  const rootResult = runLedger(['root'], { capture: true });
  if (rootResult.status !== 0) {
    process.exit(rootResult.status ?? 1);
  }

  const parsed = JSON.parse(rootResult.stdout);
  const configPath = resolve(parsed.root, 'identity', 'node.json');
  if (existsSync(configPath)) {
    return;
  }

  const initResult = runLedger(['init', 'ocentra-parent', '--lane', defaultLane], { capture: false });
  if (initResult.status !== 0) {
    process.exit(initResult.status ?? 1);
  }
}

function runLedger(args, options) {
  return spawnSync(process.execPath, [wrapperPath, ...args], {
    cwd: repoRoot,
    encoding: options.capture ? 'utf8' : undefined,
    env: process.env,
    stdio: options.capture ? ['ignore', 'pipe', 'inherit'] : 'inherit',
    windowsHide: true,
  });
}

function inferLane(path) {
  const normalized = path.replace(/\\/gu, '/');
  const match = normalized.match(/(?:^|[/_-])((?:codex-[a-z])|(?:E-[A-Z]))(?:$|[/_-])/u);
  return match?.[1] ?? 'primary';
}

function git(args) {
  return execFileSync('git', args, {
    cwd: process.cwd(),
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
}

function gitLines(args) {
  const output = git(args);
  return output.length === 0 ? [] : output.split(/\r?\n/u);
}

function isGeneratedArtifactPath(path) {
  return (
    path === 'output' ||
    path.startsWith('output/') ||
    path === 'test-results' ||
    path.startsWith('test-results/') ||
    path === 'playwright-report' ||
    path.startsWith('playwright-report/')
  );
}
