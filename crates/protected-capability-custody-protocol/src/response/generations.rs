use super::ObservedGenerations;

impl ObservedGenerations {
    pub fn authority(self) -> u64 {
        self.authority
    }

    pub fn target(self) -> u64 {
        self.target
    }

    pub fn key(self) -> u64 {
        self.key
    }

    pub fn writer(self) -> u64 {
        self.writer
    }
}
