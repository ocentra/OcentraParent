import React, { useEffect, useMemo, useRef, useState, type CSSProperties } from 'react';
import { authFacebookImageUrl, authGoogleImageUrl, authGuestImageUrl } from '@ocentra-parent/portal-assets/auth';
import { avatarImageById } from '@ocentra-parent/portal-assets/avatars';
import { CyberAuthSurface, normalizeAuthPageSvgControls, type AuthPageSvgControls } from './CyberAuthSurface';
import './LoginDialog.css';

const AuthImages = {
  Social: { facebook: authFacebookImageUrl, google: authGoogleImageUrl, guest: authGuestImageUrl },
} as const;

const AUTH_MODE_STORAGE_KEY = 'ocentra.auth.mode';
const AUTH_DIALOG_FOCUSABLE_SELECTOR =
  'button:not([disabled]), input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled]), a[href], [tabindex]:not([tabindex="-1"])';

const AvatarImages = avatarImageById;

export interface LoginDialogActionResult {
  success: boolean;
  error?: string;
}

export interface LoginDialogSignUpPayload {
  alias: string;
  avatar: string;
  username: string;
  password: string;
}

export interface LoginDialogSecondaryAction {
  label: string;
  onClick: () => void | Promise<void>;
  disabled?: boolean;
}

export interface LoginDialogStatusMessage {
  kind: 'error' | 'success' | 'info';
  text: string;
}

export type LoginDialogContextTone = 'default' | 'warning';

export interface LoginDialogProps {
  onLogin: (username: string, password: string) => Promise<LoginDialogActionResult>;
  onSignUp?: (userData: LoginDialogSignUpPayload) => Promise<LoginDialogActionResult>;
  onFacebookLogin?: () => Promise<LoginDialogActionResult>;
  onGoogleLogin?: () => Promise<LoginDialogActionResult>;
  onGuestLogin?: () => Promise<LoginDialogActionResult>;
  onWalletLogin?: () => Promise<LoginDialogActionResult>;
  onSendPasswordReset?: (email: string) => Promise<LoginDialogActionResult>;
  onTabSwitch?: () => void;
  adminRequired?: boolean;
  adminMessage?: string;
  brandTitle?: string;
  appVersion?: string;
  statusMessage?: LoginDialogStatusMessage | null;
  secondaryActions?: LoginDialogSecondaryAction[];
  contextEyebrow?: string;
  contextTitle?: string;
  contextDescription?: string;
  contextTone?: LoginDialogContextTone;
  disableCredentials?: boolean;
  disableGoogleLogin?: boolean;
  disableGuestLogin?: boolean;
  initialMode?: 'signin' | 'signup';
  onClose?: () => void | Promise<void>;
  closeAriaLabel?: string;
  layoutControls?: Partial<AuthPageSvgControls> | null;
}

function resolveInitialAuthMode(initialMode?: 'signin' | 'signup') {
  if (initialMode) {
    return initialMode;
  }

  if (typeof window === 'undefined') {
    return 'signin';
  }

  const requestedMode = window.sessionStorage.getItem(AUTH_MODE_STORAGE_KEY);
  if (requestedMode === 'signup') {
    window.sessionStorage.removeItem(AUTH_MODE_STORAGE_KEY);
    return 'signup';
  }

  return 'signin';
}

function authDialogFocusableElements(dialog: HTMLElement): Array<HTMLElement | SVGElement> {
  return Array.from(dialog.querySelectorAll<HTMLElement | SVGElement>(AUTH_DIALOG_FOCUSABLE_SELECTOR)).filter(
    (element) => element.getAttribute('aria-hidden') !== 'true' && element.getClientRects().length > 0
  );
}

function containAuthDialogTab(event: KeyboardEvent, dialog: HTMLElement): void {
  if (event.key !== 'Tab') return;
  const focusable = authDialogFocusableElements(dialog);
  const first = focusable[0];
  const last = focusable.at(-1);
  if (!first || !last) {
    event.preventDefault();
    return;
  }
  const active = document.activeElement;
  if (event.shiftKey && (active === first || !dialog.contains(active))) {
    event.preventDefault();
    last.focus();
    return;
  }
  if (!event.shiftKey && (active === last || !dialog.contains(active))) {
    event.preventDefault();
    first.focus();
  }
}

type LoginValidationErrors = {
  email?: string;
  password?: string;
  confirmPassword?: string;
};

function withoutValidationError(
  current: LoginValidationErrors,
  field: keyof LoginValidationErrors
): LoginValidationErrors {
  const next = { ...current };
  delete next[field];
  return next;
}

export function LoginDialog({
  onLogin,
  onSignUp,
  onFacebookLogin,
  onGoogleLogin,
  onGuestLogin,
  onSendPasswordReset,
  onTabSwitch,
  adminRequired = false,
  adminMessage = 'You need to be an administrator to access this page. Please sign in with an admin account.',
  brandTitle = 'Ocentra Parent',
  statusMessage = null,
  secondaryActions = [],
  contextEyebrow,
  contextTitle,
  contextDescription,
  contextTone = 'default',
  disableCredentials = false,
  disableGoogleLogin = false,
  disableGuestLogin = false,
  initialMode,
  onClose,
  closeAriaLabel = 'Close authentication dialog',
  layoutControls = null,
}: LoginDialogProps) {
  const signUpEnabled = typeof onSignUp === 'function';
  const socialOptions = [
    {
      key: 'facebook',
      handler: onFacebookLogin,
      icon: AuthImages.Social.facebook,
      alt: 'Facebook',
      error: 'Facebook login failed. Please try again.',
    },
    {
      key: 'google',
      handler: onGoogleLogin,
      icon: AuthImages.Social.google,
      alt: 'Google',
      error: 'Google login failed. Please try again.',
      disabled: disableGoogleLogin,
    },
    {
      key: 'guest',
      handler: onGuestLogin,
      icon: AuthImages.Social.guest,
      alt: 'Guest',
      error: 'Guest login failed. Please try again.',
      hidden: disableGuestLogin,
    },
  ].filter((option) => !option.hidden && typeof option.handler === 'function');
  const [isSignIn, setIsSignIn] = useState(() => resolveInitialAuthMode(initialMode) === 'signin');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [alias, setAlias] = useState('');
  const [avatar, setAvatar] = useState('');
  const [showAvatarSelector, setShowAvatarSelector] = useState(false);
  const [avatarOptions] = useState<{ id: number; url: string }[]>(() =>
    Object.entries(AvatarImages)
      .map(([key, url]) => ({
        id: parseInt(key, 10),
        url: url as string,
      }))
      .filter((entry) => entry.id >= 1 && entry.id <= 18)
      .sort((a, b) => a.id - b.id)
  );
  const [showForgotPassword, setShowForgotPassword] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');
  const [successMessage, setSuccessMessage] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [validationErrors, setValidationErrors] = useState<LoginValidationErrors>({});
  const avatarSelectorRef = useRef<HTMLDivElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const dialogRef = useRef<HTMLDivElement>(null);
  const onCloseRef = useRef(onClose);

  useEffect(() => {
    onCloseRef.current = onClose;
  }, [onClose]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as HTMLElement | null;
      if (target?.closest('.login-cyber-avatar-button')) {
        return;
      }
      if (avatarSelectorRef.current && target && !avatarSelectorRef.current.contains(target)) {
        setShowAvatarSelector(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  useEffect(() => {
    if (typeof document === 'undefined') {
      return undefined;
    }

    const { body, documentElement } = document;
    const previousBodyOverflow = body.style.overflow;
    const previousBodyOverscrollBehavior = body.style.overscrollBehavior;
    const previousDocumentOverflow = documentElement.style.overflow;

    body.style.overflow = 'hidden';
    body.style.overscrollBehavior = 'contain';
    documentElement.style.overflow = 'hidden';

    return () => {
      body.style.overflow = previousBodyOverflow;
      body.style.overscrollBehavior = previousBodyOverscrollBehavior;
      documentElement.style.overflow = previousDocumentOverflow;
    };
  }, []);

  useEffect(() => {
    const dialog = dialogRef.current;
    if (typeof document === 'undefined' || !dialog) return undefined;
    const previouslyFocused = document.activeElement;
    const siblingStates = Array.from(dialog.parentElement?.children ?? [])
      .filter((element): element is HTMLElement => element instanceof HTMLElement && element !== dialog)
      .map((element) => ({
        element,
        ariaHidden: element.getAttribute('aria-hidden'),
        inert: element.inert,
      }));
    for (const sibling of siblingStates) {
      sibling.element.inert = true;
      sibling.element.setAttribute('aria-hidden', 'true');
    }
    const initialFocus = dialog.querySelector<HTMLInputElement>('input:not([disabled]):not([type="hidden"])');
    (initialFocus ?? authDialogFocusableElements(dialog)[0])?.focus();
    const handleDialogKeyDown = (event: KeyboardEvent): void => {
      if (event.key === 'Escape' && onCloseRef.current) {
        event.preventDefault();
        void onCloseRef.current();
        return;
      }
      containAuthDialogTab(event, dialog);
    };
    document.addEventListener('keydown', handleDialogKeyDown);
    return () => {
      document.removeEventListener('keydown', handleDialogKeyDown);
      for (const sibling of siblingStates) {
        sibling.element.inert = sibling.inert;
        if (sibling.ariaHidden === null) sibling.element.removeAttribute('aria-hidden');
        else sibling.element.setAttribute('aria-hidden', sibling.ariaHidden);
      }
      if (previouslyFocused instanceof HTMLElement || previouslyFocused instanceof SVGElement) {
        if (previouslyFocused.isConnected) previouslyFocused.focus();
      }
    };
  }, []);

  useEffect(() => {
    if (!signUpEnabled || typeof window === 'undefined') {
      return;
    }

    const preloadedAvatars = avatarOptions.map((option) => {
      const image = new window.Image();
      image.decoding = 'async';
      image.src = option.url;
      return image;
    });

    void preloadedAvatars;
  }, [avatarOptions, signUpEnabled]);

  const isValidEmail = (email: string): boolean => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);

  const validatePassword = (value: string): string | undefined => {
    if (value.length < 6) {
      return 'Password must be at least 6 characters.';
    }
    return undefined;
  };

  const clearMessages = () => {
    setErrorMessage('');
    setSuccessMessage('');
  };

  const handleSocialAuthResult = async (action: () => Promise<LoginDialogActionResult>, fallbackError: string) => {
    clearMessages();
    setIsLoading(true);
    try {
      const result = await action();
      if (!result.success) {
        setErrorMessage(result.error || fallbackError);
      }
    } catch {
      setErrorMessage(fallbackError);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    if (showForgotPassword) {
      await handleForgotPassword();
      return;
    }
    clearMessages();
    setValidationErrors({});
    setIsLoading(true);

    const errors: LoginValidationErrors = {};
    if (!username) {
      errors.email = 'Email is required.';
    } else if (!isValidEmail(username)) {
      errors.email = 'Please enter a valid email address.';
    }

    if (!password) {
      errors.password = 'Password is required.';
    } else if (!isSignIn && signUpEnabled) {
      const passwordError = validatePassword(password);
      if (passwordError) {
        errors.password = passwordError;
      }
    }

    if (!isSignIn && signUpEnabled && password !== confirmPassword) {
      errors.confirmPassword = 'Passwords do not match.';
    }

    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors);
      setIsLoading(false);
      return;
    }

    try {
      if (isSignIn || !signUpEnabled) {
        const result = await onLogin(username, password);
        if (!result.success) {
          setErrorMessage(result.error || 'Login failed. Please check your credentials.');
        }
      } else if (onSignUp) {
        const result = await onSignUp({ alias, avatar, username, password });
        if (!result.success) {
          setErrorMessage(result.error || 'Sign up failed. Please try again.');
        }
      }
    } catch {
      setErrorMessage('An error occurred. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  async function handleForgotPassword() {
    if (!onSendPasswordReset) {
      return;
    }
    setValidationErrors({});
    if (!username) {
      setErrorMessage('Please enter your email address.');
      return;
    }
    if (!isValidEmail(username)) {
      setErrorMessage('Please enter a valid email address.');
      return;
    }
    clearMessages();
    setIsLoading(true);

    try {
      const result = await onSendPasswordReset(username);
      if (result.success) {
        setSuccessMessage('Password reset email sent! Please check your inbox.');
        setShowForgotPassword(false);
      } else {
        setErrorMessage(result.error || 'Failed to send password reset email.');
      }
    } catch {
      setErrorMessage('An error occurred. Please try again.');
    } finally {
      setIsLoading(false);
    }
  }

  const handleAvatarSelect = (avatarUrl: string) => {
    setAvatar(avatarUrl);
    setShowAvatarSelector(false);
  };

  const handleUploadClick = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file || !file.type.match('image.*')) {
      return;
    }

    const reader = new FileReader();
    reader.onload = (loadEvent) => {
      const image = new Image();
      image.onload = () => {
        const canvas = document.createElement('canvas');
        canvas.width = 128;
        canvas.height = 128;
        const context = canvas.getContext('2d');
        if (!context) {
          return;
        }
        context.drawImage(image, 0, 0, 128, 128);
        setAvatar(canvas.toDataURL('image/png'));
        setShowAvatarSelector(false);
      };
      image.src = String(loadEvent.target?.result ?? '');
    };
    reader.readAsDataURL(file);
  };

  const activeMessage: LoginDialogStatusMessage | null =
    errorMessage || successMessage
      ? { kind: errorMessage ? 'error' : 'success', text: errorMessage || successMessage }
      : statusMessage;
  const introEyebrow =
    contextEyebrow ?? (adminRequired ? 'Restricted feature' : disableGuestLogin ? 'Player account' : 'Player access');
  const introTitle = contextTitle ?? (adminRequired ? 'Administrator access required' : 'Sign in to continue');
  const introDescription =
    contextDescription ??
    (adminRequired
      ? adminMessage
      : disableGuestLogin
        ? 'Use a full account to continue.'
        : 'Use a full account or continue as guest when this feature allows it.');
  const introTone = adminRequired ? 'warning' : contextTone;
  const cyberSocialOptions = socialOptions.map((option) => ({
    key: option.key,
    icon: option.icon,
    alt: option.alt,
    disabled: isLoading || Boolean(option.disabled),
    onClick: () => {
      if (option.handler) {
        void handleSocialAuthResult(option.handler, option.error);
      }
    },
  }));
  const cyberSecondaryActions = secondaryActions.map((action) => ({
    label: action.label,
    disabled: action.disabled || isLoading,
    onClick: () => {
      void action.onClick();
    },
  }));
  const handleModeChange = (mode: 'signin' | 'signup') => {
    const nextIsSignIn = mode === 'signin';
    if (nextIsSignIn !== isSignIn && onTabSwitch) {
      onTabSwitch();
    }
    setIsSignIn(nextIsSignIn);
    setShowForgotPassword(false);
    setValidationErrors({});
    clearMessages();
  };
  const handleEmailChange = (value: string) => {
    setUsername(value);
    if (validationErrors.email) {
      setValidationErrors((current) => withoutValidationError(current, 'email'));
    }
  };
  const handlePasswordChange = (value: string) => {
    setPassword(value);
    if (validationErrors.password) {
      setValidationErrors((current) => withoutValidationError(current, 'password'));
    }
  };
  const handleConfirmPasswordChange = (value: string) => {
    setConfirmPassword(value);
    if (validationErrors.confirmPassword) {
      setValidationErrors((current) => withoutValidationError(current, 'confirmPassword'));
    }
  };
  const normalizedLayoutControls = useMemo(() => normalizeAuthPageSvgControls(layoutControls), [layoutControls]);
  const cyberShellStyle = useMemo(
    () =>
      ({
        '--login-cyber-overlay-opacity': normalizedLayoutControls.dialogOverlayOpacity,
        '--login-cyber-backdrop-blur': `${normalizedLayoutControls.dialogBackdropBlur}px`,
        '--login-cyber-backdrop-saturate': normalizedLayoutControls.dialogBackdropSaturate,
        '--login-cyber-fade-ms': `${normalizedLayoutControls.dialogFadeMs}ms`,
        '--login-cyber-form-max-w': `${normalizedLayoutControls.dialogFormMaxWRem}rem`,
        '--login-cyber-svg-max-w': `${normalizedLayoutControls.dialogSvgMaxWRem}rem`,
        '--login-cyber-svg-max-h': `${normalizedLayoutControls.dialogSvgMaxHRem}rem`,
      }) as CSSProperties,
    [normalizedLayoutControls]
  );

  return (
    <div
      ref={dialogRef}
      className="login-dialog-overlay login-dialog-overlay--cyber"
      style={cyberShellStyle}
      role="dialog"
      aria-modal="true"
      aria-label={introTitle}
    >
      <form className={`login-cyber-form login-cyber-form--${introTone}`} onSubmit={handleSubmit}>
        <CyberAuthSurface
          layoutControls={normalizedLayoutControls}
          mode={isSignIn ? 'signin' : 'signup'}
          signUpEnabled={signUpEnabled}
          canSendPasswordReset={Boolean(onSendPasswordReset && !disableCredentials)}
          brandTitle={brandTitle}
          eyebrow={introEyebrow}
          title={introTitle}
          description={introDescription}
          warning={introTone === 'warning'}
          alias={alias}
          email={username}
          password={password}
          confirmPassword={confirmPassword}
          avatar={avatar}
          avatarOptions={avatarOptions}
          showAvatarSelector={showAvatarSelector}
          showForgotPassword={showForgotPassword}
          notice={activeMessage}
          validationErrors={validationErrors}
          isLoading={isLoading}
          disableCredentials={disableCredentials}
          socialOptions={cyberSocialOptions}
          secondaryActions={cyberSecondaryActions}
          closeAriaLabel={closeAriaLabel}
          onModeChange={handleModeChange}
          onAliasChange={setAlias}
          onEmailChange={handleEmailChange}
          onPasswordChange={handlePasswordChange}
          onConfirmPasswordChange={handleConfirmPasswordChange}
          onToggleAvatarSelector={() => setShowAvatarSelector((value) => !value)}
          onAvatarSelect={handleAvatarSelect}
          onAvatarUploadClick={handleUploadClick}
          onFileChange={handleFileChange}
          onForgotPassword={() => {
            setShowForgotPassword(true);
            clearMessages();
            setValidationErrors({});
          }}
          onBackToSignIn={() => {
            setShowForgotPassword(false);
            clearMessages();
          }}
          onClose={
            onClose
              ? () => {
                  void onClose();
                }
              : undefined
          }
          avatarSelectorRef={avatarSelectorRef}
          fileInputRef={fileInputRef}
        />
      </form>
    </div>
  );
}
