use super::Response;

impl Response {
    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn target_generation(&self) -> u64 {
        self.target_generation
    }

    pub fn key_generation(&self) -> u64 {
        self.key_generation
    }

    pub fn writer_generation(&self) -> u64 {
        self.writer_generation
    }
}
