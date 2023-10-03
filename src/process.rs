use crate::error::ProcessError;
use std::mem::size_of;
use windows::core::PWSTR;
use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE, MAX_PATH, STATUS_PENDING},
    System::ProcessStatus::EnumProcesses,
    System::Threading::{
        GetExitCodeProcess, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
    },
};

pub struct ProcessChecker {
    selected: Option<u32>,
}

impl ProcessChecker {
    pub fn new() -> Self {
        Self { selected: None }
    }

    pub fn check(&mut self, checklist: &[Vec<u16>]) -> bool {
        if let Some(pid) = self.selected {
            if !self.check_pid(pid, checklist) && !self.check_all(checklist) {
                self.selected = None;
                false
            } else {
                true
            }
        } else {
            self.check_all(checklist)
        }
    }

    fn check_pid(&mut self, pid: u32, checklist: &[Vec<u16>]) -> bool {
        unsafe {
            if let Ok(process) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                let result = self.check_process(pid, checklist, process);
                close_handle(process);
                result
            } else {
                last_error("OpenProcess");
                false
            }
        }
    }

    unsafe fn check_process(&mut self, pid: u32, checklist: &[Vec<u16>], process: HANDLE) -> bool {
        let mut exitcode = 0u32;
        if GetExitCodeProcess(process, &mut exitcode).is_err() {
            last_error("GetExitCodeProcess");
            return false;
        }

        if exitcode != STATUS_PENDING.0 as u32 {
            println!("Process exit code is not STATUS_PENDING: {}", exitcode);
            return false;
        }

        let mut buffer = [0u16; MAX_PATH as usize];
        let mut length = buffer.len() as u32;
        if QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut length,
        )
        .is_err()
        {
            last_error("QueryFullProcessImageNameW");
            return false;
        }

        let name = &buffer[..length as usize];
        if length != 0 && checklist.iter().any(|check| name.ends_with(check)) {
            self.selected = Some(pid);
            return true;
        }

        false
    }

    fn check_all(&mut self, checklist: &[Vec<u16>]) -> bool {
        let mut processes = Vec::with_capacity(1000);
        let size = (processes.capacity() * size_of::<u32>()) as u32;
        let mut needed = 0;

        if unsafe { EnumProcesses(processes.as_mut_ptr(), size, &mut needed).is_err() } {
            last_error("EnumProcesses");
            return false;
        }
        unsafe { processes.set_len(needed as usize / size_of::<u32>()) }

        processes.iter().any(|pid| self.check_pid(*pid, checklist))
    }
}

unsafe fn close_handle(handle: HANDLE) {
    if CloseHandle(handle).is_err() {
        last_error("CloseHandle");
    }
}

fn last_error(name: &'static str) {
    let error = from_last_error();
    eprintln!("{} Error: {}", name, error);
}

fn from_last_error() -> ProcessError {
    match unsafe { GetLastError() } {
        Err(error) => ProcessError::Win32Error(error.code()),
        Ok(_) => ProcessError::UnknownError,
    }
}
