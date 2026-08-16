import {
  BrowserChildInterventionPageDefaults,
  type BrowserChildInterventionPageAction,
  type BrowserChildInterventionPageBackdrop,
  type BrowserChildInterventionPageModel,
} from './browser-child-intervention-page';

export type SocialChildInterventionPageModelOptions = {
  readonly backdrop?: BrowserChildInterventionPageBackdrop | undefined;
  readonly bridge?: string | undefined;
  readonly requestedUrlForSurface?: SocialChildInterventionRequestedUrlResolver | undefined;
};

export type SocialChildApprovalBlockSurfaceKind =
  | 'approval-request-pending'
  | 'blocked-social-route-candidate'
  | 'warning-social-route-candidate'
  | 'manual-review-required'
  | 'time-limit-candidate'
  | 'native-app-unavailable';

export type SocialChildApprovalBlockSurface = Readonly<{
  surfaceKind: SocialChildApprovalBlockSurfaceKind;
  renderedChildUiClaimed: boolean;
  notificationDeliveredClaimed: boolean;
  browserNavigationBlockedClaimed: boolean;
  blockPageRenderedClaimed: boolean;
  timeLimitAppliedClaimed: boolean;
  finalPolicyDecisionClaimed: boolean;
  connectorAuthorizationClaimed: boolean;
  nativeAppControlClaimed: boolean;
  enforcementClaimed: boolean;
}>;

export type SocialChildApprovalBlockUxSnapshot = Readonly<{
  childProfileId: string;
  surfaces: readonly SocialChildApprovalBlockSurface[];
}>;

export type SocialChildInterventionRequestedUrlResolver = (surface: SocialChildApprovalBlockSurface) => string;

export type SocialChildInterventionPageModelResult =
  | {
      readonly models: readonly BrowserChildInterventionPageModel[];
      readonly state: 'renderable';
    }
  | {
      readonly models: readonly [];
      readonly reason: 'invalid-social-child-ux-snapshot';
      readonly state: 'unavailable';
    };

const SocialChildInterventionBridge = 'social-child-approval-block-renderer';
const SocialChildInterventionTargetType = 'social-route';
const SocialChildApprovalBlockSurfaceKinds = [
  'approval-request-pending',
  'blocked-social-route-candidate',
  'warning-social-route-candidate',
  'manual-review-required',
  'time-limit-candidate',
  'native-app-unavailable',
] as const satisfies readonly SocialChildApprovalBlockSurfaceKind[];

const SocialChildInterventionPageContentBySurfaceKind: Readonly<
  Record<
    SocialChildApprovalBlockSurfaceKind,
    {
      readonly action: BrowserChildInterventionPageAction;
      readonly deliveryState: string;
      readonly outcome: string;
      readonly parentRequestEnabled: boolean;
      readonly reason: string;
      readonly ruleLabel: string;
    }
  >
> = {
  'approval-request-pending': {
    action: 'approval-hold',
    deliveryState: 'approval-hold-rendered',
    outcome: 'approval-required',
    parentRequestEnabled: true,
    reason: 'Creating or switching this social account needs parent approval first.',
    ruleLabel: 'Parent approval required for this social account action',
  },
  'blocked-social-route-candidate': {
    action: 'block',
    deliveryState: 'block-page-rendered',
    outcome: 'blocked',
    parentRequestEnabled: true,
    reason: 'This social route is blocked by a parent rule candidate.',
    ruleLabel: 'Blocked social route candidate',
  },
  'warning-social-route-candidate': {
    action: 'warn',
    deliveryState: 'warn-page-rendered',
    outcome: 'warned',
    parentRequestEnabled: true,
    reason: 'This social route is in warning mode for this child profile.',
    ruleLabel: 'Social route warning',
  },
  'manual-review-required': {
    action: 'parent-review',
    deliveryState: 'manual-required',
    outcome: 'manual-required',
    parentRequestEnabled: true,
    reason: 'A parent needs to review this social route before Ocentra can decide automatically.',
    ruleLabel: 'Manual parent review required',
  },
  'time-limit-candidate': {
    action: 'time-limit',
    deliveryState: 'time-limit-candidate-rendered',
    outcome: 'time-limit-candidate',
    parentRequestEnabled: true,
    reason: 'This social route has a time-limit candidate, but no time limit has been applied.',
    ruleLabel: 'Social route time limit candidate',
  },
  'native-app-unavailable': {
    action: 'parent-review',
    deliveryState: 'native-app-unavailable',
    outcome: 'manual-required',
    parentRequestEnabled: false,
    reason: 'Native social app control is unavailable without separate platform proof.',
    ruleLabel: 'Native social app proof unavailable',
  },
};

export function createSocialChildInterventionPageModels(
  snapshot: unknown,
  options: SocialChildInterventionPageModelOptions = {}
): SocialChildInterventionPageModelResult {
  const parsed = parseSocialChildApprovalBlockUxSnapshot(snapshot);
  if (parsed === null) {
    return {
      models: [],
      reason: 'invalid-social-child-ux-snapshot',
      state: 'unavailable',
    };
  }

  return {
    models: surfacesInRenderOrder(parsed).map((surface) => surfaceToPageModel(surface, parsed, options)),
    state: 'renderable',
  };
}

function parseSocialChildApprovalBlockUxSnapshot(value: unknown): SocialChildApprovalBlockUxSnapshot | null {
  if (!isRecord(value) || !isString(value['childProfileId']) || !Array.isArray(value['surfaces'])) {
    return null;
  }
  const surfaces = value['surfaces'].map(parseSocialChildApprovalBlockSurface);
  if (surfaces.includes(null)) {
    return null;
  }
  return {
    childProfileId: value['childProfileId'],
    surfaces: surfaces as SocialChildApprovalBlockSurface[],
  };
}

function parseSocialChildApprovalBlockSurface(value: unknown): SocialChildApprovalBlockSurface | null {
  if (!isRecord(value) || !isSocialChildApprovalBlockSurfaceKind(value['surfaceKind'])) {
    return null;
  }
  const surface = {
    surfaceKind: value['surfaceKind'],
    renderedChildUiClaimed: value['renderedChildUiClaimed'],
    notificationDeliveredClaimed: value['notificationDeliveredClaimed'],
    browserNavigationBlockedClaimed: value['browserNavigationBlockedClaimed'],
    blockPageRenderedClaimed: value['blockPageRenderedClaimed'],
    timeLimitAppliedClaimed: value['timeLimitAppliedClaimed'],
    finalPolicyDecisionClaimed: value['finalPolicyDecisionClaimed'],
    connectorAuthorizationClaimed: value['connectorAuthorizationClaimed'],
    nativeAppControlClaimed: value['nativeAppControlClaimed'],
    enforcementClaimed: value['enforcementClaimed'],
  };
  return Object.values(surface)
    .slice(1)
    .every((claim) => claim === false)
    ? (surface as SocialChildApprovalBlockSurface)
    : null;
}

function surfacesInRenderOrder(
  snapshot: SocialChildApprovalBlockUxSnapshot
): readonly SocialChildApprovalBlockSurface[] {
  return [...snapshot.surfaces].sort((left, right) => surfaceSortOrder(left) - surfaceSortOrder(right));
}

function surfaceToPageModel(
  surface: SocialChildApprovalBlockSurface,
  snapshot: SocialChildApprovalBlockUxSnapshot,
  options: SocialChildInterventionPageModelOptions
): BrowserChildInterventionPageModel {
  const content = pageContentForSurface(surface);
  return {
    action: content.action,
    backdrop: options.backdrop,
    blockMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    bridge: options.bridge ?? SocialChildInterventionBridge,
    childName: snapshot.childProfileId,
    deliveryState: content.deliveryState,
    outcome: content.outcome,
    parentRequestEnabled: content.parentRequestEnabled,
    reason: content.reason,
    requestedUrl: options.requestedUrlForSurface?.(surface) ?? requestedUrlForSurface(surface),
    ruleId: ruleIdForSurface(surface),
    ruleLabel: content.ruleLabel,
    ruleMarker: BrowserChildInterventionPageDefaults.BlockMarker,
    targetType: SocialChildInterventionTargetType,
  };
}

function pageContentForSurface(surface: SocialChildApprovalBlockSurface): {
  readonly action: BrowserChildInterventionPageAction;
  readonly deliveryState: string;
  readonly outcome: string;
  readonly parentRequestEnabled: boolean;
  readonly reason: string;
  readonly ruleLabel: string;
} {
  return SocialChildInterventionPageContentBySurfaceKind[surface.surfaceKind];
}

function requestedUrlForSurface(surface: SocialChildApprovalBlockSurface): string {
  return `https://social.example.invalid/${surface.surfaceKind}`;
}

function ruleIdForSurface(surface: SocialChildApprovalBlockSurface): string {
  return `social-child-${surface.surfaceKind}`;
}

function surfaceSortOrder(surface: SocialChildApprovalBlockSurface): number {
  return {
    'approval-request-pending': 0,
    'blocked-social-route-candidate': 1,
    'warning-social-route-candidate': 2,
    'manual-review-required': 3,
    'time-limit-candidate': 4,
    'native-app-unavailable': 5,
  }[surface.surfaceKind];
}

function isSocialChildApprovalBlockSurfaceKind(value: unknown): value is SocialChildApprovalBlockSurfaceKind {
  return (
    typeof value === 'string' &&
    SocialChildApprovalBlockSurfaceKinds.includes(value as SocialChildApprovalBlockSurfaceKind)
  );
}

function isRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function isString(value: unknown): value is string {
  return typeof value === 'string';
}
