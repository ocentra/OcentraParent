use super::{Mode, SourceChars};

pub(super) fn handle_line(ch: char, mode: &mut Mode, pending_space: &mut bool) {
    if ch == '\n' {
        *mode = Mode::Normal;
        *pending_space = true;
    }
}

pub(super) fn handle_block(
    ch: char,
    chars: &mut SourceChars<'_>,
    mode: &mut Mode,
    pending_space: &mut bool,
) {
    if ch == '*' && chars.peek() == Some(&'/') {
        chars.next();
        *mode = Mode::Normal;
        *pending_space = true;
    }
}
