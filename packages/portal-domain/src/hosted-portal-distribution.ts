import {
  generatedResolveHostedPortalDistributionState,
  type GeneratedHostedPortalDistributionState,
  type GeneratedHostedPortalEnv,
  type GeneratedHostedPortalLocation,
} from './hosted-portal-distribution.generated';

export type HostedPortalDistributionState = GeneratedHostedPortalDistributionState;
export type HostedPortalEnv = GeneratedHostedPortalEnv;
export type HostedPortalLocation = GeneratedHostedPortalLocation;

export function resolveHostedPortalDistributionState(
  location: HostedPortalLocation,
  env: HostedPortalEnv,
  defaultNowMinutes?: number
): HostedPortalDistributionState | null {
  return generatedResolveHostedPortalDistributionState(location, env, defaultNowMinutes);
}
