use super::AiExplanationSection;
use crate::ai_contracts::AiSafeText;

impl AiExplanationSection {
    pub fn heading(&self) -> &AiSafeText {
        &self.heading
    }

    pub fn body(&self) -> &AiSafeText {
        &self.body
    }

    pub fn citations(&self) -> &[super::AiExplanationCitation] {
        &self.citations
    }
}
