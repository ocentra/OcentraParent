#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const parentRoot = path.resolve(scriptDir, '..', '..');
const parentConfig = path.join(parentRoot, 'ocentra-enforcer.config.json');

const checkAliases = new Map([
  ['check-no-zod-source', 'no-zod-source'],
  ['check-no-naked-domain-strings', 'no-naked-domain-strings'],
  ['check-no-test-doubles', 'no-test-doubles'],
  ['check-cross-platform-script-commands', 'cross-platform-script-commands'],
  ['check-source-shape', 'source-shape'],
  ['check-required-tests', 'required-tests'],
  ['check-single-source-contracts', 'single-source-contracts'],
  ['check-ai-rule-index', 'ai-rule-index'],
  ['check-no-validation-bypass', 'validation-bypass'],
  ['check-no-placeholder-implementation', 'placeholder-implementation'],
  ['check-no-skipped-focused-tests', 'skipped-focused-tests'],
  ['check-no-weak-assertions', 'weak-assertions'],
  ['check-no-tracked-generated-artifacts', 'generated-artifacts'],
  ['scan-staged-secrets', 'secrets'],
]);

export function runLegacyCheck(legacyName, argv = process.argv.slice(2)) {
  const checkName = checkAliases.get(legacyName) ?? legacyName;
  const checkArgs = legacyArgs(checkName, legacyName, argv);
  const cliPath = resolveEnforcerFile(path.join('scripts', 'rust-rules.mjs'));
  const root = process.cwd();
  const cliArgs = [cliPath, 'check', checkName, '--root', root, '--profile', 'ocentra-parent'];
  if (isInsideParent(root) && fs.existsSync(parentConfig) && !hasOption(checkArgs, '--config')) {
    cliArgs.push('--config', parentConfig);
  }
  cliArgs.push(...checkArgs);
  const result = spawnSync(process.execPath, cliArgs, {
    cwd: process.cwd(),
    env: process.env,
    stdio: 'inherit',
    shell: false,
  });
  if (result.error) {
    throw result.error;
  }
  process.exit(result.status ?? 1);
}

function legacyArgs(checkName, legacyName, argv) {
  if (legacyName === 'check-no-tracked-generated-artifacts') {
    return ['--tracked', ...argv.filter((arg) => arg !== '--all')];
  }
  if (legacyName === 'scan-staged-secrets') {
    const repoScan = argv.includes('--repo') || argv.includes('--workspace') || argv.includes('--all');
    return repoScan ? ['--workspace'] : ['--staged'];
  }
  if (checkName === 'single-source-contracts' && !hasOption(argv, '--check-config')) {
    return ['--check-config', 'scripts/check-single-source-contracts.json', ...argv];
  }
  return argv;
}

function hasOption(argv, optionName) {
  return argv.some((arg) => arg === optionName || arg.startsWith(`${optionName}=`));
}

function isInsideParent(root) {
  const relative = path.relative(parentRoot, path.resolve(root));
  return relative === '' || (!relative.startsWith('..') && !path.isAbsolute(relative));
}

function candidateRoots() {
  return [
    process.env.OCENTRA_ENFORCER_HOME,
    path.join(parentRoot, 'node_modules', 'ocentra-enforcer'),
    path.resolve(parentRoot, '..', 'ocentra-enforcer'),
    'E:\\ocentra-enforcer',
  ].filter(Boolean);
}

function resolveEnforcerFile(relativePath) {
  for (const candidate of candidateRoots()) {
    const fullPath = path.join(candidate, relativePath);
    if (fs.existsSync(fullPath)) return fullPath;
  }
  const searched = candidateRoots()
    .map((entry) => path.join(entry, relativePath))
    .join('\n  ');
  throw new Error(
    `Could not find Ocentra Enforcer ${relativePath}. Searched:\n  ${searched}\n` +
      'Set OCENTRA_ENFORCER_HOME to select a verified Enforcer checkout. GitHub Actions configures it from safety-main.'
  );
}
