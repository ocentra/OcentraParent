import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { evaluateReleaseVersionPolicy } from '../version-policy.mjs';

const repoRoot = resolve(fileURLToPath(new URL('../../..', import.meta.url)));
const version = process.env['OCENTRA_PARENT_VERSION'] ?? releaseVersion();
const packageRoot = join(repoRoot, 'target', 'release-packages', 'android');
const apkSource = join(
  repoRoot,
  'platforms',
  'android',
  'agent',
  'app',
  'build',
  'outputs',
  'apk',
  'debug',
  'app-debug.apk'
);
const apkName = `ocentra-child-agent-android-debug-v${version}.apk`;
const apkPath = join(packageRoot, apkName);
const latestPath = join(packageRoot, 'ocentra-child-agent-android-debug-latest.apk');

mkdirSync(packageRoot, { recursive: true });
runGradle();
copyFileSync(apkSource, apkPath);
copyFileSync(apkSource, latestPath);
writeChecksum(apkPath);
writeChecksum(latestPath);

console.log(`Built ${apkPath}`);
console.log(`Built ${latestPath}`);

function releaseVersion() {
  const result = evaluateReleaseVersionPolicy(repoRoot);
  if (!result.ok) {
    throw new Error(`Release version policy failed: ${result.findings.join('; ')}`);
  }
  return result.version;
}

function runGradle() {
  const projectPath = join(repoRoot, 'platforms', 'android', 'agent');
  const command = process.platform === 'win32' ? 'cmd.exe' : './gradlew';
  const args = process.platform === 'win32' ? ['/d', '/s', '/c', 'gradlew.bat assembleDebug'] : ['assembleDebug'];
  const result = spawnSync(command, args, {
    cwd: projectPath,
    stdio: 'inherit',
  });
  if (result.error) {
    throw result.error;
  }
  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}

function writeChecksum(path) {
  const checksum = createHash('sha256').update(readFileSync(path)).digest('hex').toUpperCase();
  writeFileSync(`${path}.sha256`, `${checksum}  ${path.split(/[\\/]/u).at(-1)}\n`, 'utf8');
}
