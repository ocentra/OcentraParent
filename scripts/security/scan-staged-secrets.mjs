import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const scanRepo = process.argv.includes('--repo');
const ignoredSegments = new Set(['.git', '.turbo', 'coverage', 'dist', 'node_modules', 'ocentra-ledger', 'target']);
const allowedPathPatterns = [/(^|\/)\.env\.example$/iu, /(^|\/)\.env\.sample$/iu, /(^|\/)\.env\.template$/iu];
const forbiddenPathPatterns = [
  /(^|\/)\.env(\..+)?$/iu,
  /(^|\/)google-services\.json$/iu,
  /(^|\/)GoogleService-Info\.plist$/u,
  /(^|\/)id_rsa(\.pub)?$/iu,
  /\.(pem|p12|pfx|key)$/iu,
];
const textExtensions = new Set([
  '.cjs',
  '.js',
  '.json',
  '.jsx',
  '.md',
  '.mjs',
  '.ps1',
  '.psm1',
  '.toml',
  '.ts',
  '.tsx',
  '.xml',
  '.yml',
  '.yaml',
]);
const secretPatterns = [
  { label: 'OpenAI key', pattern: /(?<![A-Za-z0-9_-])sk-[A-Za-z0-9_-]{20,}/u },
  { label: 'GitHub token', pattern: /gh[pousr]_[A-Za-z0-9_]{30,}/u },
  { label: 'Google API key', pattern: /AIza[0-9A-Za-z_-]{35}/u },
  { label: 'Google OAuth client secret', pattern: /GOCSPX-[0-9A-Za-z_-]{28,}/u },
  { label: 'Stripe live secret key', pattern: /\bsk_live_[0-9A-Za-z]{16,}\b/u },
  {
    label: 'AWS access key id',
    pattern: /\b(A3T[A-Z0-9]|AKIA|ASIA|AGPA|AIDA|ANPA|ANVA|AROA|AIPA)[A-Z0-9]{16}\b/u,
  },
  { label: 'Cloudflare token literal', pattern: /CLOUDFLARE_API_TOKEN\s*=\s*['"]?[A-Za-z0-9_-]{20,}/u },
  { label: 'Private key block', pattern: /-----BEGIN (?:RSA |EC |OPENSSH |DSA |PGP )?PRIVATE KEY-----/u },
];
const findings = [];
const forbiddenFiles = [];

function toPosix(path) {
  return path.split('\\').join('/');
}

function shouldIgnore(path) {
  return toPosix(relative(repoRoot, path))
    .split('/')
    .some((part) => ignoredSegments.has(part));
}

function extensionOf(path) {
  const index = path.lastIndexOf('.');
  return index === -1 ? '' : path.slice(index);
}

function isAllowedPath(path) {
  const normalized = toPosix(relative(repoRoot, path));
  return allowedPathPatterns.some((pattern) => pattern.test(normalized));
}

function isForbiddenPath(path) {
  if (isAllowedPath(path)) {
    return false;
  }
  const normalized = toPosix(relative(repoRoot, path));
  return forbiddenPathPatterns.some((pattern) => pattern.test(normalized));
}

function shouldCollectFile(path) {
  return textExtensions.has(extensionOf(path)) || isAllowedPath(path) || isForbiddenPath(path);
}

function collectRepoFiles(path, files) {
  if (!existsSync(path) || shouldIgnore(path)) {
    return;
  }

  const stat = statSync(path);
  if (stat.isDirectory()) {
    for (const entry of readdirSync(path)) {
      collectRepoFiles(join(path, entry), files);
    }
    return;
  }

  if (stat.isFile() && shouldCollectFile(path)) {
    files.push(path);
  }
}

function collectStagedFiles() {
  const output = execFileSync('git', ['diff', '--cached', '--name-only', '--diff-filter=ACMR'], {
    cwd: repoRoot,
    encoding: 'utf8',
  });
  return output
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter(Boolean)
    .map((path) => join(repoRoot, path))
    .filter((path) => existsSync(path) && !shouldIgnore(path) && shouldCollectFile(path));
}

function scanFile(path) {
  if (isForbiddenPath(path)) {
    forbiddenFiles.push(toPosix(relative(repoRoot, path)));
    return;
  }

  const text = readFileSync(path, 'utf8');
  for (const rule of secretPatterns) {
    if (rule.pattern.test(text)) {
      findings.push({ path: toPosix(relative(repoRoot, path)), label: rule.label });
    }
  }
}

const files = [];
if (scanRepo) {
  collectRepoFiles(repoRoot, files);
} else {
  files.push(...collectStagedFiles());
}

for (const file of files) {
  scanFile(file);
}

if (forbiddenFiles.length > 0 || findings.length > 0) {
  console.error('Potential secrets found:');
  for (const file of forbiddenFiles) {
    console.error(`${file}: forbidden sensitive file path`);
  }
  for (const finding of findings) {
    console.error(`${finding.path}: ${finding.label}`);
  }
  console.error('');
  console.error('Remove the secret, unstage the file, and rotate any credential that was exposed.');
  process.exit(1);
}

console.log(`Secret scan passed for ${files.length} file(s).`);
