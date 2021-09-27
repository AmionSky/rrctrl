use bindings::Windows::Win32::{
    Foundation::{CloseHandle, MAX_PATH, PWSTR},
    System::ProcessStatus::{K32EnumProcesses, K32GetProcessImageFileNameW},
    System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
};
use std::mem::size_of;

pub struct ProcessChecker {
    pub checklist: Vec<String>,
    selected: Option<u32>,
}

impl ProcessChecker {
    pub fn new() -> Self {
        Self {
            checklist: Vec::new(),
            selected: None,
        }
    }

    pub fn check(&mut self) -> bool {
        if let Some(pid) = self.selected {
            let result = self.check_pid(pid);
            if !result {
                self.selected = None;
            }
            result
        } else {
            self.check_all()
        }
    }

    fn check_pid(&mut self, pid: u32) -> bool {
        unsafe {
            let mut buffer = [0u16; MAX_PATH as usize];

            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);
            let length = K32GetProcessImageFileNameW(
                process,
                PWSTR(buffer.as_mut_ptr()),
                buffer.len() as u32,
            ) as usize;
            CloseHandle(process);

            if length != 0 {
                let name = String::from_utf16_lossy(&buffer[..length]);
                if self.checklist.iter().any(|check| name.ends_with(check)) {
                    self.selected = Some(pid);
                    return true;
                }
            }
        }

        false
    }

    fn check_all(&mut self) -> bool {
        let mut processes = Vec::with_capacity(1000);
        let size = (processes.capacity() * size_of::<u32>()) as u32;
        let mut needed = 0;

        if unsafe { !K32EnumProcesses(processes.as_mut_ptr(), size, &mut needed).as_bool() } {
            return false;
        }
        unsafe { processes.set_len(needed as usize / size_of::<u32>()) }

        processes.iter().any(|pid| self.check_pid(*pid))
    }
}
