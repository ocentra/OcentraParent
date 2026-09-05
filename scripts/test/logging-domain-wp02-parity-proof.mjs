#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

const Plan = 'logging-domain-parity';
const Workpack = '02-typescript-logging-package-parity';
const PackageRelativePath = 'packages/logging-domain';
const PackageManifestRelativePath = `${PackageRelativePath}/package.json`;
const NoClaim = 'local TypeScript package parity only; not production telemetry or product runtime logging readiness';
const ExpectedScopes = ['parent-agent', 'parent-cloudflare', 'parent-codex', 'parent-portal', 'parent-test'];
const RequiredExports = [
  './app-log/appNdjsonWriter',
  './core/log-redaction',
  './core/logger',
  './logging-contracts',
  './test-log/ndjsonWriter',
  './transport/bridgeTransport',
];
const FileGroups = {
  appLog: 'src/app-log',
  integrationTests: 'tests/integration',
  packageScripts: 'scripts',
  testLog: 'src/test-log',
  transport: 'src/transport',
  unitTests: 'tests/unit',
};

export const Wp02ProofArtifactNames = Object.freeze([
  '00-package-export-before-after.json',
  '01-typescript-parity-file-map.json',
  '02-scope-defaults-proof.json',
  '03-query-script-smoke.json',
  '16-validation-commands.log',
]);

function metadata(artifact, status) {
  return { schemaVersion: 1, plan: Plan, workpack: Workpack, artifact, status, noClaim: NoClaim };
}

function sortedExportNames(manifest) {
  const exports = manifest?.exports;
  return exports != null && typeof exports === 'object' && !Array.isArray(exports) ? Object.keys(exports).sort() : [];
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function readBaselineManifest(repoRoot, baselineRef) {
  const result = spawnSync('git', ['show', `${baselineRef}:${PackageManifestRelativePath}`], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
  if (result.status !== 0) return { state: 'unavailable', reason: 'baseline package manifest is unavailable' };
  try {
    return { state: 'available', manifest: JSON.parse(result.stdout) };
  } catch {
    return { state: 'unavailable', reason: 'baseline package manifest is not valid JSON' };
  }
}

export function collectPackageExportProof(repoRoot, baselineRef = 'HEAD^') {
  const current = readJson(path.join(repoRoot, PackageManifestRelativePath));
  const baseline = readBaselineManifest(repoRoot, baselineRef);
  const after = sortedExportNames(current);
  const before = baseline.state === 'available' ? sortedExportNames(baseline.manifest) : [];
  const missingRequired = RequiredExports.filter((entry) => !after.includes(entry));
  const status = baseline.state !== 'available' ? 'blocked' : missingRequired.length === 0 ? 'passed' : 'failed';
  return {
    ...metadata('package-export-before-after', status),
    baselineRef,
    before: baseline.state === 'available' ? { state: 'available', exports: before } : baseline,
    after: { packageName: current.name ?? null, packageVersion: current.version ?? null, exports: after },
    added: after.filter((entry) => !before.includes(entry)),
    removed: before.filter((entry) => !after.includes(entry)),
    requiredExports: RequiredExports,
    missingRequiredExports: missingRequired,
  };
}

function collectTypeScriptFiles(root) {
  if (!fs.existsSync(root)) return [];
  const files = [];
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    const absolute = path.join(root, entry.name);
    if (entry.isDirectory()) {
      files.push(...collectTypeScriptFiles(absolute));
    } else if (entry.isFile() && entry.name.endsWith('.ts')) {
      files.push(absolute);
    }
  }
  return files.sort();
}

export function collectParityFileMapProof(repoRoot) {
  const packageRoot = path.join(repoRoot, PackageRelativePath);
  const groups = Object.fromEntries(
    Object.entries(FileGroups).map(([name, relativeRoot]) => {
      const files = collectTypeScriptFiles(path.join(packageRoot, relativeRoot)).map((entry) =>
        path.relative(packageRoot, entry).split(path.sep).join('/')
      );
      return [name, { root: relativeRoot, files }];
    })
  );
  const missingGroups = Object.entries(groups)
    .filter(([, group]) => group.files.length === 0)
    .map(([name]) => name);
  return {
    ...metadata('typescript-parity-file-map', missingGroups.length === 0 ? 'passed' : 'failed'),
    packageRoot: PackageRelativePath,
    groups,
    missingGroups,
  };
}

export function blockedScopeDefaultsProof(reason) {
  return { ...metadata('scope-defaults-proof', 'blocked'), expectedScopes: ExpectedScopes, reason };
}

async function collectScopeDefaultsProof(repoRoot, buildResult) {
  if (buildResult.status !== 'passed') return blockedScopeDefaultsProof('package build did not pass');
  try {
    const modulePath = path.join(repoRoot, PackageRelativePath, 'dist/test-log/types.js');
    const runtime = await import(`${pathToFileURL(modulePath).href}?proof=${Date.now()}`);
    const scopes = Object.values(runtime.TestLogScope).sort();
    const nullDefault = runtime.parseTestLogScopeOrDefault(null);
    const blankDefault = runtime.parseTestLogScopeOrDefault('  ');
    const passed =
      JSON.stringify(scopes) === JSON.stringify(ExpectedScopes) &&
      nullDefault === 'parent-test' &&
      blankDefault === 'parent-test';
    return {
      ...metadata('scope-defaults-proof', passed ? 'passed' : 'failed'),
      expectedScopes: ExpectedScopes,
      observedScopes: scopes,
      nullDefault,
      blankDefault,
      cloudflareIsExplicitOnly: scopes.includes('parent-cloudflare') && nullDefault !== 'parent-cloudflare',
    };
  } catch {
    return blockedScopeDefaultsProof('built scope runtime could not be imported');
  }
}

function npmExecutable() {
  return process.platform === 'win32' ? 'npm.cmd' : 'npm';
}

export function resolveNpmInvocation(args, npmExecPath = process.env.npm_execpath) {
  return typeof npmExecPath === 'string' && npmExecPath.length > 0
    ? { command: process.execPath, args: [npmExecPath, ...args] }
    : { command: npmExecutable(), args };
}

function runNpm(repoRoot, args, display) {
  const invocation = resolveNpmInvocation(args);
  const result = spawnSync(invocation.command, invocation.args, {
    cwd: repoRoot,
    encoding: 'utf8',
    maxBuffer: 32 * 1024 * 1024,
  });
  const exitCode = result.status ?? 1;
  return {
    command: display,
    exitCode,
    status: exitCode === 0 ? 'passed' : 'failed',
    stdout: result.stdout ?? '',
    diagnostics: result.error == null ? `command exited with code ${exitCode}` : 'command could not be started',
  };
}

export function createQueryScriptSmokeProof(commandResult) {
  let stats = null;
  try {
    stats = JSON.parse(commandResult.stdout.trim());
  } catch {
    stats = null;
  }
  const expectedKeys = ['distinctRuns', 'distinctTests', 'errorLogs', 'newestTimestamp', 'totalLogs', 'warnLogs'];
  const observedKeys =
    stats != null && typeof stats === 'object' && !Array.isArray(stats) ? Object.keys(stats).sort() : [];
  const parsed = commandResult.status === 'passed' && JSON.stringify(observedKeys) === JSON.stringify(expectedKeys);
  return {
    ...metadata('query-script-smoke', parsed ? 'passed' : 'failed'),
    command: commandResult.command,
    exitCode: commandResult.exitCode,
    scope: 'parent-test',
    stats: parsed ? stats : null,
    diagnostics: parsed ? 'bounded stats payload parsed' : commandResult.diagnostics,
  };
}

export function formatValidationCommandLog(results) {
  return `${results
    .map(
      (result) =>
        `plan: ${Plan}\nworkpack: ${Workpack}\nowner: logging-domain\nscope: parent-test\ncommand: ${result.command}\nexit: ${result.exitCode}\nresult: ${result.status}\nartifact: ${result.artifact ?? 'n/a'}\nnotes: ${result.diagnostics}\nno_claim: ${NoClaim}`
    )
    .join('\n\n')}\n`;
}

export function writeWp02ProofArtifacts(outputDirectory, artifacts) {
  const names = Object.keys(artifacts).sort();
  const expected = [...Wp02ProofArtifactNames].sort();
  if (JSON.stringify(names) !== JSON.stringify(expected)) throw new Error('WP02 proof artifact set is incomplete');
  fs.mkdirSync(outputDirectory, { recursive: true });
  const unexpected = fs.readdirSync(outputDirectory).filter((entry) => !Wp02ProofArtifactNames.includes(entry));
  if (unexpected.length > 0) throw new Error('WP02 proof output contains unexpected retained artifacts');
  for (const name of Wp02ProofArtifactNames) {
    const value = artifacts[name];
    const content = name.endsWith('.json') ? `${JSON.stringify(value, null, 2)}\n` : String(value);
    fs.writeFileSync(path.join(outputDirectory, name), content, 'utf8');
  }
}

export async function runWp02ParityProof({ repoRoot, outputDirectory, baselineRef = 'HEAD^' }) {
  const build = runNpm(
    repoRoot,
    ['run', '--silent', 'build', '--workspace', '@ocentra-parent/logging-domain'],
    'npm run build --workspace @ocentra-parent/logging-domain'
  );
  const tests = runNpm(
    repoRoot,
    ['run', '--silent', 'test', '--workspace', '@ocentra-parent/logging-domain'],
    'npm run test --workspace @ocentra-parent/logging-domain'
  );
  const query = runNpm(
    repoRoot,
    [
      'run',
      '--silent',
      'test:query',
      '--workspace',
      '@ocentra-parent/logging-domain',
      '--',
      'stats',
      '--scope=parent-test',
    ],
    'npm run test:query --workspace @ocentra-parent/logging-domain -- stats --scope=parent-test'
  );
  const exportProof = collectPackageExportProof(repoRoot, baselineRef);
  const fileMap = collectParityFileMapProof(repoRoot);
  const scopeProof = await collectScopeDefaultsProof(repoRoot, build);
  const queryProof = createQueryScriptSmokeProof(query);
  const commandLog = formatValidationCommandLog([
    { ...build, artifact: '00-package-export-before-after.json' },
    { ...tests, artifact: '01-typescript-parity-file-map.json' },
    { ...query, artifact: '03-query-script-smoke.json' },
  ]);
  writeWp02ProofArtifacts(outputDirectory, {
    '00-package-export-before-after.json': exportProof,
    '01-typescript-parity-file-map.json': fileMap,
    '02-scope-defaults-proof.json': scopeProof,
    '03-query-script-smoke.json': queryProof,
    '16-validation-commands.log': commandLog,
  });
  const artifactsPassed = [exportProof, fileMap, scopeProof, queryProof].every(
    (artifact) => artifact.status === 'passed'
  );
  return artifactsPassed && build.status === 'passed' && tests.status === 'passed' ? 0 : 1;
}

function optionValue(name) {
  const prefix = `--${name}=`;
  return (
    process.argv
      .slice(2)
      .find((entry) => entry.startsWith(prefix))
      ?.slice(prefix.length) ?? null
  );
}

const invokedPath = process.argv[1] == null ? null : path.resolve(process.argv[1]);
if (invokedPath === fileURLToPath(import.meta.url)) {
  const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
  const outputDirectory = path.resolve(
    optionValue('output-dir') ??
      path.join(repoRoot, 'output/logging-domain-parity-proof/02-typescript-logging-package-parity')
  );
  const exitCode = await runWp02ParityProof({ repoRoot, outputDirectory, baselineRef: optionValue('base') ?? 'HEAD^' });
  process.stdout.write(`${path.relative(repoRoot, outputDirectory).split(path.sep).join('/')}\n`);
  process.exitCode = exitCode;
}
