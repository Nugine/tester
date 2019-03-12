use crate::error::Error;

use std::ffi::CString;
use std::fmt::Display;
use std::path::Path;

use nix::sys::signal::Signal;
use nix::unistd::{fork, ForkResult, Pid};

pub fn exec(path: &Path) -> Result<Pid, Error> {
    let p = path
        .to_str()
        .and_then(|s| CString::new(s).ok())
        .ok_or_else(|| Error::sys_error("ascii path required".into()))?;

    let r = fork().map_err(Error::nix_error)?;

    use ForkResult::*;
    match r {
        Parent { child } => Ok(child),
        Child => {
            let r = nix::unistd::execve(&p, &[], &[]);

            if r.is_err() {
                eprintln!("fail to execute path: {:?}", p);
                std::process::exit(127);
            } else {
                std::process::exit(0);
            }
        }
    }
}

#[derive(Debug)]
pub struct WaitOutput {
    pub code: i32,
    pub signal: Option<Signal>,
    pub time: u64,     // in ms
    pub memory: usize, // in kb
}

impl Display for WaitOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "code: {}", self.code)?;

        if let Some(sig) = self.signal {
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

pub fn wait(pid: Pid) -> Result<WaitOutput, Error> {
    use libc::{c_int, rusage, WSTOPPED};

    unsafe {
        let mut status = std::mem::uninitialized::<c_int>();
        let mut ru = std::mem::uninitialized::<rusage>();

        let p = libc::wait4(
            pid.as_raw(),
            &mut status as *mut c_int,
            WSTOPPED,
            &mut ru as *mut rusage,
        );

        let time = (ru.ru_stime.tv_sec * 1000
            + ru.ru_stime.tv_usec / 1000
            + ru.ru_utime.tv_sec * 1000
            + ru.ru_utime.tv_usec / 1000)
            .max(0) as u64;

        let memory = ru.ru_maxrss as usize;

        if p == -1 {
            return Err(Error::sys_error("fail to wait child process".into()));
        }

        let code = libc::WEXITSTATUS(status);

        let signal: Option<Signal> = if libc::WIFSIGNALED(status) {
            Signal::from_c_int(libc::WTERMSIG(status)).ok()
        } else {
            None
        };

        Ok(WaitOutput {
            code,
            signal,
            time,
            memory,
        })
    }
}
