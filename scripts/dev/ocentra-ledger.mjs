#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { existsSync, readdirSync, statSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const ledgerRoot = join(repoRoot, 'tools', 'ocentra-ledger');
const packagePath = join(ledgerRoot, 'package.json');
const nodeModulesPath = join(ledgerRoot, 'node_modules');
const cliPath = join(ledgerRoot, 'dist', 'cli.js');
const sourcePath = join(ledgerRoot, 'src');

const args = process.argv.slice(2);
const command = args[0];

if (!existsSync(packagePath)) {
  run('git', ['submodule', 'update', '--init', '--recursive', 'tools/ocentra-ledger'], repoRoot);
}

if (!existsSync(packagePath)) {
  fail(['Ocentra Ledger submodule is missing.', 'Run: git submodule update --init --recursive tools/ocentra-ledger']);
}

if (command === 'install') {
  install();
  build();
  process.exit(0);
}

if (command === 'build') {
  install();
  build();
  process.exit(0);
}

installIfNeeded();
buildIfNeeded();
ensureIdentityIfNeeded();

const result = spawnSync(process.execPath, [cliPath, ...args], {
  cwd: repoRoot,
  env: process.env,
  stdio: 'inherit',
  windowsHide: true,
});

process.exit(result.status ?? 1);

function installIfNeeded() {
  if (existsSync(nodeModulesPath)) {
    return;
  }
  install();
}

function buildIfNeeded() {
  if (existsSync(cliPath) && !isSourceNewerThanCli()) {
    return;
  }
  build();
}

function isSourceNewerThanCli() {
  if (!existsSync(cliPath) || !existsSync(sourcePath)) {
    return true;
  }
  const cliMtime = statSync(cliPath).mtimeMs;
  return newestMtime(sourcePath) > cliMtime;
}

function newestMtime(path) {
  const stats = statSync(path);
  if (!stats.isDirectory()) {
    return stats.mtimeMs;
  }
  let newest = stats.mtimeMs;
  for (const entry of readdirSync(path)) {
    newest = Math.max(newest, newestMtime(join(path, entry)));
  }
  return newest;
}

function ensureIdentityIfNeeded() {
  if (identityOptionalCommands().has(command)) {
    return;
  }

  const rootResult = spawnSync(process.execPath, [cliPath, 'root'], {
    cwd: repoRoot,
    encoding: 'utf8',
    env: process.env,
    stdio: ['ignore', 'pipe', 'inherit'],
    windowsHide: true,
  });
  if ((rootResult.status ?? 1) !== 0) {
    process.exit(rootResult.status ?? 1);
  }

  const root = JSON.parse(rootResult.stdout).root;
  const identityPath = join(root, 'identity', 'node.json');
  if (existsSync(identityPath)) {
    return;
  }

  const initResult = spawnSync(process.execPath, [cliPath, 'init', 'ocentra-parent', '--lane', defaultLane()], {
    cwd: repoRoot,
    env: process.env,
    stdio: 'inherit',
    windowsHide: true,
  });
  if ((initResult.status ?? 1) !== 0) {
    process.exit(initResult.status ?? 1);
  }
}

function identityOptionalCommands() {
  return new Set(['init', 'root', 'doctor', 'materialize', 'streams', 'sync', 'peer']);
}

function defaultLane() {
  return process.env.LEDGER_LANE ?? process.env.OCENTRA_PARENT_LEDGER_LANE ?? inferLane(repoRoot);
}

function inferLane(path) {
  const normalized = path.replace(/\\/gu, '/');
  const match = normalized.match(/(?:^|[/_-])((?:codex-[a-z])|(?:E-[A-Z]))(?:$|[/_-])/u);
  return match?.[1] ?? 'primary';
}

function install() {
  runNpm(['install'], ledgerRoot);
}

function build() {
  runNpm(['run', 'build'], ledgerRoot);
}

function runNpm(commandArgs, cwd) {
  if (process.platform === 'win32') {
    run('cmd.exe', ['/d', '/s', '/c', ['npm', ...commandArgs].join(' ')], cwd);
    return;
  }
  run('npm', commandArgs, cwd);
}

function run(commandName, commandArgs, cwd) {
  const result = spawnSync(commandName, commandArgs, {
    cwd,
    env: process.env,
    stdio: 'inherit',
    shell: false,
    windowsHide: true,
  });
  if (result.error) {
    console.error(`${commandName} failed: ${result.error.message}`);
    process.exit(1);
  }
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function fail(lines) {
  console.error(lines.join('\n'));
  process.exit(1);
}
