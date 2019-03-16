use std::fmt::Display;

#[derive(Debug)]
pub struct Output {
    pub code: Option<i64>,
    pub signal: Option<String>,
    pub time: u64,   // in ms
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

        writeln!(f, "time: {} ms", self.time)?;

        if self.memory > 4096 {
            write!(f, "memory: {} MB", (self.memory as f64) / 1024.0)
        } else {
            write!(f, "memory: {} KB", self.memory)
        }
    }
}
