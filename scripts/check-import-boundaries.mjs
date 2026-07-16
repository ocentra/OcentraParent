import path from 'node:path';
import { spawnSync } from 'node:child_process';
import ts from 'typescript';
import { pathToFileURL } from 'node:url';
import { readRepoFile, repoRoot, resolveScopedFiles, toPosix } from './check-architecture-scope.mjs';

const scriptName = 'node scripts/check-import-boundaries.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];

function extensionOf(filePath) {
  return path.extname(filePath);
}

function isTypeScriptSource(filePath) {
  return ['.js', '.jsx', '.ts', '.tsx', '.mjs', '.mts', '.cjs', '.cts'].includes(extensionOf(filePath));
}

function isRustSource(filePath) {
  return extensionOf(filePath) === '.rs';
}

function normalizeSpecifier(filePath, specifier) {
  if (!specifier.startsWith('.')) {
    return specifier;
  }

  const baseDirectory = path.posix.dirname(filePath);
  return path.posix.normalize(path.posix.join(baseDirectory, specifier));
}

function packageNameFor(filePath) {
  const match = /^packages\/([^/]+)\//u.exec(filePath);
  return match?.[1] ?? null;
}

function collectTypeScriptSpecifiers(filePath) {
  const sourceText = readRepoFile(filePath);
  const sourceFile = ts.createSourceFile(filePath, sourceText, ts.ScriptTarget.Latest, true);
  const specifiers = [];

  function addSpecifier(node, moduleSpecifier) {
    if (moduleSpecifier === undefined || !ts.isStringLiteralLike(moduleSpecifier)) {
      return;
    }
    const position = sourceFile.getLineAndCharacterOfPosition(moduleSpecifier.getStart(sourceFile));
    specifiers.push({
      line: position.line + 1,
      specifier: moduleSpecifier.text,
      normalized: normalizeSpecifier(filePath, moduleSpecifier.text),
    });
  }

  function visit(node) {
    if (ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) {
      addSpecifier(node, node.moduleSpecifier);
    }
    if (
      ts.isImportEqualsDeclaration(node) &&
      ts.isExternalModuleReference(node.moduleReference) &&
      ts.isStringLiteralLike(node.moduleReference.expression)
    ) {
      addSpecifier(node, node.moduleReference.expression);
    }
    if (
      ts.isCallExpression(node) &&
      node.expression.kind === ts.SyntaxKind.ImportKeyword &&
      node.arguments.length > 0 &&
      ts.isStringLiteralLike(node.arguments[0])
    ) {
      addSpecifier(node, node.arguments[0]);
    }
    ts.forEachChild(node, visit);
  }

  visit(sourceFile);
  return specifiers;
}

function collectTypeScriptFindings(filePath) {
  const findings = [];
  const packageName = packageNameFor(filePath);

  for (const entry of collectTypeScriptSpecifiers(filePath)) {
    if (
      packageName !== null &&
      packageName.endsWith('-domain') &&
      (entry.normalized.startsWith('apps/') || entry.specifier.startsWith('apps/'))
    ) {
      findings.push(`${filePath}:${entry.line} domain packages must not import from apps/`);
    }

    if (
      filePath.startsWith('packages/schema-domain/') &&
      (entry.specifier.startsWith('@ocentra-parent/') || entry.normalized.startsWith('packages/'))
    ) {
      const importPackage = entry.specifier.startsWith('@ocentra-parent/')
        ? entry.specifier.replace('@ocentra-parent/', '').split('/')[0]
        : packageNameFor(entry.normalized);
      if (importPackage !== null && importPackage !== 'schema-domain') {
        findings.push(`${filePath}:${entry.line} schema-domain must not import from other product domains.`);
      }
    }

    if (
      filePath.startsWith('apps/portal/') &&
      (/^@ocentra-parent\/[^/]+\/src\//u.test(entry.specifier) || /^packages\/[^/]+\/src\//u.test(entry.normalized))
    ) {
      findings.push(`${filePath}:${entry.line} apps/portal must not deep import package src paths.`);
    }

    if (
      filePath.startsWith('scripts/test/') &&
      (/^@ocentra-parent\/[^/]+\/src\//u.test(entry.specifier) ||
        /^apps\/[^/]+\/src\//u.test(entry.normalized) ||
        /^packages\/[^/]+\/src\//u.test(entry.normalized))
    ) {
      findings.push(`${filePath}:${entry.line} scripts/test must not import production app internals.`);
    }
  }

  return findings;
}

let cachedMetadata = null;

function loadCargoMetadata() {
  if (cachedMetadata !== null) {
    return cachedMetadata;
  }

  const metadataResult = spawnSync('cargo', ['metadata', '--no-deps', '--format-version', '1'], {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });

  if (metadataResult.error) {
    throw metadataResult.error;
  }

  if ((metadataResult.status ?? 1) !== 0) {
    throw new Error(metadataResult.stderr?.trim() || 'cargo metadata failed');
  }

  cachedMetadata = JSON.parse(metadataResult.stdout);
  return cachedMetadata;
}

function collectRustFindings(filePath) {
  const metadata = loadCargoMetadata();
  const workspaceCrateByCodeName = new Map();
  const manifestByDirectory = new Map();

  for (const packageInfo of metadata.packages ?? []) {
    const manifestPath = toPosix(path.relative(repoRoot, packageInfo.manifest_path));
    const crateDirectory = toPosix(path.posix.dirname(manifestPath));
    manifestByDirectory.set(crateDirectory, packageInfo);
    workspaceCrateByCodeName.set(packageInfo.name.replace(/-/gu, '_'), packageInfo.name);
  }

  const crateDirectory = toPosix(path.posix.dirname(path.posix.dirname(filePath)));
  const currentPackage = manifestByDirectory.get(crateDirectory);
  if (currentPackage === undefined) {
    return [];
  }

  const declaredDependencies = new Set(
    (currentPackage.dependencies ?? []).map((dependency) => dependency.name.replace(/-/gu, '_'))
  );
  const currentCodeName = currentPackage.name.replace(/-/gu, '_');
  const lines = readRepoFile(filePath).split(/\r?\n/u);
  const findings = [];

  lines.forEach((line, index) => {
    const useMatch = /^\s*use\s+([a-zA-Z_][\w]*)/u.exec(line);
    if (useMatch === null) {
      return;
    }
    const importRoot = useMatch[1];
    if (!workspaceCrateByCodeName.has(importRoot) || importRoot === currentCodeName) {
      return;
    }
    if (!declaredDependencies.has(importRoot)) {
      findings.push(`${filePath}:${index + 1} Rust sibling crate imports require a declared Cargo dependency.`);
    }
  });

  return findings;
}

export function main(rawArgs = process.argv.slice(2)) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['apps', 'packages', 'crates', 'scripts'],
    acceptPath: (filePath) => isTypeScriptSource(filePath) || isRustSource(filePath),
  });

  if (scope.mode === 'skip') {
    console.log(scope.reason);
    return;
  }

  const findings = [];
  for (const filePath of scope.files) {
    if (isTypeScriptSource(filePath)) {
      findings.push(...collectTypeScriptFindings(filePath));
      continue;
    }
    if (isRustSource(filePath)) {
      findings.push(...collectRustFindings(filePath));
    }
  }

  if (findings.length > 0) {
    console.error('Import boundary guard failed.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(`Import boundary guard passed for ${scope.files.length} file(s).`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
