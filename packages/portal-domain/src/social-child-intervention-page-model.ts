import {
  type SocialChildApprovalBlockSurface,
  SocialChildApprovalBlockUxSnapshotSchema,
  type SocialChildApprovalBlockUxSnapshot,
} from '@ocentra-parent/social-domain/social-child-approval-block-ux';

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

export function createSocialChildInterventionPageModels(
  snapshot: unknown,
  options: SocialChildInterventionPageModelOptions = {}
): SocialChildInterventionPageModelResult {
  const parsed = SocialChildApprovalBlockUxSnapshotSchema.safeParse(snapshot);
  if (!parsed.success) {
    return {
      models: [],
      reason: 'invalid-social-child-ux-snapshot',
      state: 'unavailable',
    };
  }

  return {
    models: surfacesInRenderOrder(parsed.data).map((surface) => surfaceToPageModel(surface, parsed.data, options)),
    state: 'renderable',
  };
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
  switch (surface.surfaceKind) {
    case 'approval-request-pending':
      return {
        action: 'approval-hold',
        deliveryState: 'approval-hold-rendered',
        outcome: 'approval-required',
        parentRequestEnabled: true,
        reason: 'Creating or switching this social account needs parent approval first.',
        ruleLabel: 'Parent approval required for this social account action',
      };
    case 'blocked-social-route-candidate':
      return {
        action: 'block',
        deliveryState: 'block-page-rendered',
        outcome: 'blocked',
        parentRequestEnabled: true,
        reason: 'This social route is blocked by a parent rule candidate.',
        ruleLabel: 'Blocked social route candidate',
      };
    case 'warning-social-route-candidate':
      return {
        action: 'warn',
        deliveryState: 'warn-page-rendered',
        outcome: 'warned',
        parentRequestEnabled: true,
        reason: 'This social route is in warning mode for this child profile.',
        ruleLabel: 'Social route warning',
      };
    case 'manual-review-required':
      return {
        action: 'parent-review',
        deliveryState: 'manual-required',
        outcome: 'manual-required',
        parentRequestEnabled: true,
        reason: 'A parent needs to review this social route before Ocentra can decide automatically.',
        ruleLabel: 'Manual parent review required',
      };
    case 'time-limit-candidate':
      return {
        action: 'time-limit',
        deliveryState: 'time-limit-candidate-rendered',
        outcome: 'time-limit-candidate',
        parentRequestEnabled: true,
        reason: 'This social route has a time-limit candidate, but no time limit has been applied.',
        ruleLabel: 'Social route time limit candidate',
      };
    case 'native-app-unavailable':
      return {
        action: 'parent-review',
        deliveryState: 'native-app-unavailable',
        outcome: 'manual-required',
        parentRequestEnabled: false,
        reason: 'Native social app control is unavailable without separate platform proof.',
        ruleLabel: 'Native social app proof unavailable',
      };
  }
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
