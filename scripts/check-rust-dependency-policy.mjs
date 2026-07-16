import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { pathToFileURL } from 'node:url';
import { repoRoot, resolveScopedFiles, toPosix } from './check-architecture-scope.mjs';

const scriptName = 'node scripts/check-rust-dependency-policy.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const blockedProtocolDependencies = new Set([
  'ocentra-parent-agent-core',
  'ocentra-parent-agent-service',
  'ocentra-child-runtime',
  'ocentra-parent-runtime-core',
  'ocentra-parent-screen-capture-adapter',
]);
const runtimeCrates = new Set([
  'ocentra-parent-agent-core',
  'ocentra-parent-agent-service',
  'ocentra-child-runtime',
  'ocentra-parent-runtime-core',
  'ocentra-parent-screen-capture-adapter',
]);
const testOnlyCrates = new Set(['criterion', 'mockall', 'pretty_assertions', 'proptest', 'rstest', 'wiremock']);
const allowedGitDependencies = new Set();

function isCargoPolicyPath(filePath) {
  return filePath === 'Cargo.toml' || filePath === 'Cargo.lock' || filePath.endsWith('/Cargo.toml');
}

function loadMetadata() {
  const result = spawnSync('cargo', ['metadata', '--no-deps', '--format-version', '1'], {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 1) !== 0) {
    throw new Error(result.stderr?.trim() || 'cargo metadata failed');
  }

  return JSON.parse(result.stdout);
}

function packageKeyForScope(scopedFiles) {
  const packageDirectories = new Set();
  let includeWorkspaceRoot = false;

  for (const filePath of scopedFiles) {
    if (filePath === 'Cargo.toml' || filePath === 'Cargo.lock') {
      includeWorkspaceRoot = true;
      continue;
    }
    if (filePath.endsWith('/Cargo.toml')) {
      packageDirectories.add(toPosix(path.posix.dirname(filePath)));
    }
  }

  return { includeWorkspaceRoot, packageDirectories };
}

function collectFindingsForPackage(packageInfo, metadata, scopedPackages) {
  const findings = [];
  const workspaceRoot = toPosix(metadata.workspace_root);

  for (const dependency of packageInfo.dependencies ?? []) {
    if ((dependency.source ?? '').startsWith('git+') && !allowedGitDependencies.has(dependency.name)) {
      findings.push(`${packageInfo.manifest_path}: git dependencies are forbidden without an explicit allowlist.`);
    }

    const dependencyPath = dependency.path ?? null;
    if (dependencyPath !== null) {
      const normalizedPath = toPosix(dependencyPath);
      if (!normalizedPath.startsWith(workspaceRoot)) {
        findings.push(`${packageInfo.manifest_path}: path dependencies must stay inside the workspace root.`);
      }
    }

    if (dependencyPath === null && dependency.req.trim() === '*') {
      findings.push(`${packageInfo.manifest_path}: wildcard registry dependency versions are forbidden.`);
    }

    if (blockedProtocolDependencies.has(dependency.name) && packageInfo.name === 'ocentra-parent-agent-protocol') {
      findings.push(`${packageInfo.manifest_path}: agent-protocol must not depend on runtime or service crates.`);
    }

    if (runtimeCrates.has(packageInfo.name) && testOnlyCrates.has(dependency.name) && dependency.kind !== 'dev') {
      findings.push(`${packageInfo.manifest_path}: runtime crates must not depend on test-only crates.`);
    }
  }

  return findings;
}

export function main(rawArgs = process.argv.slice(2)) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['Cargo.toml', 'Cargo.lock', 'crates'],
    acceptPath: isCargoPolicyPath,
  });

  if (scope.mode === 'skip') {
    console.log(scope.reason);
    return;
  }

  const metadata = loadMetadata();
  const { includeWorkspaceRoot, packageDirectories } = packageKeyForScope(scope.files);
  const scopedPackages =
    scope.mode === 'all' || includeWorkspaceRoot
      ? metadata.packages
      : (metadata.packages ?? []).filter((packageInfo) =>
          packageDirectories.has(
            toPosix(path.posix.dirname(toPosix(path.relative(repoRoot, packageInfo.manifest_path))))
          )
        );

  if (scopedPackages.length === 0) {
    console.log('Rust dependency policy skipped: no Cargo manifests matched the requested scope.');
    return;
  }

  const findings = scopedPackages.flatMap((packageInfo) =>
    collectFindingsForPackage(packageInfo, metadata, scopedPackages)
  );
  const registryReqsByName = new Map();
  for (const packageInfo of scopedPackages) {
    for (const dependency of packageInfo.dependencies ?? []) {
      if (dependency.path !== null || dependency.kind === 'dev' || (dependency.source ?? '').startsWith('git+')) {
        continue;
      }
      if (!registryReqsByName.has(dependency.name)) {
        registryReqsByName.set(dependency.name, new Set());
      }
      registryReqsByName.get(dependency.name).add(dependency.req);
    }
  }
  for (const [dependencyName, reqs] of registryReqsByName) {
    if (reqs.size > 1) {
      findings.push(
        `changed Cargo manifests request multiple direct versions for ${dependencyName}; align them before landing.`
      );
    }
  }
  if (findings.length > 0) {
    console.error('Rust dependency policy guard failed.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(`Rust dependency policy guard passed for ${scopedPackages.length} manifest(s).`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
