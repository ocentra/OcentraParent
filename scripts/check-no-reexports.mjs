import { spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { repoRoot, resolveScopedFiles } from './check-architecture-scope.mjs';

const require = createRequire(import.meta.url);
const eslintPackageRoot = path.dirname(require.resolve('eslint/package.json'));
const eslintCli = path.join(eslintPackageRoot, 'bin', 'eslint.js');
const jsExtensions = new Set(['.js', '.jsx', '.ts', '.tsx', '.mjs', '.mts', '.cjs', '.cts']);
const rustExtension = '.rs';
const scriptName = 'node scripts/check-no-reexports.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const maxJavaScriptChunkChars = process.platform === 'win32' ? 7000 : 20000;

function extensionOf(filePath) {
  return path.extname(filePath);
}

function isArchitectureSource(filePath) {
  const normalized = filePath.replace(/\\/gu, '/');
  return (
    normalized.startsWith('apps/') ||
    normalized.startsWith('packages/') ||
    normalized.startsWith('crates/') ||
    normalized.startsWith('tools/')
  );
}

function chunkJavaScriptFiles(files) {
  const chunks = [];
  let currentChunk = [];
  let currentLength = 0;

  for (const file of files) {
    const nextLength = currentLength + file.length + 1;
    if (currentChunk.length > 0 && nextLength > maxJavaScriptChunkChars) {
      chunks.push(currentChunk);
      currentChunk = [file];
      currentLength = file.length + 1;
      continue;
    }

    currentChunk.push(file);
    currentLength = nextLength;
  }

  if (currentChunk.length > 0) {
    chunks.push(currentChunk);
  }

  return chunks;
}

function lintJavaScript(files) {
  if (files.length === 0) {
    console.log('No TypeScript/JavaScript files matched the re-export gate scope.');
    return;
  }

  for (const fileChunk of chunkJavaScriptFiles(files)) {
    const result = spawnSync(process.execPath, [eslintCli, '--max-warnings=0', ...fileChunk], {
      cwd: repoRoot,
      encoding: 'utf8',
      env: {
        ...process.env,
        OCENTRA_ARCHITECTURE_LINT: '1',
      },
      shell: false,
    });

    if (result.stdout) {
      process.stdout.write(result.stdout);
    }
    if (result.stderr) {
      process.stderr.write(result.stderr);
    }

    if (result.error) {
      throw result.error;
    }

    if ((result.status ?? 1) !== 0) {
      process.exit(result.status ?? 1);
    }
  }

  console.log(`TypeScript/JavaScript re-export gate passed for ${files.length} file(s).`);
}

function lintRust(scope, files) {
  if (files.length === 0) {
    console.log('No Rust files matched the re-export gate scope.');
    return;
  }

  const args =
    scope.mode === 'all'
      ? ['lint-architecture', '--all']
      : scope.mode === 'diff'
        ? ['lint-architecture', '--base', scope.base, '--head', scope.head]
        : ['lint-architecture', ...files];
  const result = spawnSync('cargo', args, {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}

export function main(rawArgs = process.argv.slice(2)) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['apps', 'packages', 'crates', 'tools'],
    acceptPath: (filePath) => isArchitectureSource(filePath),
  });

  if (scope.mode === 'skip') {
    console.log(scope.reason);
    return;
  }

  const jsFiles = scope.files.filter((filePath) => jsExtensions.has(extensionOf(filePath)));
  const rustFiles = scope.files.filter((filePath) => extensionOf(filePath) === rustExtension);

  lintJavaScript(jsFiles);
  lintRust(scope, rustFiles);

  if (jsFiles.length === 0 && rustFiles.length === 0) {
    console.log('Re-export gate skipped: no JS/TS or Rust files matched the requested scope.');
    return;
  }

  console.log('Re-export gate passed.');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
