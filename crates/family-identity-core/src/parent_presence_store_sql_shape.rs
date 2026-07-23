use crate::parent_presence_store_sql_tokenizer::{tokenize, word, SqlToken};

pub(crate) fn challenge_lifecycle_column_is_canonical(sql: &str) -> bool {
    definition_matches(
        sql,
        "LIFECYCLE_STATE",
        &[
            word("LIFECYCLE_STATE"),
            word("TEXT"),
            word("NOT"),
            word("NULL"),
            word("CHECK"),
            SqlToken::Symbol('('),
            word("LIFECYCLE_STATE"),
            word("IN"),
            SqlToken::Symbol('('),
            SqlToken::Literal("issued".to_owned()),
            SqlToken::Symbol(','),
            SqlToken::Literal("consumed".to_owned()),
            SqlToken::Symbol(')'),
            SqlToken::Symbol(')'),
        ],
    )
}

pub(crate) fn receipt_sequence_column_is_canonical(sql: &str) -> bool {
    definition_matches(
        sql,
        "RECEIPT_SEQUENCE",
        &[
            word("RECEIPT_SEQUENCE"),
            word("INTEGER"),
            word("PRIMARY"),
            word("KEY"),
            word("AUTOINCREMENT"),
        ],
    )
}

pub(crate) fn decision_delivery_column_is_canonical(sql: &str) -> bool {
    definition_matches(
        sql,
        "DELIVERY_STATE",
        &[
            word("DELIVERY_STATE"),
            word("TEXT"),
            word("NOT"),
            word("NULL"),
            word("CHECK"),
            SqlToken::Symbol('('),
            word("DELIVERY_STATE"),
            word("IN"),
            SqlToken::Symbol('('),
            SqlToken::Literal("pending".to_owned()),
            SqlToken::Symbol(','),
            SqlToken::Literal("delivered".to_owned()),
            SqlToken::Symbol(')'),
            SqlToken::Symbol(')'),
        ],
    )
}

fn definition_matches(sql: &str, column_name: &str, expected: &[SqlToken]) -> bool {
    tokenize(sql)
        .and_then(|tokens| column_definition(&tokens, column_name))
        .is_some_and(|definition| definition == expected)
}

fn column_definition(tokens: &[SqlToken], column_name: &str) -> Option<Vec<SqlToken>> {
    let body_start = tokens
        .iter()
        .position(|token| *token == SqlToken::Symbol('('))?
        + 1;
    definition_ranges(tokens, body_start)
        .into_iter()
        .find_map(|(start, end)| matching_definition(&tokens[start..end], column_name))
}

fn definition_ranges(tokens: &[SqlToken], body_start: usize) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut depth = 0_u32;
    let mut definition_start = body_start;
    for (index, token) in tokens.iter().enumerate().skip(body_start) {
        match token {
            SqlToken::Symbol('(') => depth += 1,
            SqlToken::Symbol(')') if depth == 0 => {
                ranges.push((definition_start, index));
                break;
            }
            SqlToken::Symbol(')') => depth -= 1,
            SqlToken::Symbol(',') if depth == 0 => {
                ranges.push((definition_start, index));
                definition_start = index + 1;
            }
            _ => {}
        }
    }
    ranges
}

fn matching_definition(tokens: &[SqlToken], column_name: &str) -> Option<Vec<SqlToken>> {
    match tokens.first() {
        Some(SqlToken::Word(name)) if name == column_name => Some(tokens.to_vec()),
        _ => None,
    }
}
