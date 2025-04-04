use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let ui_path = Path::new("assets/ui");
    let main_ui = ui_path.join("main.xml");

    let profile = env::var("PROFILE").unwrap();

    let dest_file = Path::new(&format!("target/{}/", profile)).join("asssets/ui/main.ui");

    fs::create_dir_all(Path::new(dest_file.parent().unwrap())).expect("Failed to create directories for UI file");

    fs::copy(&main_ui, dest_file).expect("Failed to copy UI file");

    println!("cargo:out-dir={}", &std::env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed={}", main_ui.to_str().unwrap());
}