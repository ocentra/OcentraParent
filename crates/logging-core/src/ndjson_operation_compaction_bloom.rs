const BLOOM_WORDS_PER_SEGMENT: usize = 16 * 1024;
const BLOOM_BITS_PER_SEGMENT: usize = BLOOM_WORDS_PER_SEGMENT * u64::BITS as usize;
const MAX_KEYS_PER_SEGMENT: usize = 64 * 1024;

pub(crate) struct CommitBloom {
    segments: Vec<CommitBloomSegment>,
    saturated: bool,
}

struct CommitBloomSegment {
    words: Vec<u64>,
    key_count: usize,
}

impl Default for CommitBloom {
    fn default() -> Self {
        Self {
            segments: vec![CommitBloomSegment::default()],
            saturated: false,
        }
    }
}

impl Default for CommitBloomSegment {
    fn default() -> Self {
        Self {
            words: vec![0; BLOOM_WORDS_PER_SEGMENT],
            key_count: 0,
        }
    }
}

impl CommitBloom {
    pub(crate) fn clear(&mut self) {
        self.segments.clear();
        self.segments.push(CommitBloomSegment::default());
        self.saturated = false;
    }

    pub(crate) fn insert(&mut self, key: &str) {
        if self.saturated {
            return;
        }
        if self
            .segments
            .last()
            .is_some_and(|segment| segment.key_count >= MAX_KEYS_PER_SEGMENT)
        {
            self.saturated = true;
            return;
        }
        if let Some(segment) = self.segments.last_mut() {
            segment.insert(key);
        }
    }

    pub(crate) fn might_contain(&self, key: &str) -> bool {
        self.saturated
            || self
                .segments
                .iter()
                .any(|segment| segment.might_contain(key))
    }

    #[cfg(feature = "test-support")]
    pub(crate) fn segment_count(&self) -> usize {
        self.segments.len()
    }
}

#[cfg(feature = "test-support")]
pub(crate) fn segment_count_after_inserts(key_count: usize) -> usize {
    let mut bloom = CommitBloom::default();
    for key in 0..key_count {
        bloom.insert(&format!("segmented-key-{key}"));
    }
    bloom.segment_count()
}

impl CommitBloomSegment {
    fn insert(&mut self, key: &str) {
        for bit in bloom_bits(key) {
            self.words[bit / u64::BITS as usize] |= 1 << (bit % u64::BITS as usize);
        }
        self.key_count += 1;
    }

    fn might_contain(&self, key: &str) -> bool {
        bloom_bits(key).into_iter().all(|bit| {
            self.words[bit / u64::BITS as usize] & (1 << (bit % u64::BITS as usize)) != 0
        })
    }
}

fn bloom_bits(key: &str) -> [usize; 4] {
    use sha2::{Digest, Sha256};

    let digest = Sha256::digest(key.as_bytes());
    std::array::from_fn(|index| {
        let start = index * 4;
        let value = u32::from_le_bytes([
            digest[start],
            digest[start + 1],
            digest[start + 2],
            digest[start + 3],
        ]);
        value as usize % BLOOM_BITS_PER_SEGMENT
    })
}
