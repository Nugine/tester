use crate::error::Error;

use std::ffi::CString;
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
            let _ = nix::unistd::execve(&p, &[], &[]);
            std::process::exit(0);
        }
    }
}

pub struct WaitOutput {
    pub code: i32,
    pub signal: Option<Signal>,
    pub time: u64,
    pub memory: usize,
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

        let memory = (ru.ru_maxrss as usize) * 1024;

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
