#!/usr/bin/env bash
set -euo pipefail

package_path="${1:?Usage: linux-deb-smoke.sh <package.deb>}"
package_name="ocentra-parent-agent"
unit_path="/lib/systemd/system/ocentra-parent-agent.service"
binary_path="/opt/ocentra/ocentra-parent-agent/bin/ocentra-parent-agent-service"

sudo dpkg -i "$package_path"

dpkg-query -W "$package_name" >/dev/null
test -f "$unit_path"
test -x "$binary_path"

sudo dpkg -r "$package_name"

if dpkg-query -W "$package_name" >/dev/null 2>&1; then
  echo "Package remained installed after remove: $package_name" >&2
  exit 1
fi

echo "linux-deb-smoke-ok:$package_path"
