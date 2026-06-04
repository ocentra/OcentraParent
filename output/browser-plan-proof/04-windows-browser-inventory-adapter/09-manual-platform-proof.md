# Manual Platform Proof

Status: partial live-source proof; product manual proof remains
manual-required.

This slice is still fixture-backed plus deterministic caller-provided root,
registry-value, and shortcut-target normalization only. It deduplicates
candidate roots before path expansion, tests packaged `WindowsApps` path
classification, and tests registry display-icon/install-location plus shortcut
target inputs as caller-provided values. Unquoted command target strings with
known browser executable paths and trailing launch arguments are trimmed back to
the executable path before classification. Leading Windows environment-variable
segments in caller-provided registry/shortcut target strings are expanded
through the local process environment before the same known-executable filter
runs. Service inventory read-model default-root consumption is fixture-backed:
the proof substitutes a temp `PROGRAMFILES` root and verifies the service scan
feeds default Windows candidate paths into the read model without making exact
URL, UI, live registry, shortcut enumeration, AppX/MSIX, signature, or
enforcement claims.

This continuation adds bounded live Windows Uninstall registry source
collection and bounded Start Menu shortcut target extraction into the service
browser inventory candidate path flow. The proof ran on Windows (`win32`) and
is captured at `11-live-source-proof.json` plus
`test-results/windows-browser-inventory-source-proof/proof.json`. The service
test is host-aware: if live registry rows are present, every returned row must
still pass `claim_boundary_is_honest`, and the temp Edge fixture remains exact
URL unavailable.

A later pass must capture product-complete Windows version and installed
browser evidence, full shell `.lnk` parser artifacts beyond known executable
target extraction, live AppX/MSIX inputs, redacted executable refs,
screenshots/logs, and signature/hash refs before WP04 can be marked complete.
