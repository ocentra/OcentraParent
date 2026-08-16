pub(crate) fn is_line_comment(characters: &[char], index: usize) -> bool {
    starts_with(characters, index, '-', '-')
}

pub(crate) fn is_block_comment(characters: &[char], index: usize) -> bool {
    starts_with(characters, index, '/', '*')
}

pub(crate) fn skip_line_comment(characters: &[char], mut index: usize) -> usize {
    while index < characters.len() && !matches!(characters[index], '\r' | '\n') {
        index += 1;
    }
    index
}

pub(crate) fn skip_block_comment(characters: &[char], mut index: usize) -> Option<usize> {
    while index + 1 < characters.len() {
        if starts_with(characters, index, '*', '/') {
            return Some(index + 2);
        }
        index += 1;
    }
    None
}

fn starts_with(characters: &[char], index: usize, first: char, second: char) -> bool {
    characters.get(index) == Some(&first) && characters.get(index + 1) == Some(&second)
}
