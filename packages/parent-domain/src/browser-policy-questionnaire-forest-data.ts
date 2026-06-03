import {
  all,
  answerEquals,
  answerHasAnySelected,
  answerIncludes,
  answerIncludesAny,
  anyCondition,
  BrowserPolicyQuestionIds,
  computedFlag,
  not,
  type BrowserPolicyCondition,
  type BrowserPolicyOption,
  type BrowserPolicyQuestion,
  type BrowserPolicyQuestionId,
  type BrowserPolicySelectionMode,
  type BrowserPolicySurface,
} from './browser-policy-questionnaire-forest-contract';

const policyOn = computedFlag('policyIsOn');
const policyPaused = computedFlag('policyPaused');
const emergencyOverride = computedFlag('emergencyOverrideActive');
const policyIsActive = all([policyOn, not(emergencyOverride)]);
const policyModeSelected = all([policyIsActive, answerHasAnySelected('1.2')]);
const dryRunSelected = answerIncludesAny('1.3', ['simulate', 'simulate-report']);
const notDryRun = not(dryRunSelected);
const warnAskLimitBlock = answerIncludesAny('1.2', ['warn', 'ask-parent', 'limit', 'block']);

const option = (id: string, label: string): BrowserPolicyOption => ({ id, label });

function question(
  id: BrowserPolicyQuestionId,
  title: string,
  selectionMode: BrowserPolicySelectionMode,
  options: readonly BrowserPolicyOption[],
  showWhen: readonly BrowserPolicyCondition[] = [],
  surface: BrowserPolicySurface = 'rules'
): BrowserPolicyQuestion {
  return {
    id,
    title,
    selectionMode,
    surface,
    options,
    showWhen,
  };
}

export const BrowserPolicyQuestionnaireCompactOrder = BrowserPolicyQuestionIds.filter((id) =>
  id.startsWith('A') ? false : true
) as readonly BrowserPolicyQuestionId[];

export const BrowserPolicyQuestions = [
  question('1.1', 'Should browser policy be active?', 'single', [
    option('off', 'Off'),
    option('on', 'On'),
    option('paused', 'Paused'),
    option('emergency-allow', 'Emergency allow'),
    option('emergency-block', 'Emergency block'),
  ]),
  question(
    '1.2',
    'What should controlled browser activity do?',
    'multi',
    [
      option('observe', 'Observe'),
      option('warn', 'Warn'),
      option('ask-parent', 'Ask parent'),
      option('limit', 'Limit'),
      option('block', 'Block'),
    ],
    [policyIsActive]
  ),
  question(
    '1.3',
    'Should test or dry-run mode be used?',
    'single',
    [option('no', 'No'), option('simulate', 'Simulate only'), option('simulate-report', 'Simulate + report')],
    [policyIsActive]
  ),
  question(
    '2.1',
    'Which browser coverage level should be used?',
    'single',
    [option('common', 'Common'), option('known', 'All known'), option('strict', 'Strict'), option('custom', 'Custom')],
    [anyCondition([policyModeSelected, dryRunSelected])]
  ),
  question(
    '2.2',
    'How aggressively should browsers be discovered?',
    'single',
    [option('basic', 'Basic'), option('standard', 'Standard'), option('strict', 'Strict'), option('custom', 'Custom')],
    [all([policyIsActive, anyCondition([answerHasAnySelected('2.1'), dryRunSelected])])]
  ),
  question(
    '2.3',
    'What should happen when a new browser is found?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('notify-parent', 'Notify'),
      option('ask-parent', 'Ask parent'),
      option('unmanaged', 'Treat unmanaged'),
      option('block-until-approved', 'Block until approved'),
    ],
    [all([policyIsActive, answerHasAnySelected('2.1'), notDryRun])]
  ),
  question(
    '3.1',
    'Should a managed browser be required?',
    'single',
    [
      option('any', 'Any covered'),
      option('prefer-managed', 'Prefer managed'),
      option('managed-exact', 'Managed exact'),
      option('managed-all', 'Managed only'),
      option('platform-capability', 'Use capability'),
    ],
    [
      all([
        policyIsActive,
        anyCondition([
          answerIncludesAny('1.2', ['ask-parent', 'limit', 'block']),
          computedFlag('exactEvidenceSelected'),
          answerIncludesAny('2.1', ['strict', 'custom']),
        ]),
      ]),
    ]
  ),
  question(
    '3.2',
    'What managed-browser setup behavior should be used?',
    'multi',
    [
      option('auto-signin', 'Auto sign-in'),
      option('open-links', 'Open links'),
      option('keep-running', 'Keep running'),
      option('close-bedtime', 'Close bedtime'),
      option('close-inactive', 'Close inactive'),
      option('restore-session', 'Restore session'),
      option('restore-tabs', 'Restore tabs'),
      option('home-page', 'Home page'),
      option('blank-page', 'Blank page'),
      option('school-dashboard', 'School dash'),
      option('notify-launch-fail', 'Notify failure'),
      option('child-repair', 'Child repair'),
    ],
    [all([policyIsActive, answerIncludesAny('3.1', ['prefer-managed', 'managed-exact', 'managed-all'])])]
  ),
  question(
    '3.3',
    'What browser features should be locked in managed browser?',
    'multi',
    [
      option('lock-profile', 'Profile switch'),
      option('lock-private', 'Private mode'),
      option('lock-guest', 'Guest mode'),
      option('lock-extension-installs', 'Extensions'),
      option('approved-extensions', 'Approved only'),
      option('block-devtools', 'DevTools'),
      option('block-settings', 'Settings'),
      option('block-clear-history', 'Clear history'),
      option('block-search-engine', 'Search engine'),
      option('block-download-folder', 'Download folder'),
      option('safe-protection', 'Safe protection'),
    ],
    [all([policyIsActive, answerIncludesAny('3.1', ['managed-exact', 'managed-all'])])]
  ),
  question(
    '4.1',
    'What should happen if an unmanaged browser is used?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('warn', 'Warn child'),
      option('notify-parent', 'Notify'),
      option('ask-parent', 'Ask parent'),
      option('close', 'Close'),
      option('close-open-managed', 'Open managed'),
      option('block-launch', 'Block launch'),
    ],
    [
      all([
        policyIsActive,
        notDryRun,
        anyCondition([
          answerIncludesAny('1.2', ['warn', 'ask-parent', 'limit', 'block']),
          answerIncludesAny('3.1', ['prefer-managed', 'managed-all']),
          answerIncludesAny('2.1', ['known', 'strict', 'custom']),
        ]),
      ]),
    ]
  ),
  question(
    '4.2',
    'Which unmanaged browser exceptions are allowed?',
    'multi',
    [
      option('edge', 'Edge'),
      option('chrome', 'Chrome'),
      option('firefox', 'Firefox'),
      option('brave-opera', 'Brave/Opera'),
      option('path', 'Path'),
      option('signature', 'Signature'),
      option('schedule', 'Schedule'),
      option('approval', 'Approval'),
      option('grace', 'Grace'),
    ],
    [all([policyIsActive, answerIncludesAny('2.1', ['known', 'strict', 'custom'])])]
  ),
  question(
    '4.3',
    'Which bypass browsers should be treated specially?',
    'multi',
    [
      option('unknown-process', 'Unknown process'),
      option('portable', 'Portable'),
      option('renamed', 'Renamed'),
      option('unsupported', 'Unsupported'),
      option('tor-private', 'Tor/private'),
      option('chromium-forks', 'Chromium forks'),
      option('webviews', 'WebViews'),
      option('electron', 'Electron'),
      option('game-launchers', 'Game launchers'),
    ],
    [
      all([
        policyIsActive,
        anyCondition([answerIncludesAny('2.1', ['strict', 'custom']), answerEquals('4.1', 'block-launch')]),
      ]),
    ]
  ),
  question(
    '5.1',
    'What should browser rules target?',
    'multi',
    [
      option('exact-url', 'Exact URL'),
      option('domain', 'Domain'),
      option('category', 'Category'),
      option('search-terms', 'Search terms'),
      option('safe-search', 'Safe search'),
      option('video', 'Video'),
      option('downloads', 'Downloads'),
      option('browser-games', 'Browser games'),
      option('session', 'Session'),
      option('app-time', 'App time'),
      option('unknown-web', 'Unknown web'),
    ],
    [anyCondition([policyModeSelected, dryRunSelected])]
  ),
  question(
    '5.2',
    'What should happen to unknown pages?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('warn', 'Warn'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('classification-service', 'Classify'),
      option('deterministic-only', 'Lists only'),
    ],
    [all([policyIsActive, answerHasAnySelected('5.1')])]
  ),
  question(
    '5.3',
    'What should happen when exact evidence is unavailable?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('domain-fallback', 'Use domain'),
      option('category-fallback', 'Use category'),
      option('managed-proof', 'Need proof'),
      option('unsupported', 'Unsupported'),
    ],
    [all([policyIsActive, anyCondition([computedFlag('exactEvidenceSelected'), answerIncludes('1.2', 'block')])])]
  ),
  question(
    '6.1',
    'Which actions can a rule perform?',
    'multi',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe/log'),
      option('warn', 'Warn child'),
      option('ask-parent', 'Ask parent'),
      option('limit-time', 'Limit time'),
      option('block', 'Block'),
      option('redirect', 'Redirect'),
      option('close-browser', 'Close browser'),
      option('require-managed', 'Need managed'),
    ],
    [all([policyIsActive, answerHasAnySelected('5.1')])]
  ),
  question(
    '6.2',
    'Should rule actions be chosen per target type?',
    'single',
    [option('no', 'Same model'), option('yes', 'Per target')],
    [computedFlag('multiTargetActionMatrixRelevant')]
  ),
  question(
    '7.1',
    'Should search be controlled?',
    'single',
    [
      option('no', 'No'),
      option('observe', 'Observe'),
      option('safe-search', 'Safe search'),
      option('warn-terms', 'Warn terms'),
      option('ask-parent', 'Ask parent'),
      option('block-terms', 'Block terms'),
    ],
    [computedFlag('searchSelected')]
  ),
  question(
    '7.2',
    'What search evidence is allowed?',
    'single',
    [
      option('domain-only', 'Domain only'),
      option('provider-only', 'Provider only'),
      option('decision-only', 'Decision only'),
      option('local-journal', 'Local journal'),
      option('parent-report', 'Parent report'),
    ],
    [all([computedFlag('searchSelected'), not(answerEquals('7.1', 'no'))])]
  ),
  question(
    '8.1',
    'Should video and web media be controlled?',
    'single',
    [
      option('no', 'No'),
      option('observe', 'Observe'),
      option('limit', 'Limit'),
      option('warn', 'Warn'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
    ],
    [computedFlag('videoSelected')]
  ),
  question(
    '8.2',
    'What video targets are controlled?',
    'multi',
    [
      option('platform', 'Platform'),
      option('channel', 'Channel'),
      option('category', 'Category'),
      option('video-url', 'Video URL'),
      option('embedded', 'Embedded'),
      option('autoplay-shorts', 'Shorts/reels'),
      option('unknown-source', 'Unknown source'),
    ],
    [all([computedFlag('videoSelected'), not(answerEquals('8.1', 'no'))])]
  ),
  question(
    '9.1',
    'Should downloads be controlled?',
    'single',
    [
      option('ignore', 'Ignore'),
      option('observe', 'Observe'),
      option('notify-parent', 'Notify'),
      option('ask-parent', 'Ask parent'),
      option('block-risky', 'Block risky'),
      option('block-all-approved', 'Approve all'),
    ],
    [computedFlag('downloadsSelected')]
  ),
  question(
    '9.2',
    'What download evidence is allowed?',
    'multi',
    [
      option('filename', 'File name'),
      option('file-type', 'File type'),
      option('file-size', 'File size'),
      option('source-domain', 'Source domain'),
      option('download-url', 'Download URL'),
      option('danger-status', 'Danger status'),
      option('completion-status', 'Completion'),
      option('interruption', 'Interruption'),
      option('file-hash', 'File hash'),
    ],
    [all([computedFlag('downloadsSelected'), not(answerEquals('9.1', 'ignore'))])]
  ),
  question(
    '9.3',
    'What download actions are allowed?',
    'multi',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('warn', 'Warn'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('quarantine', 'Quarantine'),
      option('report-only', 'Report only'),
    ],
    [all([computedFlag('downloadsSelected'), not(answerEquals('9.1', 'ignore'))])]
  ),
  question(
    '10.1',
    'When should browser policy apply?',
    'multi',
    [
      option('always', 'Always'),
      option('school', 'School'),
      option('homework', 'Homework'),
      option('bedtime', 'Bedtime'),
      option('weekdays', 'Weekdays'),
      option('weekends', 'Weekends'),
      option('travel', 'Travel'),
      option('guest', 'Guest'),
      option('custom', 'Custom'),
    ],
    [all([policyIsActive, warnAskLimitBlock])],
    'schedule'
  ),
  question(
    '10.2',
    'Should browser policy support temporary overrides?',
    'multi',
    [
      option('pause-until', 'Pause until'),
      option('emergency-allow', 'Emergency allow'),
      option('emergency-block', 'Emergency block'),
      option('allow-once', 'Allow once'),
      option('allow-session', 'Session'),
      option('allow-schedule', 'Schedule end'),
      option('custom-window', 'Custom window'),
    ],
    [anyCondition([policyOn, policyPaused, emergencyOverride])],
    'schedule'
  ),
  question(
    '11.1',
    'Should browser activity have time limits?',
    'single',
    [option('no', 'No'), option('yes', 'Yes')],
    [
      anyCondition([
        computedFlag('limitExists'),
        answerIncludesAny('5.1', ['session', 'app-time']),
        answerEquals('8.1', 'limit'),
      ]),
    ],
    'schedule'
  ),
  question(
    '11.2',
    'What type of time limit?',
    'multi',
    [
      option('daily-browser', 'Daily browser'),
      option('session', 'Session'),
      option('site', 'Site quota'),
      option('domain', 'Domain quota'),
      option('category', 'Category quota'),
      option('video', 'Video quota'),
      option('schedule-quota', 'Schedule quota'),
      option('grace', 'Grace'),
      option('extension', 'Extension'),
    ],
    [answerEquals('11.1', 'yes')],
    'schedule'
  ),
  question(
    '12.1',
    'What requires parent approval?',
    'multi',
    [
      option('unknown-site', 'Unknown site'),
      option('blocked-site', 'Blocked site'),
      option('new-domain', 'New domain'),
      option('new-browser', 'New browser'),
      option('unmanaged-browser', 'Unmanaged'),
      option('download', 'Download'),
      option('time-extension', 'More time'),
      option('emergency', 'Emergency'),
      option('setup-repair', 'Setup repair'),
      option('policy-exception', 'Exception'),
    ],
    [computedFlag('askParentExists')],
    'approvals'
  ),
  question(
    '12.2',
    'What happens if parent does not answer?',
    'single',
    [
      option('deny', 'Deny'),
      option('allow-temporary', 'Allow temp'),
      option('observe', 'Observe'),
      option('warn-child', 'Warn child'),
      option('wait', 'Keep waiting'),
      option('default-policy', 'Default policy'),
    ],
    [answerHasAnySelected('12.1')],
    'approvals'
  ),
  question(
    '12.3',
    'How long does approval last?',
    'single',
    [
      option('once', 'Once'),
      option('session', 'Session'),
      option('today', 'Today'),
      option('schedule-end', 'Schedule end'),
      option('custom-time', 'Custom time'),
      option('always-domain', 'Always domain'),
      option('always-browser', 'Always browser'),
      option('always-rule', 'Always rule'),
    ],
    [answerHasAnySelected('12.1')],
    'approvals'
  ),
  question(
    '13.1',
    'What browser evidence may be collected?',
    'multi',
    [
      option('status-only', 'Status only'),
      option('process-window', 'Process/window'),
      option('managed-state', 'Managed state'),
      option('domain', 'Domain'),
      option('title', 'Page title'),
      option('exact-url', 'Exact URL'),
      option('search-term', 'Search term'),
      option('video-metadata', 'Video metadata'),
      option('download-metadata', 'Download data'),
      option('time-used', 'Time used'),
      option('decision-result', 'Decision'),
      option('proof-state', 'Proof state'),
    ],
    [computedFlag('evidencePrivacyVisible')]
  ),
  question(
    '13.2',
    'What evidence must never be collected?',
    'multi',
    [
      option('no-page-body', 'No page body'),
      option('no-screenshots', 'No screenshots'),
      option('no-raw-upload', 'No raw upload'),
      option('no-chat', 'No chat'),
      option('no-file-contents', 'No file content'),
      option('no-exact-url', 'No exact URL'),
      option('no-search-term', 'No search term'),
      option('local-summary', 'Local summary'),
      option('evidence-refs', 'Evidence refs'),
    ],
    [computedFlag('evidencePrivacyVisible')]
  ),
  question(
    '14.1',
    'What should parent see?',
    'multi',
    [
      option('policy-status', 'Policy status'),
      option('setup-health', 'Setup health'),
      option('managed-state', 'Managed state'),
      option('domains', 'Domains'),
      option('urls', 'URLs'),
      option('titles', 'Titles'),
      option('time-used', 'Time used'),
      option('blocked-events', 'Blocks'),
      option('approval-events', 'Approvals'),
      option('downloads', 'Downloads'),
      option('bypass', 'Bypass'),
      option('new-browser', 'New browser'),
      option('proof-warnings', 'Proof warnings'),
    ],
    [],
    'audit'
  ),
  question(
    '14.2',
    'What report detail level?',
    'single',
    [
      option('status-only', 'Status only'),
      option('summary', 'Summary'),
      option('detailed', 'Detailed'),
      option('exact-if-allowed', 'Exact if allowed'),
      option('custom', 'Custom'),
    ],
    [computedFlag('reportsEnabled')],
    'audit'
  ),
  question(
    '14.3',
    'When should parent be notified?',
    'multi',
    [
      option('new-browser', 'New browser'),
      option('unmanaged', 'Unmanaged'),
      option('unknown-browser', 'Unknown browser'),
      option('blocked-site', 'Blocked site'),
      option('blocked-download', 'Blocked download'),
      option('approval-request', 'Approval'),
      option('time-limit', 'Time limit'),
      option('bypass', 'Bypass'),
      option('managed-launch-fail', 'Launch fail'),
      option('setup-repair', 'Setup repair'),
      option('emergency', 'Emergency'),
    ],
    [computedFlag('notificationEventsRelevant')],
    'audit'
  ),
  question(
    '15.1',
    'What should happen if child device or local agent is offline?',
    'single',
    [
      option('allow-last-known', 'Last known'),
      option('observe', 'Observe'),
      option('require-online', 'Require online'),
      option('block-until-online', 'Block online'),
      option('ask-when-online', 'Ask later'),
    ],
    [all([policyIsActive, answerHasAnySelected('1.2')])]
  ),
  question(
    '15.2',
    'What should happen if browser capability is unsupported?',
    'single',
    [
      option('monitor-only', 'Monitor only'),
      option('unmanaged', 'Unmanaged'),
      option('blocked', 'Blocked'),
      option('ask-parent', 'Ask parent'),
      option('setup-repair', 'Setup repair'),
      option('unavailable', 'Unavailable'),
    ],
    [computedFlag('unsupportedCapabilityRelevant')]
  ),
  question(
    '15.3',
    'What should happen if exact proof is missing?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('degrade', 'Degrade'),
      option('require-managed', 'Need managed'),
    ],
    [computedFlag('exactEvidenceSelected')]
  ),
  question(
    '16.1',
    'Should setup and provisioning controls be shown?',
    'single',
    [
      option('hide-unless-broken', 'Hide unless broken'),
      option('basic-status', 'Basic status'),
      option('full-controls', 'Full controls'),
    ],
    [computedFlag('setupRelevant')]
  ),
  question(
    '16.2',
    'What setup tasks are allowed?',
    'multi',
    [
      option('install-managed', 'Install managed'),
      option('repair-managed', 'Repair managed'),
      option('install-extension', 'Install extension'),
      option('repair-extension', 'Repair extension'),
      option('apply-policy', 'Apply policy'),
      option('repair-policy', 'Repair policy'),
      option('profile-readiness', 'Profile check'),
      option('adapter-permissions', 'Adapter perms'),
      option('rescan', 'Re-scan'),
      option('rescan-install', 'Scan install'),
    ],
    [answerEquals('16.1', 'full-controls')]
  ),
  question(
    '17.1',
    'How long should browser-control data be kept?',
    'single',
    [
      option('no-store', 'Do not store'),
      option('short', 'Short'),
      option('standard', 'Standard'),
      option('long', 'Long'),
      option('custom', 'Custom'),
    ],
    [computedFlag('storedBrowserDataExists')],
    'audit'
  ),
  question(
    '17.2',
    'What data can parent export or delete?',
    'multi',
    [
      option('export-reports', 'Export reports'),
      option('export-audit', 'Export audit'),
      option('export-approvals', 'Export approvals'),
      option('export-evidence', 'Export evidence'),
      option('delete-reports', 'Delete reports'),
      option('delete-evidence', 'Delete evidence'),
      option('delete-approvals', 'Delete approvals'),
      option('delete-old', 'Delete old'),
    ],
    [all([answerHasAnySelected('17.1'), not(answerEquals('17.1', 'no-store'))])],
    'audit'
  ),
  question(
    '18.1',
    'How much audit history should be kept?',
    'single',
    [
      option('minimal', 'Minimal'),
      option('standard', 'Standard'),
      option('detailed', 'Detailed'),
      option('custom', 'Custom'),
    ],
    [],
    'audit'
  ),
  question(
    '18.2',
    'What must be audited?',
    'multi',
    [
      option('policy-changed', 'Policy changed'),
      option('parent-change', 'Parent change'),
      option('setup-change', 'Setup change'),
      option('approval-decision', 'Approval'),
      option('rule-matched', 'Rule matched'),
      option('decision-applied', 'Decision'),
      option('capability-failed', 'Capability failed'),
      option('emergency-used', 'Emergency'),
      option('data-export-delete', 'Data change'),
    ],
    [answerIncludesAny('18.1', ['standard', 'detailed', 'custom'])],
    'audit'
  ),
  question(
    '19.1',
    'How should educational browser games be handled?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('warn', 'Warn'),
      option('ask-parent', 'Ask parent'),
      option('limit', 'Limit'),
      option('block', 'Block'),
      option('manual-required', 'Manual required'),
    ],
    [computedFlag('browserGamesRelevant')]
  ),
  question(
    '19.2',
    'What should happen to unknown browser games?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('warn', 'Warn'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('classify', 'Classify first'),
      option('manual-required', 'Manual required'),
    ],
    [computedFlag('browserGamesRelevant')]
  ),
  question(
    '19.3',
    'How should cloud gaming be approved?',
    'single',
    [
      option('allow', 'Allow'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('manual-required', 'Manual required'),
    ],
    [computedFlag('browserGamesRelevant')]
  ),
  question(
    '19.4',
    'How should browser game purchases and accounts be handled?',
    'single',
    [
      option('allow', 'Allow'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('manual-required', 'Manual required'),
    ],
    [computedFlag('browserGamesRelevant')]
  ),
  question(
    '19.5',
    'What should happen to unblocked game portals?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('warn', 'Warn'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('manual-required', 'Manual required'),
    ],
    [computedFlag('browserGamesRelevant')]
  ),
  question(
    '19.6',
    'What should happen to WebGL or canvas games?',
    'single',
    [
      option('allow', 'Allow'),
      option('observe', 'Observe'),
      option('warn', 'Warn'),
      option('ask-parent', 'Ask parent'),
      option('block', 'Block'),
      option('manual-required', 'Manual required'),
    ],
    [computedFlag('browserGamesRelevant')]
  ),
  question(
    '19.7',
    'Should browser games have a time budget?',
    'single',
    [
      option('no', 'No'),
      option('daily', 'Daily'),
      option('session', 'Session'),
      option('site', 'Per site'),
      option('manual-required', 'Manual required'),
    ],
    [computedFlag('browserGamesRelevant')]
  ),
  question(
    'A1',
    'Can classification help browser decisions?',
    'single',
    [
      option('deterministic', 'Rules only'),
      option('local', 'Local classify'),
      option('portal-cloud', 'Portal classify'),
      option('ask-parent', 'Ask parent'),
    ],
    [computedFlag('classificationServiceReferenced')],
    'ai'
  ),
  question(
    'A2',
    'What AI/browser assistance is allowed?',
    'multi',
    [
      option('summarize', 'Summarize'),
      option('explain', 'Explain'),
      option('draft-note', 'Draft note'),
      option('classify-unknown', 'Classify unknown'),
      option('suggest-rule', 'Suggest rule'),
      option('suggest-alternative', 'Suggest safer'),
      option('local-summary', 'Local summary'),
      option('evidence-refs', 'Evidence refs'),
    ],
    [all([not(answerEquals('A1', 'deterministic')), answerHasAnySelected('A1')])],
    'ai'
  ),
  question(
    'A3',
    'What happens if classification is unavailable?',
    'single',
    [
      option('unknown', 'Treat unknown'),
      option('ask-parent', 'Ask parent'),
      option('deterministic', 'Use rules'),
      option('observe', 'Observe'),
      option('block', 'Block'),
      option('allow', 'Allow'),
    ],
    [answerIncludesAny('A1', ['local', 'portal-cloud'])],
    'ai'
  ),
] as const satisfies readonly BrowserPolicyQuestion[];
