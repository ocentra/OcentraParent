use super::{Mode, SourceChars};

pub(super) fn handle(
    ch: char,
    chars: &mut SourceChars<'_>,
    compacted: &mut String,
    mode: &mut Mode,
    pending_space: &mut bool,
) {
    if let Some(quote_mode) = quote_mode(ch) {
        push_pending_space(compacted, pending_space);
        compacted.push(ch);
        *mode = quote_mode;
        return;
    }

    if ch == '/' && chars.peek() == Some(&'/') {
        chars.next();
        *pending_space = true;
        *mode = Mode::LineComment;
        return;
    }

    if ch == '/' && chars.peek() == Some(&'*') {
        chars.next();
        *pending_space = true;
        *mode = Mode::BlockComment;
        return;
    }

    if ch.is_whitespace() {
        *pending_space = true;
        return;
    }

    push_pending_space(compacted, pending_space);
    compacted.push(ch);
}

fn quote_mode(ch: char) -> Option<Mode> {
    match ch {
        '\'' => Some(Mode::SingleQuote),
        '"' => Some(Mode::DoubleQuote),
        '`' => Some(Mode::Backtick),
        _ => None,
    }
}

fn push_pending_space(compacted: &mut String, pending_space: &mut bool) {
    if *pending_space {
        compacted.push(' ');
        *pending_space = false;
    }
}
