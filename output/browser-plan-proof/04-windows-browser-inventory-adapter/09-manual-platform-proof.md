# WP04 Live Windows Browser Inventory Proof

Generated: 2026-06-05T21:34:09.756Z

This proof ran against the local Windows host and stores only redacted refs, hashes, counts, executable basenames, and capability labels.
It does not store raw paths, raw URLs, page titles, page bodies, browser profile data, cookies, tokens, tabs, or browsing content.

Rows captured: 4
Source counts: {"known-path":3,"registry-uninstall":4,"running-process":2,"start-menu-shortcut":3}

| Product | Executable | Sources | Management | Support | Exact URL | Signature | Path Ref |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Google Chrome | chrome.exe | known-path, registry-uninstall, running-process, start-menu-shortcut | managed | candidate | unavailable | Valid | 5fea49019479 |
| Mozilla Firefox | firefox.exe | known-path, registry-uninstall, running-process, start-menu-shortcut | unsupported | unsupported | unsupported | Valid | 016ac9991f18 |
| Microsoft Edge | msedge.exe | known-path, registry-uninstall, start-menu-shortcut | managed | candidate | unavailable | Valid | 66e9b76d48ae |
| Microsoft Edge | msedge.exe | registry-uninstall | managed | candidate | unavailable | Valid | 87fddfe4dfed |

No product checklist upgrade is claimed. Live registry, shortcut, known-path, and process evidence improves the WP04 manual platform proof, but exact URL/tab evidence, active-tab certainty, browser content capture, AppLocker/App Control application, blocking, rollback, and enforcement remain unclaimed.
