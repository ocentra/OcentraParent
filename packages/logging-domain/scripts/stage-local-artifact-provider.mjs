import childProcess from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';
import { stageLocalArtifactProvider } from './stage-local-artifact-provider-library.mjs';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const workspaceRoot = path.resolve(packageRoot, '..', '..');
const packageJsonPath = path.join(packageRoot, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
if (
  packageJson.name !== '@ocentra-parent/logging-domain' ||
  typeof packageJson.version !== 'string' ||
  packageJson.version.length === 0
) {
  throw new Error('logging package identity is invalid');
}

if (process.platform !== 'win32') {
  process.stdout.write('local artifact provider staging is not required on this platform\n');
  process.exit(0);
}

const cargoTargetRoot = process.env.CARGO_TARGET_DIR
  ? path.resolve(workspaceRoot, process.env.CARGO_TARGET_DIR)
  : path.join(workspaceRoot, 'target');
const executableName = 'ocentra-logging-local-artifact-provider.exe';
const sourceBinary = path.join(cargoTargetRoot, 'release', executableName);
const outputRoot = path.join(packageRoot, 'dist');
const providerDirectory = path.join(outputRoot, 'local-artifact-provider');
const manifestPath = path.join(outputRoot, 'local-artifact-provider.manifest.json');

const build = childProcess.spawnSync(
  'cargo',
  ['build', '--locked', '--release', '--package', 'ocentra-logging-local-artifact-provider'],
  { cwd: workspaceRoot, encoding: 'utf8', stdio: 'inherit' }
);
if (build.error != null) throw build.error;
if (build.status !== 0) {
  throw new Error(`local artifact provider build failed with exit code ${String(build.status)}`);
}

const staged = stageLocalArtifactProvider({
  sourceBinary,
  providerDirectory,
  manifestPath,
  packageName: '@ocentra-parent/logging-domain',
  packageVersion: packageJson.version,
});
process.stdout.write(`staged pinned local artifact provider ${staged.binarySha256}\n`);
