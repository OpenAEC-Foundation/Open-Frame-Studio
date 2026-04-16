use super::CncPart;

pub trait CncPostProcessor {
    fn name(&self) -> &str;
    fn extension(&self) -> &str;
    fn generate(&self, parts: &[CncPart]) -> Result<Vec<(String, String)>, String>;
}
