import { spawn, spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { repoRoot, resolveScopedFiles } from '../check-architecture-scope.mjs';
import { buildCrateRustValidationCommands } from '../ci/rust-validation-commands.mjs';

const scriptName = 'node scripts/git-hooks/run-precommit-validation.mjs';
const usageLines = ['--full', '--all', '--base <sha> --head <sha>'];
const validationRoots = [
  'apps',
  'packages',
  'crates',
  'scripts',
  'tests',
  'docs',
  '.github',
  'vendor',
  'AGENTS.md',
  '.ocentra-ai',
  '.prettierignore',
  'package.json',
  'eslint.config.js',
];
const prettierExtensions = new Set([
  '.cjs',
  '.css',
  '.cts',
  '.html',
  '.js',
  '.json',
  '.jsx',
  '.md',
  '.mjs',
  '.mts',
  '.scss',
  '.ts',
  '.tsx',
  '.yaml',
  '.yml',
]);
const maxPrettierChunkChars = process.platform === 'win32' ? 4000 : 16000;
const maxScopedFileChunkChars = process.platform === 'win32' ? 4000 : 14000;
const defaultCommandTimeoutMs = 30 * 60 * 1000;

const fullValidations = [
  ['npm', ['run', 'format:check']],
  ['npm', ['run', 'validate']],
  ['npm', ['run', 'build']],
];

function executableFor(command) {
  if (process.platform !== 'win32') {
    return command;
  }

  if (command === 'npm' || command === 'npx') {
    return `${command}.cmd`;
  }

  return command;
}

function quoteWindowsCommandPart(value) {
  if (value.length === 0) {
    return '""';
  }

  if (!/[\s"&()]/.test(value)) {
    return value;
  }

  let quoted = '"';
  let trailingBackslashes = 0;

  for (const character of value) {
    if (character === '\\') {
      trailingBackslashes += 1;
      continue;
    }

    if (character === '"') {
      quoted += '\\'.repeat(trailingBackslashes * 2 + 1);
      quoted += '"';
      trailingBackslashes = 0;
      continue;
    }

    quoted += '\\'.repeat(trailingBackslashes);
    quoted += character;
    trailingBackslashes = 0;
  }

  quoted += '\\'.repeat(trailingBackslashes * 2);
  quoted += '"';
  return quoted;
}

function commandDisplay(command, args) {
  return [command, ...args].map(quoteWindowsCommandPart).join(' ');
}

function commandTimeoutMs() {
  const configured = Number.parseInt(process.env.OCENTRA_PRECOMMIT_COMMAND_TIMEOUT_MS ?? '', 10);
  return Number.isSafeInteger(configured) && configured > 0 ? configured : defaultCommandTimeoutMs;
}

function terminateProcessTree(child) {
  if (child.pid === undefined) {
    return;
  }

  if (process.platform === 'win32') {
    const killer = spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], {
      stdio: 'ignore',
      windowsHide: true,
      shell: false,
    });
    killer.once('error', () => child.kill());
    return;
  }

  try {
    process.kill(-child.pid, 'SIGTERM');
  } catch {
    child.kill('SIGTERM');
  }
}

export function runCommand(command, args, { timeoutMs = commandTimeoutMs() } = {}) {
  const env =
    process.platform === 'win32' && command === 'cargo'
      ? { ...process.env, CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS ?? '1' }
      : process.env;

  const isWindowsBatchCommand = process.platform === 'win32' && (command === 'npm' || command === 'npx');
  const file = isWindowsBatchCommand ? (process.env.ComSpec ?? 'cmd.exe') : executableFor(command);
  const childArgs = isWindowsBatchCommand ? ['/d', '/s', '/c', commandDisplay(executableFor(command), args)] : args;
  const display = commandDisplay(command, args);

  console.log(`[validation] running: ${display}`);

  return new Promise((resolve) => {
    let timedOut = false;
    let settled = false;
    const child = spawn(file, childArgs, {
      cwd: repoRoot,
      stdio: 'inherit',
      shell: false,
      env,
      windowsHide: true,
      detached: process.platform !== 'win32',
    });
    const timeout = setTimeout(() => {
      timedOut = true;
      console.error(`[validation] timed out after ${timeoutMs}ms: ${display}`);
      terminateProcessTree(child);
    }, timeoutMs);

    const finish = (result) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timeout);
      resolve(result);
    };

    child.once('error', (error) => finish({ status: 1, error, timedOut }));
    child.once('close', (status, signal) => finish({ status: status ?? 1, signal, timedOut }));
  });
}

function runGitCommand(args, failureLabel) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 1) !== 0) {
    throw new Error(`${failureLabel}: ${result.stderr?.trim() || 'git command failed'}`);
  }

  return result.stdout.trim();
}

function gitOutputLines(args, failureLabel) {
  const output = runGitCommand(args, failureLabel);
  return output === '' ? [] : output.split(/\r?\n/u).filter(Boolean);
}

function extensionOf(filePath) {
  return path.extname(filePath);
}

function chunkValues(values, maxChars) {
  const chunks = [];
  let currentChunk = [];
  let currentLength = 0;

  for (const value of values) {
    const nextLength = currentLength + value.length + 1;
    if (currentChunk.length > 0 && nextLength > maxChars) {
      chunks.push(currentChunk);
      currentChunk = [value];
      currentLength = value.length + 1;
      continue;
    }

    currentChunk.push(value);
    currentLength = nextLength;
  }

  if (currentChunk.length > 0) {
    chunks.push(currentChunk);
  }

  return chunks;
}

function parseArgs(rawArgs) {
  let fullMode = false;
  const scopeArgs = [];

  for (const arg of rawArgs) {
    if (arg === '--full') {
      fullMode = true;
      continue;
    }
    scopeArgs.push(arg);
  }

  if (fullMode && scopeArgs.length > 0) {
    throw new Error(
      `Usage: ${scriptName}\n  ${scriptName}\n  ${scriptName} --full\n  ${scriptName} --files <path> [more paths]\n  ${scriptName} --base <sha> --head <sha>\n  ${scriptName} --all`
    );
  }

  return { fullMode, scopeArgs };
}

function resolveValidationScope(scopeArgs) {
  const scope = resolveScopedFiles(scopeArgs, {
    scriptName,
    usageLines,
    roots: validationRoots,
    acceptPath: () => true,
  });

  if (scope.mode === 'skip') {
    return { ...scope, files: [], workspaces: [], crates: [] };
  }

  const workspaces = new Map();
  const crates = new Map();

  for (const filePath of scope.files) {
    const segments = filePath.split('/');
    if (segments.length < 2) {
      continue;
    }

    if (segments[0] === 'packages' || segments[0] === 'apps') {
      const workspaceDir = `${segments[0]}/${segments[1]}`;
      const manifestPath = path.join(repoRoot, workspaceDir, 'package.json');
      if (!existsSync(manifestPath) || workspaces.has(workspaceDir)) {
        continue;
      }
      const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
      if (typeof manifest.name === 'string' && manifest.name.length > 0) {
        workspaces.set(workspaceDir, manifest.name);
      }
      continue;
    }

    if (segments[0] === 'crates') {
      const crateDir = `${segments[0]}/${segments[1]}`;
      if (crates.has(crateDir)) {
        continue;
      }
      const cargoTomlPath = path.join(repoRoot, crateDir, 'Cargo.toml');
      if (!existsSync(cargoTomlPath)) {
        continue;
      }
      const cargoToml = readFileSync(cargoTomlPath, 'utf8');
      const nameMatch = cargoToml.match(/^\s*name\s*=\s*"([^"]+)"/mu);
      if (nameMatch !== null) {
        crates.set(crateDir, nameMatch[1]);
      }
    }
  }

  return {
    ...scope,
    files: scope.files,
    workspaces: [...workspaces.entries()],
    crates: [...crates.entries()],
  };
}

function collectStagedFiles() {
  const tracked = gitOutputLines(
    ['diff', '--cached', '--name-only', '--diff-filter=ACMR', '--', ...validationRoots],
    'failed to list staged files'
  );

  return [...new Set(tracked)];
}

function scopedValidationArgBatches(scope, fallbackScopeArgs) {
  if (scope.mode !== 'files') {
    return [fallbackScopeArgs];
  }

  return chunkValues(scope.files, maxScopedFileChunkChars).map((files) => ['--files', ...files]);
}

function buildScopedValidations(scopeArgs, { prettierFiles: explicitPrettierFiles = null } = {}) {
  const scope = resolveValidationScope(scopeArgs);
  const validations = [];
  const validationScopeArgsBatches = scopedValidationArgBatches(scope, scopeArgs);
  const prettierInputs = explicitPrettierFiles ?? scope.files;
  const prettierFiles = prettierInputs.filter((filePath) => prettierExtensions.has(extensionOf(filePath)));
  const workspaceFilters = scope.workspaces.flatMap(([, packageName]) => ['--filter', packageName]);
  const crateDirs = scope.crates.map(([crateDir]) => crateDir);
  const touchesPortalApp = scope.files.some((filePath) => filePath.startsWith('apps/portal/'));
  const touchesPortalVendor = scope.files.some((filePath) =>
    filePath.startsWith('vendor/ocentra-parent-core-ui/AppPages/ParentPortal/')
  );
  const touchesLoggingDomain = scope.files.some((filePath) => filePath.startsWith('packages/logging-domain/'));
  const touchesRuleIndexSurface = scope.files.some(
    (filePath) => filePath === 'AGENTS.md' || filePath.startsWith('.ocentra-ai/') || filePath.startsWith('docs/agent/')
  );

  if (prettierFiles.length > 0) {
    for (const prettierChunk of chunkValues(prettierFiles, maxPrettierChunkChars)) {
      validations.push(['npx', ['prettier', '--check', ...prettierChunk]]);
    }
  }

  for (const validationScopeArgs of validationScopeArgsBatches) {
    validations.push([
      process.execPath,
      ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'generated-artifacts', ...validationScopeArgs],
    ]);
    validations.push(['npm', ['run', 'lint:architecture', '--', ...validationScopeArgs]]);
    validations.push([
      process.execPath,
      ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'no-zod-source', ...validationScopeArgs],
    ]);
    validations.push([
      process.execPath,
      ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'no-naked-domain-strings', ...validationScopeArgs],
    ]);
    if (touchesPortalVendor) {
      validations.push([process.execPath, ['scripts/check-vendor-portal-asset-imports.mjs', ...validationScopeArgs]]);
    }
    validations.push([
      process.execPath,
      ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'required-tests', ...validationScopeArgs],
    ]);
    validations.push([
      process.execPath,
      [
        'scripts/enforcer/run-ocentra-enforcer.mjs',
        'check',
        'single-source-contracts',
        '--check-config',
        'scripts/check-single-source-contracts.json',
        ...validationScopeArgs,
      ],
    ]);
    validations.push([
      process.execPath,
      ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'no-test-doubles', ...validationScopeArgs],
    ]);
    validations.push([
      process.execPath,
      ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'cross-platform-script-commands', ...validationScopeArgs],
    ]);
    validations.push([
      process.execPath,
      ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'source-shape', ...validationScopeArgs],
    ]);
  }
  if (touchesPortalApp) {
    const portalSourceScope = ['--files', 'apps/portal/src'];
    validations.push([process.execPath, ['scripts/check-no-app-string-literals.mjs', ...portalSourceScope]]);
    validations.push([process.execPath, ['scripts/check-portal-route-panel-contracts.mjs', ...portalSourceScope]]);
  }
  if (touchesLoggingDomain) {
    validations.push(['npm', ['run', 'lint:logging-parity']]);
  }
  if (touchesRuleIndexSurface) {
    validations.push([process.execPath, ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'ai-rule-index']]);
  }

  if (workspaceFilters.length > 0) {
    validations.push(['npx', ['turbo', 'run', 'build', 'type-check', 'test', ...workspaceFilters]]);
  }

  if (crateDirs.length > 0) {
    validations.push([
      process.execPath,
      ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'reexports', '--files', crateDirs.join(',')],
    ]);
    for (const [crateDir] of scope.crates) {
      validations.push(...buildCrateRustValidationCommands(crateDir));
    }
  }

  return validations;
}

function buildFastPreCommitValidations() {
  const stagedFiles = collectStagedFiles();
  if (stagedFiles.length === 0) {
    return [];
  }

  const validations = buildScopedValidations(['--files', ...stagedFiles], { prettierFiles: stagedFiles });
  const touchesRust = stagedFiles.some((filePath) => extensionOf(filePath) === '.rs');
  const touchesTooling = stagedFiles.some(
    (filePath) =>
      filePath === 'package.json' ||
      filePath === 'eslint.config.js' ||
      filePath.startsWith('scripts/') ||
      filePath.startsWith('tests/repo-tooling/')
  );

  if (touchesTooling) {
    validations.push(['npm', ['run', 'test:tooling']]);
  }

  if (touchesRust) {
    validations.push(['npm', ['run', 'format:rust']]);
  }

  return validations;
}

async function main() {
  const { fullMode, scopeArgs } = parseArgs(process.argv.slice(2));
  const scopedMode = scopeArgs.length > 0;
  const validations = fullMode
    ? fullValidations
    : scopedMode
      ? buildScopedValidations(scopeArgs)
      : buildFastPreCommitValidations();

  console.log(
    `[validation] Running ${fullMode ? 'full integration' : scopedMode ? 'scoped batch' : 'fast pre-commit'} gate.`
  );

  if (validations.length === 0) {
    console.log('[validation] No relevant working-tree files detected.');
    return;
  }

  for (const [command, args] of validations) {
    const result = await runCommand(command, args);
    if (result.error) {
      console.error(`[validation] failed to start ${commandDisplay(command, args)}: ${result.error.message}`);
      process.exitCode = 1;
      return;
    }
    if (result.timedOut) {
      process.exitCode = 1;
      return;
    }
    if (result.status !== 0) {
      console.error(`[validation] failed with exit ${result.status}: ${commandDisplay(command, args)}`);
      process.exitCode = result.status;
      return;
    }
  }
}

if (process.argv[1] !== undefined && fileURLToPath(import.meta.url) === path.resolve(process.argv[1])) {
  await main();
}
