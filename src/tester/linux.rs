use super::{Tester, TraitTester};

use crate::error::Error;
use crate::output::Output;

use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus};

use nix::sys::signal::Signal;
use nix::unistd::Pid;

use libc::{c_int, rusage, wait4, WSTOPPED};

fn u32_to_i32(u: u32) -> Option<i32> {
    if u > i32::max_value() as u32 {
        None
    } else {
        Some(u as i32)
    }
}

impl TraitTester for Tester {
    fn run(&self) -> Result<Output, Error> {
        let child = Command::new(&self.path).spawn().map_err(Error::from)?;

        let pid = Some(child.id())
            .and_then(u32_to_i32)
            .ok_or_else(|| Error::new("Pid overflow"))
            .map(Pid::from_raw)?;

        let (time, memory, status) = unsafe {
            let mut status = std::mem::zeroed::<c_int>();
            let mut ru = std::mem::zeroed::<rusage>();

            if wait4(
                pid.as_raw(),
                &mut status as *mut c_int,
                WSTOPPED,
                &mut ru as *mut rusage,
            ) == -1
            {
                return Err(Error::new("fail to wait child process"));
            }

            let time: u64 = (ru.ru_utime.tv_sec * 1000 + ru.ru_utime.tv_usec / 1000).max(0) as u64;
            let memory: u64 = ru.ru_maxrss.max(0) as u64;
            let status = ExitStatus::from_raw(status);

            (time, memory, status)
        };

        let code: Option<i64> = status.code().map(i64::from);

        let signal: Option<String> = status
            .signal()
            .and_then(|s| Signal::from_c_int(s).ok())
            .map(|sig| format!("{}", sig));

        Ok(Output {
            code,
            signal,
            time,
            memory,
        })
    }
}
