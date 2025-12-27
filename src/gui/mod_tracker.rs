pub struct ModTracker {
    spans: Vec<(f32, f32)>,
}

impl ModTracker {
    pub fn new() -> Self {
        Self { spans: vec![] }
    }

    pub fn add(&mut self, start: f32, end: f32) {
        self.spans.push((start, end));
    }

    pub fn count_at_beat(&self, beat: f32) -> usize {
        self.spans
            .iter()
            .filter(|s| s.0 <= beat && beat < s.1)
            .count()
    }
}
