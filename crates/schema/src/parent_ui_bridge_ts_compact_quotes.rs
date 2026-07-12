use super::{Mode, SourceChars};

pub(super) fn handle_single(
    ch: char,
    chars: &mut SourceChars<'_>,
    compacted: &mut String,
    mode: &mut Mode,
) {
    handle_quoted(ch, chars, compacted, mode, Mode::SingleQuote, '\'');
}

pub(super) fn handle_double(
    ch: char,
    chars: &mut SourceChars<'_>,
    compacted: &mut String,
    mode: &mut Mode,
) {
    handle_quoted(ch, chars, compacted, mode, Mode::DoubleQuote, '"');
}

pub(super) fn handle_backtick(
    ch: char,
    chars: &mut SourceChars<'_>,
    compacted: &mut String,
    mode: &mut Mode,
) {
    handle_quoted(ch, chars, compacted, mode, Mode::Backtick, '`');
}

fn handle_quoted(
    ch: char,
    chars: &mut SourceChars<'_>,
    compacted: &mut String,
    mode: &mut Mode,
    active_mode: Mode,
    delimiter: char,
) {
    compacted.push(ch);
    if ch == '\\' {
        if let Some(next) = chars.next() {
            compacted.push(next);
        }
    } else if *mode == active_mode && ch == delimiter {
        *mode = Mode::Normal;
    }
}
