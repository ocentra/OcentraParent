import { spawnSync } from 'node:child_process';
import { mkdirSync, renameSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const outputRoot = join(process.cwd(), 'target', 'security');
mkdirSync(outputRoot, { recursive: true });

runToFile('npm', ['sbom', '--sbom-format=cyclonedx'], join(outputRoot, 'npm-sbom.cdx.json'));
runToFile('cargo', ['metadata', '--format-version=1', '--locked'], join(outputRoot, 'cargo-metadata.json'));

console.log(`SBOM and dependency metadata written to ${outputRoot}`);

function runToFile(command, args, outputPath) {
  const result = spawnSync(command, args, {
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
    shell: process.platform === 'win32',
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 1) !== 0) {
    console.error(result.stderr);
    process.exit(result.status ?? 1);
  }

  const tempPath = `${outputPath}.tmp`;
  writeFileSync(tempPath, result.stdout, 'utf8');
  renameSync(tempPath, outputPath);
}
