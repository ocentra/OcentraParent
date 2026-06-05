import { PortalDom, PortalTheme, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';

export function resolveTheme(): PortalThemeValue {
  const savedTheme = window.localStorage.getItem(PortalTheme.LocalStorageKey);
  if (savedTheme === PortalTheme.Dark || savedTheme === PortalTheme.Light) {
    return savedTheme;
  }
  return PortalTheme.Dark;
}

export function saveTheme(theme: PortalThemeValue): void {
  window.localStorage.setItem(PortalTheme.LocalStorageKey, theme);
}

export function applyTheme(theme: PortalThemeValue): void {
  document.documentElement.setAttribute(PortalDom.Attributes.DataTheme, theme);
}

export function selectTheme(theme: PortalThemeValue): void {
  saveTheme(theme);
  applyTheme(theme);
}

export function themeToggleButtonClassName(active: boolean) {
  if (!active) {
    return PortalDom.Classes.ThemeToggleButton;
  }
  return [PortalDom.Classes.ThemeToggleButton, PortalDom.Classes.ThemeToggleButtonActive].join(
    PortalDom.Classes.ClassNameSeparator
  );
}
