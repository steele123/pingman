use serde::Serialize;

#[derive(Serialize)]
pub struct TestResults {
    pub site: String,
    pub failed: Vec<FailedTest>,
    pub succeeded: Vec<SucceededTest>,
}

#[derive(Serialize)]
pub struct SucceededTest {
    pub average_ping: i32,
    pub ip: String,
    pub port: i32,
}

#[derive(Serialize)]
pub struct FailedTest {
    pub reason: String,
    pub ip: String,
    pub port: i32,
}

impl TestResults {
    pub fn new(site: String) -> Self {
        Self {
            site,
            failed: Vec::new(),
            succeeded: Vec::new(),
        }
    }

    pub fn add_success(&mut self, url: &str, port: i32, average_ping: i32) {
        self.succeeded.push(SucceededTest {
            ip: url.to_string(),
            port,
            average_ping,
        });
    }

    pub fn add_failure(&mut self, url: &str, port: i32, reason: String) {
        self.failed.push(FailedTest {
            ip: url.to_string(),
            port,
            reason,
        });
    }
}
