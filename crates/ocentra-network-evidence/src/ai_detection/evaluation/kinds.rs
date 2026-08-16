use super::super::*;

pub(super) fn normalized_input_kinds(
    values: &[NetworkAiDetectionInputKind],
) -> Result<Vec<NetworkAiDetectionInputKind>, NetworkAiDetectionEvaluationError> {
    if values.is_empty() {
        return Err(NetworkAiDetectionEvaluationError::EmptyInputKinds);
    }
    let mut kinds = Vec::new();
    for value in values {
        if !kinds.contains(value) {
            kinds.push(*value);
        }
    }
    Ok(kinds)
}
