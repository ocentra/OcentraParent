import { useState, type FormEvent, type ReactElement } from 'react';
import {
  PortalAuthChrome,
  PortalDom,
  PortalText,
  PortalTextToken,
  type PortalAuthAutoComplete,
  type PortalAuthInputType,
  type PortalAuthMode,
} from '@ocentra-parent/portal-domain/contracts';
import { PortalAuthFrame } from './PortalAuthFrame';
import './portal-auth-dialog.css';

type PortalAuthDialogProps = {
  readonly onClose: () => void;
};

type SocialOption = {
  readonly icon: (typeof PortalAuthChrome.Assets)[keyof typeof PortalAuthChrome.Assets];
  readonly labelToken: PortalAuthTextToken;
};

type PortalAuthTextToken = (typeof PortalTextToken)[keyof typeof PortalTextToken];

const socialOptions: readonly SocialOption[] = [
  {
    icon: PortalAuthChrome.Assets.Google,
    labelToken: PortalTextToken.AuthGoogle,
  },
  {
    icon: PortalAuthChrome.Assets.Facebook,
    labelToken: PortalTextToken.AuthFacebook,
  },
  {
    icon: PortalAuthChrome.Assets.Guest,
    labelToken: PortalTextToken.AuthGuest,
  },
] as const;

export function PortalAuthDialog({ onClose }: PortalAuthDialogProps): ReactElement {
  const [mode, setMode] = useState<PortalAuthMode>(PortalAuthChrome.Modes.SignIn);
  const [statusVisible, setStatusVisible] = useState(false);
  const isSignUp = mode === PortalAuthChrome.Modes.SignUp;
  const submit = (event: FormEvent<HTMLFormElement>): void => {
    event.preventDefault();
    setStatusVisible(true);
  };
  return (
    <div className={PortalAuthChrome.Classes.Backdrop}>
      <section
        aria-label={PortalText.Resolve(PortalTextToken.AuthTitle)}
        aria-modal={true}
        className={PortalAuthChrome.Classes.Dialog}
        role={PortalAuthChrome.Roles.Dialog}
      >
        <PortalAuthFrame />
        <button
          aria-label={PortalText.Resolve(PortalTextToken.AuthClose)}
          className={PortalAuthChrome.Classes.CloseButton}
          onClick={onClose}
          type={PortalDom.ButtonType.Button}
        >
          {PortalText.Resolve(PortalTextToken.AuthClose)}
        </button>
        <div className={PortalAuthChrome.Classes.Panel}>
          <AuthBrand />
          <p className={PortalAuthChrome.Classes.Eyebrow}>{PortalText.Resolve(PortalTextToken.AuthEyebrow)}</p>
          <h2 className={PortalAuthChrome.Classes.Title}>{PortalText.Resolve(PortalTextToken.AuthTitle)}</h2>
          <p className={PortalAuthChrome.Classes.Body}>{PortalText.Resolve(PortalTextToken.AuthBody)}</p>
          <ModeTabs mode={mode} onModeChange={setMode} />
          <form className={PortalAuthChrome.Classes.Form} onSubmit={submit}>
            {isSignUp ? (
              <AuthField
                autoComplete={PortalAuthChrome.AutoComplete.Name}
                labelToken={PortalTextToken.AuthParentName}
                type={PortalAuthChrome.InputTypes.Text}
              />
            ) : null}
            <AuthField
              autoComplete={PortalAuthChrome.AutoComplete.Email}
              labelToken={PortalTextToken.AuthParentEmail}
              type={PortalAuthChrome.InputTypes.Email}
            />
            <AuthField
              autoComplete={
                isSignUp ? PortalAuthChrome.AutoComplete.NewPassword : PortalAuthChrome.AutoComplete.CurrentPassword
              }
              labelToken={PortalTextToken.AuthPassword}
              type={PortalAuthChrome.InputTypes.Password}
            />
            {isSignUp ? (
              <AuthField
                autoComplete={PortalAuthChrome.AutoComplete.NewPassword}
                labelToken={PortalTextToken.AuthConfirmPassword}
                type={PortalAuthChrome.InputTypes.Password}
              />
            ) : null}
            <button className={PortalAuthChrome.Classes.PrimaryButton} type={PortalDom.ButtonType.Submit}>
              {PortalText.Resolve(PortalTextToken.AuthPrimaryAction)}
            </button>
          </form>
          <SocialPanel onSelect={() => setStatusVisible(true)} />
          <div className={PortalAuthChrome.Classes.TrustPanel}>
            <strong>{PortalText.Resolve(PortalTextToken.AuthTrustTitle)}</strong>
            <span>{PortalText.Resolve(PortalTextToken.AuthTrustBody)}</span>
          </div>
          {statusVisible ? (
            <p className={PortalAuthChrome.Classes.Status} role={PortalAuthChrome.Roles.Status}>
              {PortalText.Resolve(PortalTextToken.AuthUnavailable)}
            </p>
          ) : null}
        </div>
      </section>
    </div>
  );
}

function AuthBrand(): ReactElement {
  return (
    <div className={PortalAuthChrome.Classes.Brand}>
      <img
        alt={PortalText.Resolve(PortalTextToken.AppTitle)}
        className={PortalAuthChrome.Classes.BrandMark}
        src={PortalAuthChrome.Assets.Logo}
      />
      <span className={PortalAuthChrome.Classes.BrandText}>{PortalText.Resolve(PortalTextToken.AppTitle)}</span>
    </div>
  );
}

function ModeTabs({
  mode,
  onModeChange,
}: {
  readonly mode: PortalAuthMode;
  readonly onModeChange: (mode: PortalAuthMode) => void;
}): ReactElement {
  return (
    <div className={PortalAuthChrome.Classes.ModeTabs}>
      <ModeTab
        active={mode === PortalAuthChrome.Modes.SignIn}
        labelToken={PortalTextToken.AuthSignIn}
        onClick={() => onModeChange(PortalAuthChrome.Modes.SignIn)}
      />
      <ModeTab
        active={mode === PortalAuthChrome.Modes.SignUp}
        labelToken={PortalTextToken.AuthSignUp}
        onClick={() => onModeChange(PortalAuthChrome.Modes.SignUp)}
      />
    </div>
  );
}

function ModeTab({
  active,
  labelToken,
  onClick,
}: {
  readonly active: boolean;
  readonly labelToken: PortalAuthTextToken;
  readonly onClick: () => void;
}): ReactElement {
  const className = active
    ? [PortalAuthChrome.Classes.ModeTab, PortalAuthChrome.Classes.ModeTabActive].join(
        PortalDom.Classes.ClassNameSeparator
      )
    : PortalAuthChrome.Classes.ModeTab;
  return (
    <button aria-pressed={active} className={className} onClick={onClick} type={PortalDom.ButtonType.Button}>
      {PortalText.Resolve(labelToken)}
    </button>
  );
}

function AuthField({
  autoComplete,
  labelToken,
  type,
}: {
  readonly autoComplete: PortalAuthAutoComplete;
  readonly labelToken: PortalAuthTextToken;
  readonly type: PortalAuthInputType;
}): ReactElement {
  return (
    <label className={PortalAuthChrome.Classes.Field}>
      <span className={PortalAuthChrome.Classes.FieldLabel}>{PortalText.Resolve(labelToken)}</span>
      <input autoComplete={autoComplete} className={PortalAuthChrome.Classes.FieldInput} type={type} />
    </label>
  );
}

function SocialPanel({ onSelect }: { readonly onSelect: () => void }): ReactElement {
  return (
    <div className={PortalAuthChrome.Classes.SocialPanel}>
      <strong>{PortalText.Resolve(PortalTextToken.AuthSocialTitle)}</strong>
      {socialOptions.map((option) => (
        <button
          className={PortalAuthChrome.Classes.SocialButton}
          key={option.labelToken}
          onClick={onSelect}
          type={PortalDom.ButtonType.Button}
        >
          <img
            alt={PortalText.Resolve(option.labelToken)}
            className={PortalAuthChrome.Classes.SocialIcon}
            src={option.icon}
          />
          <span>{PortalText.Resolve(option.labelToken)}</span>
        </button>
      ))}
    </div>
  );
}
