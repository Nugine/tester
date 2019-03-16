#[cfg(unix)]
mod linux;

#[cfg(windows)]
mod windows;

use crate::error::Error;
use crate::output::Output;

use std::path::{Path, PathBuf};

pub trait TraitTester {
    fn run(&self) -> Result<Output, Error>;
}

pub struct Tester {
    path: PathBuf,
}

impl Tester {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
}
