import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';
import { PARENT_PORTAL_NAV_LABELS } from './parent-portal-nav';

export const PARENT_PORTAL_PRIVACY_GUIDES: readonly ParentPortalGuideTopic[] = [
  {
    id: 'data-custody',
    navLabel: 'PRIVATE',
    rank: 10,
    title: 'Privacy And Data',
    subtitle: 'Local-first custody by default',
    detail: 'Who owns the data',
    tone: 'gold',
    category: 'Privacy',
    subcategory: 'Data custody',
    pages: [
      {
        eyebrow: 'LOCAL FIRST',
        title: 'Ocentra does not store child activity by default',
        body: 'Raw journals, SQLite evidence, browser evidence, app/game sessions, screen summaries, parent rules, approvals, and generated reports live on child or parent devices by default. Ocentra-hosted services handle public site, downloads, updates, accounts, billing, entitlement, relay metadata, and minimal notifications.',
        steps: [
          'Use local/LAN at home without sending child activity evidence to Ocentra.',
          'Show every data source as live local, LAN, parent cache, parent-owned storage, Ocentra non-activity metadata, or unavailable.',
          'Keep raw evidence, reports, and rules out of Ocentra-hosted storage unless a future explicit custody feature exists.',
          'Make support exports explicit so parents see included data classes before sharing.',
        ],
      },
      {
        eyebrow: 'WHAT CAN MOVE',
        title: 'Cross-boundary data movement must be visible',
        body: 'When data leaves the child device, the UI needs to say what moved, where it went, who owns the destination, retention behavior, deletion behavior, and what happens if that provider is offline.',
        steps: [
          'Notifications carry minimal detail and link back to the authenticated parent app.',
          'Parent-owned drives can store backups, reports, or sync bundles only after parent setup.',
          'Remote access can use local cache, reachable child agent, parent-owned storage, or stateless compile request.',
          'Provider failure must not stop local capture, local policy, or critical local safety behavior.',
        ],
      },
    ],
    tips: [
      {
        label: 'Simple promise',
        body: 'Child activity is local unless the parent chooses a destination.',
        tone: 'cyan',
      },
      {
        label: 'Be explicit',
        body: 'Every report should say where the data came from and where it is stored.',
        tone: 'gold',
      },
    ],
    actions: [
      {
        label: 'Review privacy',
        body: 'Open Private to inspect source and custody labels.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.PrivacyDesign),
        targetNavLabel: 'PRIVATE',
      },
      {
        label: 'Export carefully',
        body: 'Use Export/Delete to choose data class, destination, retention, delete, and audit behavior.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.ExportRetention),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Export,
      },
      {
        label: 'Audit movement',
        body: 'Review export, import, sync, retention, delete, and support-bundle history.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.AuditHistory),
        targetNavLabel: 'AUDIT',
      },
    ],
  },
  {
    id: 'drives-export',
    navLabel: 'DRIVES',
    rank: 11,
    title: 'Drives And Export',
    subtitle: 'Google Drive, OneDrive, iCloud, NAS',
    detail: 'Parent-owned storage',
    tone: 'cyan',
    category: 'Privacy',
    subcategory: 'Storage',
    pages: [
      {
        eyebrow: 'CONNECT YOUR DRIVES',
        title: 'Parent-owned storage is optional and explicit',
        body: 'A drive connection lets a parent use their own storage for backup, migration, remote reports, or cross-device continuity. Ocentra may provide connector status and schemas, but it should not silently become the family-data warehouse.',
        steps: [
          'Choose provider: Google Drive, OneDrive, iCloud Drive, Dropbox, NAS, or local folder.',
          'Choose data classes: encrypted evidence export, rules, approvals, report summaries, or support message record.',
          'Show connected account, folder, last sync, failure state, and revocation state.',
          'Keep local evidence intact when export or sync fails.',
        ],
      },
      {
        eyebrow: 'EXPORT TYPES',
        title: 'Different exports need different expectations',
        body: 'Encrypted machine-readable backups, intentionally human-readable reports, and support message records are different. The UI should make those choices obvious before the parent starts an export or sync.',
        steps: [
          'Encrypted backup: for restore or migration.',
          'Human-readable report: for parent review and conversation.',
          'Support message record: for parent-authored troubleshooting requests, with sensitive classes clearly listed.',
          'Delete and retention controls should match the chosen destination and data class.',
        ],
      },
    ],
    tips: [
      {
        label: 'Remote reports',
        body: 'Use a parent-owned drive or parent cache when you want reports away from home.',
        tone: 'cyan',
      },
      {
        label: 'Failure',
        body: 'A drive outage should show connector status but never corrupt local evidence.',
        tone: 'gold',
      },
    ],
    actions: [
      {
        label: 'Connect drive',
        body: 'Open Drives to choose provider, folder, scope, and retention.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.DriveConnections),
        targetNavLabel: 'DRIVES',
      },
      {
        label: 'Delete or revoke',
        body: 'Use revocation and delete controls when changing providers or family devices.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.ExportRetention),
        targetNavLabel: PARENT_PORTAL_NAV_LABELS.Export,
      },
    ],
  },
  {
    id: 'remote-access',
    navLabel: 'PRIVATE',
    rank: 12,
    title: 'Remote Access',
    subtitle: 'Away from home without Ocentra custody',
    detail: 'Local cache and relay',
    tone: 'purple',
    category: 'Privacy',
    subcategory: 'Remote',
    pages: [
      {
        eyebrow: 'AWAY FROM HOME',
        title: 'Remote views must state their source',
        body: 'Remote access should help a parent see device health, alerts, reports, or scoped controls without turning Ocentra into the default child-data store. The app should label live child agent, authenticated relay, parent cache, parent-owned storage, stateless compile, stale, and unavailable states.',
        steps: [
          'Live local/LAN is strongest when the parent is at home.',
          'Remote relay sends typed intents to a reachable child agent; the child agent still validates and executes.',
          'Parent-owned storage can provide reports and sync bundles while away.',
          'Cloud unavailable should leave local child-device safety behavior running.',
        ],
      },
      {
        eyebrow: 'CLOUD BOUNDARY',
        title: 'Cloud is control plane, not activity warehouse',
        body: 'Ocentra-hosted cloud may handle account, subscription, entitlement, device route metadata, relay delivery state, connector status, and stateless report compilation. It should not store child evidence or generated reports by default.',
        steps: [
          'Use authenticated parent identity and scoped device routes.',
          'Reject anonymous, wrong-family, wrong-device, stale, or malformed commands.',
          'Retain only minimal operational metadata for notification and relay delivery.',
          'Show queued, stale, rejected, accepted, superseded, and unavailable states.',
        ],
      },
    ],
    tips: [
      {
        label: 'Parent clarity',
        body: 'Remote does not mean Ocentra owns child data.',
        tone: 'cyan',
      },
      {
        label: 'Security',
        body: 'Remote commands must stay typed, scoped, authenticated, and auditable.',
        tone: 'purple',
      },
    ],
    actions: [
      {
        label: 'Check source',
        body: 'Look for live, cache, drive, relay, or unavailable labels before trusting a remote view.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.RemoteAccess),
        targetNavLabel: 'REMOTE',
      },
      {
        label: 'Relay capability',
        body: 'Use Capability to see whether the selected device can support LAN, relay, or local-only state.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.CapabilityStatus),
        targetNavLabel: 'CAPABILITY',
      },
      {
        label: 'Set storage',
        body: 'Use Drives for away-from-home reports that do not require Ocentra custody.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.DriveConnections),
        targetNavLabel: 'DRIVES',
      },
    ],
  },
] as const;
