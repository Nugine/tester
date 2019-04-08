use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Time {
    pub real: u64,
    pub user: u64,
    pub sys: u64,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub code: Option<i64>,
    pub signal: Option<String>,
    pub time: Time,  // in ms
    pub memory: u64, // in kb
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(code) = &self.code {
            writeln!(f, "code: {}", code)?;
        }

        if let Some(sig) = &self.signal {
            writeln!(f, "signal: {}", sig)?;
        }

        writeln!(f, "real time: {} ms", self.time.real)?;
        writeln!(f, "user time: {} ms", self.time.user)?;
        writeln!(f, "sys time: {} ms", self.time.sys)?;

        if self.memory > 4096 {
            write!(f, "memory: {:.3} MB", (self.memory as f64) / 1024.0)
        } else {
            write!(f, "memory: {} KB", self.memory)
        }
    }
}
