#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
version="${OCENTRA_PARENT_VERSION:-$(cd "$repo_root" && node scripts/release/validate-version.mjs --print-version)}"
package_root="$repo_root/target/release-packages/linux"
stage_root="$package_root/stage"
package_name="ocentra-parent-agent-linux-amd64-v${version}.deb"
package_path="$package_root/$package_name"
latest_name="ocentra-parent-agent-linux-amd64-latest.deb"
latest_path="$package_root/$latest_name"

rm -rf "$stage_root"
mkdir -p "$stage_root/DEBIAN"
mkdir -p "$stage_root/opt/ocentra/ocentra-parent-agent/bin"
mkdir -p "$stage_root/lib/systemd/system"
mkdir -p "$stage_root/var/lib/ocentra/ocentra-parent-agent"
mkdir -p "$stage_root/var/log/ocentra/ocentra-parent-agent"
mkdir -p "$package_root"

(cd "$repo_root" && cargo build --release -p ocentra-parent-agent-service)

install -m 0755 "$repo_root/target/release/ocentra-parent-agent-service" \
  "$stage_root/opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service"
install -m 0644 "$repo_root/scripts/release/linux/ocentra-parent-agent.service" \
  "$stage_root/lib/systemd/system/ocentra-parent-agent.service"

cat > "$stage_root/DEBIAN/control" <<CONTROL
Package: ocentra-parent-agent
Version: $version
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Ocentra <support@ocentra.ca>
Description: Headless local device agent for Ocentra Parent.
CONTROL

cat > "$stage_root/DEBIAN/postinst" <<'POSTINST'
#!/usr/bin/env bash
set -e
if command -v systemctl >/dev/null 2>&1; then
  systemctl daemon-reload
  systemctl enable ocentra-parent-agent.service >/dev/null 2>&1 || true
  systemctl restart ocentra-parent-agent.service >/dev/null 2>&1 || true
fi
POSTINST

cat > "$stage_root/DEBIAN/prerm" <<'PRERM'
#!/usr/bin/env bash
set -e
if command -v systemctl >/dev/null 2>&1; then
  systemctl stop ocentra-parent-agent.service >/dev/null 2>&1 || true
  systemctl disable ocentra-parent-agent.service >/dev/null 2>&1 || true
fi
PRERM

cat > "$stage_root/DEBIAN/postrm" <<'POSTRM'
#!/usr/bin/env bash
set -e
if command -v systemctl >/dev/null 2>&1; then
  systemctl daemon-reload
fi
POSTRM

chmod 0755 "$stage_root/DEBIAN/postinst" "$stage_root/DEBIAN/prerm" "$stage_root/DEBIAN/postrm"
dpkg-deb --build --root-owner-group "$stage_root" "$package_path"
cp "$package_path" "$latest_path"

sha256sum "$package_path" > "$package_path.sha256"
sha256sum "$latest_path" > "$latest_path.sha256"

echo "Built $package_path"
echo "Built $latest_path"
