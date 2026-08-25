import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

const semverPattern =
  /^(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:-[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$/u;

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function collectWorkspacePackageJsonPaths(repoRoot) {
  const paths = [join(repoRoot, 'package.json')];
  for (const workspaceRoot of ['apps', 'packages']) {
    const absoluteWorkspaceRoot = join(repoRoot, workspaceRoot);
    if (!existsSync(absoluteWorkspaceRoot)) {
      continue;
    }
    for (const entry of readdirSync(absoluteWorkspaceRoot, { withFileTypes: true })) {
      if (entry.isDirectory()) {
        const packageJsonPath = join(absoluteWorkspaceRoot, entry.name, 'package.json');
        if (existsSync(packageJsonPath)) {
          paths.push(packageJsonPath);
        }
      }
    }
  }
  return paths;
}

export function isReleaseSemver(version) {
  return semverPattern.test(version);
}

export function collectNodeVersions(repoRoot) {
  const versions = [];
  for (const packageJsonPath of collectWorkspacePackageJsonPaths(repoRoot)) {
    const packageJson = readJson(packageJsonPath);
    if (typeof packageJson.version === 'string') {
      versions.push({
        source: packageJsonPath,
        version: packageJson.version,
      });
    }
  }

  const packageLockPath = join(repoRoot, 'package-lock.json');
  if (existsSync(packageLockPath)) {
    const packageLock = readJson(packageLockPath);
    if (typeof packageLock.version === 'string') {
      versions.push({
        source: packageLockPath,
        version: packageLock.version,
      });
    }
  }

  return versions;
}

export function collectCargoVersions(repoRoot, cargoMetadataText) {
  const metadataText =
    cargoMetadataText ??
    execFileSync('cargo', ['metadata', '--format-version=1', '--no-deps'], {
      cwd: repoRoot,
      encoding: 'utf8',
    });
  const metadata = JSON.parse(metadataText);

  return metadata.packages.map((cargoPackage) => ({
    source: cargoPackage.manifest_path,
    version: cargoPackage.version,
  }));
}

export function collectPlatformVersions(repoRoot) {
  const versions = [];
  const androidBuildPath = join(repoRoot, 'platforms', 'android', 'agent', 'app', 'build.gradle');
  if (existsSync(androidBuildPath)) {
    const source = readFileSync(androidBuildPath, 'utf8');
    const match = source.match(/versionName\s*(?:=\s*)?['"]([^'"]+)['"]/u);
    if (match?.[1]) {
      versions.push({
        source: androidBuildPath,
        version: match[1],
      });
    }
  }

  const iosProjectPath = join(repoRoot, 'platforms', 'ios', 'OcentraChildAgent.xcodeproj', 'project.pbxproj');
  if (existsSync(iosProjectPath)) {
    const source = readFileSync(iosProjectPath, 'utf8');
    const matches = [...source.matchAll(/MARKETING_VERSION\s*=\s*([^;]+);/gu)].map((match) => match[1].trim());
    for (const version of [...new Set(matches)]) {
      versions.push({
        source: iosProjectPath,
        version,
      });
    }
  }

  return versions;
}

export function evaluateReleaseVersionPolicy(repoRoot, options = {}) {
  const versions = [
    ...collectNodeVersions(repoRoot),
    ...collectCargoVersions(repoRoot, options.cargoMetadataText),
    ...collectPlatformVersions(repoRoot),
  ];
  const findings = [];
  const distinctVersions = [...new Set(versions.map((entry) => entry.version))];
  const version = versions[0]?.version;

  if (versions.length === 0 || typeof version !== 'string') {
    findings.push('No release version was found.');
  }

  if (typeof version === 'string' && !isReleaseSemver(version)) {
    findings.push(`Release version ${version} is not valid SemVer.`);
  }

  if (distinctVersions.length > 1) {
    findings.push(`Release versions are not aligned: ${distinctVersions.join(', ')}.`);
  }

  for (const entry of versions) {
    if (!isReleaseSemver(entry.version)) {
      findings.push(`${entry.source} uses invalid release version ${entry.version}.`);
    }
  }

  return {
    checkedSources: versions,
    findings,
    ok: findings.length === 0,
    version,
  };
}
