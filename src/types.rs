use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ExecuteData {
    pub language: String,
    pub code: String,
    pub input: Option<String>,
    pub time_limit: Option<u64>,
    pub memory_limit: Option<u64>,
    pub wall_time_limit: Option<u64>,
}

pub struct Limit {
    pub memory: Option<u64>,
    pub time_limit: Option<u64>,
    pub walltime_limit: Option<u64>,
}

#[derive(Serialize)]
pub enum RunStatus {
    #[serde(rename = "success")]
    Success,

    #[serde(rename = "ce")]
    CompileError,

    #[serde(rename = "tle")]
    TimeLimitExceeded,

    #[serde(rename = "system_error")]
    SystemError(String),

    #[serde(rename = "security_violation")]
    SecurityViolation,

    #[serde(rename = "unknown_error")]
    UnknownError(String),
}

#[derive(Serialize)]
pub struct RunOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub runtime: u128,
    pub memory_usage: i64,
    pub status: RunStatus,
}

impl RunOutput {
    pub fn error(reason: String, stderr: Option<Vec<u8>>, stdout: Option<Vec<u8>>) -> Self {
        Self {
            stdout: stdout.unwrap_or(Vec::new()),
            stderr: stderr.unwrap_or(Vec::new()),
            runtime: 0,
            memory_usage: 0,
            status: RunStatus::SystemError(reason),
        }
    }
}
