use super::{Tester, TraitTester};

use crate::error::Error;
use crate::output::{Output, Time};

use std::os::windows::io::AsRawHandle;
use std::os::windows::io::RawHandle;
use std::process::Command;

use winapi::shared::minwindef::FILETIME;
use winapi::shared::ntdef::ULARGE_INTEGER;
use winapi::um::processthreadsapi::GetProcessTimes;
use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};

unsafe fn to_u64(ft: FILETIME) -> u64 {
    let mut ui = std::mem::zeroed::<ULARGE_INTEGER>();
    ui.s_mut().LowPart = ft.dwLowDateTime;
    ui.s_mut().HighPart = ft.dwHighDateTime;
    *ui.QuadPart()
}

unsafe fn get_time(h_process: RawHandle) -> Result<Time, Error> {
    let mut creation_time = std::mem::zeroed::<FILETIME>();
    let mut exit_time = std::mem::zeroed::<FILETIME>();
    let mut kernel_time = std::mem::zeroed::<FILETIME>();
    let mut user_time = std::mem::zeroed::<FILETIME>();

    if GetProcessTimes(
        h_process,
        &mut creation_time,
        &mut exit_time,
        &mut kernel_time,
        &mut user_time,
    ) == 0
    {
        return Err(Error::new("fail to get time of process"));
    }

    // 1ms = 10000 * 100ns
    Ok(Time {
        real: to_u64(exit_time).saturating_sub(to_u64(creation_time)) / 10000,
        user: to_u64(user_time) / 10000,
        sys: to_u64(kernel_time) / 10000,
    })
}

unsafe fn get_memory(h_process: RawHandle) -> Result<u64, Error> {
    let mut pmc = std::mem::zeroed::<PROCESS_MEMORY_COUNTERS>();
    let cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;

    if GetProcessMemoryInfo(h_process, &mut pmc, cb) == 0 {
        return Err(Error::new("fail to get memory info of process"));
    }

    Ok((pmc.PeakWorkingSetSize as u64) / 1024)
}

impl TraitTester for Tester {
    fn run(&self) -> Result<Output, Error> {
        let mut child = Command::new(&self.target)
            .args(&self.args)
            .spawn()
            .map_err(Error::from)?;
        let status = child.wait().map_err(Error::from)?;

        let code: Option<i64> = status.code().map(i64::from);

        let handle = child.as_raw_handle();
        let time = unsafe { get_time(handle)? };
        let memory = unsafe { get_memory(handle)? };

        Ok(Output {
            code,
            signal: None,
            time,
            memory,
        })
    }
}
