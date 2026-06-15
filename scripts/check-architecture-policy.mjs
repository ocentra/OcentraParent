import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { runNodeOrThrow } from './check-architecture-scope.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const architectureScripts = [
  'check-no-reexports.mjs',
  'check-no-validation-bypass.mjs',
  'check-no-placeholder-implementation.mjs',
  'check-no-skipped-focused-tests.mjs',
  'check-no-weak-assertions.mjs',
  'check-rust-domain-primitives.mjs',
  'check-rust-string-boundaries.mjs',
  'check-import-boundaries.mjs',
  'check-rust-dependency-policy.mjs',
];

export function main(rawArgs = process.argv.slice(2)) {
  if (
    rawArgs.length === 0 &&
    process.env.OCENTRA_ARCHITECTURE_BASE === undefined &&
    process.env.OCENTRA_ARCHITECTURE_HEAD === undefined
  ) {
    console.log(
      'Architecture policy skipped: provide --files, --base/--head, or --all. CI sets the diff range automatically.'
    );
    return;
  }

  for (const scriptName of architectureScripts) {
    runNodeOrThrow(path.join(scriptDir, scriptName), rawArgs);
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
