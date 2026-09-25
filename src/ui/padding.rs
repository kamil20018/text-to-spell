pub struct RelativePadding {
    pub top: f32,
    pub botton: f32,
    pub left: f32,
    pub right: f32,
    pub columns: f32,
    pub rows: f32,
}

impl Default for RelativePadding {
    fn default() -> Self {
        Self {
            top: 0.0,
            botton: 0.0,
            left: 0.0,
            right: 0.0,
            columns: 0.0,
            rows: 0.0,
        }
    }
}
