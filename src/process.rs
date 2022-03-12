use crate::error::ProcessError;
use std::mem::size_of;
use windows::core::PWSTR;
use windows::Win32::{
    Foundation::{
        CloseHandle, GetLastError, ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER, HANDLE, MAX_PATH,
        STATUS_PENDING,
    },
    System::ProcessStatus::K32EnumProcesses,
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
            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);
            if process.0 == 0 {
                match GetLastError() {
                    ERROR_INVALID_PARAMETER => (), // ProcessError::InvalidParameter
                    ERROR_ACCESS_DENIED => (),     // ProcessError::AccessDenied
                    e => eprintln!("OpenProcess Error: {}", ProcessError::UnknownError(e)),
                }
                return false;
            }

            let result = self.check_process(pid, checklist, process);
            close_handle(process);

            result
        }
    }

    unsafe fn check_process(&mut self, pid: u32, checklist: &[Vec<u16>], process: HANDLE) -> bool {
        let mut exitcode = 0u32;
        if !GetExitCodeProcess(process, &mut exitcode).as_bool() {
            last_error("GetExitCodeProcess");
            return false;
        }

        if exitcode != STATUS_PENDING.0 as u32 {
            println!("Process exit code is not STATUS_PENDING: {}", exitcode);
            return false;
        }

        let mut buffer = [0u16; MAX_PATH as usize];
        let mut length = buffer.len() as u32;
        if !QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut length,
        )
        .as_bool()
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

        if unsafe { !K32EnumProcesses(processes.as_mut_ptr(), size, &mut needed).as_bool() } {
            last_error("EnumProcesses");
            return false;
        }
        unsafe { processes.set_len(needed as usize / size_of::<u32>()) }

        processes.iter().any(|pid| self.check_pid(*pid, checklist))
    }
}

unsafe fn close_handle(handle: HANDLE) {
    if !CloseHandle(handle).as_bool() {
        last_error("CloseHandle");
    }
}

fn last_error(name: &'static str) {
    let ecode = unsafe { GetLastError() };
    let error = ProcessError::UnknownError(ecode);
    eprintln!("{} Error: {}", name, error);
}
