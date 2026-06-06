# WP17 Manual Platform Proof

Command:

```powershell
cmd /c npm run test:managed-browser-intervention
```

Latest successful run:

- Evidence JSON: `test-results/managed-browser-intervention-proof/2026-06-02T23-29-49-841Z.json`
- Screenshots: `test-results/managed-browser-intervention-proof/2026-06-02T23-29-49-841Z-screenshots`
- Supported browser count: 3
- Chrome Stable: `C:\Program Files\Google\Chrome\Application\chrome.exe`, browser version recorded as `Chrome/148.0.7778.179` in the proof JSON.
- Firefox Stable: `C:\Program Files\Mozilla Firefox\firefox.exe`
- Edge Stable: `C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe`

Passed proof counts:

- Blocked site: 3
- YouTube/video URL block: 3
- Social signup approval hold: 3
- Social short-video warning: 3
- Browser game checking hold: 3
- Game purchase approval hold: 3
- Cloud gaming approval hold: 3
- Allowed control not blocked: 3

Manual-required labels:

- This proves managed browser page intervention on this Windows environment only.
- Unmanaged browser detection/fallback remains WP18/WP19.
- Native apps, native games, mobile browsers, store signing, and cloud-streaming frame analysis remain separate platform proof.
