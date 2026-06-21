import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative, sep } from 'node:path';

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

const config = JSON.parse(readFileSync(join(repoRoot, configPath), 'utf8'));

function toPosix(path) {
  return path.split(sep).join('/');
}

function relativePath(path) {
  return toPosix(relative(repoRoot, path));
}

function shouldSkip(path, allowedPaths) {
  const pathText = relativePath(path);
  if (allowedPaths.has(pathText)) {
    return true;
  }
  return pathText.split('/').some((segment) => ignoredSegments.has(segment));
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
    if (text.includes(value.text)) {
      findings.push(
        `${pathText}: copied ${contract.name}.${value.name} ${value.text}; import or derive from ${contract.ownerPath}`
      );
    }
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

function valueAtSourceObjectPath(source, sourceObjectPath, ownerPath) {
  const lastDotIndex = sourceObjectPath.lastIndexOf('.');
  if (lastDotIndex <= 0 || lastDotIndex === sourceObjectPath.length - 1) {
    throw new Error(`${ownerPath}: ${sourceObjectPath} must be formatted as ObjectName.PropertyName`);
  }
  const objectName = sourceObjectPath.slice(0, lastDotIndex);
  const propertyName = sourceObjectPath.slice(lastDotIndex + 1);
  const objectPattern = new RegExp(
    `(?:export\\s+)?const\\s+${escapeRegExp(objectName)}\\s*=\\s*\\{([\\s\\S]*?)\\}\\s*(?:as\\s+const)?`,
    'u'
  );
  const objectMatch = objectPattern.exec(source);
  if (objectMatch === null) {
    throw new Error(`${ownerPath}: ${objectName} constant object is missing`);
  }
  const directStringPattern = new RegExp(`\\b${escapeRegExp(propertyName)}\\s*:\\s*(['"\`])([^'"\`]+)\\1`, 'u');
  const directStringMatch = directStringPattern.exec(objectMatch[1]);
  if (directStringMatch !== null) {
    return directStringMatch[2];
  }
  const parsedStringPattern = new RegExp(
    `\\b${escapeRegExp(propertyName)}\\s*:\\s*[A-Za-z0-9_$.]+\\.parse\\(\\s*(['"\`])([^'"\`]+)\\1\\s*\\)`,
    'u'
  );
  const parsedStringMatch = parsedStringPattern.exec(objectMatch[1]);
  if (parsedStringMatch !== null) {
    return parsedStringMatch[2];
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
  const separatorIndex = rustSerdeRename.indexOf('::');
  if (separatorIndex <= 0 || separatorIndex === rustSerdeRename.length - 2) {
    throw new Error(`${ownerPath}: ${rustSerdeRename} must be formatted as EnumName::VariantName`);
  }
  const enumName = rustSerdeRename.slice(0, separatorIndex);
  const variantName = rustSerdeRename.slice(separatorIndex + 2);
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
  throw new Error(
    `${ownerPath}: ${valueSpec.name} needs jsonPath, sourceObjectPath, rustConst, or rustSerdeRename`
  );
}

function loadContract(rawContract) {
  const ownerPath = rawContract.ownerPath;
  const values = rawContract.values.map((valueSpec) => {
    const text = valueFromSpec(ownerPath, valueSpec);
    if (typeof text !== 'string' || text.length === 0) {
      throw new Error(`${ownerPath}: ${valueSpec.name} must be a non-empty string`);
    }
    return { name: valueSpec.name, text };
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

for (const rawContract of config.contracts ?? []) {
  const contract = loadContract(rawContract);
  for (const root of contract.scanRoots) {
    walk(join(repoRoot, root), contract, contract.values);
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

console.log(`Single-source contract check passed for ${(config.contracts ?? []).length} declared contract(s).`);
