fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        const ICON: &str = "../../assets/application.ico";

        let mut res = tauri_winres::WindowsResource::new();
        res.set_icon(ICON);
        res.compile().unwrap();

        println!("cargo:rerun-if-changed={ICON}");
    }
}
