use super::*;

pub(super) fn prepare(
    session: &mut MutationSession<'_>,
    request_id: &str,
    relative_path: &str,
    payload: &[u8],
    descriptor: String,
) -> Result<AppendPreparation, ArtifactError> {
    let (chain, target_path, leaf) = parent_and_leaf(&session.owner.root_path, relative_path)?;
    session.verify_chain(&chain)?;
    let target = match OwnedFile::open_existing_mutation_file(&target_path) {
        Ok(file) => Some(file),
        Err(ArtifactError::NotFound) => None,
        Err(error) => return Err(error),
    };
    let (prior_length, target_identity) = existing_target(target.as_ref())?;
    let payload_length = u64::try_from(payload.len()).map_err(|_| ArtifactError::SizeLimit)?;
    let created = target.is_none();
    let intent = IntentRecord::Append {
        schema: 1,
        request_id: request_id.to_owned(),
        relative_path: relative_path.to_owned(),
        descriptor: descriptor.clone(),
        payload_digest: payload_digest(payload),
        payload_length,
        prior_length,
        created,
        target_identity,
        temp_name: None,
        phase: AppendPhase::Prepared,
    };
    let mut preparation = AppendPreparation {
        chain,
        target_path,
        leaf,
        target,
        prior_length,
        target_identity,
        created,
        intent,
        descriptor,
        payload_length,
    };
    if preparation.created {
        super::create::create(session, request_id, &mut preparation)?;
    } else {
        write_intent(
            &session.owner.root_path,
            request_id,
            &preparation.intent,
            session.metadata.intent_directory()?,
        )?;
    }
    Ok(preparation)
}

fn existing_target(
    target: Option<&OwnedFile>,
) -> Result<(u64, Option<IdentityRecord>), ArtifactError> {
    match target {
        Some(file) => {
            let metadata = verify_metadata(file, false)?;
            Ok((metadata.length, Some(identity_record(metadata.identity))))
        }
        None => Ok((0, None)),
    }
}
