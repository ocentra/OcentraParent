pub(crate) fn read_quoted(
    characters: &[char],
    index: &mut usize,
    opening: char,
    closing: char,
) -> Option<String> {
    if characters.get(*index) != Some(&opening) {
        return None;
    }
    *index += 1;
    let mut value = String::new();
    while *index < characters.len() {
        let character = characters[*index];
        if character == closing {
            if characters.get(*index + 1) == Some(&closing) {
                value.push(closing);
                *index += 2;
            } else {
                *index += 1;
                return Some(value);
            }
        } else {
            value.push(character);
            *index += 1;
        }
    }
    None
}
