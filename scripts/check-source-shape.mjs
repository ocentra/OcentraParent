import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, sep } from 'node:path';
import { pathToFileURL } from 'node:url';
import ts from 'typescript';

const repoRoot = process.cwd();
const ignoredSegments = new Set(['.git', '.turbo', 'coverage', 'dist', 'node_modules', 'target']);
const warningRatio = 0.8;
const fileLineWarningStep = 250;
const policies = [
  {
    roots: ['apps'],
    extensions: new Set(['.ts', '.tsx']),
    kind: 'typescript',
    maxClasses: 1,
    maxExports: 35,
    maxFunctionLines: 80,
    maxLines: 1000,
  },
  {
    roots: ['packages'],
    extensions: new Set(['.ts', '.tsx']),
    kind: 'typescript',
    maxClasses: 1,
    maxExports: 45,
    maxFunctionLines: 80,
    maxLines: 1000,
  },
  {
    roots: ['crates'],
    extensions: new Set(['.rs']),
    kind: 'rust',
    maxFunctionLines: 80,
    maxFunctions: 18,
    maxLines: 1000,
    maxTypes: 24,
  },
];

function toPosix(path) {
  return path.split(sep).join('/');
}

function extensionOf(path) {
  const match = path.match(/\.[^.]+$/u);
  return match?.[0] ?? '';
}

function shouldSkip(path) {
  const relativePath = toPosix(relative(repoRoot, path));
  return relativePath.split('/').some((part) => ignoredSegments.has(part));
}

function countLines(text) {
  return text.length === 0 ? 0 : text.split(/\r?\n/u).length;
}

function walk(path, files) {
  if (!existsSync(path) || shouldSkip(path)) {
    return;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), files);
    }
    return;
  }

  if (stats.isFile()) {
    files.push(path);
  }
}

function policyFor(relativePath) {
  const normalized = toPosix(relativePath);
  return policies.find(
    (policy) =>
      policy.roots.some((root) => normalized.startsWith(`${root}/`)) && policy.extensions.has(extensionOf(normalized))
  );
}

function nearLimit(value, limit) {
  return value >= Math.ceil(limit * warningRatio) && value <= limit;
}

function fileLineWarningBand(lines, policy) {
  if (lines < fileLineWarningStep || lines > policy.maxLines) {
    return null;
  }
  return Math.floor(lines / fileLineWarningStep) * fileLineWarningStep;
}

function reportFileLines(findings, warnings, relativePath, text, policy) {
  const lines = countLines(text);
  if (lines > policy.maxLines) {
    findings.push({
      path: relativePath,
      line: policy.maxLines + 1,
      reason: `file has ${lines} lines; maximum is ${policy.maxLines}`,
    });
    return;
  }
  const warningBand = fileLineWarningBand(lines, policy);
  if (warningBand !== null) {
    warnings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${lines} lines; crossed ${warningBand}-line advisory band; maximum is ${policy.maxLines}`,
    });
  }
}

export function inspectTypeScriptSource(relativePath, text, policy = policies[0]) {
  const findings = [];
  const warnings = [];
  const source = ts.createSourceFile(relativePath, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  let classCount = 0;
  let exportCount = 0;

  reportFileLines(findings, warnings, relativePath, text, policy);

  function lineSpan(node) {
    const start = source.getLineAndCharacterOfPosition(node.getStart(source)).line;
    const end = source.getLineAndCharacterOfPosition(node.getEnd()).line;
    return end - start + 1;
  }

  function hasExportModifier(node) {
    return node.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword) ?? false;
  }

  function inspectFunctionLike(node) {
    const lines = lineSpan(node);
    if (lines > policy.maxFunctionLines) {
      const position = source.getLineAndCharacterOfPosition(node.getStart(source));
      findings.push({
        path: relativePath,
        line: position.line + 1,
        reason: `function has ${lines} lines; maximum is ${policy.maxFunctionLines}`,
      });
      return;
    }
    if (nearLimit(lines, policy.maxFunctionLines)) {
      const position = source.getLineAndCharacterOfPosition(node.getStart(source));
      warnings.push({
        path: relativePath,
        line: position.line + 1,
        reason: `function has ${lines} lines; warning starts at ${Math.ceil(policy.maxFunctionLines * warningRatio)} of ${policy.maxFunctionLines}`,
      });
    }
  }

  function visit(node) {
    if (ts.isClassDeclaration(node)) {
      classCount += 1;
    }
    if (hasExportModifier(node) || ts.isExportDeclaration(node) || ts.isExportAssignment(node)) {
      exportCount += 1;
    }
    if (
      ts.isFunctionDeclaration(node) ||
      ts.isFunctionExpression(node) ||
      ts.isArrowFunction(node) ||
      ts.isMethodDeclaration(node)
    ) {
      inspectFunctionLike(node);
    }
    ts.forEachChild(node, visit);
  }

  visit(source);

  if (classCount > policy.maxClasses) {
    findings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${classCount} classes; maximum is ${policy.maxClasses}`,
    });
  }
  if (exportCount > policy.maxExports) {
    findings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${exportCount} exports; maximum is ${policy.maxExports}`,
    });
  }

  return { findings, warnings };
}

export function inspectRustSource(relativePath, text, policy = policies[2]) {
  const findings = [];
  const warnings = [];
  const lines = text.split(/\r?\n/u);
  const functionStarts = [];
  let typeCount = 0;

  reportFileLines(findings, warnings, relativePath, text, policy);

  lines.forEach((line, index) => {
    if (/^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+\w+/u.test(line)) {
      functionStarts.push(index);
    }
    if (/^\s*(?:pub\s+)?(?:struct|enum)\s+\w+/u.test(line)) {
      typeCount += 1;
    }
  });

  if (functionStarts.length > policy.maxFunctions) {
    findings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${functionStarts.length} functions; maximum is ${policy.maxFunctions}`,
    });
  } else if (nearLimit(functionStarts.length, policy.maxFunctions)) {
    warnings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${functionStarts.length} functions; warning starts at ${Math.ceil(policy.maxFunctions * warningRatio)} of ${policy.maxFunctions}`,
    });
  }
  if (typeCount > policy.maxTypes) {
    findings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${typeCount} structs/enums; maximum is ${policy.maxTypes}`,
    });
  } else if (nearLimit(typeCount, policy.maxTypes)) {
    warnings.push({
      path: relativePath,
      line: 1,
      reason: `file has ${typeCount} structs/enums; warning starts at ${Math.ceil(policy.maxTypes * warningRatio)} of ${policy.maxTypes}`,
    });
  }

  for (const start of functionStarts) {
    const end = findRustFunctionEnd(lines, start);
    const functionLines = end - start + 1;
    if (functionLines > policy.maxFunctionLines) {
      findings.push({
        path: relativePath,
        line: start + 1,
        reason: `function has ${functionLines} lines; maximum is ${policy.maxFunctionLines}`,
      });
    } else if (nearLimit(functionLines, policy.maxFunctionLines)) {
      warnings.push({
        path: relativePath,
        line: start + 1,
        reason: `function has ${functionLines} lines; warning starts at ${Math.ceil(policy.maxFunctionLines * warningRatio)} of ${policy.maxFunctionLines}`,
      });
    }
  }

  return { findings, warnings };
}

function findRustFunctionEnd(lines, start) {
  let depth = 0;
  let seenBody = false;
  for (let index = start; index < lines.length; index += 1) {
    for (const char of lines[index]) {
      if (char === '{') {
        seenBody = true;
        depth += 1;
      } else if (char === '}') {
        depth -= 1;
      }
    }
    if (seenBody && depth === 0) {
      return index;
    }
  }
  return start;
}

export function collectSourceShapeReport(root = repoRoot) {
  const files = [];
  for (const policy of policies) {
    for (const sourceRoot of policy.roots) {
      walk(join(root, sourceRoot), files);
    }
  }

  const findings = [];
  const warnings = [];
  for (const file of files) {
    const relativePath = toPosix(relative(root, file));
    const policy = policyFor(relativePath);
    if (policy === undefined) {
      continue;
    }

    const text = readFileSync(file, 'utf8');
    const result =
      policy.kind === 'rust'
        ? inspectRustSource(relativePath, text, policy)
        : inspectTypeScriptSource(relativePath, text, policy);
    findings.push(...result.findings);
    warnings.push(...result.warnings);
  }
  return { findings, warnings };
}

export function collectSourceShapeFindings(root = repoRoot) {
  return collectSourceShapeReport(root).findings;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const { findings, warnings } = collectSourceShapeReport();
  if (warnings.length > 0) {
    console.log('Source shape warnings: files/functions are near their size limits.');
    for (const warning of warnings) {
      console.log(`${warning.path}:${warning.line} ${warning.reason}`);
    }
  }

  if (findings.length > 0) {
    console.error('Source shape guard failed. Split oversized files/functions/classes before adding behavior.');
    for (const finding of findings) {
      console.error(`${finding.path}:${finding.line} ${finding.reason}`);
    }
    process.exit(1);
  }

  console.log('Source shape guard passed.');
}
