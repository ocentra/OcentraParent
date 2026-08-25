use super::AiExplanationSection;
use crate::ai_contracts::AiSafeText;

impl AiExplanationSection {
    pub(crate) fn new(
        heading: AiSafeText,
        body: AiSafeText,
        citations: Vec<super::AiExplanationCitation>,
    ) -> Result<Self, &'static str> {
        if citations.is_empty() {
            return Err("AI explanation section requires grounded citations");
        }
        Ok(Self {
            heading,
            body,
            citations,
        })
    }

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
