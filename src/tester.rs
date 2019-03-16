#[cfg(unix)]
mod linux;

#[cfg(windows)]
mod windows;

use crate::error::Error;
use crate::output::Output;

use std::ffi::OsString;

pub trait TraitTester {
    fn run(&self) -> Result<Output, Error>;
}

pub struct Tester {
    target: OsString,
    args: Vec<OsString>,
}

impl Tester {
    pub fn new(target: OsString, args: Vec<OsString>) -> Self {
        Self { target, args }
    }
}
