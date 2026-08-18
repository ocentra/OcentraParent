use super::{authority::*, support::*, *};

impl SqliteAccountIdentityAuthorityRepository {
    pub fn begin_recovery(
        &mut self,
        proof: &VerifiedRecoveryIdentityProof,
        support_authorization: Option<&VerifiedSupportRecoveryAuthorization>,
    ) -> Result<RecoveryId, InviteRecoveryRepositoryError> {
        if !recovery_request_is_allowed(proof.role, proof.kind, proof.support_channel)
            || (proof.support_channel == RecoverySupportChannel::SupportAssisted
                && support_authorization.is_none())
            || proof.proof_id.trim().is_empty()
            || proof.member_id.trim().is_empty()
            || proof.device_id.trim().is_empty()
        {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        let recovery_id = RecoveryId::parse(opaque_id("recovery-")?)
            .map_err(InviteRecoveryRepositoryError::InvalidValue)?;
        let state = if owner_approval_required(proof.kind, proof.support_channel) {
            "owner-approval-required"
        } else {
            "approved"
        };
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        if proof.expires_at_epoch_millis <= now {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        if let Some(authorization) = support_authorization {
            if authorization.expires_at_epoch_millis <= now
                || authorization.authorization_id.is_empty()
                || authorization.issuer.is_empty()
                || authorization.household_id != proof.household_id
                || authorization.account_id != proof.account_id
                || authorization.kind != proof.kind
                || !matches!(
                    authorization.scope,
                    AccountIdentitySupportScope::Household
                        | AccountIdentitySupportScope::DeviceControl
                )
            {
                return Err(InviteRecoveryRepositoryError::RecoveryRejected);
            }
        }
        enforce_recovery_rate_limit(&transaction, &proof.provider, &proof.provider_subject, now)?;
        transaction
            .execute(
                "INSERT INTO account_identity_recovery (
                     recovery_id, household_id, account_id, requester_member_id,
                     requester_device_id, requester_role, kind, support_channel,
                     identity_proof_id, identity_proof_provider, identity_proof_subject,
                     identity_proof_expires_at_epoch_millis, identity_proof_state,
                     owner_effect,
                     state, delete_export_handoff_required, created_at_epoch_millis,
                     last_transition_at_epoch_millis
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, 'verified', ?13, ?14, ?15, ?16, ?16)",
                params![
                    recovery_id.as_str(),
                    proof.household_id.to_string(),
                    proof.account_id.to_string(),
                    proof.member_id.as_str(),
                    proof.device_id.as_str(),
                    role_label(proof.role),
                    recovery_kind_label(proof.kind),
                    support_channel_label(proof.support_channel),
                    proof.proof_id.as_str(),
                    provider_label(&proof.provider),
                    proof.provider_subject.as_str(),
                    proof.expires_at_epoch_millis,
                    owner_effect_label(owner_effect(proof.kind)),
                    state,
                    i64::from(recovery_requires_custody_handoff(proof.kind)),
                    now,
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        Ok(recovery_id)
    }

    pub fn approve_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &RecoveryId,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        if authority.role() != AccountIdentityRole::ParentOwner {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        ensure_current_authority(&transaction, authority, now)?;
        let transition_at = next_transition_at(&transaction, recovery_id, now)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_recovery
                 SET state = 'approved', last_transition_at_epoch_millis = ?3
                 WHERE recovery_id = ?1 AND household_id = ?2
                   AND state = 'owner-approval-required'
                   AND identity_proof_state = 'verified'",
                params![
                    recovery_id.as_str(),
                    authority.household_id().to_string(),
                    transition_at
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
    }

    pub fn complete_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &RecoveryId,
    ) -> Result<RecoveryCompletion, InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        ensure_current_authority(&transaction, authority, now)?;
        let transition_at = next_transition_at(&transaction, recovery_id, now)?;
        let row = transaction
            .query_row(
                "SELECT account_id, requester_member_id, requester_device_id, kind,
                        delete_export_handoff_required
                 FROM account_identity_recovery
                 WHERE recovery_id = ?1 AND household_id = ?2 AND state = 'approved'
                 LIMIT 1",
                params![recovery_id.as_str(), authority.household_id().to_string()],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, i64>(4)?,
                    ))
                },
            )
            .optional()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?
            .ok_or(InviteRecoveryRepositoryError::RecoveryRejected)?;
        let kind = recovery_kind_from_label(&row.3)
            .ok_or(InviteRecoveryRepositoryError::RecoveryRejected)?;
        let handoff_enqueued = row.4 == 1;
        if handoff_enqueued {
            let handoff_id = opaque_id("handoff-")?;
            let correlation_id = opaque_id("correlation-")?;
            transaction
                .execute(
                    "INSERT INTO account_identity_recovery_custody_handoff (
                         handoff_id, correlation_id, recovery_id, household_id, account_id,
                         member_id, device_id, kind, requested_at_epoch_millis, state,
                         attempt_count, active_attempt_id
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'pending', 0, NULL)
                     ON CONFLICT(recovery_id) DO NOTHING",
                    params![
                        handoff_id,
                        correlation_id,
                        recovery_id.as_str(),
                        authority.household_id().to_string(),
                        row.0,
                        row.1,
                        row.2,
                        recovery_kind_label(kind),
                        transition_at,
                    ],
                )
                .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
            transaction
                .commit()
                .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
            return Ok(RecoveryCompletion {
                state: RecoveryState::Approved,
                handoff_enqueued: true,
                owner_effect: owner_effect(kind),
            });
        }
        let changed = transaction
            .execute(
                "UPDATE account_identity_recovery
                 SET state = 'completed', last_transition_at_epoch_millis = ?3
                 WHERE recovery_id = ?1 AND household_id = ?2 AND state = 'approved'",
                params![
                    recovery_id.as_str(),
                    authority.household_id().to_string(),
                    transition_at
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        Ok(RecoveryCompletion {
            state: RecoveryState::Completed,
            handoff_enqueued,
            owner_effect: owner_effect(kind),
        })
    }

    pub fn revoke_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &RecoveryId,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        ensure_current_authority(&transaction, authority, now)?;
        let transition_at = next_transition_at(&transaction, recovery_id, now)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_recovery
                 SET state = 'revoked', last_transition_at_epoch_millis = ?3
                 WHERE recovery_id = ?1 AND household_id = ?2
                   AND state IN ('owner-approval-required','approved')",
                params![
                    recovery_id.as_str(),
                    authority.household_id().to_string(),
                    transition_at
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
    }

    pub fn claim_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<Option<RecoveryHandoffDeliveryAttempt>, InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        let lease_expires = now
            .checked_add(HANDOFF_LEASE_MILLIS)
            .ok_or(InviteRecoveryRepositoryError::HandoffConflict)?;
        ensure_current_authority(&transaction, authority, now)?;
        let row = transaction
            .query_row(
                "SELECT handoff_id, correlation_id, recovery_id, account_id, member_id,
                        device_id, kind, requested_at_epoch_millis
                 FROM account_identity_recovery_custody_handoff
                 WHERE household_id = ?1 AND (state = 'pending'
                    OR (state = 'in-flight' AND lease_expires_at_epoch_millis <= ?2))
                 ORDER BY requested_at_epoch_millis, handoff_id LIMIT 1",
                params![authority.household_id().to_string(), now],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                        row.get::<_, String>(6)?,
                        row.get::<_, i64>(7)?,
                    ))
                },
            )
            .optional()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let Some(row) = row else {
            transaction
                .commit()
                .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
            return Ok(None);
        };
        let attempt_id = opaque_id("attempt-")?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_recovery_custody_handoff
                 SET state = 'in-flight', lease_expires_at_epoch_millis = ?2,
                     attempt_count = attempt_count + 1, active_attempt_id = ?3
                 WHERE handoff_id = ?1 AND household_id = ?4
                   AND (state = 'pending'
                     OR (state = 'in-flight' AND lease_expires_at_epoch_millis <= ?5))",
                params![
                    row.0,
                    lease_expires,
                    attempt_id,
                    authority.household_id().to_string(),
                    now
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::HandoffConflict);
        }
        let handoff = durable_handoff(
            row.0,
            row.1,
            row.2,
            authority.household_id().to_string(),
            row.3,
            row.4,
            row.5,
            row.6,
            row.7,
        )?;
        let lease_expires_at = timestamp(lease_expires)?;
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        Ok(Some(RecoveryHandoffDeliveryAttempt {
            handoff,
            attempt_id,
            lease_expires_at,
        }))
    }

    pub fn acknowledge_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &RecoveryCustodyDeliveryReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        ensure_current_authority(&transaction, authority, now)?;
        if receipt.handoff_id != attempt.handoff.handoff_id()
            || receipt.correlation_id != attempt.handoff.correlation_id()
            || receipt.attempt_id != attempt.attempt_id
        {
            return Err(InviteRecoveryRepositoryError::HandoffConflict);
        }
        let changed = transaction
            .execute(
                "UPDATE account_identity_recovery_custody_handoff
                 SET state = 'delivered', lease_expires_at_epoch_millis = NULL,
                     active_attempt_id = NULL
                 WHERE handoff_id = ?1 AND household_id = ?2 AND state = 'in-flight'
                   AND active_attempt_id = ?3",
                params![
                    attempt.handoff.handoff_id(),
                    authority.household_id().to_string(),
                    attempt.attempt_id,
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::HandoffConflict);
        }
        let completed = transaction
            .execute(
                "UPDATE account_identity_recovery
                 SET state = 'completed', last_transition_at_epoch_millis = ?2
                 WHERE recovery_id = ?1 AND state = 'approved'",
                params![attempt.handoff.recovery_id().as_str(), now],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if completed != 1 {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected);
        }
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
    }

    pub fn release_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        ensure_current_authority(&transaction, authority, now)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_recovery_custody_handoff
                 SET state = 'pending', lease_expires_at_epoch_millis = NULL,
                     active_attempt_id = NULL
                 WHERE handoff_id = ?1 AND household_id = ?2 AND state = 'in-flight'
                   AND active_attempt_id = ?3",
                params![
                    attempt.handoff.handoff_id(),
                    authority.household_id().to_string(),
                    attempt.attempt_id,
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::HandoffConflict);
        }
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
    }
}
