import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { test } from 'node:test';

const repoRoot = process.cwd();

function readRepoFile(path) {
  return readFileSync(join(repoRoot, path), 'utf8');
}

test('Linux package preview builds on the Ubuntu 22.04 glibc 2.35 baseline', () => {
  const workflow = readRepoFile('.github/workflows/package-preview.yml');

  assert.match(workflow, /linux-deb:[\s\S]*?runs-on: ubuntu-22\.04/u);
  assert.match(workflow, /Record Linux package baseline/u);
  assert.match(workflow, /getconf GNU_LIBC_VERSION/u);
  assert.match(workflow, /target\/release-packages\/linux\/linux-baseline\.json/u);
  assert.match(workflow, /test-results\/linux-package-smoke\/\*/u);
});

test('Linux package builder refuses accidental newer-glibc release builds', () => {
  const builder = readRepoFile('scripts/release/linux/build-agent-package.sh');

  assert.match(builder, /bootstrap_cargo_path\(\)/u);
  assert.match(builder, /\[\[ -f "\$\{HOME\}\/\.cargo\/env" \]\]/u);
  assert.match(builder, /\. "\$\{HOME\}\/\.cargo\/env"/u);
  assert.match(builder, /\[\[ -x "\$\{HOME\}\/\.cargo\/bin\/cargo" \]\]/u);
  assert.match(builder, /export PATH="\$\{HOME\}\/\.cargo\/bin:\$\{PATH\}"/u);
  assert.match(builder, /Linux package builds require a Linux cargo toolchain on PATH\./u);
  assert.match(builder, /baseline_id="\$\{OCENTRA_PARENT_LINUX_BASELINE_ID:-ubuntu\}"/u);
  assert.match(builder, /baseline_version="\$\{OCENTRA_PARENT_LINUX_BASELINE_VERSION:-22\.04\}"/u);
  assert.match(builder, /baseline_glibc_min="\$\{OCENTRA_PARENT_LINUX_GLIBC_MIN:-2\.35\}"/u);
  assert.match(builder, /stage_parent="\$\{OCENTRA_PARENT_LINUX_STAGE_PARENT:-\$\{TMPDIR:-\/tmp\}\}"/u);
  assert.match(builder, /stage_root="\$\(mktemp -d "\$stage_parent\/ocentra-parent-linux-package\.XXXXXX"\)"/u);
  assert.match(builder, /OCENTRA_PARENT_LINUX_ALLOW_NON_BASELINE/u);
  assert.match(builder, /Linux package builds must run on \$\{baseline_label\} with glibc \$\{baseline_glibc_min\}/u);
  assert.match(builder, /Depends: libc6 \(>= \$baseline_glibc_min\)/u);
  assert.match(builder, /X-Ocentra-Linux-Baseline: \$baseline_label/u);
  assert.match(builder, /X-Ocentra-Min-GLIBC: \$baseline_glibc_min/u);
  assert.match(builder, /X-Ocentra-Build-GLIBC: \$host_glibc/u);
  assert.match(builder, /"minimumGlibc": "\$baseline_glibc_min"/u);
  assert.match(builder, /cd "\$package_root" && sha256sum "\$package_name" > "\$package_name\.sha256"/u);
});

test('Linux package smoke proves integrity, payload, launch, and install lifecycle', () => {
  const smoke = readRepoFile('scripts/smoke/linux-deb-smoke.sh');
  const unit = readRepoFile('scripts/release/linux/ocentra-parent-agent.service');

  assert.match(unit, /Environment=OCENTRA_PARENT_AGENT_ADDR=127\.0\.0\.1:4477/u);
  assert.match(smoke, /test-results\/linux-package-smoke/u);
  assert.match(smoke, /sha256sum --check/u);
  assert.match(smoke, /dpkg-deb --field "\$package_path"/u);
  assert.match(smoke, /require_field Depends "libc6 \(>= 2\.35\)"/u);
  assert.match(smoke, /field X-Ocentra-Linux-Baseline/u);
  assert.match(smoke, /field X-Ocentra-Min-GLIBC/u);
  assert.match(smoke, /field X-Ocentra-Build-GLIBC/u);
  assert.match(smoke, /dpkg-deb --contents "\$package_path"/u);
  assert.match(smoke, /dpkg-deb --extract "\$package_path" "\$extract_root"/u);
  assert.match(smoke, /OCENTRA_PARENT_AGENT_ADDR="\$smoke_addr" "\$extract_root\$binary_path"/u);
  assert.match(smoke, /curl --fail --silent "\$health_url"/u);
  assert.match(smoke, /sudo -n true/u);
  assert.match(smoke, /sudo dpkg -i "\$package_path"/u);
  assert.match(smoke, /sudo dpkg -r "\$package_name"/u);
});
