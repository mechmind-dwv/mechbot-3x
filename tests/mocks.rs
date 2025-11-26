// tests/mocks.rs
pub struct MockLidar {
    data: Vec<Point>,
}

impl MockLidar {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn with_sample_data() -> Self {
        Self {
            data: vec![Point::new(1.0, 0.0), Point::new(2.0, 0.0)],
        }
    }
}

#[async_trait]
impl Lidar for MockLidar {
    async fn scan(&mut self) -> Result<Vec<Point>> {
        Ok(self.data.clone())
    }
}
