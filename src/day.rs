pub struct Solution {
    pub name: &'static str,
    pub run: fn(&str) -> Result<(), Box<dyn std::error::Error>>,
}
