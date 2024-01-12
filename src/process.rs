use crate::error::WinError;
use std::mem::size_of;
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER, HANDLE, MAX_PATH, STATUS_PENDING,
};
use windows_sys::Win32::System::ProcessStatus::EnumProcesses;
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
    PROCESS_QUERY_LIMITED_INFORMATION,
};

pub struct ProcessChecker {
    monitored: Option<u32>,
    buffer: [u16; MAX_PATH as usize],
}

impl ProcessChecker {
    pub fn new() -> Self {
        Self {
            monitored: None,
            buffer: [0u16; MAX_PATH as usize],
        }
    }

    pub fn check(&mut self, checklist: &[Vec<u16>]) -> bool {
        if let Some(pid) = self.monitored {
            if self.check_pid(pid, checklist) {
                return true;
            } else {
                self.monitored = None;
            }
        }

        self.check_all(checklist)
    }

    fn check_all(&mut self, checklist: &[Vec<u16>]) -> bool {
        let mut processes = Vec::with_capacity(4096);
        let size = (processes.capacity() * size_of::<u32>()) as u32;
        let mut needed = 0;

        if unsafe { EnumProcesses(processes.as_mut_ptr(), size, &mut needed) } == 0 {
            eprint_error("EnumProcesses", WinError::last());
            return false;
        }

        unsafe { processes.set_len(needed as usize / size_of::<u32>()) }
        processes.iter().any(|pid| self.check_pid(*pid, checklist))
    }

    fn check_pid(&mut self, pid: u32, checklist: &[Vec<u16>]) -> bool {
        match unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) } {
            0 => {
                let error = WinError::last();
                match error.code() {
                    ERROR_INVALID_PARAMETER | ERROR_ACCESS_DENIED => (), // Ignore these errors
                    _ => eprint_error("OpenProcess", error),
                }
                false // return
            }
            process => {
                let result = self.check_process(pid, checklist, process);
                close_handle(process);
                result // return
            }
        }
    }

    fn check_process(&mut self, pid: u32, checklist: &[Vec<u16>], process: HANDLE) -> bool {
        // Check if the process is still running
        if !is_process_running(process) {
            return false;
        }

        // Reset buffer
        self.buffer.fill(0);
        let mut length = self.buffer.len() as u32;

        // Get the process name
        if unsafe {
            QueryFullProcessImageNameW(
                process,
                PROCESS_NAME_WIN32,
                self.buffer.as_mut_ptr(),
                &mut length,
            )
        } == 0
        {
            eprint_error("QueryFullProcessImageName", WinError::last());
            return false;
        }

        // Check if the name matches with the checklist
        let name = &self.buffer[..length as usize];
        if !name.is_empty() && checklist.iter().any(|check| name.ends_with(check)) {
            self.monitored = Some(pid);
            return true;
        }

        false // If no match was found
    }
}

fn eprint_error(name: &'static str, error: WinError) {
    eprintln!("{name} error: {error}");
}

fn close_handle(handle: HANDLE) {
    if unsafe { CloseHandle(handle) } == 0 {
        eprint_error("CloseHandle", WinError::last());
    }
}

/// Checks if the process is still running
fn is_process_running(process: HANDLE) -> bool {
    let mut exitcode = 0u32;

    // Get the exit code
    if unsafe { GetExitCodeProcess(process, &mut exitcode) } == 0 {
        eprint_error("GetExitCodeProcess", WinError::last());
        return false;
    }

    // Is the process pending
    exitcode == STATUS_PENDING as u32
}
