import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_NAV_LABELS } from './parent-portal-nav';

export const PARENT_PORTAL_OPERATION_GUIDES: readonly ParentPortalGuideTopic[] = [
  {
    id: 'alerts-notifications',
    navLabel: 'ALERTS',
    rank: 13,
    title: 'Alerts And Notifications',
    subtitle: 'Portal, push, email, WhatsApp candidates',
    detail: 'Minimal detail',
    tone: 'red',
    category: 'Manage',
    subcategory: 'Alerts',
    pages: [
      {
        eyebrow: 'ALERT RULES',
        title: 'Notifications should reduce anxiety',
        body: 'Alerts should come from typed alert rules, policy decisions, evidence refs, health state, sync status, or provider status. Raw activity noise should not become a notification until it matches an explicit reason and parent preference.',
        steps: [
          'Choose alert types: policy violation, ask-parent, suspicious unknown, device offline, sync failure, or provider failure.',
          'Choose channels: in-app, push, email, SMS, WhatsApp, or later provider adapters.',
          'Set quiet hours, frequency, deduplication, escalation, and retry behavior.',
          'Keep provider failures visible without disabling local child-device safety decisions.',
        ],
      },
      {
        eyebrow: 'PRIVATE ALERTS',
        title: 'Provider messages need minimal content',
        body: 'A notification may cross a third-party provider boundary. It should carry enough to tell a parent to open the app, but avoid sensitive URLs, titles, screenshots, message text, filenames, or generated reports by default.',
        steps: [
          'Use reason code, severity, child/device scope, and action link.',
          'Put sensitive detail behind authenticated parent app drill-in.',
          'Audit delivery attempt, delivery result, retry state, and parent action.',
          'Let parents tune noise without deleting the underlying audit event.',
        ],
      },
    ],
    tips: [
      {
        label: 'Best default',
        body: 'Start with in-app alerts and quiet hours before adding external channels.',
        tone: 'cyan',
      },
      {
        label: 'Privacy',
        body: 'Do not put sensitive child detail into push, email, SMS, or WhatsApp preview text by default.',
        tone: 'red',
      },
    ],
    actions: [
      {
        label: 'Open Alerts',
        body: 'Set channel, quiet hours, escalation, and sensitive-detail behavior.',
        tone: 'red',
        targetRoutePath: portalRouteHashPath(PortalRoute.Notifications),
        targetNavLabel: 'ALERTS',
      },
      {
        label: 'Set channels',
        body: 'Configure in-app, push, email, SMS, WhatsApp, retry, quiet hours, and privacy text.',
        tone: 'red',
        targetRoutePath: portalRouteHashPath(PortalRoute.NotificationChannels),
        targetNavLabel: 'CHANNELS',
      },
      {
        label: 'Review history',
        body: 'Use alert history to see delivery state and parent actions.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.AuditHistory),
        targetNavLabel: 'AUDIT',
      },
    ],
  },
  {
    id: 'subscription-plans',
    navLabel: PARENT_PORTAL_NAV_LABELS.Plan,
    rank: 14,
    title: 'Subscription',
    subtitle: 'Plans, trials, device limits, entitlements',
    detail: 'Billing boundary',
    tone: 'gold',
    category: 'Manage',
    subcategory: 'Subscription',
    pages: [
      {
        eyebrow: 'PLANS',
        title: 'Subscription gates paid value, not child evidence truth',
        body: 'Billing can gate paid product value such as device limits, cloud relay, remote access, advanced reports, generated explanations, long-window summaries, and exports. It must not silently break local evidence capture, journal integrity, audit history, or critical local safety behavior.',
        steps: [
          'Show plan, trial, renewal, cancellation, grace, unknown, and unavailable states.',
          'Show device limits and which child devices count against them.',
          'Explain which features are included, locked, trial-only, or in grace.',
          'Keep Stripe/provider details behind the backend boundary, not inside child safety modules.',
        ],
      },
      {
        eyebrow: 'FAILURE BEHAVIOR',
        title: 'Parents need to know what happens if billing cannot check',
        body: 'Cloud or billing outages should not look like the child is unprotected. The UI needs to explain local-only, grace, restricted, payment-required, expired, unavailable, and stale-snapshot states.',
        steps: [
          'Use signed or schema-valid entitlement snapshots where child devices need entitlement state.',
          'Deny new paid-device activation through typed decisions when limits are exceeded.',
          'Keep local evidence export and safety-critical audit visibility available.',
          'Audit every billing status sync, device-limit decision, and validation failure.',
        ],
      },
    ],
    tips: [
      {
        label: 'Parent wording',
        body: 'Subscription controls product access. It is not the child-device safety engine.',
        tone: 'gold',
      },
      {
        label: 'Offline',
        body: 'Local safety should degrade deliberately and visibly, not silently stop.',
        tone: 'cyan',
      },
    ],
    actions: [
      {
        label: 'Change plan',
        body: 'Open Subscription to review plan, trial, renewal, device limit, and grace state.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.Subscription),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Plan,
      },
      {
        label: 'Resolve billing',
        body: 'Use account billing controls without exposing child activity to the billing provider.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.Entitlements),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Access,
      },
    ],
  },
  {
    id: 'platforms-install',
    navLabel: 'SUPPORT',
    rank: 15,
    title: 'Platforms And Install',
    subtitle: 'Desktop, mobile, web, honest limits',
    detail: 'What ships where',
    tone: 'cyan',
    category: 'Support',
    subcategory: 'Platforms',
    pages: [
      {
        eyebrow: 'PRODUCT SURFACES',
        title: 'The final parent product is packaged desktop and mobile',
        body: 'The Vite web portal is a development scaffold for exercising the real Rust service path. Production parent control should be a local packaged parent app or mobile app. Tauri is the preferred desktop-shell candidate unless an architecture decision replaces it.',
        steps: [
          'Windows is first for the production-grade child-device agent.',
          'Parent desktop app can connect over loopback, LAN, relay, parent cache, or parent-owned storage.',
          'Parent mobile app is separate from child mobile agent support.',
          'Web is public/download/account/subscription/docs and typed parent surface, not a child agent.',
        ],
      },
      {
        eyebrow: 'PLATFORM LIMITS',
        title: 'Do not claim control the OS does not allow',
        body: 'Windows, macOS, Linux, Android, and iOS have different permission, service, capture, network, notification, and enforcement capabilities. Scaffolded packaging is not the same as product support.',
        steps: [
          'Windows proves service, MSI, capture, network, policy, and enforcement first.',
          'macOS and Linux need separate API, permission, service-manager, and package proof.',
          'Android support must distinguish foreground service, accessibility, VPN, DNS, device-owner, and managed-profile paths.',
          'iOS support must respect approved APIs and entitlements such as Family Controls and Screen Time.',
        ],
      },
      {
        eyebrow: 'INSTALL AND UPDATE',
        title: 'Install paths are parent-facing product features',
        body: 'Parents need understandable install, update, rollback, uninstall, and support paths. Release claims must match real signing, package, installer, store, and entitlement state.',
        steps: [
          'Use Windows MSI and updater scaffold for Windows agent path.',
          'Show install health, service status, update status, and rollback state where available.',
          'Separate CI package preview from production release readiness.',
          'Keep product claims honest when signing, store, or mobile entitlement proof is still pending.',
        ],
      },
    ],
    tips: [
      {
        label: 'Desktop first',
        body: 'Use the web portal for fast development. Ship parent control as desktop/mobile app surfaces.',
        tone: 'cyan',
      },
      {
        label: 'Honest support',
        body: 'Scaffolded platform does not mean capture or enforcement is supported there.',
        tone: 'gold',
      },
    ],
    actions: [
      {
        label: 'Open Support',
        body: 'Review platform status, install health, update state, and support diagnostics.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.PlatformsInstall),
        targetNavLabel: 'PLATFORMS',
      },
      {
        label: 'Update path',
        body: 'Check install status, updater state, rollback status, and production proof gaps.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.InstallUpdates),
        targetNavLabel: 'UPDATES',
      },
      {
        label: 'Download app',
        body: 'Use family.ocentra.ca for public download, account, subscription, and docs surfaces later.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.PlatformsInstall),
        targetNavLabel: 'PLATFORMS',
      },
    ],
  },
  {
    id: 'support-contact',
    navLabel: 'SUPPORT',
    rank: 16,
    title: 'Support Contact',
    subtitle: 'Send parent messages without attachments',
    detail: 'Message-only help',
    tone: 'purple',
    category: 'Support',
    subcategory: 'Contact',
    pages: [
      {
        eyebrow: 'SUPPORT MESSAGE',
        title: 'Support starts as a parent-authored message',
        body: 'The Support route is a contact form. It collects category, reply email, subject, and parent message text without attaching child evidence, screenshots, browser URLs, service logs, or local files.',
        steps: [
          'Keep the form message-only until a parent explicitly chooses a future attachment workflow.',
          'Use parent language for category, subject, reply email, and message body.',
          'Do not silently include child-device evidence, screenshots, browser URLs, logs, or local paths.',
          'Route send intent through a future typed connector instead of treating Vite as the backend.',
        ],
      },
      {
        eyebrow: 'REPLY PATH',
        title: 'Replies belong to verified parent contact channels',
        body: 'Support delivery should eventually use the parent account email or a verified contact channel. Until that backend is wired, the UI should stay honest that the message connector is pending.',
        steps: [
          'Show whether the reply email is verified before a real send is enabled.',
          'Keep failed, pending, sent, and draft states typed and parent-readable.',
          'Do not mix account support with device data export or developer diagnostics.',
          'Escalate platform, billing, or device capability questions to the right product page.',
        ],
      },
    ],
    tips: [
      {
        label: 'Message only',
        body: 'Support contact does not include attachments from the device or portal.',
        tone: 'gold',
      },
      {
        label: 'Parent language',
        body: 'The form should help parents describe the issue without exposing child data.',
        tone: 'purple',
      },
    ],
    actions: [
      {
        label: 'Open support',
        body: 'Use Support to draft or send a parent-authored message.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.Diagnostics),
        targetNavLabel: 'SUPPORT',
      },
      {
        label: 'Capabilities',
        body: 'Open Capability to distinguish supported, degraded, unavailable, planned, and manual-only states.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.CapabilityStatus),
        targetNavLabel: 'CAPABILITY',
      },
      {
        label: 'Review account',
        body: 'Use Account for plan, access, and support contact state.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.Diagnostics),
        targetNavLabel: 'SUPPORT',
      },
    ],
  },
] as const;
