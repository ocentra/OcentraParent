import { PortalRoute } from './portal-contract-adapter';
import { portalRouteHashPath } from './routes';
import type { ParentPortalGuideTopic } from './parent-portal-guide-types';

export const PARENT_PORTAL_API_GUIDES: readonly ParentPortalGuideTopic[] = [
  {
    id: 'api-providers',
    navLabel: 'AI',
    rank: 10,
    title: 'AI Providers',
    subtitle: 'Local models first, API providers only by parent choice',
    detail: 'Models and custody',
    tone: 'purple',
    category: 'AI',
    subcategory: 'Providers',
    pages: [
      {
        eyebrow: 'AI SETUP',
        title: 'Choose the AI path per family and per device',
        body: 'Ocentra should treat AI as a helper layer, not as hidden household authority. The normal safety path is local and evidence-cited. External API AI can be useful for richer explanations and report writing only after a parent chooses provider, data scope, and device policy.',
        steps: [
          'Pick a default family AI profile: local only, local with API explainers, API report writing only, or disabled.',
          'Override per child device when one device has a weaker local model, different school needs, or stricter privacy requirements.',
          'Show provider status as supported, disabled, missing key, rate limited, degraded, unavailable, or blocked by custody policy.',
          'Never let API text directly trigger blocking, timeout, or enforcement. Parent rules and typed policy decisions still decide actions.',
        ],
      },
      {
        eyebrow: 'SUPPORTED STATES',
        title: 'Supported means a specific model path is configured',
        body: 'The UI needs to separate local model support from API provider support. A child device can have a local evaluator, a parent-device model, a household local hub, or an external provider. Each path has different latency, privacy, and failure behavior.',
        steps: [
          'Local evaluator: runs on the child device where available and returns typed evidence-cited results.',
          'Parent-device local model: runs on the parent device for reports or explanations when it has enough local data.',
          'Local AI hub: a stronger home machine accepts queued typed requests from paired devices.',
          'API provider: an external AI service used only for parent-authorized summaries, explanations, or reports.',
        ],
      },
      {
        eyebrow: 'LOCAL AI HUB',
        title: 'A strong local hub can serve weaker devices',
        body: 'A household may have one stronger local machine that handles heavier model work while laptops or smaller devices keep capturing evidence locally. The hub should accept only paired, typed, auditable requests and should not become an unauthenticated LAN service.',
        steps: [
          'Pair the child device and local hub before any request is accepted.',
          'Queue typed requests when a device model is busy, unavailable, or too weak for a report task.',
          'Return evidence refs, model id, confidence, and degraded state instead of raw private blobs.',
          'Fail closed to local-only, unavailable, or parent-review states when the hub is offline.',
        ],
      },
      {
        eyebrow: 'API PROVIDERS',
        title: 'External AI needs explicit provider and data controls',
        body: 'API AI is useful for answering parent questions, drafting report language, and explaining patterns. It should never be required for local child safety, and the parent must be able to see what can be sent before enabling a provider.',
        steps: [
          'Choose provider and model profile, then test connectivity from the parent app.',
          'Choose allowed data classes: summaries only, selected evidence refs, reports, or no child activity.',
          'Store provider keys in a parent-owned secret boundary, not in child-facing UI text or exported bundles.',
          'Show no-retention, provider retention, region, failure, and billing implications before enabling.',
        ],
      },
    ],
    tips: [
      {
        label: 'Plain rule',
        body: 'Local AI can help explain. API AI is optional and parent-authorized.',
        tone: 'cyan',
        targetPage: 0,
      },
      {
        label: 'Supported now',
        body: 'Current UI should show provider/status concepts while runtime provider setup remains an explicit integration gap.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.Diagnostics),
        targetNavLabel: 'SUPPORT',
      },
      {
        label: 'Privacy',
        body: 'External AI must not receive raw screenshots, secrets, or unbounded child activity by default.',
        tone: 'red',
        targetTopicId: 'data-custody',
        targetNavLabel: 'PRIVATE',
      },
    ],
    actions: [
      {
        label: 'Open AI setup',
        body: 'Manage model profile, API provider state, local hub status, and per-device AI policy.',
        tone: 'purple',
        targetRoutePath: portalRouteHashPath(PortalRoute.AiRuntime),
        targetNavLabel: 'AI SETUP',
      },
      {
        label: 'Set API keys',
        body: 'Choose provider, model, data scope, and retention before any external AI call is allowed.',
        tone: 'gold',
        targetRoutePath: portalRouteHashPath(PortalRoute.ApiProviders),
        targetNavLabel: 'API KEYS',
      },
      {
        label: 'Pick per-device model',
        body: 'Choose local-only, local hub queue, API report writing, or disabled for each child device.',
        tone: 'cyan',
        targetRoutePath: portalRouteHashPath(PortalRoute.CapabilityStatus),
        targetNavLabel: 'CAPABILITY',
      },
      {
        label: 'Review custody',
        body: 'Check what can leave a device before allowing an external provider.',
        tone: 'gold',
        targetTopicId: 'data-custody',
        targetNavLabel: 'PRIVATE',
      },
    ],
  },
];
