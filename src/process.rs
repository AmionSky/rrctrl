use sysinfo::{RefreshKind, System, SystemExt};

pub struct ProcessChecker {
    system: System,
    checklist: Vec<String>,
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
