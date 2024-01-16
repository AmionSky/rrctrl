use tray_indicator::{MenuItem, Tray, TrayError};

pub fn show() -> Result<(), TrayError> {
    let mut tray = Tray::new(0x183BB6D6236646B4B69100E8F815DCCF, "Refresh-Rate Control");
    // tray.set_click(super::open_settings);
    tray.set_menu(vec![
        // MenuItem::button("Settings", super::open_settings),
        // MenuItem::separator(),
        MenuItem::button("Quit", Tray::exit),
    ]);

    tray.display()
}
