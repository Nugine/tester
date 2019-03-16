#[cfg(unix)]
mod linux;

#[cfg(windows)]
mod windows;

use crate::error::Error;
use crate::output::Output;

use std::path::PathBuf;

pub trait TraitTester {
    fn run(&self) -> Result<Output, Error>;
}

pub struct Tester {
    path: PathBuf,
    args: Vec<String>,
}

impl Tester {
    pub fn new(path: PathBuf, args: Vec<String>) -> Self {
        Self { path, args }
    }
}
