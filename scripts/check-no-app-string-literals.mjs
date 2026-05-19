import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import ts from 'typescript';

const repoRoot = process.cwd();
const sourceRoots = ['apps/portal/src'];
const ignoredPathParts = new Set(['.git', '.turbo', 'coverage', 'dist', 'node_modules']);
const sourceExtension = /\.(?:ts|tsx)$/u;
const findings = [];

function toPosix(path) {
  return path.split('\\').join('/');
}

function shouldIgnorePath(path) {
  return toPosix(relative(repoRoot, path))
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

function inspectFile(path) {
  const text = readFileSync(path, 'utf8');
  const source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);

  function visit(node) {
    if ((ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node)) && !isModuleSpecifier(node)) {
      const position = source.getLineAndCharacterOfPosition(node.getStart(source));
      findings.push({
        path: toPosix(relative(repoRoot, path)),
        line: position.line + 1,
        text: node.getText(source),
      });
    }

    ts.forEachChild(node, visit);
  }

  visit(source);
}

const files = [];
for (const root of sourceRoots) {
  walk(join(repoRoot, root), files);
}

for (const file of files) {
  inspectFile(file);
}

if (findings.length > 0) {
  console.error(
    'App source cannot contain inline string literals. Move text, routes, ids, and protocol values into domain schemas/constants.'
  );
  for (const finding of findings) {
    console.error(`${finding.path}:${finding.line} ${finding.text}`);
  }
  process.exit(1);
}

console.log(`No inline app string literals found across ${files.length} checked files.`);
