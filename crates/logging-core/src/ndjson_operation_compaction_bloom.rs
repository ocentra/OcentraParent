const BLOOM_WORDS: usize = 16 * 1024;
const BLOOM_BITS: usize = BLOOM_WORDS * u64::BITS as usize;

pub(crate) struct CommitBloom {
    words: Vec<u64>,
}

impl Default for CommitBloom {
    fn default() -> Self {
        Self {
            words: vec![0; BLOOM_WORDS],
        }
    }
}

impl CommitBloom {
    pub(crate) fn clear(&mut self) {
        self.words.fill(0);
    }

    pub(crate) fn insert(&mut self, key: &str) {
        for bit in bloom_bits(key) {
            self.words[bit / u64::BITS as usize] |= 1 << (bit % u64::BITS as usize);
        }
    }

    pub(crate) fn might_contain(&self, key: &str) -> bool {
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
        value as usize % BLOOM_BITS
    })
}
