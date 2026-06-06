# WP04 Live Windows Browser Inventory Proof

Generated: 2026-06-06T03:03:53.264Z

This proof ran against the local Windows host and stores only redacted refs, hashes, counts, executable basenames, and capability labels.
It does not store raw paths, raw URLs, page titles, page bodies, browser profile data, cookies, tokens, tabs, or browsing content.

Rows captured: 16
Source counts: {"known-path":3,"registry-uninstall":4,"running-process":2,"start-menu-shortcut":3,"store-package":12}

| Product | Executable | Sources | Management | Support | Exact URL | Signature | Path Ref |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Google Chrome | chrome.exe | known-path, registry-uninstall, running-process, start-menu-shortcut | managed | candidate | unavailable | Valid | 5fea49019479 |
| Mozilla Firefox | firefox.exe | known-path, registry-uninstall, running-process, start-menu-shortcut | unsupported | unsupported | unsupported | Valid | 016ac9991f18 |
| Microsoft Edge | msedge.exe | known-path, registry-uninstall, start-menu-shortcut | managed | candidate | unavailable | Valid | 66e9b76d48ae |
| Microsoft Edge | msedge.exe | registry-uninstall | managed | candidate | unavailable | Valid | 87fddfe4dfed |
| Tor Browser | package-ref-only | store-package | unsupported | unsupported | unsupported | not-checked-package | dd4b6ce71c04 |
| Tor Browser | package-ref-only | store-package | unsupported | unsupported | unsupported | not-checked-package | 80d033bcf112 |
| Microsoft Edge | package-ref-only | store-package | manual-required | manual-required | manual-required | not-checked-package | c75dcd9395ab |
| Microsoft Edge | package-ref-only | store-package | manual-required | manual-required | manual-required | not-checked-package | 8df66e304b36 |
| Tor Browser | package-ref-only | store-package | unsupported | unsupported | unsupported | not-checked-package | e0959356b63e |
| Arc Browser | package-ref-only | store-package | unsupported | unsupported | unsupported | not-checked-package | 2485c4443942 |
| Microsoft Edge | package-ref-only | store-package | manual-required | manual-required | manual-required | not-checked-package | 7c5b35e6c4f3 |
| Microsoft Edge | package-ref-only | store-package | manual-required | manual-required | manual-required | not-checked-package | 12b55bf0cf85 |
| Tor Browser | package-ref-only | store-package | unsupported | unsupported | unsupported | not-checked-package | d654f48336dd |
| Microsoft Edge | package-ref-only | store-package | manual-required | manual-required | manual-required | not-checked-package | e9440135849a |
| Tor Browser | package-ref-only | store-package | unsupported | unsupported | unsupported | not-checked-package | b2d7fbb39674 |
| Tor Browser | package-ref-only | store-package | unsupported | unsupported | unsupported | not-checked-package | 2d021053e3a4 |

No product checklist upgrade is claimed. Live registry, shortcut, known-path, store-package, and process evidence improves the WP04 manual platform proof, but exact URL/tab evidence, active-tab certainty, browser content capture, AppLocker/App Control application, blocking, rollback, and enforcement remain unclaimed.
