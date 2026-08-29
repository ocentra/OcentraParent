use std::time::Duration;

pub(crate) struct PassiveDiscoveryCycleCursor {
    next: usize,
    len: usize,
}

impl PassiveDiscoveryCycleCursor {
    pub(crate) fn new(next: usize, len: usize) -> Self {
        Self {
            next: next % len.max(1),
            len,
        }
    }

    pub(crate) fn take_next(&mut self) -> Option<usize> {
        if self.len == 0 {
            return None;
        }
        let index = self.next;
        self.next = (index + 1) % self.len;
        Some(index)
    }

    pub(crate) fn resume_index(&self) -> usize {
        self.next
    }

    pub(crate) fn should_continue(
        running: bool,
        received: usize,
        max: usize,
        remaining: Duration,
    ) -> bool {
        running && received < max && !remaining.is_zero()
    }
}
