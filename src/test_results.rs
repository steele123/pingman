pub struct TestResults {
    pub failed: Vec<FailedTest>,
    pub succeeded: Vec<SucceededTest>,
}

pub struct SucceededTest {
    pub ping: i32,
    pub ip: String,
}

pub struct FailedTest {
    pub ip: String,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            failed: Vec::new(),
            succeeded: Vec::new(),
        }
    }

    pub fn add_success(&mut self, url: &str, ping: i32) {
        self.succeeded.push(SucceededTest {
            ip: url.to_string(),
            ping,
        });
    }

    pub fn add_failure(&mut self, url: &str) {
        self.failed.push(FailedTest {
            ip: url.to_string(),
        });
    }
}
