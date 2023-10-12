use std::mem::size_of;
use windows::core::{HRESULT, PWSTR};
use windows::Win32::{
    Foundation::{
        CloseHandle, ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER, HANDLE, MAX_PATH, STATUS_PENDING,
    },
    System::ProcessStatus::EnumProcesses,
    System::Threading::{
        GetExitCodeProcess, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
    },
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
        let mut processes = Vec::with_capacity(4000);
        let size = (processes.capacity() * size_of::<u32>()) as u32;
        let mut needed = 0;

        if let Err(error) = unsafe { EnumProcesses(processes.as_mut_ptr(), size, &mut needed) } {
            eprint_win32("EnumProcesses", error);
            return false;
        }

        unsafe { processes.set_len(needed as usize / size_of::<u32>()) }
        processes.iter().any(|pid| self.check_pid(*pid, checklist))
    }

    fn check_pid(&mut self, pid: u32, checklist: &[Vec<u16>]) -> bool {
        match unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) } {
            Ok(process) => {
                let result = self.check_process(pid, checklist, process);
                close_handle(process);
                result // return
            }
            Err(error) => {
                const EIP: HRESULT = ERROR_INVALID_PARAMETER.to_hresult();
                const EAD: HRESULT = ERROR_ACCESS_DENIED.to_hresult();

                match error.code() {
                    EIP | EAD => (),
                    _ => eprint_win32("OpenProcess", error),
                }

                false // return
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
        if let Err(error) = unsafe {
            QueryFullProcessImageNameW(
                process,
                PROCESS_NAME_WIN32,
                PWSTR(self.buffer.as_mut_ptr()),
                &mut length,
            )
        } {
            eprint_win32("QueryFullProcessImageName", error);
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

fn eprint_win32(name: &'static str, error: windows::core::Error) {
    eprintln!("{} error ({}): {}", name, error.code(), error.message())
}

fn close_handle(handle: HANDLE) {
    if let Err(error) = unsafe { CloseHandle(handle) } {
        eprint_win32("CloseHandle", error);
    }
}

/// Checks if the process is still running
fn is_process_running(process: HANDLE) -> bool {
    let mut exitcode = 0u32;

    // Get the exit code
    if let Err(error) = unsafe { GetExitCodeProcess(process, &mut exitcode) } {
        eprint_win32("GetExitCodeProcess", error);
        return false;
    }

    // Is the process pending
    exitcode == STATUS_PENDING.0 as u32
}
