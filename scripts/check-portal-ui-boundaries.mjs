import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import ts from 'typescript';

import { repoAbsolutePath, resolveScopedFiles } from './check-architecture-scope.mjs';

const ignoredPathParts = new Set(['.git', '.turbo', 'coverage', 'dist', 'node_modules', 'ocentra-ledger']);
const sourceExtension = /\.(?:ts|tsx)$/u;
const cssExtension = /\.css$/u;
const globalCssEntryFiles = new Set(['apps/portal/src/main.ts']);
const allowedInlineStyleObjectFiles = new Map([
  ['apps/portal/src/PortalBackgroundDevTool.tsx', 'background dev overlay is an isolated dev-tool surface'],
  ['apps/portal/src/PortalBackgroundLayer.tsx', 'background layer uses SVG fit math'],
  ['apps/portal/src/PortalBackgroundSvg.tsx', 'background SVG accepts typed render style props'],
  ['apps/portal/src/PortalBackgroundTunerControls.tsx', 'background tuner is a temporary isolated control surface'],
]);
const colorOwnerCssFiles = new Set([
  'apps/portal/src/portal-unified-chrome.css',
  'apps/portal/src/styles/app-shell.css',
  'apps/portal/src/styles/base.css',
  'apps/portal/src/styles/control-card-frame.css',
  'apps/portal/src/styles/deck-frame-fit.css',
  'apps/portal/src/styles/frame-tuner.css',
  'apps/portal/src/styles/parent-portal-route.css',
  'apps/portal/src/styles/portal-theme-tokens.css',
  'apps/portal/src/styles/product-frame.css',
  'apps/portal/src/styles/sidebar.css',
  'apps/portal/src/styles.css',
]);
const colorOwnerFoundationCssFiles = new Set([
  'apps/portal/src/styles/app-shell-foundation.css',
  'apps/portal/src/styles/base-foundation.css',
  'apps/portal/src/styles/deck-frame-fit-foundation.css',
]);
const rawColorPattern = /(?:#[0-9a-f]{3,8}\b|(?<!-)rgba?\(|(?<!-)hsla?\()/iu;
const cssVarDefinitionPattern = /(?<![\w-])(--[a-z0-9_-]+)\s*:/giu;
const cssVarUsePattern = /var\(\s*(--[a-z0-9_-]+)\b/giu;
const tsOwnedCssVarFiles = [
  'packages/portal-domain/src/frame-tuner.ts',
  'packages/portal-domain/src/unified-chrome.ts',
];
const tsCssVarLiteralPattern = /'(--[a-z0-9_-]+)'/giu;
const scriptName = 'node scripts/check-portal-ui-boundaries.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const portalUiRoots = ['apps/portal/src'];

function toPosix(path) {
  return path.split(sep).join('/');
}

function relativePath(repoRoot, path) {
  return toPosix(relative(repoRoot, path));
}

function shouldIgnorePath(repoRoot, path) {
  return relativePath(repoRoot, path)
    .split('/')
    .some((part) => ignoredPathParts.has(part));
}

function walk(repoRoot, path, files) {
  if (!existsSync(path) || shouldIgnorePath(repoRoot, path)) {
    return;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(repoRoot, join(path, entry), files);
    }
    return;
  }

  if (stats.isFile() && (sourceExtension.test(path) || cssExtension.test(path))) {
    files.push(path);
  }
}

function lineOf(source, node) {
  return source.getLineAndCharacterOfPosition(node.getStart(source)).line + 1;
}

function report(findings, path, line, reason, text) {
  findings.push({ path, line, reason, text });
}

function isCssImport(moduleSpecifier) {
  return moduleSpecifier.endsWith('.css');
}

function isCssModuleImport(moduleSpecifier) {
  return moduleSpecifier.endsWith('.module.css');
}

function hasStringLiteral(node) {
  let found = false;
  function visit(candidate) {
    if (ts.isStringLiteral(candidate) || ts.isNoSubstitutionTemplateLiteral(candidate)) {
      found = true;
      return;
    }
    ts.forEachChild(candidate, visit);
  }
  visit(node);
  return found;
}

function inspectTypeScriptFile({ findings, path, repoRoot }) {
  const text = readFileSync(path, 'utf8');
  const source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const pathText = relativePath(repoRoot, path);

  function visit(node) {
    if (ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier)) {
      const moduleSpecifier = node.moduleSpecifier.text;
      if (isCssImport(moduleSpecifier) && !isCssModuleImport(moduleSpecifier) && !globalCssEntryFiles.has(pathText)) {
        report(
          findings,
          pathText,
          lineOf(source, node.moduleSpecifier),
          'raw CSS import',
          `${moduleSpecifier} must be imported from a global style entry file or converted to a typed CSS module`
        );
      }
      if (moduleSpecifier === '@tauri-apps/api/core') {
        const namedBindings = node.importClause?.namedBindings;
        if (
          namedBindings !== undefined &&
          ts.isNamedImports(namedBindings) &&
          namedBindings.elements.some((element) => element.name.text === 'invoke')
        ) {
          report(
            findings,
            pathText,
            lineOf(source, node.moduleSpecifier),
            'direct Tauri invoke import',
            'use a typed command wrapper instead of importing invoke in app code'
          );
        }
      }
    }

    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === 'invoke') {
      report(
        findings,
        pathText,
        lineOf(source, node),
        'direct Tauri invoke call',
        'use a typed command wrapper instead of invoke(commandName)'
      );
    }

    if (ts.isJsxAttribute(node) && ts.isIdentifier(node.name)) {
      if (node.name.text === 'className') {
        inspectClassNameAttribute({ findings, node, pathText, source });
      }
      if (node.name.text === 'style') {
        inspectStyleAttribute({ findings, node, pathText, source });
      }
    }

    ts.forEachChild(node, visit);
  }

  visit(source);
}

function inspectClassNameAttribute({ findings, node, pathText, source }) {
  const initializer = node.initializer;
  if (initializer === undefined) {
    return;
  }
  if (ts.isStringLiteral(initializer)) {
    report(findings, pathText, lineOf(source, node), 'raw JSX className', initializer.getText(source));
    return;
  }
  if (!ts.isJsxExpression(initializer) || initializer.expression === undefined) {
    return;
  }
  const expression = initializer.expression;
  if (
    ts.isNoSubstitutionTemplateLiteral(expression) ||
    ts.isTemplateExpression(expression) ||
    (ts.isBinaryExpression(expression) && hasStringLiteral(expression))
  ) {
    report(
      findings,
      pathText,
      lineOf(source, node),
      'string-built JSX className',
      'use PortalDom/PortalUnifiedChrome class constants plus the approved class-name join helper'
    );
  }
}

function inspectStyleAttribute({ findings, node, pathText, source }) {
  const initializer = node.initializer;
  if (!ts.isJsxExpression(initializer) || initializer.expression === undefined) {
    return;
  }
  if (!ts.isObjectLiteralExpression(initializer.expression)) {
    return;
  }
  if (allowedInlineStyleObjectFiles.has(pathText)) {
    return;
  }
  report(
    findings,
    pathText,
    lineOf(source, node),
    'raw JSX style object',
    'move style values into typed style constants, CSS variables, or the frame/layout style helpers'
  );
}

function inspectCssFiles({ cssFiles, findings, repoRoot }) {
  const definedVars = collectTsOwnedCssVars(repoRoot);
  const cssTexts = cssFiles.map((path) => ({
    path,
    pathText: relativePath(repoRoot, path),
    text: readFileSync(path, 'utf8'),
  }));

  for (const { text } of cssTexts) {
    for (const match of text.matchAll(cssVarDefinitionPattern)) {
      definedVars.add(match[1]);
    }
  }

  for (const { pathText, text } of cssTexts) {
    const lines = text.split(/\r?\n/u);
    lines.forEach((line, index) => {
      for (const match of line.matchAll(cssVarUsePattern)) {
        if (!definedVars.has(match[1])) {
          report(findings, pathText, index + 1, 'unknown CSS variable', match[1]);
        }
      }
      if (
        rawColorPattern.test(line) &&
        !colorOwnerCssFiles.has(pathText) &&
        !colorOwnerFoundationCssFiles.has(pathText)
      ) {
        report(
          findings,
          pathText,
          index + 1,
          'raw CSS color outside style owner',
          'move color values into portal theme tokens or an approved style-owner file'
        );
      }
    });
  }
}

function collectTsOwnedCssVars(repoRoot) {
  const vars = new Set();
  for (const relativeFile of tsOwnedCssVarFiles) {
    const path = join(repoRoot, relativeFile);
    if (!existsSync(path)) {
      continue;
    }
    const text = readFileSync(path, 'utf8');
    for (const match of text.matchAll(tsCssVarLiteralPattern)) {
      vars.add(match[1]);
    }
  }
  return vars;
}

export function runPortalUiBoundaryCheck({ repoRoot = process.cwd() } = {}) {
  const files = [];
  walk(repoRoot, join(repoRoot, 'apps/portal/src'), files);

  return runPortalUiBoundaryCheckForFiles(files, { repoRoot });
}

export function runPortalUiBoundaryCheckForFiles(files, { repoRoot = process.cwd() } = {}) {
  const absoluteFiles = files.map((file) => repoAbsolutePath(file));

  const findings = [];
  const cssFiles = [];
  for (const file of absoluteFiles) {
    if (sourceExtension.test(file)) {
      inspectTypeScriptFile({ findings, path: file, repoRoot });
    } else if (cssExtension.test(file)) {
      cssFiles.push(file);
    }
  }
  inspectCssFiles({ cssFiles, findings, repoRoot });
  return findings;
}

function collectScopedPortalUiFiles(rawArgs) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: portalUiRoots,
    acceptPath: (filePath) => sourceExtension.test(filePath) || cssExtension.test(filePath),
  });

  if (scope.mode === 'skip') {
    return [];
  }

  return scope.files;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const rawArgs = process.argv.slice(2);
  const findings =
    rawArgs.length === 0
      ? runPortalUiBoundaryCheck()
      : runPortalUiBoundaryCheckForFiles(collectScopedPortalUiFiles(rawArgs));
  if (findings.length > 0) {
    console.error('Portal UI boundary violations found.');
    for (const finding of findings) {
      console.error(`${finding.path}:${finding.line} ${finding.reason}: ${finding.text}`);
    }
    process.exit(1);
  }
  console.log('No portal UI boundary violations found.');
}
