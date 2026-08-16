import {
  BrowserChildInterventionPageDefaults as BrowserChildInterventionPageDefaultsImpl,
  BrowserChildInterventionPageSamples as BrowserChildInterventionPageSamplesImpl,
  renderBrowserChildInterventionPage as renderBrowserChildInterventionPageImpl,
} from './browser-child-intervention-page-impl';
import type {
  BrowserChildInterventionPageBackdrop as BrowserChildInterventionPageBackdropImpl,
  BrowserChildInterventionPageModel as BrowserChildInterventionPageModelImpl,
} from './browser-child-intervention-page-impl';

export type BrowserChildInterventionPageTheme = 'auto' | 'dark' | 'light';

export type BrowserChildInterventionPageAction =
  | 'approval-hold'
  | 'block'
  | 'checking-hold'
  | 'parent-review'
  | 'time-limit'
  | 'unknown'
  | 'warn';

export type BrowserChildInterventionPageBackdrop = BrowserChildInterventionPageBackdropImpl;
export type BrowserChildInterventionPageModel = BrowserChildInterventionPageModelImpl;

export const BrowserChildInterventionPageDefaults = BrowserChildInterventionPageDefaultsImpl;
export const BrowserChildInterventionPageSamples = BrowserChildInterventionPageSamplesImpl;
export const renderBrowserChildInterventionPage = renderBrowserChildInterventionPageImpl;
