use sysinfo::{RefreshKind, System, SystemExt, ProcessExt};

pub struct ProcessChecker {
    system: System,
    pub checklist: Vec<String>,
    selected: Option<usize>,
}

impl ProcessChecker {
    pub fn new()->Self {
        Self {
            system: System::new_with_specifics(RefreshKind::new().with_processes()),
            checklist: Vec::new(),
            selected: None,
        }
    }

    pub fn check(&mut self)->bool {
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

    fn selected_pid(&self) -> usize {
        self.selected.expect("selected PID was None")
    }

    fn check_pid(&mut self, pid: usize)->bool {
        if self.system.refresh_process(pid) {
            if let Some(process) = self.system.process(pid) {
                return self.checklist.iter().any(|a| a == process.name());
            }
        }

        false
    }

    fn check_all(&mut self)->bool {
        self.system.refresh_processes();

        for (pid,process) in self.system.processes() {
            if self.checklist.iter().any(|a| a == process.name()) {
                println!("FOUND: PID: {}, NAME: {}",pid,process.name());
                self.selected = Some(*pid);
                return true;
            }
        }

        false
    }
}

pub fn asd() {
    let mut system = System::new_with_specifics(RefreshKind::new().with_processes());
    system.refresh_processes();
    let theprocess = system.process_by_name("notepad");
    println!("IS RUNNING: {}", theprocess.len());
    std::thread::sleep(std::time::Duration::from_secs(5));
    system.refresh_processes();
    let theprocess = system.process_by_name("notepad");
    println!("IS RUNNING: {}", theprocess.len());
}
