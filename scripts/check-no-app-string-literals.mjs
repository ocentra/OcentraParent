import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, sep } from 'node:path';
import { pathToFileURL } from 'node:url';
import ts from 'typescript';

import { repoAbsolutePath, resolveScopedFiles } from './check-architecture-scope.mjs';
import { runPortalUiBoundaryCheck, runPortalUiBoundaryCheckForFiles } from './check-portal-ui-boundaries.mjs';

const repoRoot = process.cwd();
const sourceRoots = ['apps/portal/src'];
const ignoredPathParts = new Set(['.git', '.turbo', 'coverage', 'dist', 'node_modules', 'ocentra-ledger']);
const sourceExtension = /\.(?:ts|tsx)$/u;
const findings = [];
const typeGuardLiteralValues = new Set([
  '',
  'bigint',
  'boolean',
  'function',
  'number',
  'object',
  'string',
  'symbol',
  'undefined',
]);
const styleLiteralOwnerPaths = new Set([
  'apps/portal/src/PortalBackgroundDevTool.tsx',
  'apps/portal/src/PortalBackgroundTunerControls.tsx',
]);
const ownedConstantNamePattern =
  /^(?:[A-Z0-9_]+|[A-Z][A-Za-z0-9]*(?:Attributes?|Assets?|Boot|Chrome|Classes?|Colors?|Config|Constants?|Copy|Defaults?|Dom|Events?|Fields?|Ids?|Labels?|Layout|Markup|Palette|Runtime|Style|Styles?|Svg|Text|Values?)|[a-z][A-Za-z0-9]*(?:Chrome|Config|Icon|Icons|Markup|Style|Styles|Svg|Text))$/u;
const appDomainLiteralPatterns = [
  { pattern: /^--[a-z0-9-]+$/iu, reason: 'CSS custom property' },
  { pattern: /^#[a-z0-9/-]/iu, reason: 'route hash or color' },
  { pattern: /^\//u, reason: 'public path or URL path' },
  { pattern: /^data-[a-z0-9-]+$/iu, reason: 'data attribute' },
  { pattern: /^portal-[a-z0-9-]+$/iu, reason: 'portal DOM key/class/id' },
  { pattern: /^ocentra-[a-z0-9-]+$/iu, reason: 'Ocentra DOM key/class/id' },
  { pattern: /^__TAURI_[A-Z0-9_]+__$/u, reason: 'Tauri runtime key' },
  { pattern: /^tauri:\/\//u, reason: 'Tauri event name' },
  { pattern: /#[0-9a-f]{3,8}\b/iu, reason: 'raw color embedded in markup' },
  { pattern: /\brgba?\(/iu, reason: 'raw RGB color embedded in markup' },
  { pattern: /\bhsla?\(/iu, reason: 'raw HSL color embedded in markup' },
];
const scriptName = 'node scripts/check-no-app-string-literals.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];

function toPosix(path) {
  return path.split(sep).join('/');
}

function relativePath(path) {
  return toPosix(relative(repoRoot, path));
}

function shouldIgnorePath(path) {
  return relativePath(path)
    .split('/')
    .some((part) => ignoredPathParts.has(part));
}

function walk(path, files) {
  if (!existsSync(path) || shouldIgnorePath(path)) {
    return;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), files);
    }
    return;
  }

  if (stats.isFile() && sourceExtension.test(path)) {
    files.push(path);
  }
}

function isModuleSpecifier(node) {
  const parent = node.parent;
  return (
    (ts.isImportDeclaration(parent) && parent.moduleSpecifier === node) ||
    (ts.isExportDeclaration(parent) && parent.moduleSpecifier === node)
  );
}

function isTypeLiteral(node) {
  let current = node.parent;
  while (current !== undefined) {
    if (
      ts.isLiteralTypeNode(current) ||
      ts.isTypeAliasDeclaration(current) ||
      ts.isInterfaceDeclaration(current) ||
      ts.isTypeReferenceNode(current)
    ) {
      return true;
    }
    if (ts.isStatement(current) || ts.isExpression(current)) {
      return false;
    }
    current = current.parent;
  }
  return false;
}

function constantOwnerFor(node) {
  let current = node.parent;
  while (current !== undefined) {
    if (ts.isVariableDeclaration(current) && ts.isIdentifier(current.name)) {
      const declarationList = current.parent;
      const statement = declarationList.parent;
      if (
        ts.isVariableDeclarationList(declarationList) &&
        ts.isVariableStatement(statement) &&
        ts.isSourceFile(statement.parent) &&
        ownedConstantNamePattern.test(current.name.text)
      ) {
        return current.name.text;
      }
      return null;
    }
    if (ts.isFunctionLike(current) || ts.isClassLike(current)) {
      return null;
    }
    current = current.parent;
  }
  return null;
}

function domainLiteralReason(value) {
  if (typeGuardLiteralValues.has(value)) {
    return null;
  }
  return appDomainLiteralPatterns.find(({ pattern }) => pattern.test(value))?.reason ?? null;
}

function inspectFile(path) {
  const text = readFileSync(path, 'utf8');
  const source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const pathText = relativePath(path);

  function visit(node) {
    if ((ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node)) && !isModuleSpecifier(node)) {
      const value = node.text;
      const reason = domainLiteralReason(value);
      if (
        reason !== null &&
        !styleLiteralOwnerPaths.has(pathText) &&
        !isTypeLiteral(node) &&
        constantOwnerFor(node) === null
      ) {
        const position = source.getLineAndCharacterOfPosition(node.getStart(source));
        findings.push({
          path: pathText,
          line: position.line + 1,
          reason,
          text: node.getText(source),
        });
      }
    }

    ts.forEachChild(node, visit);
  }

  visit(source);
}

function collectFullFiles() {
  const files = [];
  for (const root of sourceRoots) {
    walk(join(repoRoot, root), files);
  }
  return files;
}

function collectScopedFiles(rawArgs) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: sourceRoots,
    acceptPath: (filePath) => /\.(?:ts|tsx|css)$/u.test(filePath),
  });

  if (scope.mode === 'skip') {
    return [];
  }

  return scope.files.map((filePath) => repoAbsolutePath(filePath));
}

export function main(rawArgs = process.argv.slice(2)) {
  findings.length = 0;
  const files = rawArgs.length === 0 ? collectFullFiles() : collectScopedFiles(rawArgs);

  for (const file of files) {
    if (sourceExtension.test(file)) {
      inspectFile(file);
    }
  }

  const uiBoundaryFindings =
    rawArgs.length === 0
      ? runPortalUiBoundaryCheck({ repoRoot })
      : runPortalUiBoundaryCheckForFiles(files, { repoRoot });
  findings.push(...uiBoundaryFindings);

  if (findings.length > 0) {
    console.error(
      'App source cannot contain inline domain/UI boundary strings. Move routes, selectors, CSS hooks, colors, and protocol values into typed constants or approved style/token owners.'
    );
    for (const finding of findings) {
      const reason = finding.reason === undefined ? '' : ` ${finding.reason}:`;
      console.error(`${finding.path}:${finding.line}${reason} ${finding.text}`);
    }
    process.exit(1);
  }

  console.log(`No inline app domain strings or UI boundary violations found across ${files.length} checked files.`);
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
