import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

import { repoRoot, resolveScopedFiles } from '../check-architecture-scope.mjs';

const scriptName = 'node scripts/git-hooks/run-precommit-validation.mjs';
const usageLines = ['--full', '--all', '--base <sha> --head <sha>'];
const prettierExtensions = new Set([
  '.cjs',
  '.css',
  '.cts',
  '.html',
  '.js',
  '.json',
  '.jsx',
  '.md',
  '.mdc',
  '.mjs',
  '.mts',
  '.scss',
  '.ts',
  '.tsx',
  '.yaml',
  '.yml',
]);
const maxPrettierChunkChars = process.platform === 'win32' ? 4000 : 16000;

const preCommitValidations = [
  ['npm', ['run', 'format:check']],
  ['npm', ['run', 'lint:schema-boundaries']],
  ['npm', ['run', 'test:tooling']],
  ['npm', ['run', 'format:rust']],
  ['cargo', ['check', '--workspace']],
];

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

function runCommand(command, args) {
  if (process.platform === 'win32' && (command === 'npm' || command === 'npx')) {
    const commandLine = [executableFor(command), ...args].map(quoteWindowsCommandPart).join(' ');
    return spawnSync(process.env.ComSpec ?? 'cmd.exe', ['/d', '/s', '/c', commandLine], {
      cwd: repoRoot,
      stdio: 'inherit',
      shell: false,
    });
  }

  return spawnSync(executableFor(command), args, {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: false,
  });
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
    roots: [
      'apps',
      'packages',
      'crates',
      'scripts',
      'docs',
      '.github',
      'vendor',
      'AGENTS.md',
      '.ocentra-ai',
      'package.json',
      'eslint.config.js',
    ],
    acceptPath: () => true,
  });

  if (scope.mode === 'skip') {
    return { files: [], workspaces: [], crates: [] };
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
    files: scope.files,
    workspaces: [...workspaces.entries()],
    crates: [...crates.entries()],
  };
}

function buildScopedValidations(scopeArgs) {
  const scope = resolveValidationScope(scopeArgs);
  const validations = [];
  const prettierFiles = scope.files.filter((filePath) => prettierExtensions.has(extensionOf(filePath)));
  const workspaceFilters = scope.workspaces.flatMap(([, packageName]) => ['--filter', packageName]);
  const crateDirs = scope.crates.map(([crateDir]) => crateDir);
  const cratePackages = scope.crates.map(([, crateName]) => crateName);
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

  validations.push([
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'generated-artifacts', '--tracked', ...scopeArgs],
  ]);
  validations.push(['npm', ['run', 'lint:architecture', '--', ...scopeArgs]]);
  validations.push([
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'no-zod-source', ...scopeArgs],
  ]);
  validations.push([
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'no-naked-domain-strings', ...scopeArgs],
  ]);
  if (touchesPortalApp) {
    validations.push([process.execPath, ['scripts/check-no-app-string-literals.mjs', ...scopeArgs]]);
    validations.push([process.execPath, ['scripts/check-portal-route-panel-contracts.mjs', ...scopeArgs]]);
  }
  if (touchesPortalVendor) {
    validations.push([process.execPath, ['scripts/check-vendor-portal-asset-imports.mjs', ...scopeArgs]]);
  }
  validations.push([
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'required-tests', ...scopeArgs],
  ]);
  validations.push([
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'single-source-contracts', ...scopeArgs],
  ]);
  validations.push([
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'no-test-doubles', ...scopeArgs],
  ]);
  validations.push([
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'cross-platform-script-commands', ...scopeArgs],
  ]);
  validations.push([
    process.execPath,
    ['scripts/enforcer/run-ocentra-enforcer.mjs', 'check', 'source-shape', ...scopeArgs],
  ]);
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
    validations.push(['cargo', ['lint-architecture', ...crateDirs]]);
    validations.push(['cargo', ['check', ...cratePackages.flatMap((crateName) => ['-p', crateName])]]);
    validations.push([
      'cargo',
      ['test', ...cratePackages.flatMap((crateName) => ['-p', crateName]), '--', '--test-threads=1'],
    ]);
  }

  return validations;
}

const { fullMode, scopeArgs } = parseArgs(process.argv.slice(2));
const scopedMode = scopeArgs.length > 0;
const validations = fullMode ? fullValidations : scopedMode ? buildScopedValidations(scopeArgs) : preCommitValidations;

console.log(
  `[validation] Running ${fullMode ? 'full integration' : scopedMode ? 'scoped batch' : 'fast pre-commit'} gate.`
);

for (const [command, args] of validations) {
  const result = runCommand(command, args);
  if (result.error) {
    console.error(`[validation] ${result.error.message}`);
    process.exit(1);
  }
  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}
