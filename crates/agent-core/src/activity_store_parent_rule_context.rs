use ocentra_parent_agent_protocol::activity::policy_context::LocalAiParentRuleContextRef;
use ocentra_parent_agent_protocol::constants;
use rusqlite::{params, Connection};

use crate::ActivityStoreError;

pub(crate) fn replace_parent_rule_contexts(
    connection: &Connection,
    contexts: &[LocalAiParentRuleContextRef],
) -> Result<(), ActivityStoreError> {
    connection.execute(constants::sqlite::DELETE_PARENT_RULE_CONTEXTS, [])?;
    for context in contexts {
        upsert_parent_rule_context(connection, context)?;
    }
    Ok(())
}

pub(crate) fn parent_rule_contexts(
    connection: &Connection,
) -> Result<Vec<LocalAiParentRuleContextRef>, ActivityStoreError> {
    let mut statement = connection.prepare(constants::sqlite::SELECT_PARENT_RULE_CONTEXTS)?;
    let rows = statement.query_map([], |row| row.get::<_, String>(0))?;

    let mut contexts = Vec::new();
    for row in rows {
        contexts.push(serde_json::from_str::<LocalAiParentRuleContextRef>(&row?)?);
    }
    Ok(contexts)
}

fn upsert_parent_rule_context(
    connection: &Connection,
    context: &LocalAiParentRuleContextRef,
) -> Result<(), ActivityStoreError> {
    let context_json = serde_json::to_string(context)?;
    connection.execute(
        constants::sqlite::UPSERT_PARENT_RULE_CONTEXT,
        params![
            &context.parent_rule_ref_id,
            &context.updated_at,
            context.expires_at.as_deref(),
            context_json
        ],
    )?;
    Ok(())
}
