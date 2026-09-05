const POWERSHELL_NO_LOGO: &str = "-NoLogo";
const POWERSHELL_NO_PROFILE: &str = "-NoProfile";
const POWERSHELL_NON_INTERACTIVE: &str = "-NonInteractive";
const POWERSHELL_COMMAND: &str = "-Command";
const POWERSHELL_SCRIPT: &str = r#"
$ErrorActionPreference='Stop';$WarningPreference='SilentlyContinue';$InformationPreference='SilentlyContinue';$ProgressPreference='SilentlyContinue';
$appIdQuery=$false;$appIdPresent=$false;$appIdRunning=$false;
try{$svc=Get-CimInstance -ClassName Win32_Service -Filter 'Name=''AppIDSvc''' -ErrorAction Stop;$appIdQuery=$true;if($null -ne $svc){$appIdPresent=$true;$appIdRunning=($svc.State -eq 'Running')}}catch{}
$appLockerReadable=$false;$collectionCount=0;$ruleCount=0;
try{$policy=Get-AppLockerPolicy -Local -ErrorAction Stop;$collections=@($policy.RuleCollections);$collectionCount=$collections.Count;$ruleCount=(@($collections|ForEach-Object{@($_.Rules).Count})|Measure-Object -Sum).Sum;if($null -eq $ruleCount){$ruleCount=0};$appLockerReadable=$true}catch{}
$deviceGuardQuery=$false;$deviceGuardConfigured=$false;$deviceGuardRunning=$false;$appControlConfigured=$false;$appControlAuditOnly=$false;$appControlEnforced=$false;
try{$guard=Get-CimInstance -Namespace 'root\Microsoft\Windows\DeviceGuard' -ClassName Win32_DeviceGuard -ErrorAction Stop;$deviceGuardQuery=$true;$deviceGuardConfigured=(@($guard.SecurityServicesConfigured).Count -gt 0);$deviceGuardRunning=(@($guard.SecurityServicesRunning).Count -gt 0);$policyStatus=[int]$guard.CodeIntegrityPolicyEnforcementStatus;$appControlConfigured=($policyStatus -eq 1 -or $policyStatus -eq 2);$appControlAuditOnly=($policyStatus -eq 1);$appControlEnforced=($policyStatus -eq 2)}catch{}
[ordered]@{schemaVersion=1;probeSupported=$true;appIdServiceQuerySucceeded=$appIdQuery;appIdServicePresent=$appIdPresent;appIdServiceRunning=$appIdRunning;appLockerPolicyReadable=$appLockerReadable;appLockerCollectionCount=[uint64]$collectionCount;appLockerRuleCount=[uint64]$ruleCount;deviceGuardQuerySucceeded=$deviceGuardQuery;deviceGuardConfigured=$deviceGuardConfigured;deviceGuardRunning=$deviceGuardRunning;appControlConfigured=$appControlConfigured;appControlAuditOnly=$appControlAuditOnly;appControlPolicyReportsEnforced=$appControlEnforced}|ConvertTo-Json -Compress
"#;

pub(super) const POWERSHELL_ARGUMENTS: [&str; 5] = [
    POWERSHELL_NO_LOGO,
    POWERSHELL_NO_PROFILE,
    POWERSHELL_NON_INTERACTIVE,
    POWERSHELL_COMMAND,
    POWERSHELL_SCRIPT,
];
