<!-- agent-capsule -->

> Agent Capsule
> Doc: Browser Policy Settings Catalog
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Browser Policy Settings Catalog

Status: planning catalog for parent portal Browser/Policy UI, contracts, and
future implementation slices.

This document lists the parent-configurable browser settings we may need across
family, child, device, browser, session, and one-time override levels. It is not
a final screen design. It is deliberately broad so later UI work can choose a
usable subset without losing product capability.

Use this together with
[`managed-unmanaged-browser.md`](../../../plans/browser-plan/workpacks/managed-unmanaged-browser.md),
[`expectations/browser-evidence.md`](expectations/browser-evidence.md), and
[`architecture/browser-url-tab-evidence-capture.md`](architecture/browser-url-tab-evidence-capture.md).

## Schema Proposal Snapshot

The full proposal lives in
[`browser-control-schema-proposal.md`](../../../plans/browser-plan/workpacks/browser-control-schema-proposal.md). It
is a worker handoff guide, not final runtime source. The worker who implements
it must translate the shape into repo-valid Effect Schema contracts, branded
ids, decode helpers, tests, Rust protocol parity where needed, and child-agent
validation/compile behavior.

The proposed structure separates UI authoring from enforcement:

```json
{
  "authoringManifest": {
    "sections": [
      {
        "sectionId": "browser-management",
        "fields": [
          {
            "fieldId": "browser.enabled",
            "kind": "boolean",
            "question": "Enable browser management?",
            "writesTo": "/browserPolicy/enabled"
          },
          {
            "fieldId": "browser.defaultPosture",
            "kind": "single-choice",
            "question": "What should happen to browser activity?",
            "writesTo": "/browserPolicy/defaultPosture",
            "visibleWhen": {
              "path": "/browserPolicy/enabled",
              "equals": true
            },
            "options": ["allow", "observe", "warn", "ask", "limit", "block"]
          }
        ]
      }
    ]
  },
  "policyValue": {
    "browserPolicy": {
      "enabled": true,
      "defaultPosture": "limit",
      "unmanagedBrowser": {
        "mode": "relaunch-managed"
      }
    }
  },
  "effectivePolicy": {
    "browserActivityDefaultDecision": "limit",
    "unmanagedBrowserDecision": "relaunch-managed",
    "exactUrlRequires": "fresh-managed-active-tab"
  }
}
```

Portal should render from the authoring manifest, write to the policy value
document, and ask the child agent to validate and compile the effective policy.
The child agent enforces only the locally persisted compiled policy, so browser
rules keep working when Portal is offline.

## How To Read This Catalog

Every setting should eventually answer these questions:

- Who owns it: family default, child profile, device, browser, rule, temporary
  override, or admin setup.
- What mode it uses: off, observe, dry-run, warn, ask, limit, block, relaunch, or
  enforce.
- What proof it requires: process, foreground window, network/domain, managed tab
  list, proven active tab, exact URL, download metadata, AI classification, or
  parent-entered rule.
- What platforms can support it now: Windows, macOS, Linux, Android, iOS, web
  authoring only, unavailable, or manual-required.
- What happens when proof is missing: allow, observe, warn, ask, block until
  ready, degrade, or mark unavailable.
- What gets audited: policy decision, evidence ref, source/capability, adapter
  result, timer state, parent override, rollback, and policy version.

The same visible rule can have different enforcement strength depending on the
proof layer. For example, a domain rule can run from network evidence, but an
exact URL rule requires managed browser evidence.

## Global Rule Dimensions

These dimensions should be available on most browser rules, even if hidden in a
simple UI.

### Ownership Scope

- Family default.
- Child profile.
- Child age band.
- Device.
- Platform.
- Browser family.
- Managed browser profile.
- Unmanaged browser group.
- Browser session.
- Site/domain/category target.
- Download target.
- Temporary parent override.
- Emergency/school/admin preset.

### Policy Mode

- Off.
- Observe only.
- Dry run only.
- Warn child.
- Notify parent.
- Ask parent.
- Limit.
- Block.
- Relaunch managed browser.
- Enforce with rollback.
- Unavailable on this platform.

### Enforcement Phase

- Before browser launch.
- At process launch.
- At managed browser startup.
- At navigation start.
- Before request is sent.
- After response/download starts.
- At active tab change.
- At foreground window change.
- During session timer.
- At download start.
- At download completion.
- After unmanaged browser detection.
- At network/domain observation.
- At report generation only.

### Evidence Requirement

- No evidence required.
- Parent-authored rule only.
- Installed browser inventory.
- Running process.
- Foreground browser window.
- Browser command-line URL.
- Network/domain observation.
- DNS observation.
- Managed tab list.
- Proven active tab.
- Fresh exact URL.
- Fresh page title.
- Fresh download metadata.
- Browser policy event.
- Browser extension event.
- CDP event.
- OS app-control event.
- Local AI classification with evidence refs.
- Parent approval state.

### Freshness Requirement

- Real-time only.
- Fresh within seconds.
- Fresh within minutes.
- Stale allowed for reports only.
- Stale allowed for dry-run only.
- Degraded evidence allowed.
- Unavailable means ask parent.
- Unavailable means allow.
- Unavailable means block.

### Rule Priority

- Block wins.
- Limit wins.
- Ask wins.
- Warn wins.
- Allow wins.
- Most specific target wins.
- Child override wins family default.
- Device override wins child default.
- Temporary parent override wins.
- Emergency/school mode wins.
- Latest rule wins.
- Highest severity wins.
- Safer evidence wins weaker evidence.
- Managed exact URL wins network/domain guess.

## Master Browser Control Settings

These are top-level switches.

- Enable browser controls.
- Disable browser controls.
- Enable observe-only browser mode.
- Enable browser dry-run decisions.
- Enable enforcement for browser rules.
- Require managed browser for exact web rules.
- Allow browser controls while local agent is offline.
- Require child device to be online before browser enforcement.
- Enable browser rules for this family.
- Enable browser rules for this child.
- Enable browser rules for this device.
- Enable browser rules for this platform.
- Pause all browser rules until a chosen time.
- Emergency allow all browser activity.
- Emergency block all browser activity.
- School mode browser preset.
- Bedtime mode browser preset.
- Travel mode browser preset.
- Guest device browser preset.
- New device default browser preset.

## Browser Discovery Settings

These settings decide what the child-device agent scans or classifies.

- Scan installed browsers.
- Scan running browsers.
- Scan default browser setting.
- Scan browser versions.
- Scan browser channels.
- Scan browser executable signatures.
- Scan browser executable hashes.
- Scan browser install paths.
- Scan browser profile readiness.
- Scan browser extension install state.
- Scan browser policy state.
- Scan browser-like processes.
- Scan portable browser locations.
- Scan renamed browser executables.
- Scan embedded WebView shells.
- Scan Tor/private browsers.
- Scan unknown Chromium forks.
- Scan child-accessible browser shortcuts.
- Scan protocol handlers such as HTTP and HTTPS.
- Scan browser command lines for launch URLs when available.
- Scan for unsupported browser state.
- Scan for unmanaged browser state.
- Notify parent when a new browser is found.
- Ask parent before allowing a newly found browser.
- Auto-classify new browser as unmanaged.
- Auto-classify unknown browser as blocked.
- Auto-classify unknown browser as monitor only.
- Re-scan browser inventory on schedule.
- Re-scan browser inventory after install events.

## Browser Coverage Settings

These decide which browser families the policy engine recognizes and how each
one is treated.

- Cover Microsoft Edge stable.
- Cover Microsoft Edge beta/dev/canary.
- Cover Chrome stable.
- Cover Chrome beta/dev/canary.
- Cover Chrome for Testing.
- Cover Brave.
- Cover Opera.
- Cover Vivaldi.
- Cover Firefox.
- Cover Safari/WebKit.
- Cover Android Chrome.
- Cover Android browser apps.
- Cover iOS Safari.
- Cover iOS third-party browsers.
- Cover embedded WebViews.
- Cover Electron shells.
- Cover game launchers with embedded web content.
- Cover Tor Browser.
- Cover private/incognito browser windows.
- Cover unknown Chromium browsers.
- Cover portable browsers.
- Cover renamed browsers.
- Allow unsupported browser as monitor only.
- Treat unsupported browser as unmanaged.
- Treat unsupported browser as blocked.
- Require adapter proof before exact rules apply.

## Managed Browser Setup Settings

These settings control the managed browser path.

- Allow managed browser.
- Require managed browser.
- Disable managed browser.
- Auto-create managed browser profile.
- Keep managed profile persistent.
- Clear managed profile on schedule.
- Clear managed cookies/cache on schedule.
- Clear managed downloads on schedule.
- Allow bookmarks in managed browser.
- Provision school bookmarks.
- Provision allowed-start-page bookmarks.
- Set managed browser as default browser.
- Restore previous default browser on uninstall.
- Open HTTP links in managed browser.
- Open HTTPS links in managed browser.
- Open school app links in managed browser.
- Open unknown web links in managed browser.
- Use Ocentra launcher for browser start.
- Use managed shell/owned WebView.
- Use installed Edge as managed browser.
- Use installed Chrome as managed browser.
- Use Chrome for Testing as managed browser.
- Allow parent/admin to choose browser executable.
- Require signed approved browser executable.
- Require browser version minimum.
- Block managed launch if browser version is unsupported.
- Notify parent when managed browser needs update.
- Allow managed browser without extension.
- Require managed extension.
- Allow managed extension optional mode.
- Force-install managed extension where supported.
- Enable native messaging host.
- Require native messaging host heartbeat.
- Enable browser policy provisioning.
- Enable CDP bridge.
- Enable WebDriver BiDi bridge.
- Enable extension bridge.
- Enable owned WebView bridge.
- Enable loopback-only browser bridge.
- Use random bridge port per session.
- Store bridge endpoint only as redacted reference.
- Reject default profile bridge.
- Reject unmanaged profile bridge.
- Reject stale managed session bridge.
- Close bridge when managed session ends.
- Degrade safely when bridge is unavailable.
- Block exact web rules until managed setup is ready.

## Managed Browser Operation Settings

These settings control how managed sessions behave after setup.

- Auto-launch managed browser at sign-in.
- Auto-launch managed browser when child opens a web link.
- Keep managed browser running in background.
- Close managed browser at bedtime.
- Close managed browser after inactivity.
- Restore managed session after restart.
- Restore previous tabs in managed session.
- Start with approved home page.
- Start with blank page.
- Start with school dashboard.
- Disable unmanaged profile switching.
- Disable private/incognito mode where supported.
- Disable guest mode where supported.
- Disable extension installs by child.
- Allow approved extensions only.
- Block developer tools in managed browser where supported.
- Block browser settings page where supported.
- Block clearing browser history where supported.
- Block changing managed search engine where supported.
- Block changing default download folder where supported.
- Enable safe browsing or browser-native protection where supported.
- Keep managed session visible to parent reports.
- Allow child to request managed setup repair.
- Notify parent if managed browser cannot launch.

## Unmanaged Browser Handling Settings

These are the most important bypass settings.

- Allow unmanaged browsers.
- Disallow unmanaged browsers.
- Monitor unmanaged browsers.
- Warn child on unmanaged browser.
- Notify parent on unmanaged browser.
- Ask parent before unmanaged browser continues.
- Apply grace period before action.
- Close unmanaged browser.
- Close unmanaged browser and launch managed browser.
- Block unmanaged browser launch.
- Block browser-like unknown processes.
- Block portable browsers.
- Block renamed browsers.
- Block unsupported browsers.
- Block Tor/private browsers.
- Allow Edge unmanaged.
- Allow Chrome unmanaged.
- Allow Firefox unmanaged.
- Allow Brave/Opera unmanaged.
- Allow specific unmanaged executable path.
- Allow specific unmanaged executable signature.
- Allow specific unmanaged browser during schedule.
- Allow unmanaged browser only in observe mode.
- Allow unmanaged browser only for adult/admin child profile.
- Allow unmanaged browser only with parent PIN.
- Record unmanaged browser as bypass evidence.
- Escalate after repeated unmanaged launches.
- Escalate after repeated relaunch attempts.
- Escalate if child kills managed browser.
- Escalate if bridge disappears during session.
- Do not terminate if unsaved-data warning is possible.
- Terminate immediately in strict mode.
- Relaunch captured URL in managed browser when available.
- Relaunch managed browser without URL when target is unknown.
- Keep unmanaged browser open but block network.
- Keep unmanaged browser open but hide exact URL controls.
- Treat unmanaged browser time as browser app time.
- Treat unmanaged browser time as unknown web time.

## URL And Tab Evidence Settings

These decide what exact browser state may be collected and used.

- Allow managed URL inspection.
- Disable exact URL inspection.
- Collect full URL.
- Collect domain/origin only.
- Collect page title.
- Collect active tab state.
- Collect tab list.
- Collect window id.
- Collect tab id.
- Collect target id.
- Collect active/inactive state.
- Collect navigation events.
- Collect redirect chain.
- Collect request URL for main frame.
- Collect request URL for subresources.
- Ignore subresource URLs.
- Collect favicon URL.
- Disable favicon collection.
- Collect only when rule matches.
- Collect only when active tab is proven.
- Collect only while managed browser is foreground.
- Collect while managed browser is background.
- Collect only during configured schedules.
- Redact query string by default.
- Allow query string for exact rules.
- Redact sensitive query parameters.
- Redact search terms by default.
- Store search terms only when parent enables search rules.
- Store exact URL only for blocked or asked decisions.
- Store exact URL only in local child journal.
- Show exact URL only after parent reveal action.
- Mark tab-list-only state distinctly.
- Mark unknown-active state distinctly.
- Mark stale evidence distinctly.
- Ignore stale URL evidence for enforcement.
- Allow stale URL evidence for report history.

## Rule Target Settings

These decide what a browser rule can match.

- Exact URL.
- URL prefix.
- URL pattern.
- Domain.
- Origin.
- Site.
- Site category.
- Unknown category.
- Search engine.
- Search term.
- Video platform.
- Video channel.
- Video id.
- Page title keyword.
- Browser family.
- Browser version.
- Browser channel.
- Managed state.
- Unmanaged state.
- Browser session.
- Browser process.
- Browser executable path.
- Browser signature/hash reference.
- Capability state.
- Active tab certainty.
- Evidence freshness.
- Source/custody label.
- Download source URL.
- Download filename.
- Download extension.
- Download MIME type.
- Download size.
- Browser network domain.
- Browser network destination.
- Time of day.
- Schedule window.
- Device location/profile context if a later milestone supports it.
- Parent approval state.
- Local AI classification.
- Local AI confidence/reason.

## Rule Action Settings

These are possible rule outcomes.

- Allow.
- Monitor.
- Log only.
- Warn child.
- Notify parent.
- Ask parent.
- Ask parent and pause.
- Ask parent and allow temporarily.
- Time-limit.
- Reduce remaining budget.
- Start session timer.
- End session timer.
- Block page.
- Close tab.
- Close browser.
- Redirect to allowed page.
- Redirect to managed browser.
- Pause browser access.
- Lock browser access until approval.
- Block unmanaged browser process.
- Terminate unmanaged browser process.
- Block network/domain.
- Cancel download.
- Quarantine download.
- Delete download.
- Mark for parent review.
- Require child explanation.
- Require parent PIN.
- No-op because unsupported.
- No-op because degraded.
- Roll back enforcement.

## Observe Versus Enforce Settings

Most settings should have an observe/enforce mode instead of only on/off.

- Observe installed browsers only.
- Observe running browsers only.
- Observe managed browser readiness only.
- Observe exact URL without enforcing.
- Observe unmanaged browser without action.
- Observe downloads without action.
- Observe network domains without action.
- Dry-run page block.
- Dry-run domain block.
- Dry-run unmanaged browser block.
- Dry-run download block.
- Dry-run time budget.
- Warn instead of block when proof is weak.
- Ask instead of block when proof is weak.
- Enforce only when proof is fresh.
- Enforce only from managed active tab.
- Enforce from managed tab list.
- Enforce from network/domain evidence.
- Enforce from process evidence.
- Enforce from local AI classification.
- Never enforce from stale/degraded evidence.
- Never enforce from network-only evidence for exact URL rules.
- Fallback from enforce to ask.
- Fallback from enforce to warn.
- Fallback from enforce to observe.

## Schedule Settings

These decide when rules apply.

- Always.
- School hours.
- Homework hours.
- Bedtime.
- Morning routine.
- Weekend.
- Custom schedule.
- One-time schedule.
- Holiday schedule.
- Travel schedule.
- Per-child schedule.
- Per-device schedule.
- Per-browser schedule.
- Per-site schedule.
- Per-category schedule.
- Per-download schedule.
- Schedule timezone.
- Schedule source from parent device.
- Schedule source from child device.
- Allow grace before schedule starts.
- Allow grace after schedule ends.
- Lock browser during blackout window.
- Allow only school domains during school schedule.
- Allow only managed browser during school schedule.
- Allow unmanaged browser only outside school/bedtime.

## Time Budget Settings

These settings control time limits.

- Enable browser daily quota.
- Enable browser weekly quota.
- Enable browser session limit.
- Enable exact site daily quota.
- Enable domain daily quota.
- Enable category daily quota.
- Enable video platform quota.
- Enable search engine quota.
- Enable unmanaged browser quota.
- Enable managed browser quota.
- Count only foreground browser time.
- Count background browser time.
- Count active tab time.
- Count tab-list-only time.
- Count unmanaged browser time as unknown web time.
- Count blocked/paused time against quota.
- Do not count parent-approved school sites.
- Do not count allowlisted domains.
- Do not count parent-approved session extension.
- Grace minutes before block.
- Warning threshold before budget ends.
- Ask parent when budget ends.
- Block when budget ends.
- Switch to allowlist when budget ends.
- Reset quota daily.
- Reset quota weekly.
- Carry over unused minutes.
- Do not carry over unused minutes.
- Parent can extend once.
- Parent can extend for session.
- Parent can extend until time.

## Parent Approval Settings

These settings decide what needs parent involvement.

- Require approval for blocked site.
- Require approval for new domain.
- Require approval for unknown category.
- Require approval for unmanaged browser.
- Require approval for unsupported browser.
- Require approval for private/incognito attempt.
- Require approval for download.
- Require approval for executable download.
- Require approval for archive download.
- Require approval for time extension.
- Require approval for managed browser setup repair.
- Require approval for browser extension permission.
- Require approval for new browser install.
- Require approval for browser policy change.
- Require approval for clearing managed profile.
- Require approval for disabling bridge.
- Require approval for LAN-visible browser data.
- Allow child to request site access.
- Allow child to request category access.
- Allow child to request unmanaged browser access.
- Allow child to request download approval.
- Allow child to request extra time.
- Allow child to include note.
- Allow parent to approve once.
- Allow parent to approve this session.
- Allow parent to approve until a time.
- Allow parent to approve always.
- Allow parent to deny once.
- Allow parent to deny always.
- Expire unanswered approval after N minutes.
- Default unanswered approval to deny.
- Default unanswered approval to allow temporarily.
- Default unanswered approval to continue observe only.

## Override Settings

These are parent override choices.

- Approve once.
- Approve for current tab.
- Approve for current domain.
- Approve for current category.
- Approve for current browser session.
- Approve until time.
- Approve for today.
- Approve for school schedule.
- Approve permanently.
- Deny once.
- Deny for session.
- Deny until time.
- Deny permanently.
- Extend time by minutes.
- Reset today's time budget.
- Cancel active block.
- Cancel pending ask.
- Cancel download.
- Allow download once.
- Trust this download source.
- Trust this browser executable.
- Trust this managed profile repair.
- Revoke previous override.
- Auto-expire override.
- Require override reason.
- Hide override details from child.
- Show override reason to child.

## Downloads Settings

These decide how browser downloads are observed and controlled.

- Enable download monitoring.
- Enable managed browser download metadata.
- Enable unmanaged download file observation.
- Ask parent before all downloads.
- Ask parent before executable downloads.
- Ask parent before archive downloads.
- Ask parent before unknown file types.
- Ask parent before large downloads.
- Allow document downloads.
- Allow image downloads.
- Allow school file downloads.
- Allow media downloads.
- Block executable downloads.
- Block script downloads.
- Block archive downloads.
- Block unknown file extensions.
- Block downloads from blocked domains.
- Block downloads from unknown domains.
- Block downloads from unmanaged browsers.
- Cancel managed browser download.
- Quarantine completed download.
- Delete blocked download.
- Keep blocked file metadata only.
- Show download URL when managed evidence exists.
- Hide source URL for unmanaged download.
- Show filename.
- Show MIME type.
- Show file size.
- Show browser danger status.
- Show interruption reason.
- Show download completion status.
- Scan downloaded file hash against local policy if a later adapter supports it.
- Notify parent at download start.
- Notify parent at download completion.
- Notify parent only on risky download.

## Search Settings

Search handling must be explicit because search terms can be sensitive and may
appear inside URLs.

- Enable search engine detection.
- Disable search term collection.
- Redact search terms.
- Allow search term rules.
- Allow search category rules.
- Allow safe-search enforcement where browser/search provider supports it.
- Require managed browser for search term rules.
- Use domain-only search evidence.
- Store search terms only for policy decisions.
- Store search terms only in local child journal.
- Show search terms in parent report.
- Hide search terms unless parent reveals.
- Block search queries by keyword.
- Warn on search queries by keyword.
- Ask parent on unknown/risky search.
- Treat network-only search domain as insufficient for search terms.
- Apply school-search allowlist.

## Video And Channel Settings

Video rules may need URL, title, domain, and later local AI/category evidence.

- Enable video platform detection.
- Enable video channel rules.
- Enable video URL rules.
- Enable video category rules.
- Enable video time budgets.
- Enable video title matching.
- Require managed browser for video rules.
- Allow video rules from network/domain only.
- Treat network/domain video evidence as weak.
- Ask parent for unknown video channel.
- Block specific video channels.
- Allow specific video channels.
- Limit video platform time.
- Limit video category time.
- Exclude school video domains from entertainment budget.
- Record video evidence id.
- Avoid page body or transcript collection unless a later milestone approves it.

## Private, Incognito, Tor, And Anti-Bypass Settings

These settings deal with browser states designed to reduce visibility.

- Block private/incognito windows in managed browser where supported.
- Allow private/incognito but mark evidence unavailable.
- Warn when private/incognito is attempted.
- Ask parent for private/incognito.
- Block Tor Browser.
- Monitor Tor Browser.
- Block VPN browser extensions.
- Block proxy browser extensions.
- Block browser profile switching.
- Block unmanaged extension installs.
- Block clearing managed browser history.
- Block deleting managed browser profile.
- Detect managed bridge closed unexpectedly.
- Detect managed extension disabled.
- Detect native host missing.
- Detect browser policy removed.
- Detect browser process launched outside Ocentra.
- Detect browser executable copy/rename.
- Detect new portable browser.
- Detect unknown browser network activity.
- Escalate repeated bypass attempts.
- Lock to managed browser until parent resolves.

## Network And Domain Fallback Settings

These should be separate from exact URL rules.

- Enable network/domain observation.
- Disable network/domain observation.
- Enable DNS observation.
- Enable process-attributed network observation.
- Enable domain blocking.
- Enable DNS filtering.
- Enable proxy filtering.
- Enable VPN filtering.
- Enable Windows Filtering Platform adapter.
- Enable firewall adapter.
- Enable Android always-on VPN adapter.
- Enable iOS web domain shielding where available.
- Block domain for all browsers.
- Block domain only for unmanaged browsers.
- Block domain only outside managed browser.
- Block browser network unless managed.
- Allow local network domains.
- Allow school domains.
- Allow update/service domains.
- Ignore CDN domains in parent summary.
- Show CDN domains only in diagnostics.
- Treat network-only evidence as domain-level.
- Never treat network-only evidence as exact URL.
- Use network evidence for local AI context.
- Use network evidence for dry-run only.
- Use network evidence for enforcement.
- Fallback to network block when browser bridge fails.
- Fallback to ask when network confidence is low.

## Browser App And Process Settings

These settings are about browser programs, not pages.

- Time-limit browser app use.
- Time-limit unmanaged browser app use.
- Time-limit managed browser app use.
- Block browser executable.
- Allow browser executable.
- Allow approved executable paths only.
- Allow signed browser executables only.
- Block unsigned browser-like processes.
- Block portable browser folders.
- Block browser installation.
- Detect browser installation.
- Notify parent on browser installation.
- Require approval for browser installation.
- Block browser updater if it breaks managed support.
- Allow browser updater.
- Allow only specific browser versions.
- Block unsupported browser versions.
- Block browser command-line flags that disable controls.
- Block remote debugging unless Ocentra owns the session.
- Block child-launched CDP bridge.
- Kill orphaned managed browser process.
- Roll back failed process block.
- Audit process block result.

## Child-Facing Experience Settings

These settings decide what the child sees.

- Show warning text.
- Show block reason.
- Show ask-parent state.
- Show time left.
- Show daily budget left.
- Show session budget left.
- Show managed browser required message.
- Show unmanaged browser closed message.
- Show download blocked message.
- Show approval pending message.
- Show approval approved message.
- Show approval denied message.
- Show parent note.
- Show safe page after block.
- Show open managed browser action.
- Show request access action.
- Show request more time action.
- Hide sensitive rule details.
- Hide parent-only diagnostics.
- Use age-appropriate text preset.
- Use silent enforcement.
- Use visible enforcement.
- Allow child to retry after parent action.
- Allow child to view allowed alternatives.

## Parent Report Settings

These decide what parents can see later.

- Show managed browser status.
- Show managed setup health.
- Show recent exact URL.
- Show recent domain/title.
- Show recent active tab.
- Show tab-list-only evidence.
- Show unmanaged browser use.
- Show unsupported browser use.
- Show browser app time.
- Show site/domain time.
- Show category time.
- Show download events.
- Show blocked downloads.
- Show policy decisions.
- Show block results.
- Show ask-parent results.
- Show time budget state.
- Show source/capability state.
- Show evidence freshness.
- Show stale/degraded state.
- Show adapter errors.
- Show platform unavailable controls.
- Show local-only custody label.
- Show LAN live source.
- Show parent cache source.
- Show parent export/report source.
- Hide exact URL by default.
- Require reveal for exact URL.
- Summarize by category.
- Summarize by domain.
- Summarize by browser.
- Summarize by child.
- Summarize by device.
- Summarize unmanaged bypass attempts.
- Export browser report.
- Redact exact details in report.

## Portal Display Settings

These are specific to parent portal surfaces.

- Portal may display managed status.
- Portal may display exact URL.
- Portal may display domain/title.
- Portal may display unmanaged use.
- Portal may display policy decisions.
- Portal may display block results.
- Portal may display time budget.
- Portal may display source/capability.
- Portal may display downloads.
- Portal may display platform support matrix.
- Portal may display setup guide.
- Portal may display unavailable controls.
- Portal may display local agent offline state.
- Portal may display LAN source.
- Portal may display parent cache source.
- Portal may display parent export source.
- Portal may display report-only summaries.
- Portal must not run capture code.
- Portal must not read browser profiles.
- Portal must not connect directly to browser bridge.
- Portal must not read SQLite or journal files directly.

## Portal Action Settings

These are actions the portal may send through typed service contracts.

- View browser status.
- Refresh browser status.
- Preview policy.
- Answer ask-parent request.
- Approve once.
- Deny once.
- Extend time.
- Cancel block.
- Launch managed browser through agent where allowed.
- Open setup guide.
- Request managed setup repair.
- Export browser report.
- Delete browser evidence.
- Change retention.
- Show unavailable reason.
- Show capability diagnostics.
- Show bridge diagnostics.
- Show policy audit.
- Never send raw OS commands from portal.
- Never connect to DevTools from portal.

## Portal AI Settings

These settings govern parent-facing assistant help.

- Allow Portal AI to summarize browser report.
- Allow Portal AI to explain policy decision.
- Allow Portal AI to draft parent note.
- Allow Portal AI to suggest rule changes.
- Allow Portal AI to cite evidence refs.
- Allow Portal AI to use URL metadata.
- Allow Portal AI to use domain/title metadata.
- Allow Portal AI to use source/custody labels.
- Allow Portal AI to use degraded state.
- Disallow raw page content.
- Disallow chat content.
- Disallow screenshots.
- Disallow raw protocol payloads.
- Require manual review before applying AI suggestion.
- Use local AI only.
- Use parent-authorized API AI only.
- Fall back to manual report if AI unavailable.
- Show AI unavailable state.
- Show AI confidence/reason where available.

## Data Source And Custody Settings

These decide where browser evidence may come from and where it may be used.

- Use child local live data.
- Use LAN live data.
- Use parent cache.
- Use parent-owned export.
- Use parent report.
- Mark unavailable.
- Do not use Ocentra-hosted storage for browser evidence by default.
- Allow parent-owned storage sync.
- Disable browser evidence cloud sync.
- Allow report compilation from redacted evidence refs.
- Use source/custody label in every browser row.
- Require custody label before Portal display.
- Require custody label before AI summary.
- Require custody label before export.
- Show stale parent cache separately from live child data.
- Show offline child device separately from no activity.

## Retention Settings

These decide how long browser data stays available.

- Keep fresh-only browser state.
- Keep until device reset.
- Keep 24 hours.
- Keep 7 days.
- Keep 30 days.
- Keep custom duration.
- Delete expired exact URLs.
- Delete expired titles.
- Keep redacted domain summary.
- Keep policy audit longer than raw evidence.
- Keep parent override audit.
- Keep block audit.
- Keep unmanaged bypass audit.
- Keep download audit.
- Delete browser evidence on parent request.
- Delete browser evidence when child profile removed.
- Delete managed browser profile data on schedule.
- Export before delete.
- Redact before export.
- Show retention status to parent.

## Audit Settings

Every strict action should produce an audit record.

- Audit policy decision.
- Audit rule id.
- Audit policy version.
- Audit child profile.
- Audit device id.
- Audit browser family/version.
- Audit managed/unmanaged state.
- Audit capability status.
- Audit evidence ref.
- Audit AI ref.
- Audit adapter result.
- Audit enforcement mechanism.
- Audit enforcement outcome.
- Audit timer state.
- Audit parent override.
- Audit child request.
- Audit rollback.
- Audit failed block.
- Audit failed termination.
- Audit degraded/unavailable state.
- Audit custody/source label.
- Audit download action.
- Audit network/domain fallback.
- Audit conflict resolution rule.
- Audit manual parent change.
- Audit setup/provisioning change.

## Capability Failure Settings

These decide how policy behaves when the proof layer fails.

- If managed profile missing: observe.
- If managed profile missing: ask.
- If managed profile missing: block until setup.
- If bridge missing: observe.
- If bridge missing: warn.
- If bridge missing: ask.
- If bridge missing: block exact web rules.
- If extension disabled: observe.
- If extension disabled: warn.
- If extension disabled: block.
- If native host missing: repair prompt.
- If browser unsupported: monitor only.
- If browser unsupported: ask.
- If browser unsupported: block.
- If evidence stale: do not enforce exact URL rule.
- If evidence stale: allow but report.
- If evidence stale: ask.
- If network adapter unavailable: continue managed browser controls.
- If process control unavailable: fall back to warn/ask.
- If enforcement fails: rollback and audit.
- If rollback fails: notify parent.
- If child device offline: use last known report only.
- If platform unsupported: show unavailable.

## Conflict Resolution Settings

These settings decide what happens when rules overlap.

- Specific URL beats domain.
- Domain beats category.
- Child-specific rule beats family default.
- Device-specific rule beats child default.
- Temporary override beats static rule.
- Block beats allow.
- Allow beats block for parent-approved exception.
- Ask beats block when parent approval is pending.
- Time limit beats allow.
- Bedtime rule beats daily quota.
- School mode allowlist beats entertainment block.
- Download block beats site allow.
- Managed exact evidence beats network evidence.
- Fresh evidence beats stale evidence.
- Higher confidence AI decision beats lower confidence only in dry-run.
- Local deterministic policy beats AI suggestion.
- Parent manual override beats AI suggestion.
- Emergency allow beats normal policy.
- Emergency block beats normal policy.

## Local AI Browser Settings

These decide how browser evidence can be used by the child-device AI/policy
pipeline.

- Allow AI to reference browser evidence ids.
- Allow AI to reference URL metadata.
- Allow AI to reference domain/origin.
- Allow AI to reference page title.
- Allow AI to reference active certainty.
- Allow AI to reference recent local context.
- Allow AI to reference parent rule context.
- Allow AI to reference source/custody label.
- Allow AI to reference degraded state.
- Allow AI to reference browser app/session time.
- Allow AI to reference network/domain evidence.
- Allow AI to reference download metadata.
- Require AI output to cite evidence refs.
- Require deterministic policy after AI.
- Use AI only for classification.
- Use AI only for report explanation.
- Use AI only in dry-run.
- Do not send raw page content to AI.
- Do not send screenshots to AI.
- Do not send chat content to AI.
- Do not send raw protocol payloads to AI.
- Treat AI unavailable as unknown.
- Treat AI unavailable as ask parent.
- Treat AI unavailable as deterministic rules only.

## Never-Collect Settings

These are not toggles for normal product operation. They are default boundaries
unless a future explicit product/legal/architecture decision changes them.

- Do not collect page body text.
- Do not collect chat message content.
- Do not collect screenshots for browser rules.
- Do not collect keystrokes.
- Do not collect form values.
- Do not collect passwords.
- Do not collect cookies.
- Do not collect tokens.
- Do not collect browser local storage.
- Do not collect browser session storage.
- Do not collect browser secrets.
- Do not collect decrypted HTTPS payloads.
- Do not store raw DevTools protocol dumps.
- Do not infer exact URL from window title.
- Do not infer exact URL from network destination.
- Do not attach to default personal profile bridge.
- Do not connect Portal directly to browser bridge.

## Platform Settings

These settings are mostly capability gates.

### Windows

- Enable Windows managed Edge.
- Enable Windows managed Chrome.
- Enable Windows Chrome for Testing.
- Enable Windows process observation.
- Enable Windows foreground window observation.
- Enable Windows process termination.
- Enable Windows app-control blocking.
- Enable Windows browser executable allowlist.
- Enable Windows network filtering.
- Enable Windows DNS/domain filtering.
- Enable Windows managed extension native host.
- Enable Windows CDP managed bridge.
- Enable Windows browser policy provisioning.
- Mark Windows AppLocker/WDAC controls as manual-required until proven.
- Mark Windows WFP/domain controls as manual-required until proven.

### macOS

- Enable macOS managed browser profile.
- Enable macOS browser extension/native host.
- Enable macOS process observation.
- Enable macOS process control.
- Enable macOS browser policy profile where supported.
- Enable macOS network/content filter where entitled.
- Mark macOS controls unavailable until platform proof exists.

### Linux

- Enable Linux managed browser profile.
- Enable Linux extension/native host.
- Enable Linux process observation.
- Enable Linux process control.
- Enable Linux firewall/domain filtering.
- Mark distro-specific controls manual-required.

### Android

- Enable Android browser app observation.
- Enable Android app suspension.
- Enable Android package hiding.
- Enable Android device-owner browser controls.
- Enable Android managed Chrome config where available.
- Enable Android always-on VPN/domain filtering.
- Enable Android accessibility-assisted foreground evidence where explicitly
  approved.
- Enable Android owned browser/WebView path.
- Mark exact arbitrary mobile browser URL unavailable unless adapter proof
  exists.

### iOS And iPadOS

- Enable iOS Screen Time app/category/domain controls.
- Enable iOS Managed Settings shielding.
- Enable iOS web domain shields.
- Enable iOS MDM web content filtering.
- Enable iOS Network Extension filtering where entitled.
- Enable iOS Safari/domain-level rules where approved.
- Mark exact arbitrary browser URL capture unavailable unless Apple-approved
  proof exists.
- Mark non-entitled controls unavailable.

### Web Parent Portal

- Enable browser policy authoring.
- Enable browser report viewing.
- Enable policy preview.
- Disable child-device capture in web portal.
- Disable direct local file reads in web portal.
- Disable direct browser bridge access in web portal.
- Show web authoring as control surface only.

## Setup And Provisioning Settings

These settings are likely part of onboarding or advanced setup.

- Install managed browser support.
- Repair managed browser support.
- Provision managed profile.
- Provision browser policy.
- Provision native messaging host.
- Provision managed extension.
- Verify extension heartbeat.
- Verify CDP bridge.
- Verify localhost bridge only.
- Verify profile is Ocentra-owned.
- Verify default browser route.
- Verify HTTP protocol route.
- Verify HTTPS protocol route.
- Verify unmanaged browser detection.
- Verify process termination permission.
- Verify network/domain filtering permission.
- Verify download monitoring permission.
- Verify platform capability matrix.
- Show setup incomplete state.
- Show setup ready state.
- Show setup degraded state.
- Allow parent to run setup check.
- Allow parent to export setup diagnostics.

## Notifications And Escalation Settings

These decide when parent or child is notified.

- Notify parent on blocked site.
- Notify parent on ask-parent request.
- Notify parent on unmanaged browser.
- Notify parent on repeated bypass attempts.
- Notify parent on managed bridge failure.
- Notify parent on browser policy removal.
- Notify parent on extension disabled.
- Notify parent on new browser found.
- Notify parent on risky download.
- Notify parent on time budget warning.
- Notify parent on time budget exhausted.
- Notify child on warning.
- Notify child on block.
- Notify child on parent approval.
- Notify child on parent denial.
- Quiet hours for parent notifications.
- Escalate after N unmanaged attempts.
- Escalate after N blocked downloads.
- Escalate after N failed setup checks.
- Escalate to local-only report if notification provider unavailable.

## Gaps To Decide Before UI Contracts

These are not blockers for mock/UI exploration, but they should be decided before
locking contracts.

- Do browser settings live in `parent-domain`, `activity-domain`,
  `agent-protocol-domain`, or a new policy-domain package?
- Which settings are actual parent-editable controls versus internal capability
  state?
- Which settings are per-family defaults versus per-child overrides?
- Which settings are per-device only because capability differs by OS?
- Which settings need Effect Schema brands and decode helpers first?
- What is the minimal MVP set for Windows?
- Should exact URL policy require managed browser for MVP?
- Should unmanaged browser handling default to monitor, warn, ask, relaunch, or
  block?
- Should strict unmanaged blocking require a setup checklist first?
- Should parent be allowed to block all unknown browser-like processes?
- What is the first download policy surface: report only, ask, or block?
- Should search terms ever be stored, or should search rules use redacted URL
  metadata only?
- Should page title be stored by default or only when rule matches?
- What is the default retention for exact URLs?
- What is the default retention for domain summaries?
- What is the default retention for policy audit?
- What is the first conflict-resolution rule set?
- What is the first child-facing block/warn/request flow?
- What is the first parent override flow?
- What settings must be hidden until platform proof exists?
- What settings must be visible but unavailable so parents understand limits?
- What settings belong in guide text instead of controls?

## Candidate MVP Setting Set

If the first UI needs a manageable starting point, start with these:

- Enable browser controls.
- Mode: observe, dry-run, warn/ask, enforce.
- Require managed browser for exact web rules.
- Scan installed browsers.
- Scan running browsers.
- Detect unmanaged browsers.
- Allow managed browser.
- Launch or repair managed browser setup.
- Allow URL/domain/title evidence from managed browser.
- Redact query strings.
- Keep exact URL evidence for selected retention.
- Allow unmanaged browser: monitor, warn, ask, relaunch, block.
- Choose covered browsers: Edge, Chrome, Chrome for Testing, unsupported as
  unmanaged.
- Rule targets: exact URL, domain/origin, category, browser process, browser
  session, capability state.
- Rule actions: allow, warn, ask, limit, block.
- Time budgets: daily, session, site/domain, grace, blackout.
- Parent approvals: new domain, blocked site, unmanaged browser, downloads, time
  extension.
- Reports: managed status, recent URL/domain/title, unmanaged use, decisions,
  block results, time budget, source/capability.
- Proof requirement: process, foreground, managed tab list, proven active tab,
  fresh only, stale/degraded.
- Data custody: child local, LAN live, parent cache, parent export/report,
  unavailable.
- Audit: policy decision, evidence ref, adapter result, timer state, parent
  override, rollback, policy version.

## Related Existing Thinking

- `docs/managed-unmanaged-browser.md` defines managed versus unmanaged browser
  capability.
- `docs/policy Ui fix.md` already names Browser UI areas: Overview, Managed
  Browser, Web Rules, Bypass Handling, and Preview/Audit.
- `packages/activity-domain/src/browser-schemas.ts` already models browser
  evidence status, managed state, custody, and query visibility.
- `packages/activity-domain/src/browser-intervention-schemas.ts` already models
  browser intervention actions, mechanisms, outcomes, capability states, and
  unmanaged enforcement state.
- Future work still needs a formal settings/rules domain before these become
  source contracts.
