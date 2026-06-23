import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { extname, join, relative, sep } from 'node:path';
import { pathToFileURL } from 'node:url';

import { repoAbsolutePath, resolveScopedFiles } from './check-architecture-scope.mjs';

const repoRoot = process.cwd();
const configPath = 'scripts/check-single-source-contracts.json';
const sourceExtension = /\.(?:rs|ts|tsx|mjs|cjs|js|json|md|yml|yaml)$/u;
const ignoredSegments = new Set([
  '.git',
  '.hub',
  '.turbo',
  'coverage',
  'dist',
  'node_modules',
  'ocentra-ledger',
  'output',
  'target',
  'test-results',
]);
const findings = [];
const scriptName = 'node scripts/check-single-source-contracts.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const requiredMirrorRoots = ['crates/agent-protocol/src/constants'];
const nonBlockingPathPattern =
  /^(?:docs(?:\/|$)|scripts\/test(?:\/|$))|.*(?:^|\/)tests?\/|.*(?:^|\/)[^/]*_tests?\.rs$|.*(?:^|\/)[^/]*\.(?:test|spec)\.(?:ts|tsx|js|jsx|mjs|cjs)$/u;

const config = JSON.parse(readFileSync(join(repoRoot, configPath), 'utf8'));

function toPosix(path) {
  return path.split(sep).join('/');
}

function relativePath(path) {
  return toPosix(relative(repoRoot, path));
}

function hasRustSourceExtension(pathText) {
  return extname(pathText) === '.rs';
}

function shouldSkip(path, allowedPaths) {
  const pathText = relativePath(path);
  if (allowedPaths.has(pathText)) {
    return true;
  }
  if (isNonBlockingRepoPath(pathText)) {
    return true;
  }
  return pathText.split('/').some((segment) => ignoredSegments.has(segment));
}

function isNonBlockingRepoPath(pathText) {
  return nonBlockingPathPattern.test(pathText);
}

function walk(path, contract, guardedValues) {
  if (!existsSync(path) || shouldSkip(path, contract.allowedPaths)) {
    return;
  }
  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), contract, guardedValues);
    }
    return;
  }
  if (!stats.isFile() || !sourceExtension.test(path)) {
    return;
  }
  inspectFile(path, contract, guardedValues);
}

function inspectFile(path, contract, guardedValues) {
  const pathText = relativePath(path);
  const text = readFileSync(path, 'utf8');
  for (const value of guardedValues) {
    if (value.pattern.test(text)) {
      findings.push(
        `${pathText}: copied ${contract.name}.${value.name} ${value.text}; import or derive from ${contract.ownerPath}`
      );
    }
  }
}

function inspectRelativeFile(pathText, contract, guardedValues) {
  inspectFile(repoAbsolutePath(pathText), contract, guardedValues);
}

function collectFilesUnderRoot(rootPath, files, predicate) {
  const absolutePath = repoAbsolutePath(rootPath);
  if (!existsSync(absolutePath)) {
    return;
  }

  const stats = statSync(absolutePath);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(absolutePath)) {
      collectFilesUnderRoot(join(rootPath, entry), files, predicate);
    }
    return;
  }

  if (stats.isFile() && predicate(rootPath)) {
    files.push(toPosix(rootPath));
  }
}

function valueAtPath(source, jsonPath) {
  let value = source;
  for (const segment of jsonPath.split('.')) {
    if (value === null || typeof value !== 'object' || !(segment in value)) {
      throw new Error(`${jsonPath} is missing`);
    }
    value = value[segment];
  }
  return value;
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&');
}

function createLiteralMatchPattern(value) {
  // Ignore contract ids embedded inside module specifiers or longer identifiers.
  return new RegExp(`(?<![A-Za-z0-9@._/-])${escapeRegExp(value)}(?![A-Za-z0-9@._/-])`, 'u');
}

function valueAtSourceObjectPath(source, sourceObjectPath, ownerPath) {
  const lastDotIndex = sourceObjectPath.lastIndexOf('.');
  if (lastDotIndex <= 0 || lastDotIndex === sourceObjectPath.length - 1) {
    throw new Error(
      `${ownerPath}: ${sourceObjectPath} must be formatted as ObjectName.PropertyName or ObjectName.PropertyName[index]`
    );
  }
  const objectName = sourceObjectPath.slice(0, lastDotIndex);
  const propertyPath = sourceObjectPath.slice(lastDotIndex + 1);
  const arrayIndexMatch = /^(?<propertyName>[A-Za-z0-9_]+)\[(?<index>\d+)\]$/u.exec(propertyPath);
  const propertyName = arrayIndexMatch?.groups?.propertyName ?? propertyPath;
  const objectPattern = new RegExp(
    `(?:export\\s+)?const\\s+${escapeRegExp(objectName)}\\s*=\\s*\\{([\\s\\S]*?)\\}\\s*(?:as\\s+const)?`,
    'u'
  );
  const objectMatch = objectPattern.exec(source);
  const kindGroupPattern = new RegExp(
    `(?:export\\s+)?const\\s+${escapeRegExp(objectName)}\\s*=\\s*defineLiteralKindGroup\\(\\s*\\{([\\s\\S]*?)\\}\\s*(?:as\\s+const)?\\s*\\)`,
    'u'
  );
  const kindGroupMatch = kindGroupPattern.exec(source);
  const objectBody = objectMatch?.[1] ?? kindGroupMatch?.[1];
  if (objectBody === undefined) {
    throw new Error(`${ownerPath}: ${objectName} constant object is missing`);
  }
  const directStringPattern = new RegExp(`\\b${escapeRegExp(propertyName)}\\s*:\\s*(['"\`])([^'"\`]+)\\1`, 'u');
  const directStringMatch = directStringPattern.exec(objectBody);
  if (directStringMatch !== null) {
    return directStringMatch[2];
  }
  const parsedStringPattern = new RegExp(
    `\\b${escapeRegExp(propertyName)}\\s*:\\s*[A-Za-z0-9_$.]+\\.parse\\(\\s*(['"\`])([^'"\`]+)\\1\\s*\\)`,
    'u'
  );
  const parsedStringMatch = parsedStringPattern.exec(objectBody);
  if (parsedStringMatch !== null) {
    return parsedStringMatch[2];
  }
  if (arrayIndexMatch !== null) {
    const arrayPattern = new RegExp(`\\b${escapeRegExp(propertyName)}\\s*:\\s*\\[([\\s\\S]*?)\\]`, 'u');
    const arrayMatch = arrayPattern.exec(objectBody);
    if (arrayMatch === null) {
      throw new Error(`${ownerPath}: ${sourceObjectPath} array literal is missing`);
    }
    const stringMatches = [...arrayMatch[1].matchAll(/(['"`])([^'"`]+)\1/gu)];
    const index = Number.parseInt(arrayIndexMatch.groups.index, 10);
    if (index < stringMatches.length) {
      return stringMatches[index][2];
    }
    throw new Error(`${ownerPath}: ${sourceObjectPath} array entry is missing`);
  }
  if (directStringMatch === null) {
    throw new Error(`${ownerPath}: ${sourceObjectPath} string literal is missing`);
  }
}

function valueAtRustConst(source, rustConst, ownerPath) {
  const constPattern = new RegExp(
    `(?:pub\\s+)?const\\s+${escapeRegExp(rustConst)}\\s*:\\s*&str\\s*=\\s*"([^"]+)"\\s*;`,
    'u'
  );
  const constMatch = constPattern.exec(source);
  if (constMatch === null) {
    throw new Error(`${ownerPath}: ${rustConst} string const is missing`);
  }
  return constMatch[1];
}

function valueAtRustSerdeRename(source, rustSerdeRename, ownerPath) {
  const segments = rustSerdeRename.split('::');
  if (segments.length !== 2 || segments.some((segment) => segment.length === 0)) {
    throw new Error(`${ownerPath}: ${rustSerdeRename} must be formatted as EnumName::VariantName`);
  }
  const [enumName, variantName] = segments;
  const enumPattern = new RegExp(`enum\\s+${escapeRegExp(enumName)}\\s*\\{([\\s\\S]*?)\\n\\}`, 'u');
  const enumMatch = enumPattern.exec(source);
  if (enumMatch === null) {
    throw new Error(`${ownerPath}: ${enumName} enum is missing`);
  }
  const variantPattern = new RegExp(
    `#\\[serde\\(rename\\s*=\\s*"([^"]+)"\\)\\]\\s*${escapeRegExp(variantName)}\\b`,
    'u'
  );
  const variantMatch = variantPattern.exec(enumMatch[1]);
  if (variantMatch === null) {
    throw new Error(`${ownerPath}: ${rustSerdeRename} serde rename is missing`);
  }
  return variantMatch[1];
}

function valueFromSpec(ownerPath, valueSpec) {
  const sourceText = readFileSync(join(repoRoot, ownerPath), 'utf8');
  if ('jsonPath' in valueSpec) {
    return valueAtPath(JSON.parse(sourceText), valueSpec.jsonPath);
  }
  if ('sourceObjectPath' in valueSpec) {
    return valueAtSourceObjectPath(sourceText, valueSpec.sourceObjectPath, ownerPath);
  }
  if ('rustConst' in valueSpec) {
    return valueAtRustConst(sourceText, valueSpec.rustConst, ownerPath);
  }
  if ('rustSerdeRename' in valueSpec) {
    return valueAtRustSerdeRename(sourceText, valueSpec.rustSerdeRename, ownerPath);
  }
  throw new Error(`${ownerPath}: ${valueSpec.name} needs jsonPath, sourceObjectPath, rustConst, or rustSerdeRename`);
}

function loadContract(rawContract) {
  const ownerPath = rawContract.ownerPath;
  const values = rawContract.values.map((valueSpec) => {
    const text = valueFromSpec(ownerPath, valueSpec);
    if (typeof text !== 'string' || text.length === 0) {
      throw new Error(`${ownerPath}: ${valueSpec.name} must be a non-empty string`);
    }
    return { name: valueSpec.name, text, pattern: createLiteralMatchPattern(text) };
  });
  const valueByName = new Map(values.map((value) => [value.name, value.text]));
  const mirrorPaths = [];
  for (const mirror of rawContract.mirrors ?? []) {
    mirrorPaths.push(mirror.path);
    for (const mirrorValueSpec of mirror.values ?? []) {
      const ownerText = valueByName.get(mirrorValueSpec.name);
      if (ownerText === undefined) {
        throw new Error(`${mirror.path}: ${mirrorValueSpec.name} does not match an owner value name`);
      }
      const mirrorText = valueFromSpec(mirror.path, mirrorValueSpec);
      if (mirrorText !== ownerText) {
        throw new Error(
          `${mirror.path}: ${rawContract.name}.${mirrorValueSpec.name} ${mirrorText} does not match ${ownerPath} ${ownerText}`
        );
      }
    }
  }
  return {
    ...rawContract,
    allowedPaths: new Set([ownerPath, ...mirrorPaths, ...(rawContract.allowedPaths ?? [])]),
    values,
  };
}

function collectCoveredPathsForRoot(rootPath) {
  const coveredPaths = new Set();

  for (const contract of config.contracts ?? []) {
    if (contract.ownerPath?.startsWith(`${rootPath}/`)) {
      coveredPaths.add(contract.ownerPath);
    }

    for (const mirror of contract.mirrors ?? []) {
      if (mirror.path.startsWith(`${rootPath}/`)) {
        coveredPaths.add(mirror.path);
      }
    }

    for (const allowedPath of contract.allowedPaths ?? []) {
      if (allowedPath.startsWith(`${rootPath}/`)) {
        coveredPaths.add(allowedPath);
      }
    }
  }

  return coveredPaths;
}

function enforceRequiredMirrorCoverage(scopedFiles) {
  for (const rootPath of requiredMirrorRoots) {
    const coveredPaths = collectCoveredPathsForRoot(rootPath);
    const candidates =
      scopedFiles === null
        ? (() => {
            const files = [];
            collectFilesUnderRoot(rootPath, files, hasRustSourceExtension);
            return files;
          })()
        : scopedFiles.filter((filePath) => filePath.startsWith(`${rootPath}/`) && hasRustSourceExtension(filePath));

    for (const filePath of candidates) {
      if (coveredPaths.has(filePath)) {
        continue;
      }

      findings.push(
        `${filePath}: missing single-source manifest coverage; add it as a mirror/allowed path in ${configPath}`
      );
    }
  }
}

function collectScopedFiles(rawArgs) {
  const scopeRoots = [...new Set((config.contracts ?? []).flatMap((contract) => contract.scanRoots ?? []))];
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: scopeRoots,
    acceptPath: (filePath) => sourceExtension.test(filePath),
  });

  if (scope.mode === 'skip') {
    return [];
  }

  return scope.files;
}

export function main(rawArgs = process.argv.slice(2)) {
  findings.length = 0;
  const scopedFiles = rawArgs.length === 0 ? null : collectScopedFiles(rawArgs);

  enforceRequiredMirrorCoverage(scopedFiles);

  for (const rawContract of config.contracts ?? []) {
    const contract = loadContract(rawContract);
    if (scopedFiles === null) {
      for (const root of contract.scanRoots) {
        walk(join(repoRoot, root), contract, contract.values);
      }
      continue;
    }

    for (const filePath of scopedFiles) {
      if (isNonBlockingRepoPath(filePath)) {
        continue;
      }
      if (!contract.scanRoots.some((root) => filePath === root || filePath.startsWith(`${root}/`))) {
        continue;
      }
      if (contract.allowedPaths.has(filePath)) {
        continue;
      }
      inspectRelativeFile(filePath, contract, contract.values);
    }
  }

  if (findings.length > 0) {
    console.error('Single-source contract values must not be copied across repo source.');
    console.error(`Declare owned values in ${configPath}; import or derive them from the owner contract.`);
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  if (scopedFiles === null) {
    console.log(`Single-source contract check passed for ${(config.contracts ?? []).length} declared contract(s).`);
    return;
  }

  console.log(
    `Single-source contract check passed for ${scopedFiles.length} scoped file(s) across ${(config.contracts ?? []).length} declared contract(s).`
  );
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
