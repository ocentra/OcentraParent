use ocentra_schema::export_import_backup_recovery as contracts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupJobTransition {
    pub lifecycle: contracts::ExportImportBackupJobLifecycle,
    pub updated_at: String,
    pub execution_ref: Option<String>,
    pub provider_operation_ref: Option<String>,
    pub manual_required_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupJobStateError {
    InvalidTimestamp,
    InvalidIdempotencyRef,
    AttemptOverflow,
    InvalidExecutionRef,
    InvalidProviderOperationRef,
    ManualRequiredNoteMissing,
    InvalidTransition {
        from: contracts::ExportImportBackupJobLifecycle,
        to: contracts::ExportImportBackupJobLifecycle,
    },
    ExecutionRefRequired,
    ProviderOperationRefRequired,
}

/// Applies one schema-owned lifecycle transition without performing an
/// external provider operation. References are persisted as opaque values and
/// are only attached to transitions that have reached the corresponding
/// executor boundary.
pub fn advance_backup_job(
    record: &contracts::ExportImportBackupJobRecord,
    transition: BackupJobTransition,
) -> Result<contracts::ExportImportBackupJobRecord, BackupJobStateError> {
    let updated_at = contracts::ExportImportTimestamp::parse(transition.updated_at)
        .ok_or(BackupJobStateError::InvalidTimestamp)?;
    let (execution_ref, provider_operation_ref) = parse_transition_refs(&transition)?;
    validate_transition(
        record,
        &transition,
        execution_ref.as_ref(),
        provider_operation_ref.as_ref(),
    )?;

    let attempt = if transition.lifecycle == contracts::ExportImportBackupJobLifecycle::Claimed {
        record
            .attempt
            .checked_add(1)
            .ok_or(BackupJobStateError::AttemptOverflow)?
    } else {
        record.attempt
    };
    let idempotency_ref = next_idempotency_ref(record, transition.lifecycle, attempt)?;

    Ok(contracts::ExportImportBackupJobRecord {
        job_ref: record.job_ref.clone(),
        schedule_ref: record.schedule_ref.clone(),
        bundle_id: record.bundle_id.clone(),
        household_id: record.household_id.clone(),
        cadence: record.cadence,
        lifecycle: transition.lifecycle,
        attempt,
        idempotency_ref,
        execution_ref: execution_ref.or_else(|| record.execution_ref.clone()),
        provider_operation_ref: provider_operation_ref
            .or_else(|| record.provider_operation_ref.clone()),
        created_at: record.created_at.clone(),
        updated_at,
        manual_required_note: transition
            .manual_required_note
            .or_else(|| record.manual_required_note.clone()),
    })
}

fn parse_transition_refs(
    transition: &BackupJobTransition,
) -> Result<
    (
        Option<contracts::ExportImportExecutionRef>,
        Option<contracts::ExportImportProviderOperationRef>,
    ),
    BackupJobStateError,
> {
    let execution_ref = transition
        .execution_ref
        .clone()
        .map(|value| {
            contracts::ExportImportExecutionRef::parse(value)
                .ok_or(BackupJobStateError::InvalidExecutionRef)
        })
        .transpose()?;
    let provider_operation_ref = transition
        .provider_operation_ref
        .clone()
        .map(|value| {
            contracts::ExportImportProviderOperationRef::parse(value)
                .ok_or(BackupJobStateError::InvalidProviderOperationRef)
        })
        .transpose()?;
    Ok((execution_ref, provider_operation_ref))
}

fn validate_transition(
    record: &contracts::ExportImportBackupJobRecord,
    transition: &BackupJobTransition,
    execution_ref: Option<&contracts::ExportImportExecutionRef>,
    provider_operation_ref: Option<&contracts::ExportImportProviderOperationRef>,
) -> Result<(), BackupJobStateError> {
    if !transition_is_allowed(record.lifecycle, transition.lifecycle) {
        return Err(BackupJobStateError::InvalidTransition {
            from: record.lifecycle,
            to: transition.lifecycle,
        });
    }
    if execution_required(transition.lifecycle)
        && execution_ref.is_none()
        && record.execution_ref.is_none()
    {
        return Err(BackupJobStateError::ExecutionRefRequired);
    }
    if transition.lifecycle == contracts::ExportImportBackupJobLifecycle::Succeeded
        && provider_operation_ref.is_none()
        && record.provider_operation_ref.is_none()
    {
        return Err(BackupJobStateError::ProviderOperationRefRequired);
    }
    if transition.lifecycle == contracts::ExportImportBackupJobLifecycle::ManualRequired
        && transition
            .manual_required_note
            .as_deref()
            .is_none_or(|note| note.trim().is_empty())
    {
        return Err(BackupJobStateError::ManualRequiredNoteMissing);
    }
    Ok(())
}

fn execution_required(lifecycle: contracts::ExportImportBackupJobLifecycle) -> bool {
    matches!(
        lifecycle,
        contracts::ExportImportBackupJobLifecycle::Running
            | contracts::ExportImportBackupJobLifecycle::Succeeded
            | contracts::ExportImportBackupJobLifecycle::Retryable
            | contracts::ExportImportBackupJobLifecycle::Failed
    )
}

fn next_idempotency_ref(
    record: &contracts::ExportImportBackupJobRecord,
    lifecycle: contracts::ExportImportBackupJobLifecycle,
    attempt: u32,
) -> Result<contracts::ExportImportIdempotencyRef, BackupJobStateError> {
    if lifecycle != contracts::ExportImportBackupJobLifecycle::Claimed {
        return Ok(record.idempotency_ref.clone());
    }
    contracts::ExportImportIdempotencyRef::parse(format!(
        "{}:attempt-{}",
        record.job_ref.as_str(),
        attempt
    ))
    .ok_or(BackupJobStateError::InvalidIdempotencyRef)
}

fn transition_is_allowed(
    from: contracts::ExportImportBackupJobLifecycle,
    to: contracts::ExportImportBackupJobLifecycle,
) -> bool {
    use contracts::ExportImportBackupJobLifecycle::*;
    matches!(
        (from, to),
        (Scheduled, Claimed | ManualRequired)
            | (Claimed, Running | Retryable | Failed | ManualRequired)
            | (Running, Succeeded | Retryable | Failed | ManualRequired)
            | (Retryable, Claimed | Failed | ManualRequired)
            | (Failed, ManualRequired | Reconciled)
            | (ManualRequired, Reconciled)
            | (Reconciled, Reconciled)
            | (Succeeded, Succeeded)
    )
}
