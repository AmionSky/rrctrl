use crate::error::WinError;
use crate::wstring::WString;
use std::mem::size_of;
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER, GetLastError, HANDLE, MAX_PATH,
    STATUS_PENDING,
};
use windows_sys::Win32::System::ProcessStatus::EnumProcesses;
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
    QueryFullProcessImageNameW,
};

const PROCESSES_LEN: usize = 1024;

pub struct ProcessChecker {
    monitored: Option<u32>,
    processes: [u32; PROCESSES_LEN],
    buffer: [u16; MAX_PATH as usize],
}

impl ProcessChecker {
    pub fn new() -> Self {
        Self {
            monitored: None,
            processes: [0; PROCESSES_LEN],
            buffer: [0u16; MAX_PATH as usize],
        }
    }

    pub fn check(&mut self, checklist: &[WString]) -> bool {
        if let Some(pid) = self.monitored {
            if self.check_pid(pid, checklist) {
                return true;
            } else {
                self.monitored = None;
            }
        }

        self.check_all(checklist)
    }

    fn check_all(&mut self, checklist: &[WString]) -> bool {
        const SIZE: u32 = (PROCESSES_LEN * size_of::<u32>()) as u32;
        let mut needed = 0;

        if unsafe { EnumProcesses(self.processes.as_mut_ptr(), SIZE, &mut needed) } == 0 {
            eprint_error("EnumProcesses", WinError::last());
            return false;
        }

        let len = needed as usize / size_of::<u32>();

        for i in 0..len {
            if self.check_pid(self.processes[i], checklist) {
                return true;
            }
        }

        false // No matching process
    }

    fn check_pid(&mut self, pid: u32, checklist: &[WString]) -> bool {
        let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };

        if process.is_null() {
            match unsafe { GetLastError() } {
                ERROR_INVALID_PARAMETER | ERROR_ACCESS_DENIED => (), // Ignore these errors
                _ => eprint_error("OpenProcess", WinError::last()),
            }
            return false;
        }

        let result = self.check_process(pid, checklist, process);
        close_handle(process);
        result // return
    }

    fn check_process(&mut self, pid: u32, checklist: &[WString], process: HANDLE) -> bool {
        // Check if the process is still running
        if !is_process_running(process) {
            return false;
        }

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
        if !name.is_empty() && checklist.iter().any(|check| name.ends_with(check.value())) {
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
