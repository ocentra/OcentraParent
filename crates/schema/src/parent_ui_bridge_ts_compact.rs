use std::iter::Peekable;
use std::str::Chars;

#[path = "parent_ui_bridge_ts_compact_comments.rs"]
mod comments;
#[path = "parent_ui_bridge_ts_compact_normal.rs"]
mod normal;
#[path = "parent_ui_bridge_ts_compact_quotes.rs"]
mod quotes;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Mode {
    Normal,
    SingleQuote,
    DoubleQuote,
    Backtick,
    LineComment,
    BlockComment,
}

pub(super) fn compact_generated_typescript(value: String) -> String {
    let mut compacted = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    let mut mode = Mode::Normal;
    let mut pending_space = false;

    while let Some(ch) = chars.next() {
        match mode {
            Mode::Normal => normal::handle(
                ch,
                &mut chars,
                &mut compacted,
                &mut mode,
                &mut pending_space,
            ),
            Mode::SingleQuote => quotes::handle_single(ch, &mut chars, &mut compacted, &mut mode),
            Mode::DoubleQuote => quotes::handle_double(ch, &mut chars, &mut compacted, &mut mode),
            Mode::Backtick => quotes::handle_backtick(ch, &mut chars, &mut compacted, &mut mode),
            Mode::LineComment => comments::handle_line(ch, &mut mode, &mut pending_space),
            Mode::BlockComment => {
                comments::handle_block(ch, &mut chars, &mut mode, &mut pending_space)
            }
        }
    }

    compacted.trim().to_owned()
}

pub(super) type SourceChars<'a> = Peekable<Chars<'a>>;
