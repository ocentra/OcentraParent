import { existsSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';

const packageRoot = dirname(fileURLToPath(import.meta.url));

function resolveSiblingDomainImport(id: string): string | null {
  const match = id.match(/^@ocentra-parent\/([^/]+)\/(.+)$/);
  if (match === null) {
    return null;
  }

  const [, packageName, subpath] = match;
  const siblingPackageRoot = resolve(packageRoot, '..', packageName);
  const directFile = resolve(siblingPackageRoot, 'src', `${subpath}.ts`);
  if (existsSync(directFile)) {
    return directFile;
  }

  const indexFile = resolve(siblingPackageRoot, 'src', subpath, 'index.ts');
  if (existsSync(indexFile)) {
    return indexFile;
  }

  return null;
}

export default defineConfig({
  plugins: [
    {
      name: 'ocentra-parent-sibling-domain-alias',
      resolveId(id) {
        return resolveSiblingDomainImport(id);
      },
    },
  ],
});
