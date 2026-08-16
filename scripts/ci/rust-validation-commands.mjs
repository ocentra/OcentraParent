import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

function namedTestTargetArgs(crateDir, excludedTestTargets) {
  if (excludedTestTargets.length === 0) return [];
  const manifestPath = path.join(crateDir, 'Cargo.toml');
  if (!existsSync(manifestPath)) return [];
  const targetNames = [
    ...readFileSync(manifestPath, 'utf8').matchAll(/\[\[test\]\][\s\S]*?^\s*name\s*=\s*"([^"]+)"\s*$/gmu),
  ]
    .map((match) => match[1])
    .filter((name) => excludedTestTargets.includes(name) === false);
  return targetNames.flatMap((name) => ['--test', name]);
}

export function buildCrateRustValidationCommands(crateDir, { excludedTestTargets = [] } = {}) {
  const manifestPath = `${crateDir}/Cargo.toml`;
  const testTargetArgs = namedTestTargetArgs(crateDir, excludedTestTargets);
  return [
    ['cargo', ['check', '--manifest-path', manifestPath]],
    ['cargo', ['test', '--manifest-path', manifestPath, ...testTargetArgs]],
  ];
}

export function buildWorkspaceRustValidationCommands() {
  return [
    ['npm', ['run', 'format:rust']],
    ['npm', ['run', 'lint:rust']],
    ['cargo', ['check', '--workspace']],
    ['cargo', ['test', '--workspace']],
  ];
}
