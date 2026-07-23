use crate::parent_presence_store_sql_comments::{
    is_block_comment, is_line_comment, skip_block_comment, skip_line_comment,
};
use crate::parent_presence_store_sql_quoted::read_quoted;

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum SqlToken {
    Word(String),
    Literal(String),
    Symbol(char),
}

pub(crate) fn tokenize(sql: &str) -> Option<Vec<SqlToken>> {
    let characters = sql.chars().collect::<Vec<_>>();
    let mut tokens = Vec::new();
    let mut index = 0_usize;
    while index < characters.len() {
        let character = characters[index];
        if character.is_whitespace() {
            index += 1;
        } else if is_line_comment(&characters, index) {
            index = skip_line_comment(&characters, index + 2);
        } else if is_block_comment(&characters, index) {
            index = skip_block_comment(&characters, index + 2)?;
        } else if character == '\'' {
            let literal = read_quoted(&characters, &mut index, '\'', '\'')?;
            tokens.push(SqlToken::Literal(literal));
        } else if matches!(character, '"' | '`' | '[') {
            let closing = if character == '[' { ']' } else { character };
            let identifier = read_quoted(&characters, &mut index, character, closing)?;
            tokens.push(word(&identifier));
        } else if is_word_character(character) {
            tokens.push(read_word(&characters, &mut index));
        } else {
            tokens.push(SqlToken::Symbol(character));
            index += 1;
        }
    }
    Some(tokens)
}

fn read_word(characters: &[char], index: &mut usize) -> SqlToken {
    let start = *index;
    *index += 1;
    while *index < characters.len() && is_word_character(characters[*index]) {
        *index += 1;
    }
    word(&characters[start..*index].iter().collect::<String>())
}

fn is_word_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '_' | '$')
}

pub(crate) fn word(value: &str) -> SqlToken {
    SqlToken::Word(value.to_ascii_uppercase())
}
