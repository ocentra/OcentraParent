import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';

const listed = spawnSync('git', ['ls-files', '-z', '--', '*.rs'], {
  encoding: 'utf8',
  shell: false,
});

if (listed.status !== 0) {
  process.stderr.write(listed.stderr || 'Could not list tracked Rust files.\n');
  process.exit(listed.status ?? 1);
}

// Staged deletions remain in `git ls-files` but cannot be passed to rustfmt.
const files = listed.stdout.split('\0').filter((filePath) => filePath && existsSync(filePath));
const maxChunkChars = process.platform === 'win32' ? 7000 : 24000;
const chunks = [];
let chunk = [];
let chunkChars = 0;

for (const file of files) {
  if (chunk.length > 0 && chunkChars + file.length + 1 > maxChunkChars) {
    chunks.push(chunk);
    chunk = [];
    chunkChars = 0;
  }
  chunk.push(file);
  chunkChars += file.length + 1;
}
if (chunk.length > 0) chunks.push(chunk);

for (const rustFiles of chunks) {
  // Each tracked file is formatted directly. Test entrypoints can declare
  // sibling modules that are intentionally absent from their local directory.
  const result = spawnSync(
    'rustfmt',
    ['--check', '--edition', '2021', '--config', 'skip_children=true', ...rustFiles],
    {
      stdio: 'inherit',
      shell: false,
    }
  );
  if (result.status !== 0) process.exit(result.status ?? 1);
}
