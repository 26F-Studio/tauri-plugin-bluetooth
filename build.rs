const COMMANDS: &[&str] = &["ping", "get_availability", "request_device"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
